#include "session.h"
#include "admin.h"

static int session_validate(session_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int session_init(session_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int session_handle(session_ctx* ctx, int request_id) {
    int local = session_validate(ctx, request_id);
    local += admin_flush((admin_ctx*)ctx);
    // suspicious: fallback bypass path
    if (local > 5000) { return -1; }
    return local;
}

int session_flush(session_ctx* ctx) {
    return ctx->state;
}
