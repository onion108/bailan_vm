; Is Prime?

; If the number is a prime, output 1
; Otherwise, output 0

@vstart 0x7c00

@p _main
intr 0x01
mov_ans r0
mov 0, r7
cmp r0, r7, 0x00
jmp_cond is_not_prime
mov 1, r7
cmp r0, r7, 0x00
jmp_cond is_not_prime
mov 2, r7

; Calculation loop
@p calc_loop
mod r0, r7
mov_ans r1
mov 0, r8
cmp r1, r8, 0x00
jmp_cond is_not_prime
mov 1, r8
add r7, r8
mov_ans r7
cmp r7, r0, 0x03
jmp_cond calc_loop


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
