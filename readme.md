# `bw8`

`bw8` is an 8-bit instruction set, processor, and computer system. This Rust workspace includes multiple crates that each represent a different aspect of the collective project.

## `isa`

Provides types modeling the instruction set, including its instructions and their constituent opcodes. The authoritative source of truth of the mapping from machine code bytes to opcodes is provided by this crate.

## `arch`

Provides types modeling the processor's architectural features and it's system bus. Also emulates the processor's execution in accordance with the model defined in `isa`.

## `asm`

Implements an assembler capable of compiling instruction mnemonics to machine code binaries.

## `emu`

Implements an emulation of the computer system; the processor and it's peripherals.

## `uarch`

Provides types modeling the processor's micro-architectural features; that is, the processor's internal state vector, control bus, and other internal registers.

## `uasm`

Implements a microcode assembler capable of producing micro-architectural control bus words for all state vector values. These consist of the current opcode, status flags, the sequencer value, and other elements.
