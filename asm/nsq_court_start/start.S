/*
 * NSQ Court start proof.
 *
 * Purpose:
 * - prove direct AArch64 machine-contact startup
 * - avoid libc/startfiles for this proof binary
 * - write a known message by Linux syscall
 * - exit with a known code
 *
 * This is not the runtime.
 * This is the first substrate ground-contact proof.
 */

.section .text
.global _start
.type _start, %function

_start:
    // write(1, message, message_len)
    mov x0, #1
    adrp x1, message
    add x1, x1, :lo12:message
    mov x2, #message_len
    mov x8, #64
    svc #0

    // exit(37)
    mov x0, #37
    mov x8, #93
    svc #0

.section .rodata
message:
    .ascii "NSQ_COURT_START_PROOF_OK\n"
.set message_len, . - message
