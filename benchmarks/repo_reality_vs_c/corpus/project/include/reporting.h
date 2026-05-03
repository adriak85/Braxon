#ifndef REPORTING_H
#define REPORTING_H

// module: reporting
typedef struct reporting_ctx { int state; int flags; } reporting_ctx;
int reporting_init(reporting_ctx* ctx);
int reporting_handle(reporting_ctx* ctx, int request_id);
int reporting_flush(reporting_ctx* ctx);
#endif
