#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

#define MODE_NOISE 1
#define MODE_STRUCT 2

static const char *symbols[] = {
    "wake","self","semantic","address","bit","lattice","graph","edge","node","delta",
    "macro","switch","lever","cell","membrane","matrix","pulse","noise","signal","field",
    "anchor","merge","resolve","route","align","stack","core","shard","vector","plane",
    "exception","socio","psych","thread","bridge","gate","state","flux","prime","sovereign"
};
static const char *macros_[] = {
    "wake:self",
    "semantic:address",
    "bit:lattice",
    "graph:edge:node",
    "macro:switch:lever",
    "cell:membrane:matrix",
    "noise:signal:field",
    "delta:route:align",
    "exception:socio:psych",
    "prime:flux:sovereign"
};

static uint64_t rng_state = 0x435f5052494d4555ULL;

static uint64_t next_u64(void) {
    uint64_t x = rng_state;
    x ^= x << 7;
    x ^= x >> 9;
    x ^= x << 8;
    rng_state = x;
    return x;
}

static uint64_t hash64(const unsigned char *bytes, size_t len) {
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < len; i++) {
        h ^= (uint64_t)bytes[i];
        h *= 1099511628211ULL;
    }
    return h;
}

static void put_u16(FILE *f, uint16_t v) { fwrite(&v, 2, 1, f); }
static void put_u32(FILE *f, uint32_t v) { fwrite(&v, 4, 1, f); }
static void put_u64(FILE *f, uint64_t v) { fwrite(&v, 8, 1, f); }

static int get_u16(const unsigned char *buf, size_t len, size_t *off, uint16_t *out) {
    if (*off + 2 > len) return 0;
    *out = (uint16_t)(buf[*off] | (buf[*off+1] << 8));
    *off += 2;
    return 1;
}
static int get_u32(const unsigned char *buf, size_t len, size_t *off, uint32_t *out) {
    if (*off + 4 > len) return 0;
    *out = (uint32_t)buf[*off] |
           ((uint32_t)buf[*off+1] << 8) |
           ((uint32_t)buf[*off+2] << 16) |
           ((uint32_t)buf[*off+3] << 24);
    *off += 4;
    return 1;
}
static int get_u64(const unsigned char *buf, size_t len, size_t *off, uint64_t *out) {
    if (*off + 8 > len) return 0;
    uint64_t v = 0;
    for (int i = 0; i < 8; i++) v |= ((uint64_t)buf[*off+i]) << (8*i);
    *off += 8;
    *out = v;
    return 1;
}

static void write_header(FILE *f, uint8_t mode, uint64_t secs) {
    fwrite("CPRM0001", 8, 1, f);
    fwrite(&mode, 1, 1, f);
    put_u64(f, secs);
    put_u16(f, (uint16_t)(sizeof(symbols)/sizeof(symbols[0])));
    put_u16(f, (uint16_t)(sizeof(macros_)/sizeof(macros_[0])));
    for (size_t i = 0; i < sizeof(symbols)/sizeof(symbols[0]); i++) {
        uint16_t n = (uint16_t)strlen(symbols[i]);
        put_u16(f, n);
        fwrite(symbols[i], 1, n, f);
    }
    for (size_t i = 0; i < sizeof(macros_)/sizeof(macros_[0]); i++) {
        uint16_t n = (uint16_t)strlen(macros_[i]);
        put_u16(f, n);
        fwrite(macros_[i], 1, n, f);
    }
}

static int write_noise(const char *out_path, uint64_t secs) {
    FILE *f = fopen(out_path, "wb");
    if (!f) return 2;
    write_header(f, MODE_NOISE, secs);

    time_t end = time(NULL) + (time_t)secs;
    uint16_t prev = 0;
    while (time(NULL) < end) {
        uint16_t sym = (uint16_t)(next_u64() % (sizeof(symbols)/sizeof(symbols[0])));
        uint16_t dprev = (uint16_t)(sym - prev);
        prev = sym;
        uint16_t macro_id = (uint16_t)(next_u64() % (sizeof(macros_)/sizeof(macros_[0])));
        uint8_t a = (uint8_t)(next_u64() & 0x3f);
        uint8_t b = (uint8_t)(next_u64() & 0x3f);
        uint16_t switches = (uint16_t)(((uint16_t)a << 6) | b);
        uint32_t pos = (uint32_t)(next_u64() & 0x00ffffff);
        uint16_t amp = (uint16_t)(next_u64() & 0xffff);

        put_u16(f, sym);
        put_u16(f, dprev);
        put_u16(f, macro_id);
        put_u16(f, switches);
        put_u32(f, pos);
        put_u16(f, amp);
    }

    fclose(f);
    return 0;
}

static int write_structured(const char *out_path, uint64_t secs) {
    FILE *f = fopen(out_path, "wb");
    if (!f) return 2;
    write_header(f, MODE_STRUCT, secs);

    time_t end = time(NULL) + (time_t)secs;
    uint32_t prev_anchor = 0;
    while (time(NULL) < end) {
        uint16_t subject = (uint16_t)(next_u64() % (sizeof(symbols)/sizeof(symbols[0])));
        uint16_t relation = (uint16_t)(next_u64() % (sizeof(macros_)/sizeof(macros_[0])));
        uint16_t object = (uint16_t)(next_u64() % (sizeof(symbols)/sizeof(symbols[0])));
        uint8_t layer = (uint8_t)(next_u64() & 0x1f);
        uint8_t plane = (uint8_t)(next_u64() & 0x1f);
        uint16_t packed = (uint16_t)(((uint16_t)layer << 5) | plane);
        uint32_t anchor = (uint32_t)(next_u64() & 0x00ffffff);
        uint32_t d_anchor = anchor - prev_anchor;
        prev_anchor = anchor;
        uint16_t weight = (uint16_t)(next_u64() & 0xffff);
        uint8_t flags = (uint8_t)(next_u64() & 0xff);

        put_u16(f, subject);
        put_u16(f, relation);
        put_u16(f, object);
        put_u16(f, packed);
        put_u32(f, d_anchor);
        put_u16(f, weight);
        fwrite(&flags, 1, 1, f);
    }

    fclose(f);
    return 0;
}

static char **read_string_table(const unsigned char *buf, size_t len, size_t *off, size_t count) {
    char **out = calloc(count, sizeof(char*));
    if (!out) return NULL;
    for (size_t i = 0; i < count; i++) {
        uint16_t n = 0;
        if (!get_u16(buf, len, off, &n)) return NULL;
        if (*off + n > len) return NULL;
        out[i] = calloc((size_t)n + 1, 1);
        memcpy(out[i], buf + *off, n);
        *off += n;
    }
    return out;
}

static int is_int_tok(const char *s) {
    if (*s == 0) return 0;
    while (*s) {
        if (*s < '0' || *s > '9') return 0;
        s++;
    }
    return 1;
}

static void score_and_decode(const char *native_in, const char *decoded_out, const char *score_out) {
    FILE *f = fopen(native_in, "rb");
    if (!f) exit(2);
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    rewind(f);

    unsigned char *buf = malloc((size_t)sz);
    fread(buf, 1, (size_t)sz, f);
    fclose(f);

    size_t off = 0;
    if ((size_t)sz < 9 || memcmp(buf, "CPRM0001", 8) != 0) exit(2);
    off += 8;
    uint8_t mode = buf[off++];
    uint64_t secs = 0;
    uint16_t sym_count = 0, macro_count = 0;
    get_u64(buf, (size_t)sz, &off, &secs);
    get_u16(buf, (size_t)sz, &off, &sym_count);
    get_u16(buf, (size_t)sz, &off, &macro_count);

    char **symtab = read_string_table(buf, (size_t)sz, &off, sym_count);
    char **mactab = read_string_table(buf, (size_t)sz, &off, macro_count);

    FILE *decoded = fopen(decoded_out, "wb");
    if (!decoded) exit(2);

    char **unique = NULL;
    size_t uniq_n = 0, uniq_cap = 0;
    size_t transitions = 0, records = 0;
    size_t class_int = 0, class_symbol = 0, class_mixed = 0;
    uint32_t anchor = 0;

    while (off < (size_t)sz) {
        if (mode == MODE_NOISE) {
            if (off + 14 > (size_t)sz) break;
            uint16_t sym, dprev, mid, switches, amp;
            uint32_t pos;
            get_u16(buf, (size_t)sz, &off, &sym);
            get_u16(buf, (size_t)sz, &off, &dprev);
            get_u16(buf, (size_t)sz, &off, &mid);
            get_u16(buf, (size_t)sz, &off, &switches);
            get_u32(buf, (size_t)sz, &off, &pos);
            get_u16(buf, (size_t)sz, &off, &amp);

            const char *s = (sym < sym_count) ? symtab[sym] : "<?>"; 
            const char *m = (mid < macro_count) ? mactab[mid] : "<?>";

            fprintf(decoded, "noise sym=%s macro=%s leverA=%u leverB=%u pos=%u amp=%u dprev=%u\n",
                    s, m, (switches >> 6) & 0x3f, switches & 0x3f, pos, amp, dprev);

            if (records++) transitions++;
            if (is_int_tok(s)) class_int++; else class_symbol++;

            int found = 0;
            for (size_t i = 0; i < uniq_n; i++) if (strcmp(unique[i], s) == 0) { found = 1; break; }
            if (!found) {
                if (uniq_n == uniq_cap) {
                    uniq_cap = uniq_cap ? uniq_cap * 2 : 16;
                    unique = realloc(unique, uniq_cap * sizeof(char*));
                }
                unique[uniq_n++] = strdup(s);
            }
        } else {
            if (off + 13 > (size_t)sz) break;
            uint16_t subject, relation, object, packed, weight;
            uint32_t d_anchor;
            uint8_t flags;

            get_u16(buf, (size_t)sz, &off, &subject);
            get_u16(buf, (size_t)sz, &off, &relation);
            get_u16(buf, (size_t)sz, &off, &object);
            get_u16(buf, (size_t)sz, &off, &packed);
            get_u32(buf, (size_t)sz, &off, &d_anchor);
            get_u16(buf, (size_t)sz, &off, &weight);
            flags = buf[off++];

            anchor += d_anchor;

            const char *s = (subject < sym_count) ? symtab[subject] : "<?>";
            const char *r = (relation < macro_count) ? mactab[relation] : "<?>";
            const char *o = (object < sym_count) ? symtab[object] : "<?>";

            fprintf(decoded, "triple subject=%s relation=%s object=%s layer=%u plane=%u anchor=%u weight=%u flags=%u\n",
                    s, r, o, (packed >> 5) & 0x1f, packed & 0x1f, anchor, weight, flags);

            if (records++) transitions++;
            if (is_int_tok(o)) class_int++; else class_symbol += 2;

            const char *pair[2] = { s, o };
            for (int j = 0; j < 2; j++) {
                int found = 0;
                for (size_t i = 0; i < uniq_n; i++) if (strcmp(unique[i], pair[j]) == 0) { found = 1; break; }
                if (!found) {
                    if (uniq_n == uniq_cap) {
                        uniq_cap = uniq_cap ? uniq_cap * 2 : 16;
                        unique = realloc(unique, uniq_cap * sizeof(char*));
                    }
                    unique[uniq_n++] = strdup(pair[j]);
                }
            }
        }
    }

    fclose(decoded);

    f = fopen(decoded_out, "rb");
    fseek(f, 0, SEEK_END);
    long decoded_sz = ftell(f);
    rewind(f);

    size_t decoded_lines = 0;
    int ch;
    while ((ch = fgetc(f)) != EOF) if (ch == '\n') decoded_lines++;
    fclose(f);

    FILE *score = fopen(score_out, "wb");
    fprintf(score, "{\n");
    fprintf(score, "  \"lane\": \"%s\",\n", mode == MODE_NOISE ? "c-native-noise" : "c-native-structured");
    fprintf(score, "  \"duration_secs\": %llu,\n", (unsigned long long)secs);
    fprintf(score, "  \"native_bytes\": %ld,\n", sz);
    fprintf(score, "  \"decoded_bytes\": %ld,\n", decoded_sz);
    fprintf(score, "  \"decoded_lines\": %zu,\n", decoded_lines);
    fprintf(score, "  \"decoded_records\": %zu,\n", records);
    fprintf(score, "  \"unique_symbols\": %zu,\n", uniq_n);
    fprintf(score, "  \"transitions\": %zu,\n", transitions);
    fprintf(score, "  \"class_counts\": {\n");
    fprintf(score, "    \"int\": %zu,\n", class_int);
    fprintf(score, "    \"mixed\": %zu,\n", class_mixed);
    fprintf(score, "    \"symbol\": %zu\n", class_symbol);
    fprintf(score, "  },\n");
    fprintf(score, "  \"compression_ratio\": %.6f,\n", sz ? ((double)decoded_sz / (double)sz) : 0.0);
    fprintf(score, "  \"records_per_sec\": %.6f,\n", secs ? ((double)records / (double)secs) : 0.0);
    fprintf(score, "  \"decoded_bytes_per_sec\": %.6f,\n", secs ? ((double)decoded_sz / (double)secs) : 0.0);
    fprintf(score, "  \"native_sha_like\": \"%016llx\",\n", (unsigned long long)hash64(buf, (size_t)sz));
    fprintf(score, "  \"format_notes\": [\n");
    fprintf(score, "    \"native binary lane\",\n");
    fprintf(score, "    \"symbol table\",\n");
    fprintf(score, "    \"macro table\",\n");
    fprintf(score, "    \"compact packed records\",\n");
    fprintf(score, "    \"decoded human-readable export\"\n");
    fprintf(score, "  ]\n");
    fprintf(score, "}\n");
    fclose(score);

    for (size_t i = 0; i < uniq_n; i++) free(unique[i]);
    free(unique);
    for (size_t i = 0; i < sym_count; i++) free(symtab[i]);
    for (size_t i = 0; i < macro_count; i++) free(mactab[i]);
    free(symtab);
    free(mactab);
    free(buf);
}

static void usage(void) {
    fprintf(stderr, "usage:\n");
    fprintf(stderr, "  c-pressure-bench write-noise <seconds> <native_out>\n");
    fprintf(stderr, "  c-pressure-bench write-structured <seconds> <native_out>\n");
    fprintf(stderr, "  c-pressure-bench decode <native_in> <decoded_txt> <score_json>\n");
    exit(2);
}

int main(int argc, char **argv) {
    if (argc < 2) usage();
    if (strcmp(argv[1], "write-noise") == 0) {
        if (argc != 4) usage();
        return write_noise(argv[3], (uint64_t)strtoull(argv[2], NULL, 10));
    }
    if (strcmp(argv[1], "write-structured") == 0) {
        if (argc != 4) usage();
        return write_structured(argv[3], (uint64_t)strtoull(argv[2], NULL, 10));
    }
    if (strcmp(argv[1], "decode") == 0) {
        if (argc != 5) usage();
        score_and_decode(argv[2], argv[3], argv[4]);
        return 0;
    }
    usage();
    return 2;
}
