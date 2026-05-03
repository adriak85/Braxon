#ifndef AUTH_H
#define AUTH_H

// module: auth
typedef struct auth_ctx { int state; int flags; } auth_ctx;
int auth_init(auth_ctx* ctx);
int auth_handle(auth_ctx* ctx, int request_id);
int auth_flush(auth_ctx* ctx);
#endif
