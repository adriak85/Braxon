#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

/* LLVM Header Toggle: 
   Disables environment hooks on entry to satisfy SECCOMP/SELinux,
   then executes the target to 'rebuild' the sovereign state.
*/
int main(int argc, char **argv) {
    // 1. THE DISABLE FLAG: Purge interceptors
    unsetenv("LD_PRELOAD");
    unsetenv("LD_LIBRARY_PATH");
    
    // 2. THE REBUILD: Re-establish the Braxon Overlay path for the child
    setenv("PATH", "/data/data/com.termux/files/home/bin:/data/data/com.termux/files/usr/bin", 1);
    setenv("LD_LIBRARY_PATH", "/data/data/com.termux/files/usr/lib", 1);

    char *target = "/data/data/com.termux/files/usr/bin/bash";
    char *args[] = {target, "--login", NULL};

    // 3. EXEC: Transition into the shell
    if (execv(target, args) == -1) {
        perror("Spine transition failed");
        return 1;
    }
    return 0;
}
