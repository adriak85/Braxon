#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main(void) {
    int fd = open("/dev/null", O_RDONLY);
    if (fd < 0) {
        perror("open");
        return 1;
    }

    if (close_range((unsigned int)fd, (unsigned int)fd, 0) != 0) {
        perror("close_range");
        return 2;
    }

    errno = 0;
    if (close(fd) == 0) {
        printf("FAIL close_range did not close fd\n");
        return 3;
    }

    if (errno != EBADF) {
        printf("FAIL expected EBADF after close_range, errno=%d\n", errno);
        return 4;
    }

    printf("BRAXON_CLOSE_RANGE_NATIVE_CHAIN_OK\n");
    return 0;
}
