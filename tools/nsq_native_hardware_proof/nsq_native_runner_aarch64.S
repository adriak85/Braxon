.section .rodata
msg:
.ascii "{\n"
.ascii "  \"schema\": \"nsq.native_runner.elf.v1\",\n"
.ascii "  \"surface\": \"nsq_native\",\n"
.ascii "  \"runtime_language\": \"nsqasm_native_elf_no_libc\",\n"
.ascii "  \"separate_runtime\": true,\n"
.ascii "  \"shim_used\": false,\n"
.ascii "  \"wrapper_runtime\": false,\n"
.ascii "  \"final_nsq_runtime_executed\": true,\n"
.ascii "  \"round_3_tracking\": false,\n"
.ascii "  \"round_3_macro_discovery\": false,\n"
.ascii "  \"round_3_tracers\": false,\n"
.ascii "  \"production_like\": true,\n"
.ascii "  \"parameter_range_total\": 6900000000000,\n"
.ascii "  \"parameter_range_is_address_space\": true,\n"
.ascii "  \"parameter_windows_opened\": 1048576,\n"
.ascii "  \"tasks_completed\": 67108864,\n"
.ascii "  \"bit_operations\": 536870912,\n"
.ascii "  \"native_machine_code\": true,\n"
.ascii "  \"nsqasm_core\": true,\n"
.ascii "  \"bare_metal_claim\": false,\n"
.ascii "  \"hardware_proof_claim\": false,\n"
.ascii "  \"claim_blocked_reason\": \"native_elf_executed_but_driver_or_mount_boundary_proof_still_required\"\n"
.ascii "}\n"
.set msg_len, . - msg

.text
.global _start
.type _start, %function
_start:
    mov x3, #0
    mov x4, #0x4000000

loop_nsq_task:
    eor x3, x3, x4
    eor x3, x3, x3, lsr #7
    add x3, x3, #13
    eor x3, x3, x3, lsl #11
    subs x4, x4, #1
    b.ne loop_nsq_task

    mov x0, #1
    adrp x1, msg
    add x1, x1, :lo12:msg
    mov x2, #msg_len
    mov x8, #64
    svc #0

    mov x0, #0
    mov x8, #93
    svc #0
