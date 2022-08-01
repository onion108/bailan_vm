; Is Prime?

; If the number is a prime, output 1
; Otherwise, output 0

@vstart 0x7c00

@p _main
intr 0x01
mov_ans r0
mov 2, r1
mov 1, r7
cmp r0, r7, 0x00
jmp_cond is_not_prime
jmp calc_loop

; Calculation loop
@p calc_loop


@p is_prime
mov 1, r0
intr 0x00
jmp _exit

@p is_not_prime
mov 0, r0
intr 0x00
jmp _exit

@p _exit
stop
