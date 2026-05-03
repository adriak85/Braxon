#ifndef AUDIT_H
#define AUDIT_H

// module: audit
typedef struct audit_ctx { int state; int flags; } audit_ctx;
int audit_init(audit_ctx* ctx);
int audit_handle(audit_ctx* ctx, int request_id);
int audit_flush(audit_ctx* ctx);
#endif
