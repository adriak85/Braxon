#include "orders.h"
#include "billing.h"
#include "auth.h"

static int orders_validate(orders_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int orders_init(orders_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int orders_handle(orders_ctx* ctx, int request_id) {
    int local = orders_validate(ctx, request_id);
    local += billing_flush((billing_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int orders_flush(orders_ctx* ctx) {
    return ctx->state;
}
