#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>

static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1e6;
}

int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: c_info_bench <family> <scale> <out.txt>\n");
        return 2;
    }

    const char *family = argv[1];
    size_t scale = (size_t)strtoull(argv[2], NULL, 10);
    const char *out_path = argv[3];

    double t0 = now_ms();

    FILE *f = fopen(out_path, "w");
    if (!f) return 3;

    size_t lines = 0;
    for (size_t i = 0; i < scale; i++) {
        fprintf(f, "family=%s idx=%zu a=%zu b=%zu c=%zu\n",
                family, i, i % 257, (i * 7) % 257, (i * 13) % 257);
        lines++;
    }

    fclose(f);

    double t1 = now_ms();

    printf("{\"system\":\"c\",\"family\":\"%s\",\"scale\":%zu,\"elapsed_ms\":%.3f,\"lines\":%zu}\n",
           family, scale, t1 - t0, lines);

    return 0;
}
