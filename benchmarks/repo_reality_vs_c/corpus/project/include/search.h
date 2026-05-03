#ifndef SEARCH_H
#define SEARCH_H

// module: search
typedef struct search_ctx { int state; int flags; } search_ctx;
int search_init(search_ctx* ctx);
int search_handle(search_ctx* ctx, int request_id);
int search_flush(search_ctx* ctx);
#endif
