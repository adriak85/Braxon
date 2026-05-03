#include "admin.h"
#include "reporting.h"
#include "notify.h"

static int admin_validate(admin_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int admin_init(admin_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int admin_handle(admin_ctx* ctx, int request_id) {
    int local = admin_validate(ctx, request_id);
    local += reporting_flush((reporting_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int admin_flush(admin_ctx* ctx) {
    return ctx->state;
}
