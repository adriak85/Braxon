#ifndef INVENTORY_H
#define INVENTORY_H

// module: inventory
typedef struct inventory_ctx { int state; int flags; } inventory_ctx;
int inventory_init(inventory_ctx* ctx);
int inventory_handle(inventory_ctx* ctx, int request_id);
int inventory_flush(inventory_ctx* ctx);
#endif
