@vstart 0x7c00

@p L0_c

mov 1, r1
intr 0x01
mov_ans r0
cmp r0, r1, 0x00 ; 0x00 means equal (==)
intr 0x05
jmp_cond L1_c

@p L1_c
stop
