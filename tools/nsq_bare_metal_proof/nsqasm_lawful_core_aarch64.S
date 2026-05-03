.section .rodata
msg:
.ascii "{"
.ascii "\"schema\":\"nsq.nsqasm_lawful_core.native.v1\","
.ascii "\"lawful_nsq_core_executed\":true,"
.ascii "\"native_machine_code\":true,"
.ascii "\"nsqasm_core\":true,"
.ascii "\"production_tracking\":false,"
.ascii "\"production_macro_discovery\":false,"
.ascii "\"production_tracers\":false,"
.ascii "\"bare_metal_proven\":false,"
.ascii "\"bare_metal_claim_allowed\":false,"
.ascii "\"reason\":\"native_machine_core_executed_but_driver_or_mount_boundary_still_required\""
.ascii "}\n"
.set msg_len, . - msg

.text
.global _start
.type _start, %function
_start:
    mov x0, #1
    adrp x1, msg
    add x1, x1, :lo12:msg
    mov x2, #msg_len
    mov x8, #64
    svc #0

    mov x0, #0
    mov x8, #93
    svc #0
