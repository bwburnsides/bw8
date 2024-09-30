#include "bw8.asm"
#include "emu.asm"

#const RST_VEC = 0x0000
#const NMI_VEC = 0x0004
#const IRQ_VEC = 0x0008
#const SWI_VEC = 0x000C

; These are all IO Addresses

#addr RST_VEC
    jmp.abs boot
#addr NMI_VEC
    jmp.abs spin
#addr IRQ_VEC
    jmp.abs irq_palette_foolery
#addr SWI_VEC
    jmp.abs spin

; Simple 8 Color Palette for now
#const BLACK   = 0b000_000_00
#const BLUE    = 0b000_000_11
#const GREEN   = 0b000_111_00
#const CYAN    = 0b000_111_11
#const RED     = 0b111_000_00
#const MAGENTA = 0b111_000_11
#const YELLOW  = 0b111_111_00
#const WHITE   = 0b111_111_11 

boot:
    ; Initialize stack so that interrupts can be serviced,
    ; functions can be called, and stack operations can be made.
    ld x, #0xFFFF
    mv sp, x

    ; Enable interrupts
    set.i

    ld x, #str_stack_initialized
    call.abs print_x_c_str

    ld x, #str_initializing_palette
    call.abs print_x_c_str

    ; Configure a simple palette with 8 colors. They are repeated
    ; in memory twice, since every palette has 16 colors.
    ld x, #IO_PALETTE_BASE
    ld c, #0

    .eight_colors:
        ld a, #BLACK
        out [x, c], a

        inc c
        ld a, #BLUE
        out [x, c], a

        inc c
        ld a, #GREEN
        out [x, c], a

        inc c
        ld a, #CYAN
        out [x, c], a

        inc c
        ld a, #RED
        out [x, c], a

        inc c
        ld a, #MAGENTA
        out [x, c], a

        inc c
        ld a, #YELLOW
        out [x, c], a

        inc c
        ld a, #WHITE
        out [x, c], a

        inc c

    cmp c, 16
    br.ne.abs .eight_colors

    .palette_loaded:
        ld x, #str_palette_initialized
        call.abs print_x_c_str

        ld x, #str_initializing_bitmap
        call.abs print_x_c_str

        ld c, #0
        ld y, #IO_BITMAP_BASE

    .loop:
        ld x, #bitmaps.A
        ld b, #0

        .control:
            ld a, [x, b]
            cmp b, 8
            br.eq.abs .plane_complete
        .body:
            out [y, b], a
        .footer:
            inc b
            jmp.abs .control

    .plane_complete:
        inc c
        cmp c, 4
        br.eq.abs .first_bitmap_initialized
        lea [y, 8]
        jmp.abs .loop

    .first_bitmap_initialized:
    ; A SECOND BITMAP!
        ld c, #0
        ld y, #(IO_BITMAP_BASE) + 32

    .loop2:
        ld x, #bitmaps.C
        ld b, #0

        .control2:
            ld a, [x, b]
            cmp b, 8
            br.eq.abs .plane_complete2
        .body2:
            out [y, b], a
        .footer2:
            inc b
            jmp.abs .control2

    .plane_complete2:
        inc c
        cmp c, 4
        br.eq.abs .bitmap_initialized
        lea [y, 8]
        jmp.abs .loop2

    .bitmap_initialized:
        ld x, #str_bitmap_initialized
        call.abs print_x_c_str

    ld x, #str_populating_tilemap
    call.abs print_x_c_str

    ld x, #IO_TILEMAP_BASE
    ld a, #0x00
    ld b, #0x01
    out [x, 0], b
    out [x, 1], a

    ld x, #str_first_tile_populated
    call.abs print_x_c_str

spin:
    jmp.abs spin

irq_palette_foolery:
    push x
    push a
    push b
    push c

    out [EMULATOR_IRQ_ACK], a

    ld b, #BLACK
    ld c, #WHITE

    ld x, #IO_PALETTE_BASE
    in a, [x, 0]

    cmp a, b
    br.eq.abs .to_white

    out [x, 0], b
    out [x, 15], c
    ld x, #str_first_path
    call.abs print_x_c_str

    .return:
        pop c
        pop b
        pop a
        pop x
        reti

    .to_white:
        out [x, 0], c
        out [x, 15], b

        ld x, #str_second_path
        call.abs print_x_c_str

        jmp.abs .return


; Saves and restores A, clobbers X and flags.
print_x_c_str:
    push a

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
        pop a
        ret


; Puts the ASCII hex repr of reg A into A(low) B(high)
byte_itoa_hex:
    push c
    push d
    push x
    push y

    mv b, a
    and a, 0b0000_1111
    ld x, #itoa_string
    ld a, [x, a]

    and b, 0b1111_0000
    shr b
    shr b
    shr b
    shr b

    ld b, [x, b]

    pop y
    pop x
    pop d
    pop c

    ret

str_stack_initialized:
    #d "Stack initialized.\n\0"
str_initializing_palette:
    #d "Initializing palette.\n\0"
str_palette_initialized:
    #d "Palette initialized.\n\0"
str_initializing_bitmap:
    #d "Initializing bitmap.\n\0"
str_bitmap_initialized:
    #d "Bitmap initialized.\n\0"
str_populating_tilemap:
    #d "Populating tilemap.\n\0"
str_first_tile_populated:
    #d "Index 0 tile populated.\n\0"
str_first_path:
    #d "First path.\n\0"
str_second_path:
    #d "Second path.\n\0"

bitmaps:
    .A:
        #d 0b00011000
        #d 0b00011000
        #d 0b00111100
        #d 0b01100110
        #d 0b01111110
        #d 0b01100110
        #d 0b01100110
        #d 0b00000000
    .B:
        #d 0b01111100
        #d 0b01111100
        #d 0b01100110
        #d 0b01100110
        #d 0b01111100
        #d 0b01100110
        #d 0b01100110
        #d 0b01111100
    .C:
        #d 0b00111100
        #d 0b01100110
        #d 0b01100000
        #d 0b01100000
        #d 0b01100000
        #d 0b01100110
        #d 0b00111100
        #d 0b00000000

itoa_string:
    #d "0123456789ABCDEF"