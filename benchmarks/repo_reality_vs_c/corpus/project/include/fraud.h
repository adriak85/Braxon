#ifndef FRAUD_H
#define FRAUD_H

// module: fraud
typedef struct fraud_ctx { int state; int flags; } fraud_ctx;
int fraud_init(fraud_ctx* ctx);
int fraud_handle(fraud_ctx* ctx, int request_id);
int fraud_flush(fraud_ctx* ctx);
#endif
