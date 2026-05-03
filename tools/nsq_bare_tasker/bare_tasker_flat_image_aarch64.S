
// Bare-metal-form task image.
// No libc. No Linux syscalls. No ELF loader dependency in the flat image.
// Output is stored in symbols/bytes for later boot-tasker or probe reading.

.global _start
.global nsq_bare_result
.global nsq_bare_expected
.global nsq_bare_ok
.global nsq_bare_nonce_tag

.text
_start:
    mov x1, #8
    mov x2, #13
    add x3, x1, x2
    adr x5, nsq_bare_result
    str x3, [x5]

    mov x4, #21
    adr x6, nsq_bare_expected
    str x4, [x6]

    cmp x3, x4
    b.eq ok_path

fail_path:
    mov x7, #0
    adr x8, nsq_bare_ok
    str x7, [x8]
    b halt

ok_path:
    mov x7, #1
    adr x8, nsq_bare_ok
    str x7, [x8]
    b halt

halt:
    wfe
    b halt

.data
.align 3
nsq_bare_result:
    .quad 0
nsq_bare_expected:
    .quad 21
nsq_bare_ok:
    .quad 0
nsq_bare_nonce_tag:
    .ascii "NSQ_BARE_TASKER_ACCEPTANCE_V1\0"
