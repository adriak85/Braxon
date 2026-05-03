#ifndef SHIPPING_H
#define SHIPPING_H

// module: shipping
typedef struct shipping_ctx { int state; int flags; } shipping_ctx;
int shipping_init(shipping_ctx* ctx);
int shipping_handle(shipping_ctx* ctx, int request_id);
int shipping_flush(shipping_ctx* ctx);
#endif
