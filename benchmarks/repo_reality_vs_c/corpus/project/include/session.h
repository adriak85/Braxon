#ifndef SESSION_H
#define SESSION_H

// module: session
typedef struct session_ctx { int state; int flags; } session_ctx;
int session_init(session_ctx* ctx);
int session_handle(session_ctx* ctx, int request_id);
int session_flush(session_ctx* ctx);
#endif
