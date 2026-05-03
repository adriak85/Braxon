/*
 * gguf_ingress_c_v2.c  —  BRAXON GGUF tensor manifest generator
 *
 * Reads a GGUF model file, parses its header, and emits a TSV manifest
 * with one row per tensor: position, name, type, dims, offset, span,
 * FNV-1a hash, and 16-byte hex preview of the tensor data.
 *
 * Crash-safe: every tensor row is fsynced to disk and a checkpoint file
 * is atomically updated after each row.  On restart, the checkpoint is
 * read and the manifest is appended from where it left off.
 *
 * Build (Termux / Android ARM64):
 *   cc -O2 -std=c11 -D_FILE_OFFSET_BITS=64 -Wall -Wextra \
 *      -o gguf_ingress_c_v2 gguf_ingress_c_v2.c
 *
 * Usage:
 *   ./gguf_ingress_c_v2 \
 *       --input     MODEL.gguf          \
 *       --out-manifest  manifest.tsv    \
 *       --checkpoint    manifest.ckpt   \
 *       --summary       manifest.sum    \
 *     [ --sample-bytes  4096 ]          \
 *     [ --max-tensors   N    ]          \
 *     [ --report-every  100  ]
 */

#define _FILE_OFFSET_BITS 64
#include <errno.h>
#include <fcntl.h>
#include <inttypes.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <sys/types.h>
#include <unistd.h>

/* ── constants ─────────────────────────────────────────────────── */
#define GGUF_MAGIC             "GGUF"
#define GGUF_MAX_DIMS          8
#define GGUF_DEFAULT_ALIGNMENT 32ULL
#define GGUF_MAX_STRING        (64ULL * 1024ULL * 1024ULL)
#define SAMPLE_BYTES_MAX       (16ULL * 1024ULL * 1024ULL)
#define CHECKPOINT_TMP_SUFFIX  ".tmp"

/* ── types ─────────────────────────────────────────────────────── */
typedef struct {
    char    *name;
    uint32_t n_dims;
    uint64_t dims[GGUF_MAX_DIMS];
    uint32_t ggml_type;
    uint64_t rel_offset;
    uint64_t abs_offset;
    uint64_t span_size;
    uint32_t original_index;
} tensor_info_t;

typedef struct {
    const char *input_path;
    const char *manifest_path;
    const char *checkpoint_path;
    const char *summary_path;
    uint64_t    sample_bytes;
    uint64_t    max_tensors;
    uint64_t    report_every;
} options_t;

/* ── fatal error ───────────────────────────────────────────────── */
static void die(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fputc('\n', stderr);
    exit(1);
}

/* ── allocation helpers ────────────────────────────────────────── */
static void *xcalloc(size_t n, size_t sz) {
    void *p = calloc(n, sz);
    if (!p) die("calloc(%zu, %zu) failed", n, sz);
    return p;
}

/* ── math ──────────────────────────────────────────────────────── */
static uint64_t align_up(uint64_t v, uint64_t a) {
    if (a == 0) return v;
    uint64_t r = v % a;
    return r ? (v + (a - r)) : v;
}

/* ── I/O primitives ────────────────────────────────────────────── */
static void read_exact(FILE *f, void *buf, size_t n) {
    if (n == 0) return;
    if (fread(buf, 1, n, f) != n)
        die("short read: %s", strerror(errno));
}

static uint32_t read_u32le(FILE *f) {
    uint8_t b[4]; read_exact(f, b, 4);
    return (uint32_t)b[0] | ((uint32_t)b[1]<<8)
         | ((uint32_t)b[2]<<16) | ((uint32_t)b[3]<<24);
}

static uint64_t read_u64le(FILE *f) {
    uint8_t b[8]; read_exact(f, b, 8);
    return (uint64_t)b[0] | ((uint64_t)b[1]<<8)  | ((uint64_t)b[2]<<16)
         | ((uint64_t)b[3]<<24)| ((uint64_t)b[4]<<32)| ((uint64_t)b[5]<<40)
         | ((uint64_t)b[6]<<48)| ((uint64_t)b[7]<<56);
}

static uint64_t ftell_u64(FILE *f) {
    off_t o = ftello(f);
    if (o < 0) die("ftello: %s", strerror(errno));
    return (uint64_t)o;
}

static void fseek_u64(FILE *f, uint64_t off) {
    if (fseeko(f, (off_t)off, SEEK_SET) != 0)
        die("fseeko(%"PRIu64"): %s", off, strerror(errno));
}

static void fskip(FILE *f, uint64_t n) {
    uint64_t cur = ftell_u64(f);
    if (UINT64_MAX - cur < n) die("offset overflow in fskip");
    fseek_u64(f, cur + n);
}

static uint64_t fsize(const char *path) {
    struct stat st;
    if (stat(path, &st) != 0)
        die("stat(%s): %s", path, strerror(errno));
    return (uint64_t)st.st_size;
}

static void fsync_stream(FILE *fp) {
    fflush(fp);
    int fd = fileno(fp);
    if (fd >= 0) fsync(fd);
}

/* ── GGUF string ────────────────────────────────────────────────── */
static char *read_gguf_str(FILE *f) {
    uint64_t len = read_u64le(f);
    if (len > GGUF_MAX_STRING)
        die("GGUF string too large: %"PRIu64, len);
    char *s = malloc((size_t)len + 1);
    if (!s) die("malloc failed for string len=%"PRIu64, len);
    read_exact(f, s, (size_t)len);
    s[len] = '\0';
    return s;
}

/* ── GGUF metadata value skipping ───────────────────────────────── */
static size_t prim_size(uint32_t t) {
    switch (t) {
        case  0: case  1: case  7: return 1;
        case  2: case  3:          return 2;
        case  4: case  5: case  6: return 4;
        case 10: case 11: case 12: return 8;
        default: return 0;
    }
}

static void skip_value(FILE *f, uint32_t vtype, int depth) {
    if (depth > 16) die("metadata nesting too deep");
    if (vtype == 8) { char *s = read_gguf_str(f); free(s); return; }
    if (vtype == 9) {
        uint32_t et = read_u32le(f);
        uint64_t cnt = read_u64le(f);
        size_t ps = prim_size(et);
        if (ps) { fskip(f, cnt * (uint64_t)ps); return; }
        for (uint64_t k = 0; k < cnt; k++) skip_value(f, et, depth+1);
        return;
    }
    size_t ps = prim_size(vtype);
    if (!ps) die("unsupported metadata type: %u", vtype);
    fskip(f, (uint64_t)ps);
}

/* ── tensor sort ────────────────────────────────────────────────── */
static int cmp_abs_offset(const void *a, const void *b) {
    const tensor_info_t *ta = a, *tb = b;
    return (ta->abs_offset > tb->abs_offset) - (ta->abs_offset < tb->abs_offset);
}

/* ── FNV-1a 64-bit hash ─────────────────────────────────────────── */
static uint64_t fnv1a64(const uint8_t *buf, size_t n) {
    uint64_t h = 14695981039346656037ULL;
    for (size_t i = 0; i < n; i++) {
        h ^= (uint64_t)buf[i];
        h *= 1099511628211ULL;
    }
    return h;
}

/* ── hex preview ────────────────────────────────────────────────── */
static void hex16(const uint8_t *buf, size_t n, char out[33]) {
    static const char HEX[] = "0123456789abcdef";
    size_t m = (n < 16) ? n : 16;
    for (size_t i = 0; i < m; i++) {
        out[i*2]   = HEX[(buf[i]>>4)&0xF];
        out[i*2+1] = HEX[buf[i]&0xF];
    }
    for (size_t i = m*2; i < 32; i++) out[i] = '0';
    out[32] = '\0';
}

/* ── dims to csv ────────────────────────────────────────────────── */
static void dims_csv(const tensor_info_t *t, char *buf, size_t cap) {
    buf[0] = '\0';
    for (uint32_t i = 0; i < t->n_dims; i++) {
        char tmp[32];
        snprintf(tmp, sizeof(tmp), "%"PRIu64, t->dims[i]);
        if (i) strncat(buf, "x", cap - strlen(buf) - 1);
        strncat(buf, tmp, cap - strlen(buf) - 1);
    }
}

/* ── checkpoint ─────────────────────────────────────────────────── */
static uint64_t ckpt_load(const char *path) {
    FILE *f = fopen(path, "rb");
    if (!f) return 0;
    char line[256];
    uint64_t next = 0;
    while (fgets(line, sizeof(line), f))
        if (strncmp(line, "next_index=", 11) == 0)
            next = strtoull(line+11, NULL, 10);
    fclose(f);
    return next;
}

static void ckpt_write(const char *path, uint64_t next, uint64_t total) {
    size_t n = strlen(path) + sizeof(CHECKPOINT_TMP_SUFFIX) + 1;
    char *tmp = malloc(n);
    if (!tmp) die("malloc failed for checkpoint path");
    snprintf(tmp, n, "%s%s", path, CHECKPOINT_TMP_SUFFIX);

    FILE *f = fopen(tmp, "wb");
    if (!f) { free(tmp); die("open checkpoint tmp: %s", strerror(errno)); }
    fprintf(f, "next_index=%"PRIu64"\n", next);
    fprintf(f, "total_tensors=%"PRIu64"\n", total);
    fsync_stream(f);
    fclose(f);

    if (rename(tmp, path) != 0) {
        free(tmp);
        die("rename checkpoint: %s", strerror(errno));
    }
    free(tmp);
}

/* ── summary ────────────────────────────────────────────────────── */
static void write_summary(
    const char *path, const char *input,
    uint32_t version, uint64_t tc, uint64_t mkv,
    uint64_t align, uint64_t fsz, uint64_t tds
) {
    FILE *f = fopen(path, "wb");
    if (!f) die("open summary: %s", strerror(errno));
    fprintf(f, "input=%s\n",               input);
    fprintf(f, "version=%u\n",             version);
    fprintf(f, "tensor_count=%"PRIu64"\n", tc);
    fprintf(f, "metadata_kv_count=%"PRIu64"\n", mkv);
    fprintf(f, "alignment=%"PRIu64"\n",    align);
    fprintf(f, "file_size=%"PRIu64"\n",    fsz);
    fprintf(f, "tensor_data_start=%"PRIu64"\n", tds);
    fsync_stream(f);
    fclose(f);
}

/* ── monotonic ms ───────────────────────────────────────────────── */
static uint64_t now_ms(void) {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return (uint64_t)tv.tv_sec * 1000ULL + (uint64_t)tv.tv_usec / 1000ULL;
}

/* ── argument parsing ───────────────────────────────────────────── */
static void parse_args(int argc, char **argv, options_t *opt) {
    memset(opt, 0, sizeof(*opt));
    opt->sample_bytes = 4096;
    opt->max_tensors  = UINT64_MAX;
    opt->report_every = 100;

    for (int i = 1; i < argc; i++) {
        if      (!strcmp(argv[i],"--input")        && i+1<argc) opt->input_path    = argv[++i];
        else if (!strcmp(argv[i],"--out-manifest")  && i+1<argc) opt->manifest_path = argv[++i];
        else if (!strcmp(argv[i],"--checkpoint")    && i+1<argc) opt->checkpoint_path= argv[++i];
        else if (!strcmp(argv[i],"--summary")       && i+1<argc) opt->summary_path  = argv[++i];
        else if (!strcmp(argv[i],"--sample-bytes")  && i+1<argc) opt->sample_bytes  = strtoull(argv[++i],NULL,10);
        else if (!strcmp(argv[i],"--max-tensors")   && i+1<argc) opt->max_tensors   = strtoull(argv[++i],NULL,10);
        else if (!strcmp(argv[i],"--report-every")  && i+1<argc) opt->report_every  = strtoull(argv[++i],NULL,10);
        else die("unknown or incomplete arg: %s", argv[i]);
    }

    if (!opt->input_path)     die("missing --input");
    if (!opt->manifest_path)  die("missing --out-manifest");
    if (!opt->checkpoint_path)die("missing --checkpoint");
    if (!opt->summary_path)   die("missing --summary");
    if (opt->sample_bytes > SAMPLE_BYTES_MAX)
        die("--sample-bytes %"PRIu64" exceeds phone-safe limit %"PRIu64,
            opt->sample_bytes, SAMPLE_BYTES_MAX);
}

/* ── main ───────────────────────────────────────────────────────── */
int main(int argc, char **argv) {
    options_t opt;
    parse_args(argc, argv, &opt);

    uint64_t file_sz = fsize(opt.input_path);

    FILE *f = fopen(opt.input_path, "rb");
    if (!f) die("open input: %s", strerror(errno));

    /* magic */
    uint8_t magic[4]; read_exact(f, magic, 4);
    if (memcmp(magic, GGUF_MAGIC, 4) != 0) { fclose(f); die("not a GGUF file"); }

    uint32_t version        = read_u32le(f);
    uint64_t tensor_count   = read_u64le(f);
    uint64_t metadata_count = read_u64le(f);
    uint64_t alignment      = GGUF_DEFAULT_ALIGNMENT;

    /* skip metadata — capture alignment */
    for (uint64_t i = 0; i < metadata_count; i++) {
        char    *key   = read_gguf_str(f);
        uint32_t vtype = read_u32le(f);
        if (!strcmp(key, "general.alignment")) {
            if      (vtype == 4)  alignment = (uint64_t)read_u32le(f);
            else if (vtype == 10) alignment = read_u64le(f);
            else                  skip_value(f, vtype, 0);
        } else {
            skip_value(f, vtype, 0);
        }
        free(key);
    }
    if (alignment == 0) alignment = GGUF_DEFAULT_ALIGNMENT;

    /* tensor header table */
    tensor_info_t *tensors = xcalloc((size_t)tensor_count, sizeof(tensor_info_t));

    for (uint64_t i = 0; i < tensor_count; i++) {
        tensors[i].name     = read_gguf_str(f);
        tensors[i].n_dims   = read_u32le(f);
        if (tensors[i].n_dims > GGUF_MAX_DIMS) {
            fclose(f);
            die("tensor[%"PRIu64"] has %u dims (max %d): %s",
                i, tensors[i].n_dims, GGUF_MAX_DIMS, tensors[i].name);
        }
        for (uint32_t d = 0; d < tensors[i].n_dims; d++)
            tensors[i].dims[d] = read_u64le(f);
        tensors[i].ggml_type       = read_u32le(f);
        tensors[i].rel_offset      = read_u64le(f);
        tensors[i].original_index  = (uint32_t)i;
    }

    /* compute absolute offsets */
    uint64_t info_end   = ftell_u64(f);
    uint64_t data_start = align_up(info_end, alignment);

    for (uint64_t i = 0; i < tensor_count; i++) {
        if (UINT64_MAX - data_start < tensors[i].rel_offset) {
            fclose(f); die("abs offset overflow: %s", tensors[i].name);
        }
        tensors[i].abs_offset = data_start + tensors[i].rel_offset;
        if (tensors[i].abs_offset > file_sz) {
            fclose(f);
            die("tensor offset beyond file: %s (off=%"PRIu64" fsz=%"PRIu64")",
                tensors[i].name, tensors[i].abs_offset, file_sz);
        }
    }

    /* sort by offset; compute spans */
    qsort(tensors, (size_t)tensor_count, sizeof(tensor_info_t), cmp_abs_offset);

    for (uint64_t i = 0; i < tensor_count; i++) {
        uint64_t next = (i+1 < tensor_count) ? tensors[i+1].abs_offset : file_sz;
        if (next < tensors[i].abs_offset) { fclose(f); die("offsets not monotone"); }
        tensors[i].span_size = next - tensors[i].abs_offset;
    }

    /* summary always written fresh */
    write_summary(opt.summary_path, opt.input_path,
                  version, tensor_count, metadata_count,
                  alignment, file_sz, data_start);

    /* checkpoint / resume */
    uint64_t next_idx = ckpt_load(opt.checkpoint_path);
    if (next_idx > tensor_count) next_idx = 0;

    bool append = (next_idx > 0);
    FILE *mf = fopen(opt.manifest_path, append ? "ab" : "wb");
    if (!mf) { fclose(f); die("open manifest: %s", strerror(errno)); }

    if (!append) {
        fprintf(mf,
            "sorted_index\toriginal_index\tname\tggml_type\t"
            "n_dims\tdims\tabs_offset\tspan_size\t"
            "sample_n\tfnv1a64\tpreview16\n");
        fsync_stream(mf);
    }

    /* sample buffer */
    uint8_t *sbuf = NULL;
    if (opt.sample_bytes > 0) {
        sbuf = malloc((size_t)opt.sample_bytes);
        if (!sbuf) die("malloc sample buffer (%"PRIu64" bytes)", opt.sample_bytes);
    }

    uint64_t stop = tensor_count;
    if (opt.max_tensors != UINT64_MAX &&
        next_idx + opt.max_tensors < stop)
        stop = next_idx + opt.max_tensors;

    uint64_t t0 = now_ms();
    uint64_t done_this_run = 0;

    for (uint64_t i = next_idx; i < stop; i++) {
        tensor_info_t *t = &tensors[i];

        /* sample */
        size_t   sn   = 0;
        uint64_t hash = 0;
        char     prev[33];
        memset(prev, '0', 32); prev[32] = '\0';

        if (sbuf && t->span_size > 0) {
            uint64_t want = (t->span_size < opt.sample_bytes)
                             ? t->span_size : opt.sample_bytes;
            sn = (size_t)want;
            fseek_u64(f, t->abs_offset);
            read_exact(f, sbuf, sn);
            hash = fnv1a64(sbuf, sn);
            hex16(sbuf, sn, prev);
        }

        char dc[256]; dims_csv(t, dc, sizeof(dc));

        fprintf(mf,
            "%"PRIu64"\t%u\t%s\t%u\t%u\t%s\t"
            "%"PRIu64"\t%"PRIu64"\t%zu\t%016"PRIx64"\t%s\n",
            i, t->original_index, t->name, t->ggml_type,
            t->n_dims, dc,
            t->abs_offset, t->span_size,
            sn, hash, prev);

        fsync_stream(mf);
        ckpt_write(opt.checkpoint_path, i+1, tensor_count);

        done_this_run++;

        /* progress report */
        bool report = (opt.report_every > 0 && done_this_run % opt.report_every == 0)
                   || (i+1 == stop);
        if (report) {
            uint64_t elapsed = now_ms() - t0;
            double rate = elapsed > 0
                ? (double)done_this_run / ((double)elapsed / 1000.0)
                : 0.0;
            fprintf(stderr,
                "[%"PRIu64"/%"PRIu64"] %-40s  %.1f t/s\n",
                i+1, tensor_count, t->name, rate);
        } else {
            fprintf(stderr,
                "processed %"PRIu64"/%"PRIu64": %s\n",
                i+1, tensor_count, t->name);
        }
    }

    fclose(mf);
    fclose(f);
    for (uint64_t i = 0; i < tensor_count; i++) free(tensors[i].name);
    free(tensors);
    free(sbuf);

    fprintf(stderr, "done: %"PRIu64" tensors processed this run\n", done_this_run);
    return 0;
}
