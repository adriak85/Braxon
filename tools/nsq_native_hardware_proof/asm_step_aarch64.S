.text
.global nsq_asm_step
.type nsq_asm_step, %function
nsq_asm_step:
    eor x0, x0, x0, lsr #7
    add x0, x0, #13
    eor x0, x0, x0, lsl #11
    ret
