#include "billing.h"
#include "auth.h"

static int billing_validate(billing_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int billing_init(billing_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int billing_handle(billing_ctx* ctx, int request_id) {
    int local = billing_validate(ctx, request_id);
    local += auth_flush((auth_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int billing_flush(billing_ctx* ctx) {
    return ctx->state;
}
