#ifndef BRAXON_ANDROID_UNISTD_OVERLAY_V2_H
#define BRAXON_ANDROID_UNISTD_OVERLAY_V2_H
#include_next <unistd.h>
#include <stddef.h>
#include <sys/types.h>
#ifdef __cplusplus
extern "C" {
#endif
int close_range(unsigned int first, unsigned int last, unsigned int flags);
ssize_t copy_file_range(int fd_in, loff_t *off_in, int fd_out, loff_t *off_out, size_t len, unsigned int flags);
int pipe2(int pipefd[2], int flags);
int dup3(int oldfd, int newfd, int flags);
int fexecve(int fd, char *const argv[], char *const envp[]);
int getlogin_r(char *name, size_t namesize);
int getloadavg(double loadavg[], int nelem);
#ifdef __cplusplus
}
#endif
#endif
