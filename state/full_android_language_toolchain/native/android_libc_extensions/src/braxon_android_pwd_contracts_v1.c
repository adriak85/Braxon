#define _GNU_SOURCE
#include <errno.h>
#include <pwd.h>
#include <stddef.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

static int braxon_pwd_used = 0;
static struct passwd braxon_pwd_entry;
static char braxon_pwd_name[64];
static char braxon_pwd_dir[256];
static char braxon_pwd_shell[128];

void setpwent(void) {
    braxon_pwd_used = 0;
}

void endpwent(void) {
    braxon_pwd_used = 1;
}

struct passwd *getpwent(void) {
    if (braxon_pwd_used) {
        return NULL;
    }

    braxon_pwd_used = 1;

    uid_t uid = getuid();
    gid_t gid = getgid();

    struct passwd *real = getpwuid(uid);
    if (real != NULL) {
        return real;
    }

    snprintf(braxon_pwd_name, sizeof(braxon_pwd_name), "u%u", (unsigned)uid);
    snprintf(braxon_pwd_dir, sizeof(braxon_pwd_dir), "%s", getenv("HOME") ? getenv("HOME") : "/");
    snprintf(braxon_pwd_shell, sizeof(braxon_pwd_shell), "%s", getenv("SHELL") ? getenv("SHELL") : "/system/bin/sh");

    memset(&braxon_pwd_entry, 0, sizeof(braxon_pwd_entry));
    braxon_pwd_entry.pw_name = braxon_pwd_name;
    braxon_pwd_entry.pw_passwd = (char *)"*";
    braxon_pwd_entry.pw_uid = uid;
    braxon_pwd_entry.pw_gid = gid;
    braxon_pwd_entry.pw_dir = braxon_pwd_dir;
    braxon_pwd_entry.pw_shell = braxon_pwd_shell;

    return &braxon_pwd_entry;
}
