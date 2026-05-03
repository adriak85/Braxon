#ifndef BILLING_H
#define BILLING_H

// module: billing
typedef struct billing_ctx { int state; int flags; } billing_ctx;
int billing_init(billing_ctx* ctx);
int billing_handle(billing_ctx* ctx, int request_id);
int billing_flush(billing_ctx* ctx);
#endif
