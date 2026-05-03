.text
.align 2
.global BRAXON_music_asm_fold_u8
.type BRAXON_music_asm_fold_u8, %function

// uint64_t BRAXON_music_asm_fold_u8(const unsigned char *buf, unsigned long len)
// Simple deterministic fold used as a native ASM seed kernel.
// h = h * 131 + byte
BRAXON_music_asm_fold_u8:
    mov     x2, #0
    cbz     x1, 2f
1:
    ldrb    w3, [x0], #1
    mov     x4, #131
    mul     x2, x2, x4
    add     x2, x2, x3
    subs    x1, x1, #1
    b.ne    1b
2:
    mov     x0, x2
    ret
