#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <dirent.h>
#include <sys/stat.h>

typedef struct {
    char **items;
    size_t len;
    size_t cap;
} StrVec;

static void sv_push(StrVec *v, const char *s) {
    for (size_t i = 0; i < v->len; i++) {
        if (strcmp(v->items[i], s) == 0) return;
    }
    if (v->len == v->cap) {
        v->cap = v->cap ? v->cap * 2 : 64;
        v->items = realloc(v->items, v->cap * sizeof(char *));
        if (!v->items) exit(2);
    }
    v->items[v->len++] = strdup(s);
}

static int ends_with(const char *s, const char *suf) {
    size_t a = strlen(s), b = strlen(suf);
    return a >= b && strcmp(s + a - b, suf) == 0;
}

static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1e6;
}

static char *read_all(const char *path, size_t *out_len) {
    FILE *f = fopen(path, "rb");
    if (!f) return NULL;
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    if (n < 0) { fclose(f); return NULL; }
    char *buf = malloc((size_t)n + 1);
    if (!buf) { fclose(f); return NULL; }
    size_t got = fread(buf, 1, (size_t)n, f);
    fclose(f);
    buf[got] = 0;
    *out_len = got;
    return buf;
}

static void scan_file(const char *path, StrVec *symbols, StrVec *imports, StrVec *calls,
                      StrVec *entrypoints, StrVec *findings, size_t *brief_lines, size_t *brief_bytes) {
    size_t n = 0;
    char *buf = read_all(path, &n);
    if (!buf) return;

    *brief_bytes += n;
    for (size_t i = 0; i < n; i++) if (buf[i] == '\n') (*brief_lines)++;

    char *save = NULL;
    char *line = strtok_r(buf, "\n", &save);
    char current_mod[128] = {0};

    while (line) {
        if (strncmp(line, "#include \"", 10) == 0) {
            char dep[128] = {0};
            sscanf(line, "#include \"%127[^\"]", dep);
            char item[256];
            snprintf(item, sizeof(item), "%s", dep);
            sv_push(imports, item);
        }

        if (strncmp(line, "typedef struct ", 15) == 0) {
            char sym[128] = {0};
            sscanf(line, "typedef struct %127s", sym);
            sv_push(symbols, sym);
        }

        if (strncmp(line, "int ", 4) == 0 || strncmp(line, "static int ", 11) == 0) {
            const char *p = strstr(line, "int ");
            if (p) {
                p += 4;
                char fn[128] = {0};
                size_t k = 0;
                while (*p && *p != '(' && *p != ' ' && k < sizeof(fn) - 1) fn[k++] = *p++;
                fn[k] = 0;
                if (fn[0]) {
                    sv_push(symbols, fn);
                    if (strstr(fn, "_handle")) sv_push(entrypoints, fn);
                }
            }
        }

        char *c = strstr(line, "call_");
        if (c) sv_push(calls, line);

        if (strstr(line, "_flush(") || strstr(line, "_validate(") || strstr(line, "_handle(")) {
            sv_push(calls, line);
        }

        if (strstr(line, "fallback_bypass") || strstr(line, "nullable admin override") ||
            strstr(line, "CORRUPT") || strstr(line, "bypass")) {
            sv_push(findings, line);
        }

        line = strtok_r(NULL, "\n", &save);
    }

    free(buf);
}

static void walk(const char *dir, StrVec *files) {
    DIR *d = opendir(dir);
    if (!d) return;
    struct dirent *ent;
    while ((ent = readdir(d))) {
        if (strcmp(ent->d_name, ".") == 0 || strcmp(ent->d_name, "..") == 0) continue;
        char path[4096];
        snprintf(path, sizeof(path), "%s/%s", dir, ent->d_name);
        struct stat st;
        if (stat(path, &st) != 0) continue;
        if (S_ISDIR(st.st_mode)) {
            walk(path, files);
        } else {
            if (ends_with(path, ".c") || ends_with(path, ".h") || ends_with(path, ".json") ||
                ends_with(path, ".md") || ends_with(path, ".sql") || ends_with(path, ".log") ||
                ends_with(path, ".diff") || ends_with(path, ".code")) {
                sv_push(files, path);
            }
        }
    }
    closedir(d);
}

static unsigned long long hash_mix(const StrVec *a, const StrVec *b, const StrVec *c) {
    unsigned long long h = 1469598103934665603ULL;
    const unsigned long long prime = 1099511628211ULL;
    for (size_t pass = 0; pass < 3; pass++) {
        const StrVec *v = pass == 0 ? a : (pass == 1 ? b : c);
        for (size_t i = 0; i < v->len; i++) {
            const unsigned char *p = (const unsigned char *)v->items[i];
            while (*p) {
                h ^= (unsigned long long)(*p++);
                h *= prime;
            }
        }
    }
    return h;
}

static void emit_json_array(FILE *f, const char *name, const StrVec *v) {
    fprintf(f, "\"%s\":[", name);
    for (size_t i = 0; i < v->len; i++) {
        if (i) fprintf(f, ",");
        fprintf(f, "%s\"%s\"", (i ? "" : ""), v->items[i]);
    }
    fprintf(f, "]");
}

int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "usage: repo_reality_c <corpus_dir> <out.json>\n");
        return 2;
    }

    const char *corpus_dir = argv[1];
    const char *out_path = argv[2];

    StrVec files = {0}, symbols = {0}, imports = {0}, calls = {0}, entrypoints = {0}, findings = {0};
    size_t brief_lines = 0, brief_bytes = 0;

    double t0 = now_ms();
    walk(corpus_dir, &files);
    for (size_t i = 0; i < files.len; i++) {
        scan_file(files.items[i], &symbols, &imports, &calls, &entrypoints, &findings, &brief_lines, &brief_bytes);
    }
    double t1 = now_ms();

    unsigned long long h = hash_mix(&symbols, &imports, &calls);

    FILE *f = fopen(out_path, "w");
    if (!f) return 3;

    fprintf(f, "{");
    fprintf(f, "\"system\":\"c\",");
    fprintf(f, "\"elapsed_ms\":%.3f,", t1 - t0);

    emit_json_array(f, "symbols", &symbols); fprintf(f, ",");
    emit_json_array(f, "imports", &imports); fprintf(f, ",");
    emit_json_array(f, "calls", &calls); fprintf(f, ",");
    emit_json_array(f, "entrypoints", &entrypoints); fprintf(f, ",");
    emit_json_array(f, "findings", &findings); fprintf(f, ",");

    fprintf(f, "\"briefing\":{");
    fprintf(f, "\"readable_output_bytes\":%zu,", brief_bytes);
    fprintf(f, "\"readable_output_lines\":%zu", brief_lines);
    fprintf(f, "},");

    fprintf(f, "\"replay_hash\":\"%016llx\"", h);
    fprintf(f, "}\n");
    fclose(f);
    return 0;
}
