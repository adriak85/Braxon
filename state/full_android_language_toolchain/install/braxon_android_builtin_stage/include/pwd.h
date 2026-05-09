#ifndef BRAXON_ANDROID_PWD_CONTRACT_OVERLAY_V2_H
#define BRAXON_ANDROID_PWD_CONTRACT_OVERLAY_V2_H

#include_next <pwd.h>

#ifdef __cplusplus
extern "C" {
#endif

void setpwent(void);
struct passwd *getpwent(void);

/*
 * Do NOT redeclare endpwent here.
 * Android/Bionic already provides it as a static no-op in pwd.h.
 */

#ifdef __cplusplus
}
#endif

#endif
