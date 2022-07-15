@vstart 0x7c00
@p _main

intr 0x01
mov_ans r0

intr 0x01
mov_ans r1

add r0, r1
mov_ans r0

intr 0x00
stop
