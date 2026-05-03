
.global _start
.text

_start:
    adr x1, msg
    mov x2, #msg_len
    bl write_stdout
    mov x0, #7
    b exit_now

write_stdout:
    mov x0, #1
    mov x8, #64
    svc #0
    ret

exit_now:
    mov x8, #93
    svc #0

.section .rodata
msg:
.ascii "NSQ_PERPETUAL_RUNTIME_CRASH_PROBE\nauthority=NSQ_COURT\narchitecture_root=true\nking=compositor\nqueen=linter\nroute=perpetual_runtime\nsource_language=NSQ\ncompiled_surface=NSQASM_AARCH64\nruntime_kind=nsqasm_aarch64_no_libc\nc_runner_used=false\nseq=2\nintentional_crash_probe=true\nok=false\n"
.equ msg_len, . - msg
