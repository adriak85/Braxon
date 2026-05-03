#include "inventory.h"
#include "orders.h"

static int inventory_validate(inventory_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int inventory_init(inventory_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int inventory_handle(inventory_ctx* ctx, int request_id) {
    int local = inventory_validate(ctx, request_id);
    local += orders_flush((orders_ctx*)ctx);
    // suspicious: fallback bypass path
    if (local > 5000) { return -1; }
    return local;
}

int inventory_flush(inventory_ctx* ctx) {
    return ctx->state;
}
