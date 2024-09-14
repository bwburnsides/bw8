#include "emu.asm"
#include "bw8.asm"

#const RST_VEC = 0x0000
#const NMI_VEC = 0x0004
#const IRQ_VEC = 0x0008
#const SWI_VEC = 0x000C

#addr RST_VEC
    jmp.abs boot
#addr NMI_VEC
    jmp.abs spin
#addr IRQ_VEC
    jmp.abs isr
#addr SWI_VEC
    jmp.abs isr

boot:
    ; Configure the stack
    ld x, #0xffff
    mv sp, x

    ; Prepare for a jump into user-address-space #5 with IRQs enabled.
    ld a, #5
    mv br, a
    ld b, #0b0101_1000

    ; Set user entry point to `spin`    
    ld y, #spin

    ; Prepare stack then jump to user mode.
    push y
    push b
    reti

spin:
    jmp.abs spin

isr:
    out [EMULATOR_IRQ_ACK], a
    call.abs print_hello_world
    inc c
    out [0x03], c
    reti

print_hello_world:
    ld x, #hello_world

    loop:
        .control:
            ld a, [x, #0]
            cmp a, 0
            br.eq.abs return

        .body:
            out [EMULATOR_PUTCHAR], a

        .footer:
            inc x
            jmp.abs .control

    return:
        ret

hello_world:
    #d "Hello, World!\n\0"


