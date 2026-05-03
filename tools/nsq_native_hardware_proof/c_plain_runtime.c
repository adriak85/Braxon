#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <signal.h>
#include <time.h>
#include <string.h>

static volatile sig_atomic_t unwound = 0;

static void handle_fault(int sig) {
    (void)sig;
    unwound = 1;
}

static uint64_t mix64(uint64_t x) {
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    x ^= x >> 33;
    return x;
}

static double elapsed_sec(struct timespec a, struct timespec b) {
    return (double)(b.tv_sec - a.tv_sec) + (double)(b.tv_nsec - a.tv_nsec) / 1000000000.0;
}

int main(int argc, char **argv) {
    signal(SIGSEGV, handle_fault);
    signal(SIGBUS, handle_fault);
    signal(SIGILL, handle_fault);

    const char *out = argc > 1 ? argv[1] : "c_plain_report.json";
    uint64_t limit = argc > 2 ? strtoull(argv[2], NULL, 10) : 6900000000000ULL;
    uint64_t window = argc > 3 ? strtoull(argv[3], NULL, 10) : 65536ULL;

    FILE *f = fopen(out, "w");
    if (!f) return 2;

    struct timespec start, now;
    clock_gettime(CLOCK_MONOTONIC, &start);

    uint64_t tasks = 0;
    uint64_t parameter_windows = 0;
    uint64_t bit_ops = 0;
    uint64_t digest = 0;

    for (uint64_t base = 0; base < limit && !unwound; base += window) {
        for (uint64_t i = 0; i < window && base + i < limit; i++) {
            uint64_t x = base + i;
            digest ^= mix64(x ^ (tasks << 1));
            bit_ops += 8;
            tasks += 1;
            if ((tasks & 0xFFFFF) == 0) {
                clock_gettime(CLOCK_MONOTONIC, &now);
                if (elapsed_sec(start, now) > 9.85) goto done;
            }
        }
        parameter_windows += 1;
    }

done:
    clock_gettime(CLOCK_MONOTONIC, &now);
    double elapsed = elapsed_sec(start, now);

    fprintf(f, "{\n");
    fprintf(f, "  \"schema\": \"nsq.c_plain_runtime.report.v1\",\n");
    fprintf(f, "  \"surface\": \"c_plain\",\n");
    fprintf(f, "  \"runtime_language\": \"c\",\n");
    fprintf(f, "  \"separate_runtime\": true,\n");
    fprintf(f, "  \"parameter_range_total\": %llu,\n", (unsigned long long)limit);
    fprintf(f, "  \"parameter_windows_opened\": %llu,\n", (unsigned long long)parameter_windows);
    fprintf(f, "  \"tasks_completed\": %llu,\n", (unsigned long long)tasks);
    fprintf(f, "  \"bit_operations\": %llu,\n", (unsigned long long)bit_ops);
    fprintf(f, "  \"unwound_before_crash\": %s,\n", unwound ? "true" : "false");
    fprintf(f, "  \"elapsed_seconds\": %.6f,\n", elapsed);
    fprintf(f, "  \"digest_mix\": \"%016llx\",\n", (unsigned long long)digest);
    fprintf(f, "  \"bare_metal_claim\": false\n");
    fprintf(f, "}\n");
    fclose(f);
    return 0;
}
