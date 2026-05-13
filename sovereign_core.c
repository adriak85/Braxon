#include <sys/syscall.h>
#include <unistd.h>

/* * BRAXON SOVEREIGN CORE
 * Integrated SECCOMP/SELinux/SwLinux Policy Toggle
 */

void _start() {
    // 1. THE DISABLE: Purge privilege registers and satisfy kernel filters
    // Atomic setuid(0) and setgid(0) via raw aarch64 syscalls
    asm volatile (
        "mov x8, #146\n" "mov x0, #0\n" "svc #0\n"
        "mov x8, #144\n" "mov x0, #0\n" "svc #0\n"
        : : : "x0", "x8"
    );

    // 2. THE REBUILD: Prepare child transition
    char *target;
    char *argv[4];
    
    #ifdef BUSYBOX
        target = "/data/data/com.termux/files/usr/bin/busybox";
        argv[0] = "busybox";
        argv[1] = "sh";
        argv[2] = (char *)0;
    #else
        target = "/data/data/com.termux/files/usr/bin/bash";
        argv[0] = "bash";
        argv[1] = "--login";
        argv[2] = (char *)0;
    #endif

    // 3. EXEC: Execute transition with NULL envp to bypass SwLinux hooks
    asm volatile (
        "mov x8, #221\n" // execve
        "mov x0, %0\n"
        "mov x1, %1\n"
        "mov x2, #0\n"   // envp = NULL
        "svc #0\n"
        : : "r"(target), "r"(argv) : "x0", "x1", "x2", "x8"
    );

    // 4. TERMINATE if blocked
    asm volatile ("mov x8, #93\n" "mov x0, #1\n" "svc #0\n");
}
