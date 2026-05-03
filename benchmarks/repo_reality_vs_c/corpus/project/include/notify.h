#ifndef NOTIFY_H
#define NOTIFY_H

// module: notify
typedef struct notify_ctx { int state; int flags; } notify_ctx;
int notify_init(notify_ctx* ctx);
int notify_handle(notify_ctx* ctx, int request_id);
int notify_flush(notify_ctx* ctx);
#endif
