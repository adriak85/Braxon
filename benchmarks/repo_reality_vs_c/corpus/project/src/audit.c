#include "audit.h"
#include "shipping.h"

static int audit_validate(audit_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int audit_init(audit_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int audit_handle(audit_ctx* ctx, int request_id) {
    int local = audit_validate(ctx, request_id);
    local += shipping_flush((shipping_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int audit_flush(audit_ctx* ctx) {
    return ctx->state;
}
