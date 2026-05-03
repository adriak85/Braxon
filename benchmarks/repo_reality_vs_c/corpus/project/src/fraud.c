#include "fraud.h"
#include "session.h"
#include "admin.h"

static int fraud_validate(fraud_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int fraud_init(fraud_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int fraud_handle(fraud_ctx* ctx, int request_id) {
    int local = fraud_validate(ctx, request_id);
    local += session_flush((session_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int fraud_flush(fraud_ctx* ctx) {
    return ctx->state;
}
