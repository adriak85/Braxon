#include "shipping.h"
#include "inventory.h"
#include "orders.h"

static int shipping_validate(shipping_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int shipping_init(shipping_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int shipping_handle(shipping_ctx* ctx, int request_id) {
    int local = shipping_validate(ctx, request_id);
    local += inventory_flush((inventory_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int shipping_flush(shipping_ctx* ctx) {
    return ctx->state;
}
