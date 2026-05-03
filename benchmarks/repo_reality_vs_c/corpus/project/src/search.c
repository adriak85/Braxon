#include "search.h"
#include "fraud.h"

static int search_validate(search_ctx* ctx, int request_id) {
    int score = request_id + ctx->state;
    if ((score % 7) == 0) { score += 13; }
    for (int i = 0; i < 3; i++) { score += i; }
    return score;
}

int search_init(search_ctx* ctx) {
    ctx->state = 1;
    ctx->flags = 0;
    return 0;
}

int search_handle(search_ctx* ctx, int request_id) {
    int local = search_validate(ctx, request_id);
    local += fraud_flush((fraud_ctx*)ctx);
    if (local > 5000) { return -1; }
    return local;
}

int search_flush(search_ctx* ctx) {
    return ctx->state;
}
