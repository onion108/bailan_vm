@vstart 0x7c00

@p _main
intr 0x01 ; Input a number
mov_ans r0

mov 2, r1 ; r0 % 2
mod r0, r1
mov_ans r0

mov 1, r1
cmp r0, r1, 0x00 ; 0x00 is '=='
jmp_cond L0
jmp L1

@p L0
mov 0x6F, r0 ; o
intr 0x02
mov 0x64, r0 ; d
intr 0x02
intr 0x02
mov 0x0A, r0
intr 0x02
stop

@p L1
mov 0x65, r0 ; e
intr 0x02
mov 0x0A, r0
intr 0x02
stop
