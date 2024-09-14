#include "emu.asm"
#include "bw8.asm"

#addr 0x0000
    jmp.abs boot
#addr 0x0004
    jmp.abs isr
#addr 0x0008
    jmp.abs isr
#addr 0x000C
    jmp.abs isr

isr:
    reti

boot:
    ld x, #hello_world

    loop:
        .control:
            ld a, [x, #0]
            cmp a, 0
            br.eq.abs halt

        .body:
            out [EMULATOR_PUTCHAR], a

        .footer:
            inc x
            jmp.abs .control

    halt:
        jmp.abs halt

hello_world:
    #d "Hello, World!\n\0"


