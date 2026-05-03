#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

static size_t count_tokens(const char *s) {
    size_t count = 0;
    int in_tok = 0;
    while (*s) {
        if (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') {
            in_tok = 0;
        } else if (!in_tok) {
            in_tok = 1;
            count++;
        }
        s++;
    }
    return count;
}

static int cmd_parse(int argc, char **argv) {
    if (argc < 3) {
        fprintf(stderr, "usage: c-compare parse <text...>\n");
        return 2;
    }

    size_t total_len = 0;
    for (int i = 2; i < argc; i++) total_len += strlen(argv[i]) + 1;

    char *buf = malloc(total_len + 1);
    if (!buf) {
        fprintf(stderr, "alloc failed\n");
        return 2;
    }
    buf[0] = '\0';

    for (int i = 2; i < argc; i++) {
        strcat(buf, argv[i]);
        if (i + 1 < argc) strcat(buf, " ");
    }

    printf("C parse\n");
    printf("input: %s\n", buf);
    printf("token_count: %zu\n", count_tokens(buf));

    free(buf);
    return 0;
}

static int cmd_ingest(const char *path) {
    struct stat st;
    if (stat(path, &st) != 0) {
        perror("stat");
        return 2;
    }

    printf("C ingest\n");
    printf("path: %s\n", path);
    if (S_ISDIR(st.st_mode)) {
        printf("type: directory\n");
    } else if (S_ISREG(st.st_mode)) {
        printf("type: file\n");
        printf("bytes: %lld\n", (long long)st.st_size);
    } else {
        printf("type: other\n");
    }

    return 0;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: c-compare <parse|ingest> ...\n");
        return 2;
    }

    if (strcmp(argv[1], "parse") == 0) {
        return cmd_parse(argc, argv);
    }
    if (strcmp(argv[1], "ingest") == 0) {
        if (argc != 3) {
            fprintf(stderr, "usage: c-compare ingest <path>\n");
            return 2;
        }
        return cmd_ingest(argv[2]);
    }

    fprintf(stderr, "unknown command: %s\n", argv[1]);
    return 2;
}
