#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <time.h>

typedef struct {
    uint16_t symbol;
    uint16_t macro_id;
    uint8_t a;
    uint8_t b;
    uint32_t pos;
    uint16_t amp;
} NoiseRec;

typedef struct {
    uint16_t subject;
    uint16_t relation;
    uint16_t object;
    uint8_t layer;
    uint8_t plane;
    uint32_t anchor_delta;
    uint16_t weight;
    uint8_t flags;
} TripleRec;

typedef struct {
    uint16_t cell;
    uint16_t state;
    uint16_t flux;
    uint8_t gate;
    uint8_t phase;
} MembraneRec;

static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1e6;
}

static uint64_t fnv1a64(const void *data, size_t len) {
    const unsigned char *p = (const unsigned char *)data;
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < len; i++) {
        h ^= (uint64_t)p[i];
        h *= 1099511628211ULL;
    }
    return h;
}

int main(int argc, char **argv) {
    if (argc != 3) {
        fprintf(stderr, "usage: nsq_c_bench <scale> <out.json>\n");
        return 2;
    }

    size_t scale = (size_t)strtoull(argv[1], NULL, 10);
    const char *out_path = argv[2];

    double t0 = now_ms();

    NoiseRec *noise = (NoiseRec *)calloc(scale, sizeof(NoiseRec));
    TripleRec *triple = (TripleRec *)calloc(scale, sizeof(TripleRec));
    MembraneRec *mem = (MembraneRec *)calloc(scale, sizeof(MembraneRec));
    if (!noise || !triple || !mem) {
        fprintf(stderr, "alloc failed\n");
        return 3;
    }

    for (size_t i = 0; i < scale; i++) {
        noise[i].symbol = (uint16_t)(i % 257);
        noise[i].macro_id = (uint16_t)(i % 64);
        noise[i].a = (uint8_t)(i % 255);
        noise[i].b = (uint8_t)((i * 7) % 255);
        noise[i].pos = (uint32_t)(i * 3);
        noise[i].amp = (uint16_t)(i % 1024);

        triple[i].subject = (uint16_t)(i % 257);
        triple[i].relation = (uint16_t)(i % 64);
        triple[i].object = (uint16_t)((i * 13) % 257);
        triple[i].layer = (uint8_t)(i % 26);
        triple[i].plane = (uint8_t)(i % 33);
        triple[i].anchor_delta = (uint32_t)(i * 5);
        triple[i].weight = (uint16_t)(i % 4096);
        triple[i].flags = (uint8_t)(i % 8);

        mem[i].cell = (uint16_t)(i % 257);
        mem[i].state = (uint16_t)((i * 11) % 257);
        mem[i].flux = (uint16_t)(i % 2048);
        mem[i].gate = (uint8_t)(i % 16);
        mem[i].phase = (uint8_t)(i % 16);
    }

    double t1 = now_ms();

    uint64_t h1 = fnv1a64(noise, scale * sizeof(NoiseRec));
    uint64_t h2 = fnv1a64(triple, scale * sizeof(TripleRec));
    uint64_t h3 = fnv1a64(mem, scale * sizeof(MembraneRec));

    double t2 = now_ms();

    size_t artifact_bytes =
        scale * sizeof(NoiseRec) +
        scale * sizeof(TripleRec) +
        scale * sizeof(MembraneRec);

    FILE *f = fopen(out_path, "w");
    if (!f) {
        fprintf(stderr, "open failed\n");
        return 4;
    }

    fprintf(f,
        "{\n"
        "  \"version\": 1,\n"
        "  \"system\": \"c\",\n"
        "  \"scale\": %zu,\n"
        "  \"alloc_fill_ms\": %.3f,\n"
        "  \"hash_ms\": %.3f,\n"
        "  \"artifact_bytes\": %zu,\n"
        "  \"noise_hash\": \"%016llx\",\n"
        "  \"triple_hash\": \"%016llx\",\n"
        "  \"membrane_hash\": \"%016llx\"\n"
        "}\n",
        scale,
        t1 - t0,
        t2 - t1,
        artifact_bytes,
        (unsigned long long)h1,
        (unsigned long long)h2,
        (unsigned long long)h3
    );
    fclose(f);

    free(noise);
    free(triple);
    free(mem);
    return 0;
}
