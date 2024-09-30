use crate::{Address, Byte};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Pointer {
    X,
    Y,
    SP,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Register8 {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Register16 {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Memory8Mode {
    Absolute(Address),
    ConstantOffset(Pointer, Byte),
    RegisterOffset(Pointer, Register8),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum IOMode {
    Port(Byte),
    ConstantOffset(Register16, Byte),
    RegisterOffset(Register16, Register8),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Memory16Mode {
    Absolute(Address),
    ConstantOffset(Pointer, Byte),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum LeaMode {
    Register(Register8),
    Constant(Byte),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum RegisterPair {
    Ab,
    Cd,
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum Alu2Op {
    Addc,
    Subb,
    And,
    Or,
    Xor,
    Cmp,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Alu2OpMode {
    Register(Register8),
    Constant(Byte),
}

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
pub enum Alu1Op {
    Shl,
    Shr,
    Asr,
    Not,
    Neg,
    Inc,
    Dec,
    Test,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum JumpMode {
    Relative(Byte),
    Absolute(Address),
    Indirect(Register16, Byte),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Condition {
    Always,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    LessThanSigned,
    GreaterThanSigned,
    LessEqualSigned,
    GreaterEqualSigned,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Nop,
    SetCarry,
    ClearCarry,
    SetInterruptEnable,
    ClearInterruptEnable,
    SetBankEnable,
    ClearBankEnable,
    ReadBankRegister,
    WriteBankRegister,
    Move8(Register8, Register8),
    Load8Immediate(Register8, Byte),
    Load8(Register8, Memory8Mode),
    Store8(Memory8Mode, Register8),
    In(Register8, IOMode),
    Out(IOMode, Register8),
    ReadStackPointer,
    WriteStackPointer,
    Move16(Register16, Register16),
    Move16FromPair(Register16, RegisterPair),
    Move16ToPair(RegisterPair, Register16),
    Load16Immediate(Register16, Address),
    Load16(Register16, Memory16Mode),
    Store16(Memory16Mode, Register16),
    Lea(Pointer, LeaMode),
    Inc16(Register16),
    Dec16(Register16),
    Alu2(Alu2Op, Register8, Alu2OpMode),
    Alu1(Alu1Op, Register8),
    Push8(Register8),
    Push16(Register16),
    Pop8(Register8),
    Pop16(Register16),
    Call(JumpMode),
    Ret,
    Swi,
    Reti,
    Jmp(Condition, JumpMode),
}
