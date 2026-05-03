
.global _start
.text

_start:
    mov x1, #8
    mov x2, #13
    add x3, x1, x2
    mov x4, #21
    cmp x3, x4
    b.eq ok_path

fail_path:
    adr x1, fail_msg
    mov x2, #fail_len
    bl write_stdout
    mov x0, #1
    b exit_now

ok_path:
    adr x1, ok_msg
    mov x2, #ok_len
    bl write_stdout
    mov x0, #0
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
ok_msg:
.ascii "NSQ_SEMANTIC_RUNTIME_BEGIN\nauthority=NSQ_COURT\narchitecture_root=true\nking=compositor\nqueen=linter\ncourt_is_agents=false\nruntime_kind=nsqasm_aarch64_no_libc\ntask_file_semantics=executed\nsource_language=NSQ\ncompiled_surface=NSQASM_AARCH64\nc_runner_used=false\ninput_a=8\ninput_b=13\noperation=add\nnonce=NSQ_SEMANTIC_BENCHMARK_20260424_190345\nexpected_result=21\nsemantic_result_verified=true\nok=true\nNSQ_SEMANTIC_RUNTIME_END\n"
.equ ok_len, . - ok_msg

fail_msg:
.ascii "NSQ_SEMANTIC_RUNTIME_BEGIN\nauthority=NSQ_COURT\narchitecture_root=true\nking=compositor\nqueen=linter\ncourt_is_agents=false\nruntime_kind=nsqasm_aarch64_no_libc\ntask_file_semantics=executed\nsource_language=NSQ\ncompiled_surface=NSQASM_AARCH64\nc_runner_used=false\ninput_a=8\ninput_b=13\noperation=add\nnonce=NSQ_SEMANTIC_BENCHMARK_20260424_190345\nexpected_result=21\nsemantic_result_verified=false\nok=false\nNSQ_SEMANTIC_RUNTIME_END\n"
.equ fail_len, . - fail_msg
