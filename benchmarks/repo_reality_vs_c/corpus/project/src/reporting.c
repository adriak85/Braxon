#include "reporting.h"
#include "notify.h"

static int reporting_validate(reporting_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int reporting_init(reporting_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int reporting_handle(reporting_ctx* ctx, int request_id) {
    int local = reporting_validate(ctx, request_id);
    local += notify_flush((notify_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int reporting_flush(reporting_ctx* ctx) {
    return ctx->state;
}
