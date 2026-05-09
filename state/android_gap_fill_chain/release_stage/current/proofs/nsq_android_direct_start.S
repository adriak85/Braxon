.section .rodata
message:
    .ascii "BRAXON_ANDROID_DIRECT_START_OK\n"
message_len = . - message

.section .text
.global _start
.type _start, %function
_start:
    mov x0, #1
    adr x1, message
    mov x2, #31
    mov x8, #64
    svc #0

    mov x0, #37
    mov x8, #93
    svc #0
