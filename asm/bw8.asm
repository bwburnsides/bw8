#const(noemit) EXT = 1`8

#ruledef bw8 {
    nop   => 0x00
    _ext  => EXT

    set.c => 0x02
    clr.c => 0x03

    set.i => 0x04
    clr.i => 0x05

    set.b => 0x06
    clr.b => 0x07

    mv a, br              => 0x08
    mv br, a              => 0x09

    mv a, a               => 0x0a
    mv a, b               => 0x0b
    mv a, c               => 0x0c
    mv a, d               => 0x0d
    mv b, a               => 0x0e
    mv b, b               => 0x0f
    mv b, c               => 0x10
    mv b, d               => 0x11
    mv c, a               => 0x12
    mv c, b               => 0x13
    mv c, c               => 0x14
    mv c, d               => 0x15
    mv d, a               => 0x16
    mv d, b               => 0x17
    mv d, c               => 0x18
    mv d, d               => 0x19

    ld a, #{imm: i8}      => 0x1a @ imm
    ld b, #{imm: i8}      => 0x1b @ imm
    ld c, #{imm: i8}      => 0x1c @ imm
    ld d, #{imm: i8}      => 0x1d @ imm

    ld a, [{abs: u16}]     => 0x1e @ le(abs)
    ld a, [x, #{imm: s8}]  => 0x1f @ imm
    ld a, [y, #{imm: s8}]  => 0x20 @ imm
    ld a, [sp, #{imm: s8}] => 0x21 @ imm
    ld a, [x, a]           => 0x22
    ld a, [x, b]           => 0x23
    ld a, [x, c]           => 0x24
    ld a, [x, d]           => 0x25
    ld a, [y, a]           => 0x26
    ld a, [y, b]           => 0x27
    ld a, [y, c]           => 0x28
    ld a, [y, d]           => 0x29
    ld a, [sp, a]          => 0x2a
    ld a, [sp, b]          => 0x2b
    ld a, [sp, c]          => 0x2c
    ld a, [sp, d]          => 0x2d

    ld b, [{abs: u16}]     => 0x2e @ le(abs)
    ld b, [x, #{imm: s8}]  => 0x2f @ imm
    ld b, [y, #{imm: s8}]  => 0x30 @ imm
    ld b, [sp, #{imm: s8}] => 0x31 @ imm
    ld b, [x, a]           => 0x32
    ld b, [x, b]           => 0x33
    ld b, [x, c]           => 0x34
    ld b, [x, d]           => 0x35
    ld b, [y, a]           => 0x36
    ld b, [y, b]           => 0x37
    ld b, [y, c]           => 0x38
    ld b, [y, d]           => 0x39
    ld b, [sp, a]          => 0x3a
    ld b, [sp, b]          => 0x3b
    ld b, [sp, c]          => 0x3c
    ld b, [sp, d]          => 0x3d

    ld c, [{abs: u16}]     => 0x3e @ le(abs)
    ld c, [x, #{imm: s8}]  => 0x3f @ imm
    ld c, [y, #{imm: s8}]  => 0x40 @ imm
    ld c, [sp, #{imm: s8}] => 0x41 @ imm
    ld c, [x, a]           => 0x42
    ld c, [x, b]           => 0x43
    ld c, [x, c]           => 0x44
    ld c, [x, d]           => 0x45
    ld c, [y, a]           => 0x46
    ld c, [y, b]           => 0x47
    ld c, [y, c]           => 0x48
    ld c, [y, d]           => 0x49
    ld c, [sp, a]          => 0x4a
    ld c, [sp, b]          => 0x4b
    ld c, [sp, c]          => 0x4c
    ld c, [sp, d]          => 0x4d

    ld d, [{abs: u16}]     => 0x4e @ le(abs)
    ld d, [x, #{imm: s8}]  => 0x4f @ imm
    ld d, [y, #{imm: s8}]  => 0x50 @ imm
    ld d, [sp, #{imm: s8}] => 0x51 @ imm
    ld d, [x, a]           => 0x52
    ld d, [x, b]           => 0x53
    ld d, [x, c]           => 0x54
    ld d, [x, d]           => 0x55
    ld d, [y, a]           => 0x56
    ld d, [y, b]           => 0x57
    ld d, [y, c]           => 0x58
    ld d, [y, d]           => 0x59
    ld d, [sp, a]          => 0x5a
    ld d, [sp, b]          => 0x5b
    ld d, [sp, c]          => 0x5c
    ld d, [sp, d]          => 0x5d

    st [{abs: u16}], a     => 0x5e @ le(abs)
    st [x, #{imm: s8}], a  => 0x5f @ imm
    st [y, #{imm: s8}], a  => 0x60 @ imm
    st [sp, #{imm: s8}], a => 0x61 @ imm
    st [x, a], a           => 0x62
    st [x, b], a           => 0x63
    st [x, c], a           => 0x64
    st [x, d], a           => 0x65
    st [y, a], a           => 0x66
    st [y, b], a           => 0x67
    st [y, c], a           => 0x68
    st [y, d], a           => 0x69
    st [sp, a], a          => 0x6a
    st [sp, b], a          => 0x6b
    st [sp, c], a          => 0x6c
    st [sp, d], a          => 0x6d

    st [{abs: u16}], b     => 0x6e @ le(abs)
    st [x, #{imm: s8}], b  => 0x6f @ imm
    st [y, #{imm: s8}], b  => 0x70 @ imm
    st [sp, #{imm: s8}], b => 0x71 @ imm
    st [x, a], b           => 0x72
    st [x, b], b           => 0x73
    st [x, c], b           => 0x74
    st [x, d], b           => 0x75
    st [y, a], b           => 0x76
    st [y, b], b           => 0x77
    st [y, c], b           => 0x78
    st [y, d], b           => 0x79
    st [sp, a], b          => 0x7a
    st [sp, b], b          => 0x7b
    st [sp, c], b          => 0x7c
    st [sp, d], b          => 0x7d

    st [{abs: u16}], c     => 0x7e @ le(abs)
    st [x, #{imm: s8}], c  => 0x7f @ imm
    st [y, #{imm: s8}], c  => 0x80 @ imm
    st [sp, #{imm: s8}], c => 0x81 @ imm
    st [x, a], c           => 0x82
    st [x, b], c           => 0x83
    st [x, c], c           => 0x84
    st [x, d], c           => 0x85
    st [y, a], c           => 0x86
    st [y, b], c           => 0x87
    st [y, c], c           => 0x88
    st [y, d], c           => 0x89
    st [sp, a], c          => 0x8a
    st [sp, b], c          => 0x8b
    st [sp, c], c          => 0x8c
    st [sp, d], c          => 0x8d

    st [{abs: u16}], d     => 0x8e @ le(abs)
    st [x, #{imm: s8}], d  => 0x8f @ imm
    st [y, #{imm: s8}], d  => 0x90 @ imm
    st [sp, #{imm: s8}], d => 0x91 @ imm
    st [x, a], d           => 0x92
    st [x, b], d           => 0x93
    st [x, c], d           => 0x94
    st [x, d], d           => 0x95
    st [y, a], d           => 0x96
    st [y, b], d           => 0x97
    st [y, c], d           => 0x98
    st [y, d], d           => 0x99
    st [sp, a], d          => 0x9a
    st [sp, b], d          => 0x9b
    st [sp, c], d          => 0x9c
    st [sp, d], d          => 0x9d

    in a, [{port: u8}]   => 0x9e @ port
    in a, [x, {imm: s8}] => 0x9f @ imm
    in a, [y, {imm: s8}] => 0xa0 @ imm
    in a, [x, a]         => 0xa1
    in a, [x, b]         => 0xa2
    in a, [x, c]         => 0xa3
    in a, [x, d]         => 0xa4
    in a, [y, a]         => 0xa5
    in a, [y, b]         => 0xa6
    in a, [y, c]         => 0xa7
    in a, [y, d]         => 0xa8

    in b, [{port: u8}]   => 0xa9 @ port
    in b, [x, {imm: s8}] => 0xaa @ imm
    in b, [y, {imm: s8}] => 0xab @ imm
    in b, [x, a]         => 0xac
    in b, [x, b]         => 0xad
    in b, [x, c]         => 0xae
    in b, [x, d]         => 0xaf
    in b, [y, a]         => 0xb0
    in b, [y, b]         => 0xb1
    in b, [y, c]         => 0xb2
    in b, [y, d]         => 0xb3

    in c, [{port: u8}]   => 0xb4 @ port
    in c, [x, {imm: s8}] => 0xb5 @ imm
    in c, [y, {imm: s8}] => 0xb6 @ imm
    in c, [x, a]         => 0xb7
    in c, [x, b]         => 0xb8
    in c, [x, c]         => 0xb9
    in c, [x, d]         => 0xba
    in c, [y, a]         => 0xbb
    in c, [y, b]         => 0xbc
    in c, [y, c]         => 0xbd
    in c, [y, d]         => 0xbe

    in d, [{port: u8}]   => 0xbf @ port
    in d, [x, {imm: s8}] => 0xc0 @ imm
    in d, [y, {imm: s8}] => 0xc1 @ imm
    in d, [x, a]         => 0xc2
    in d, [x, b]         => 0xc3
    in d, [x, c]         => 0xc4
    in d, [x, d]         => 0xc5
    in d, [y, a]         => 0xc6
    in d, [y, b]         => 0xc7
    in d, [y, c]         => 0xc8
    in d, [y, d]         => 0xc9

    out [{port: u8}], a   => 0xca @ port
    out [x, {imm: s8}], a => 0xcb @ imm
    out [y, {imm: s8}], a => 0xcc @ imm
    out [x, a], a         => 0xcd
    out [x, b], a         => 0xce
    out [x, c], a         => 0xcf
    out [x, d], a         => 0xd0
    out [y, a], a         => 0xd1
    out [y, b], a         => 0xd2
    out [y, c], a         => 0xd3
    out [y, d], a         => 0xd4

    out [{port: u8}], b   => 0xd5 @ port
    out [x, {imm: s8}], b => 0xd6 @ imm
    out [y, {imm: s8}], b => 0xd7 @ imm
    out [x, a], b         => 0xd8
    out [x, b], b         => 0xd9
    out [x, c], b         => 0xda
    out [x, d], b         => 0xdb
    out [y, a], b         => 0xdc
    out [y, b], b         => 0xdd
    out [y, c], b         => 0xde
    out [y, d], b         => 0xdf

    out [{port: u8}], c   => 0xe0 @ port
    out [x, {imm: s8}], c => 0xe1 @ imm
    out [y, {imm: s8}], c => 0xe2 @ imm
    out [x, a], c         => 0xe3
    out [x, b], c         => 0xe4
    out [x, c], c         => 0xe5
    out [x, d], c         => 0xe6
    out [y, a], c         => 0xe7
    out [y, b], c         => 0xe8
    out [y, c], c         => 0xe9
    out [y, d], c         => 0xea

    out [{port: u8}], d   => 0xeb @ port
    out [x, {imm: s8}], d => 0xec @ imm
    out [y, {imm: s8}], d => 0xed @ imm
    out [x, a], d         => 0xee
    out [x, b], d         => 0xef
    out [x, c], d         => 0xf0
    out [x, d], d         => 0xf1
    out [y, a], d         => 0xf2
    out [y, b], d         => 0xf3
    out [y, c], d         => 0xf4
    out [y, d], d         => 0xf5

    mv x, sp  => 0xf6
    mv sp, x  => 0xf7

    mv x, x   => 0xf8
    mv x, y   => 0xf9
    mv x, ab  => 0xfa
    mv x, cd  => 0xfb

    mv y, x   => 0xfc
    mv y, y   => 0xfd
    mv y, ab  => 0xfe
    mv y, cd  => 0xff

    mv ab, x  => EXT @ 0x00
    mv ab, y  => EXT @ 0x01

    mv cd, x  => EXT @ 0x02
    mv cd, y  => EXT @ 0x03

    ld x, #{imm: i16}  => EXT @ 0x04 @ le(imm)
    ld y, #{imm: i16}  => EXT @ 0x05 @ le(imm)

    ld x, [{abs: u16}]   => EXT @ 0x06 @ le(abs)
    ld x, [x, {imm: s8}] => EXT @ 0x07 @ imm
    ld x, [y, {imm: s8}] => EXT @ 0x08 @ imm
    ld x, [sp, {imm: s8}] => EXT @ 0x09 @ imm

    ld y, [{abs: u16}]   => EXT @ 0x0a @ le(abs)
    ld y, [x, {imm: s8}] => EXT @ 0x0b @ imm
    ld y, [y, {imm: s8}] => EXT @ 0x0c @ imm
    ld y, [sp, {imm: s8}] => EXT @ 0x0d @ imm

    st [{abs: u16}], x    => EXT @ 0x0e @ le(abs)
    st [x, {imm: s8}], x  => EXT @ 0x0f @ imm
    st [y, {imm: s8}], x  => EXT @ 0x10 @ imm
    st [sp, {imm: s8}], x => EXT @ 0x11 @ imm

    st [{abs: u16}], y    => EXT @ 0x12 @ le(abs)
    st [x, {imm: s8}], y  => EXT @ 0x13 @ imm
    st [y, {imm: s8}], y  => EXT @ 0x14 @ imm
    st [sp, {imm: s8}], y => EXT @ 0x15 @ imm

    lea [x, a]          => EXT @ 0x16
    lea [x, b]          => EXT @ 0x17
    lea [x, c]          => EXT @ 0x18
    lea [x, d]          => EXT @ 0x19
    lea [x, {imm: s8}]  => EXT @ 0x1a @ imm

    lea [y, a]          => EXT @ 0x1b
    lea [y, b]          => EXT @ 0x1c
    lea [y, c]          => EXT @ 0x1d
    lea [y, d]          => EXT @ 0x1e
    lea [y, {imm: s8}]  => EXT @ 0x1f @ imm

    lea [sp, a]         => EXT @ 0x20
    lea [sp, b]         => EXT @ 0x21
    lea [sp, c]         => EXT @ 0x22
    lea [sp, d]         => EXT @ 0x23
    lea [sp, {imm: s8}] => EXT @ 0x24 @ imm

    inc x => EXT @ 0x25
    inc y => EXT @ 0x26

    dec x => EXT @ 0x27
    dec y => EXT @ 0x28

    addc a, a => EXT @ 0x29
    addc a, b => EXT @ 0x2a
    addc a, c => EXT @ 0x2b
    addc a, d => EXT @ 0x2c

    addc b, a => EXT @ 0x2d
    addc b, b => EXT @ 0x2e
    addc b, c => EXT @ 0x2f
    addc b, d => EXT @ 0x30

    addc c, a => EXT @ 0x31
    addc c, b => EXT @ 0x32
    addc c, c => EXT @ 0x33
    addc c, d => EXT @ 0x34

    addc d, a => EXT @ 0x35
    addc d, b => EXT @ 0x36
    addc d, c => EXT @ 0x37
    addc d, d => EXT @ 0x38

    addc a, {imm: i8} => EXT @ 0x39 @ imm
    addc b, {imm: i8} => EXT @ 0x3a @ imm
    addc c, {imm: i8} => EXT @ 0x3b @ imm
    addc d, {imm: i8} => EXT @ 0x3c @ imm

    subb a, a => EXT @ 0x3d
    subb a, b => EXT @ 0x3e
    subb a, c => EXT @ 0x3f
    subb a, d => EXT @ 0x40

    subb b, a => EXT @ 0x41
    subb b, b => EXT @ 0x42
    subb b, c => EXT @ 0x43
    subb b, d => EXT @ 0x44

    subb c, a => EXT @ 0x45
    subb c, b => EXT @ 0x46
    subb c, c => EXT @ 0x47
    subb c, d => EXT @ 0x48

    subb d, a => EXT @ 0x49
    subb d, b => EXT @ 0x4a
    subb d, c => EXT @ 0x4b
    subb d, d => EXT @ 0x4c

    subb a, {imm: i8} => EXT @ 0x4d @ imm
    subb b, {imm: i8} => EXT @ 0x4e @ imm
    subb c, {imm: i8} => EXT @ 0x4f @ imm
    subb d, {imm: i8} => EXT @ 0x50 @ imm

    and a, a => EXT @ 0x51
    and a, b => EXT @ 0x52
    and a, c => EXT @ 0x53
    and a, d => EXT @ 0x54

    and b, a => EXT @ 0x55
    and b, b => EXT @ 0x56
    and b, c => EXT @ 0x57
    and b, d => EXT @ 0x58

    and c, a => EXT @ 0x59
    and c, b => EXT @ 0x5a
    and c, c => EXT @ 0x5b
    and c, d => EXT @ 0x5c

    and d, a => EXT @ 0x5d
    and d, b => EXT @ 0x5e
    and d, c => EXT @ 0x5f
    and d, d => EXT @ 0x60

    and a, {imm: i8} => EXT @ 0x61 @ imm
    and b, {imm: i8} => EXT @ 0x62 @ imm
    and c, {imm: i8} => EXT @ 0x63 @ imm
    and d, {imm: i8} => EXT @ 0x64 @ imm

    or a, a  => EXT @ 0x65
    or a, b  => EXT @ 0x66
    or a, c  => EXT @ 0x67
    or a, d  => EXT @ 0x68

    or b, a  => EXT @ 0x69
    or b, b  => EXT @ 0x6a
    or b, c  => EXT @ 0x6b
    or b, d  => EXT @ 0x6c

    or c, a  => EXT @ 0x6d
    or c, b  => EXT @ 0x6e
    or c, c  => EXT @ 0x6f
    or c, d  => EXT @ 0x70

    or d, a  => EXT @ 0x71
    or d, b  => EXT @ 0x72
    or d, c  => EXT @ 0x73
    or d, d  => EXT @ 0x74

    or a, {imm: i8} => EXT @ 0x75 @ imm
    or b, {imm: i8} => EXT @ 0x76 @ imm
    or c, {imm: i8} => EXT @ 0x77 @ imm
    or d, {imm: i8} => EXT @ 0x78 @ imm

    xor a, a => EXT @ 0x79
    xor a, b => EXT @ 0x7a
    xor a, c => EXT @ 0x7b
    xor a, d => EXT @ 0x7c

    xor b, a => EXT @ 0x7d
    xor b, b => EXT @ 0x7e
    xor b, c => EXT @ 0x7f
    xor b, d => EXT @ 0x80

    xor c, a => EXT @ 0x81
    xor c, b => EXT @ 0x82
    xor c, c => EXT @ 0x83
    xor c, d => EXT @ 0x84

    xor d, a => EXT @ 0x85
    xor d, b => EXT @ 0x86
    xor d, c => EXT @ 0x87
    xor d, d => EXT @ 0x88

    xor a, {imm: i8} => EXT @ 0x89 @ imm
    xor b, {imm: i8} => EXT @ 0x8a @ imm
    xor c, {imm: i8} => EXT @ 0x8b @ imm
    xor d, {imm: i8} => EXT @ 0x8c @ imm

    shl a    => EXT @ 0x8d
    shl b    => EXT @ 0x8e
    shl c    => EXT @ 0x8f
    shl d    => EXT @ 0x90

    shr a    => EXT @ 0x91
    shr b    => EXT @ 0x92
    shr c    => EXT @ 0x93
    shr d    => EXT @ 0x94

    asr a    => EXT @ 0x95
    asr b    => EXT @ 0x96
    asr c    => EXT @ 0x97
    asr d    => EXT @ 0x98

    not a => EXT @ 0x99
    not b => EXT @ 0x9a
    not c => EXT @ 0x9b
    not d => EXT @ 0x9c

    neg a => EXT @ 0x9d
    neg b => EXT @ 0x9e
    neg c => EXT @ 0x9f
    neg d => EXT @ 0xa0

    inc a => EXT @ 0xa1
    inc b => EXT @ 0xa2
    inc c => EXT @ 0xa3
    inc d => EXT @ 0xa4

    dec a => EXT @ 0xa5
    dec b => EXT @ 0xa6
    dec c => EXT @ 0xa7
    dec d => EXT @ 0xa8

    cmp a, a => EXT @ 0xa9
    cmp a, b => EXT @ 0xaa
    cmp a, c => EXT @ 0xab
    cmp a, d => EXT @ 0xac

    cmp b, a => EXT @ 0xad
    cmp b, b => EXT @ 0xae
    cmp b, c => EXT @ 0xaf
    cmp b, d => EXT @ 0xb0

    cmp c, a => EXT @ 0xb1
    cmp c, b => EXT @ 0xb2
    cmp c, c => EXT @ 0xb3
    cmp c, d => EXT @ 0xb4

    cmp d, a => EXT @ 0xb5
    cmp d, b => EXT @ 0xb6
    cmp d, c => EXT @ 0xb7
    cmp d, d => EXT @ 0xb8

    cmp a, {imm: i8} => EXT @ 0xb9 @ imm
    cmp b, {imm: i8} => EXT @ 0xba @ imm
    cmp c, {imm: i8} => EXT @ 0xbb @ imm
    cmp d, {imm: i8} => EXT @ 0xbc @ imm

    test a => EXT @ 0xbd
    test b => EXT @ 0xbe
    test c => EXT @ 0xbf
    test d => EXT @ 0xc0

    push a => EXT @ 0xc1
    push b => EXT @ 0xc2
    push c => EXT @ 0xc3
    push d => EXT @ 0xc4

    push x => EXT @ 0xc5
    push y => EXT @ 0xc6

    pop a => EXT @ 0xc7
    pop b => EXT @ 0xc8
    pop c => EXT @ 0xc9
    pop d => EXT @ 0xca

    pop x => EXT @ 0xcb
    pop y => EXT @ 0xcc

    call {rel: s8}         => EXT @ 0xcd @ rel
    call {abs: u16}        => EXT @ 0xce @ le(abs)
    call.abs {abs: u16}    => EXT @ 0xce @ le(abs)
    call (x, {idx: s8})    => EXT @ 0xcf @ idx
    call (y, {idx: s8})    => EXT @ 0xd0 @ idx
    ret                    => EXT @ 0xd1

    swi                    => EXT @ 0xd2
    reti                   => EXT @ 0xd3

    jmp {rel: s8}          => EXT @ 0xd4 @ rel
    jmp {abs: u16}         => EXT @ 0xd5 @ le(abs)
    jmp (x, {idx: s8})     => EXT @ 0xd6 @ idx
    jmp (y, {idx: s8})     => EXT @ 0xd7 @ idx

    br.eq {rel: s8}        => EXT @ 0xd8 @ rel
    br.eq.abs {abs: u16}   => EXT @ 0xd9 @ le(abs)
    br.eq (x, {idx: s8})   => EXT @ 0xda @ idx
    br.eq (y, {idx: s8})   => EXT @ 0xdb @ idx

    br.ne {rel: s8}        => EXT @ 0xdc @ rel
    br.ne {abs: u16}       => EXT @ 0xdd @ le(abs)
    br.ne.abs {abs: u16}   => EXT @ 0xdd @ le(abs)
    br.ne (x, {idx: s8})   => EXT @ 0xde @ idx
    br.ne (y, {idx: s8})   => EXT @ 0xdf @ idx

    br.lt {rel: s8}        => EXT @ 0xe0 @ rel
    br.lt {abs: u16}       => EXT @ 0xe1 @ le(abs)
    br.lt (x, {idx: s8})   => EXT @ 0xe2 @ idx
    br.lt (y, {idx: s8})   => EXT @ 0xe3 @ idx

    br.gt {rel: s8}        => EXT @ 0xe4 @ rel
    br.gt {abs: u16}       => EXT @ 0xe5 @ le(abs)
    br.gt (x, {idx: s8})   => EXT @ 0xe6 @ idx
    br.gt (y, {idx: s8})   => EXT @ 0xe7 @ idx

    br.le {rel: s8}        => EXT @ 0xe8 @ rel
    br.le {abs: u16}       => EXT @ 0xe9 @ le(abs)
    br.le (x, {idx: s8})   => EXT @ 0xea @ idx
    br.le (y, {idx: s8})   => EXT @ 0xeb @ idx

    br.ge {rel: s8}        => EXT @ 0xec @ rel
    br.ge {abs: u16}       => EXT @ 0xed @ le(abs)
    br.ge (x, {idx: s8})   => EXT @ 0xee @ idx
    br.ge (y, {idx: s8})   => EXT @ 0xef @ idx

    br.lts {rel: s8}       => EXT @ 0xf0 @ rel
    br.lts {abs: u16}      => EXT @ 0xf1 @ le(abs)
    br.lts (x, {idx: s8})  => EXT @ 0xf2 @ idx
    br.lts (y, {idx: s8})  => EXT @ 0xf3 @ idx

    br.gts {rel: s8}       => EXT @ 0xf4 @ rel
    br.gts {abs: u16}      => EXT @ 0xf5 @ le(abs)
    br.gts (x, {idx: s8})  => EXT @ 0xf6 @ idx
    br.gts (y, {idx: s8})  => EXT @ 0xf7 @ idx

    br.les {rel: s8}       => EXT @ 0xf8 @ rel
    br.les {abs: u16}      => EXT @ 0xf9 @ le(abs)
    br.les (x, {idx: s8})  => EXT @ 0xfa @ idx
    br.les (y, {idx: s8})  => EXT @ 0xfb @ idx

    br.ges {rel: s8}       => EXT @ 0xfc @ rel
    br.ges {abs: u16}      => EXT @ 0xfd @ le(abs)
    br.ges (x, {idx: s8})  => EXT @ 0xfe @ idx
    br.ges (y, {idx: s8})  => EXT @ 0xff @ idx
}
#ruledef psuedo {
    jmp.abs {abs: u16} => EXT @ 0xd5 @ le(abs)
}
