#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern uint64_t BRAXON_music_asm_fold_u8(const unsigned char *buf, unsigned long len);

static void put_u16(FILE *f, uint16_t v) {
    fputc(v & 255, f);
    fputc((v >> 8) & 255, f);
}

static void put_u32(FILE *f, uint32_t v) {
    fputc(v & 255, f);
    fputc((v >> 8) & 255, f);
    fputc((v >> 16) & 255, f);
    fputc((v >> 24) & 255, f);
}

static int16_t clamp16(int x) {
    if (x > 32767) return 32767;
    if (x < -32768) return -32768;
    return (int16_t)x;
}

static void write_wav_header(FILE *f, uint32_t sample_rate, uint32_t samples) {
    uint32_t bytes = samples * 2;
    fwrite("RIFF", 1, 4, f);
    put_u32(f, 36 + bytes);
    fwrite("WAVE", 1, 4, f);

    fwrite("fmt ", 1, 4, f);
    put_u32(f, 16);
    put_u16(f, 1);
    put_u16(f, 1);
    put_u32(f, sample_rate);
    put_u32(f, sample_rate * 2);
    put_u16(f, 2);
    put_u16(f, 16);

    fwrite("data", 1, 4, f);
    put_u32(f, bytes);
}

int main(int argc, char **argv) {
    const char *out_path = argc > 1 ? argv[1] : "state/braxon/music_app/current/preview.wav";
    const char *seed_text = argc > 2 ? argv[2] : "BRAXON_music_app default seed citadel699 beyond699";

    uint64_t seed = BRAXON_music_asm_fold_u8((const unsigned char *)seed_text, (unsigned long)strlen(seed_text));
    const uint32_t sr = 22050;
    const uint32_t seconds = 9;
    const uint32_t samples = sr * seconds;

    FILE *f = fopen(out_path, "wb");
    if (!f) {
        fprintf(stderr, "BRAXON_MUSIC_SYNTH_ERROR open_failed path=%s errno=%d\n", out_path, errno);
        return 2;
    }

    write_wav_header(f, sr, samples);

    uint32_t p1 = 0, p2 = 0, p3 = 0;
    uint32_t base = 120 + (uint32_t)(seed % 180);
    uint32_t step1 = (uint32_t)(((uint64_t)base << 32) / sr);
    uint32_t step2 = (uint32_t)(((uint64_t)(base * 2 + 7) << 32) / sr);
    uint32_t step3 = (uint32_t)(((uint64_t)(base / 2 + 55) << 32) / sr);

    for (uint32_t i = 0; i < samples; i++) {
        uint32_t section = (i * 6) / samples;
        uint32_t amp = 3000 + section * 600;

        p1 += step1 + section * 17;
        p2 += step2 + section * 29;
        p3 += step3 + section * 11;

        int s1 = (p1 & 0x80000000u) ? (int)amp : -(int)amp;
        int s2 = (p2 & 0x80000000u) ? (int)(amp / 2) : -(int)(amp / 2);
        int tri = (int)((p3 >> 20) & 0xfff);
        if (tri > 2047) tri = 4095 - tri;
        tri = (tri - 1024) * (int)(amp / 1024);

        uint32_t local = i % (sr * seconds / 6);
        uint32_t section_len = sr * seconds / 6;
        uint32_t env = 1024;
        if (local < sr / 20) env = (local * 1024) / (sr / 20);
        if (section_len - local < sr / 20) env = ((section_len - local) * 1024) / (sr / 20);

        int mixed = (s1 + s2 + tri) * (int)env / 1024;
        int16_t sample = clamp16(mixed);

        put_u16(f, (uint16_t)sample);
    }

    fclose(f);

    printf("BRAXON_MUSIC_SYNTH_OK\n");
    printf("runtime_kind=native_c_plus_aarch64_asm_seed_fold\n");
    printf("output=%s\n", out_path);
    printf("sample_rate=%u\n", sr);
    printf("seconds=%u\n", seconds);
    printf("seed=%llu\n", (unsigned long long)seed);
    printf("tracking=false\n");
    printf("macro_discovery=false\n");
    printf("tracers=false\n");
    printf("bare_metal_claim=false\n");
    return 0;
}
