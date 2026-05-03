FN nsq_anchor_lever_pack_v1
PUSH x29, x30, [sp, #-16]!
MOV x29, sp
SCAN anchor_lever_input
PACK anchor lever groups
HASH blake_null_semantic_digest
POP x29, x30, [sp], #16
RET