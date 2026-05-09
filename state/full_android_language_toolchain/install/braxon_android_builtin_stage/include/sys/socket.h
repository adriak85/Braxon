#ifndef BRAXON_ANDROID_BUILTIN_SYS_SOCKET_H
#define BRAXON_ANDROID_BUILTIN_SYS_SOCKET_H
#include_next <sys/socket.h>
#ifdef __cplusplus
extern "C" {
#endif
int accept4(int sockfd, struct sockaddr *addr, socklen_t *addrlen, int flags);
#ifdef __cplusplus
}
#endif
#endif
