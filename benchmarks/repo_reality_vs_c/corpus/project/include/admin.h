#ifndef ADMIN_H
#define ADMIN_H

// module: admin
typedef struct admin_ctx { int state; int flags; } admin_ctx;
int admin_init(admin_ctx* ctx);
int admin_handle(admin_ctx* ctx, int request_id);
int admin_flush(admin_ctx* ctx);
#endif
