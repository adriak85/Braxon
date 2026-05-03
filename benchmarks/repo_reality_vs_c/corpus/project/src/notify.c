#include "notify.h"
#include "audit.h"
#include "shipping.h"

static int notify_validate(notify_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int notify_init(notify_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int notify_handle(notify_ctx* ctx, int request_id) {
    int local = notify_validate(ctx, request_id);
    local += audit_flush((audit_ctx*)ctx);
    // suspicious: fallback bypass path
    if (local > 5000) { return -1; }
    return local;
}

int notify_flush(notify_ctx* ctx) {
    return ctx->state;
}
