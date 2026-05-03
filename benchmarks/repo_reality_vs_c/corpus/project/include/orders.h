#ifndef ORDERS_H
#define ORDERS_H

// module: orders
typedef struct orders_ctx { int state; int flags; } orders_ctx;
int orders_init(orders_ctx* ctx);
int orders_handle(orders_ctx* ctx, int request_id);
int orders_flush(orders_ctx* ctx);
#endif
