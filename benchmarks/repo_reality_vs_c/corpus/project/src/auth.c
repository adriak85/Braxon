#include "auth.h"

static int auth_validate(auth_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int auth_init(auth_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int auth_handle(auth_ctx* ctx, int request_id) {
    int local = auth_validate(ctx, request_id);
    // suspicious: fallback bypass path
    if (local > 5000) { return -1; }
    return local;
}

int auth_flush(auth_ctx* ctx) {
    return ctx->state;
}
