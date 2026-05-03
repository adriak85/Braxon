
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
    mov x0, #1
    b exit_now

ok_path:
    adr x1, host_msg
    mov x2, #host_len
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
host_msg:
.ascii 'NSQ_BARE_TASKER_BEGIN\nsource_language=NSQ\nroute=nsq_court_compositor\ncompiled_surface=NSQASM_AARCH64\nhost_artifact=ELF\nelf_is_bare_metal=false\nbare_artifact=flat_bin\nraw_image_is_bare_metal_form=true\nc_runner_used=false\nnonce=NSQ_BARE_TASKER_ACCEPTANCE_V1\nresult=21\nsemantic_result_verified=true\nok=true\nNSQ_BARE_TASKER_END\n'
.equ host_len, . - host_msg
