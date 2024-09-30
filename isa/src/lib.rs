mod instruction;
mod opcode;

use instruction::*;
use opcode::*;

pub type Address = u16;
pub type Byte = u8;
pub type Word = u16;

pub enum ExtensionMode {
    Normal,
    Extended,
}

pub enum InstructionBytes {
    One(Byte),
    Two(Byte, Byte),
    Three(Byte, Byte, Byte),
    Four(Byte, Byte, Byte, Byte),
}

pub fn fetch_word(stream: &mut impl Iterator<Item = Byte>) -> Option<Word> {
    let low = stream.next()?;
    let high = stream.next()?;
    Some(((high as u16) << 8) | ((low as u16) << 0))    
}

pub fn decode(
    extension_mode: ExtensionMode,
    stream: &mut impl Iterator<Item = Byte>,
) -> Option<Instruction> {
    use Instruction as Inst;
    use Memory8Mode as Mem8;
    use Memory16Mode as Mem16;
    use Pointer as Ptr;
    use Pointer::SP;
    use Register8::A;
    use Register8::B;
    use Register8::C;
    use Register8::D;

    let byte = stream.next()?;

    let instruction = match extension_mode {
        ExtensionMode::Normal => match Opcode::Normal(byte) {
            Opcode::Extended(_) => panic!(),

            NOP => Instruction::Nop,
            EXT => decode(ExtensionMode::Extended, stream)?,
            SET_C => Inst::SetCarry,
            CLR_C => Inst::ClearCarry,

            SET_I => Inst::SetInterruptEnable,
            CLR_I => Inst::ClearInterruptEnable,

            SET_B => Inst::SetBankEnable,
            CLR_B => Inst::ClearBankEnable,

            MV_A_BR => Inst::ReadBankRegister,
            MV_BR_A => Inst::WriteBankRegister,

            MV_A_A => Inst::Move8(A, A),
            MV_A_B => Inst::Move8(A, B),
            MV_A_C => Inst::Move8(A, C),
            MV_A_D => Inst::Move8(A, D),
            MV_B_A => Inst::Move8(B, A),
            MV_B_B => Inst::Move8(B, B),
            MV_B_C => Inst::Move8(B, C),
            MV_B_D => Inst::Move8(B, D),
            MV_C_A => Inst::Move8(C, A),
            MV_C_B => Inst::Move8(C, B),
            MV_C_C => Inst::Move8(C, C),
            MV_C_D => Inst::Move8(C, D),
            MV_D_A => Inst::Move8(D, A),
            MV_D_B => Inst::Move8(D, B),
            MV_D_C => Inst::Move8(D, C),
            MV_D_D => Inst::Move8(D, D),

            LD_A_IMM => Inst::Load8Immediate(A, stream.next()?),
            LD_B_IMM => Inst::Load8Immediate(B, stream.next()?),
            LD_C_IMM => Inst::Load8Immediate(C, stream.next()?),
            LD_D_IMM => Inst::Load8Immediate(D, stream.next()?),

            LD_A_ABS => Inst::Load8(A, Mem8::Absolute(fetch_word(stream)?)),
            LD_A_REL_X_BY_IMM => Inst::Load8(A, Mem8::ConstantOffset(Ptr::X, stream.next()?)),
            LD_A_REL_Y_BY_IMM => Inst::Load8(A, Mem8::ConstantOffset(Ptr::Y, stream.next()?)),
            LD_A_REL_SP_BY_IMM => Inst::Load8(A, Mem8::ConstantOffset(SP, stream.next()?)),
            LD_A_REL_X_BY_A => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, A)),
            LD_A_REL_X_BY_B => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, B)),
            LD_A_REL_X_BY_C => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, C)),
            LD_A_REL_X_BY_D => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, D)),
            LD_A_REL_Y_BY_A => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, A)),
            LD_A_REL_Y_BY_B => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, B)),
            LD_A_REL_Y_BY_C => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, C)),
            LD_A_REL_Y_BY_D => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, D)),
            LD_A_REL_SP_BY_A => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, A)),
            LD_A_REL_SP_BY_B => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, B)),
            LD_A_REL_SP_BY_C => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, C)),
            LD_A_REL_SP_BY_D => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, D)),

            LD_B_ABS => Inst::Load8(B, Mem8::Absolute(fetch_word(stream)?)),
            LD_B_REL_X_BY_IMM => Inst::Load8(B, Mem8::ConstantOffset(Ptr::X, stream.next()?)),
            LD_B_REL_Y_BY_IMM => Inst::Load8(B, Mem8::ConstantOffset(Ptr::Y, stream.next()?)),
            LD_B_REL_SP_BY_IMM => Inst::Load8(B, Mem8::ConstantOffset(Ptr::SP, stream.next()?)),
            LD_B_REL_X_BY_A => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, A)),
            LD_B_REL_X_BY_B => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, B)),
            LD_B_REL_X_BY_C => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, C)),
            LD_B_REL_X_BY_D => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, D)),
            LD_B_REL_Y_BY_A => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, A)),
            LD_B_REL_Y_BY_B => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, B)),
            LD_B_REL_Y_BY_C => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, C)),
            LD_B_REL_Y_BY_D => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, D)),
            LD_B_REL_SP_BY_A => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, A)),
            LD_B_REL_SP_BY_B => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, B)),
            LD_B_REL_SP_BY_C => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, C)),
            LD_B_REL_SP_BY_D => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, D)),

            LD_C_ABS => Inst::Load8(C, Mem8::Absolute(fetch_word(stream)?)),
            LD_C_REL_X_BY_IMM => Inst::Load8(C, Mem8::ConstantOffset(Ptr::X, stream.next()?)),
            LD_C_REL_Y_BY_IMM => Inst::Load8(C, Mem8::ConstantOffset(Ptr::Y, stream.next()?)),
            LD_C_REL_SP_BY_IMM => Inst::Load8(C, Mem8::ConstantOffset(Ptr::SP, stream.next()?)),
            LD_C_REL_X_BY_A => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, A)),
            LD_C_REL_X_BY_B => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, B)),
            LD_C_REL_X_BY_C => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, C)),
            LD_C_REL_X_BY_D => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, D)),
            LD_C_REL_Y_BY_A => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, A)),
            LD_C_REL_Y_BY_B => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, B)),
            LD_C_REL_Y_BY_C => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, C)),
            LD_C_REL_Y_BY_D => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, D)),
            LD_C_REL_SP_BY_A => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, A)),
            LD_C_REL_SP_BY_B => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, B)),
            LD_C_REL_SP_BY_C => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, C)),
            LD_C_REL_SP_BY_D => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, D)),

            LD_D_ABS => Inst::Load8(D, Mem8::Absolute(fetch_word(stream)?)),
            LD_D_REL_X_BY_IMM => Inst::Load8(D, Mem8::ConstantOffset(Ptr::X, stream.next()?)),
            LD_D_REL_Y_BY_IMM => Inst::Load8(D, Mem8::ConstantOffset(Ptr::Y, stream.next()?)),
            LD_D_REL_SP_BY_IMM => Inst::Load8(D, Mem8::ConstantOffset(Ptr::SP, stream.next()?)),
            LD_D_REL_X_BY_A => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, A)),
            LD_D_REL_X_BY_B => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, B)),
            LD_D_REL_X_BY_C => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, C)),
            LD_D_REL_X_BY_D => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, D)),
            LD_D_REL_Y_BY_A => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, A)),
            LD_D_REL_Y_BY_B => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, B)),
            LD_D_REL_Y_BY_C => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, C)),
            LD_D_REL_Y_BY_D => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, D)),
            LD_D_REL_SP_BY_A => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, A)),
            LD_D_REL_SP_BY_B => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, B)),
            LD_D_REL_SP_BY_C => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, C)),
            LD_D_REL_SP_BY_D => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, D)),

            ST_ABS_A => Inst::Store8(Mem8::Absolute(fetch_word(stream)?), A),
            ST_REL_X_BY_IMM_A => Inst::Store8(Mem8::ConstantOffset(Ptr::X, stream.next()?), A),
            ST_REL_Y_BY_IMM_A => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, stream.next()?), A),
            ST_REL_SP_BY_IMM_A => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, stream.next()?), A),
            ST_REL_X_BY_A_A => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), A),
            ST_REL_X_BY_B_A => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), A),
            ST_REL_X_BY_C_A => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), A),
            ST_REL_X_BY_D_A => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), A),
            ST_REL_Y_BY_A_A => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), A),
            ST_REL_Y_BY_B_A => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), A),
            ST_REL_Y_BY_C_A => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), A),
            ST_REL_Y_BY_D_A => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), A),
            ST_REL_SP_BY_A_A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), A),
            ST_REL_SP_BY_B_A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), A),
            ST_REL_SP_BY_C_A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), A),
            ST_REL_SP_BY_D_A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), A),

            ST_ABS_B => Inst::Store8(Mem8::Absolute(fetch_word(stream)?), B),
            ST_REL_X_BY_IMM_B => Inst::Store8(Mem8::ConstantOffset(Ptr::X, stream.next()?), B),
            ST_REL_Y_BY_IMM_B => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, stream.next()?), B),
            ST_REL_SP_BY_IMM_B => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, stream.next()?), B),
            ST_REL_X_BY_A_B => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), B),
            ST_REL_X_BY_B_B => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), B),
            ST_REL_X_BY_C_B => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), B),
            ST_REL_X_BY_D_B => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), B),
            ST_REL_Y_BY_A_B => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), B),
            ST_REL_Y_BY_B_B => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), B),
            ST_REL_Y_BY_C_B => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), B),
            ST_REL_Y_BY_D_B => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), B),
            ST_REL_SP_BY_A_B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), B),
            ST_REL_SP_BY_B_B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), B),
            ST_REL_SP_BY_C_B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), B),
            ST_REL_SP_BY_D_B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), B),

            ST_ABS_C => Inst::Store8(Mem8::Absolute(fetch_word(stream)?), C),
            ST_REL_X_BY_IMM_C => Inst::Store8(Mem8::ConstantOffset(Ptr::X, stream.next()?), C),
            ST_REL_Y_BY_IMM_C => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, stream.next()?), C),
            ST_REL_SP_BY_IMM_C => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, stream.next()?), C),
            ST_REL_X_BY_A_C => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), C),
            ST_REL_X_BY_B_C => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), C),
            ST_REL_X_BY_C_C => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), C),
            ST_REL_X_BY_D_C => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), C),
            ST_REL_Y_BY_A_C => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), C),
            ST_REL_Y_BY_B_C => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), C),
            ST_REL_Y_BY_C_C => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), C),
            ST_REL_Y_BY_D_C => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), C),
            ST_REL_SP_BY_A_C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), C),
            ST_REL_SP_BY_B_C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), C),
            ST_REL_SP_BY_C_C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), C),
            ST_REL_SP_BY_D_C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), C),

            ST_ABS_D => Inst::Store8(Mem8::Absolute(fetch_word(stream)?), D),
            ST_REL_X_BY_IMM_D => Inst::Store8(Mem8::ConstantOffset(Ptr::X, stream.next()?), D),
            ST_REL_Y_BY_IMM_D => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, stream.next()?), D),
            ST_REL_SP_BY_IMM_D => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, stream.next()?), D),
            ST_REL_X_BY_A_D => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), D),
            ST_REL_X_BY_B_D => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), D),
            ST_REL_X_BY_C_D => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), D),
            ST_REL_X_BY_D_D => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), D),
            ST_REL_Y_BY_A_D => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), D),
            ST_REL_Y_BY_B_D => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), D),
            ST_REL_Y_BY_C_D => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), D),
            ST_REL_Y_BY_D_D => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), D),
            ST_REL_SP_BY_A_D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), D),
            ST_REL_SP_BY_B_D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), D),
            ST_REL_SP_BY_C_D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), D),
            ST_REL_SP_BY_D_D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), D),

            IN_A_PORT => Inst::In(A, IOMode::Port(stream.next()?)),
            IN_A_REL_X_BY_IMM => Inst::In(A, IOMode::ConstantOffset(Register16::X, stream.next()?)),
            IN_A_REL_Y_BY_IMM => Inst::In(A, IOMode::ConstantOffset(Register16::Y, stream.next()?)),
            IN_A_REL_X_BY_A => Inst::In(A, IOMode::RegisterOffset(Register16::X, A)),
            IN_A_REL_X_BY_B => Inst::In(A, IOMode::RegisterOffset(Register16::X, B)),
            IN_A_REL_X_BY_C => Inst::In(A, IOMode::RegisterOffset(Register16::X, C)),
            IN_A_REL_X_BY_D => Inst::In(A, IOMode::RegisterOffset(Register16::X, D)),
            IN_A_REL_Y_BY_A => Inst::In(A, IOMode::RegisterOffset(Register16::Y, A)),
            IN_A_REL_Y_BY_B => Inst::In(A, IOMode::RegisterOffset(Register16::Y, B)),
            IN_A_REL_Y_BY_C => Inst::In(A, IOMode::RegisterOffset(Register16::Y, C)),
            IN_A_REL_Y_BY_D => Inst::In(A, IOMode::RegisterOffset(Register16::Y, D)),

            IN_B_PORT => Inst::In(B, IOMode::Port(stream.next()?)),
            IN_B_REL_X_BY_IMM => Inst::In(B, IOMode::ConstantOffset(Register16::X, stream.next()?)),
            IN_B_REL_Y_BY_IMM => Inst::In(B, IOMode::ConstantOffset(Register16::Y, stream.next()?)),
            IN_B_REL_X_BY_A => Inst::In(B, IOMode::RegisterOffset(Register16::X, A)),
            IN_B_REL_X_BY_B => Inst::In(B, IOMode::RegisterOffset(Register16::X, B)),
            IN_B_REL_X_BY_C => Inst::In(B, IOMode::RegisterOffset(Register16::X, C)),
            IN_B_REL_X_BY_D => Inst::In(B, IOMode::RegisterOffset(Register16::X, D)),
            IN_B_REL_Y_BY_A => Inst::In(B, IOMode::RegisterOffset(Register16::Y, A)),
            IN_B_REL_Y_BY_B => Inst::In(B, IOMode::RegisterOffset(Register16::Y, B)),
            IN_B_REL_Y_BY_C => Inst::In(B, IOMode::RegisterOffset(Register16::Y, C)),
            IN_B_REL_Y_BY_D => Inst::In(B, IOMode::RegisterOffset(Register16::Y, D)),

            IN_C_PORT => Inst::In(C, IOMode::Port(stream.next()?)),
            IN_C_REL_X_BY_IMM => Inst::In(C, IOMode::ConstantOffset(Register16::X, stream.next()?)),
            IN_C_REL_Y_BY_IMM => Inst::In(C, IOMode::ConstantOffset(Register16::Y, stream.next()?)),
            IN_C_REL_X_BY_A => Inst::In(C, IOMode::RegisterOffset(Register16::X, A)),
            IN_C_REL_X_BY_B => Inst::In(C, IOMode::RegisterOffset(Register16::X, B)),
            IN_C_REL_X_BY_C => Inst::In(C, IOMode::RegisterOffset(Register16::X, C)),
            IN_C_REL_X_BY_D => Inst::In(C, IOMode::RegisterOffset(Register16::X, D)),
            IN_C_REL_Y_BY_A => Inst::In(C, IOMode::RegisterOffset(Register16::Y, A)),
            IN_C_REL_Y_BY_B => Inst::In(C, IOMode::RegisterOffset(Register16::Y, B)),
            IN_C_REL_Y_BY_C => Inst::In(C, IOMode::RegisterOffset(Register16::Y, C)),
            IN_C_REL_Y_BY_D => Inst::In(C, IOMode::RegisterOffset(Register16::Y, D)),

            IN_D_PORT => Inst::In(D, IOMode::Port(stream.next()?)),
            IN_D_REL_X_BY_IMM => Inst::In(D, IOMode::ConstantOffset(Register16::X, stream.next()?)),
            IN_D_REL_Y_BY_IMM => Inst::In(D, IOMode::ConstantOffset(Register16::Y, stream.next()?)),
            IN_D_REL_X_BY_A => Inst::In(D, IOMode::RegisterOffset(Register16::X, A)),
            IN_D_REL_X_BY_B => Inst::In(D, IOMode::RegisterOffset(Register16::X, B)),
            IN_D_REL_X_BY_C => Inst::In(D, IOMode::RegisterOffset(Register16::X, C)),
            IN_D_REL_X_BY_D => Inst::In(D, IOMode::RegisterOffset(Register16::X, D)),
            IN_D_REL_Y_BY_A => Inst::In(D, IOMode::RegisterOffset(Register16::Y, A)),
            IN_D_REL_Y_BY_B => Inst::In(D, IOMode::RegisterOffset(Register16::Y, B)),
            IN_D_REL_Y_BY_C => Inst::In(D, IOMode::RegisterOffset(Register16::Y, C)),
            IN_D_REL_Y_BY_D => Inst::In(D, IOMode::RegisterOffset(Register16::Y, D)),

            OUT_PORT_A => Inst::Out(IOMode::Port(stream.next()?), A),
            OUT_REL_X_BY_IMM_A => Inst::Out(IOMode::ConstantOffset(Register16::X, stream.next()?), A),
            OUT_REL_Y_BY_IMM_A => Inst::Out(IOMode::ConstantOffset(Register16::Y, stream.next()?), A),
            OUT_REL_X_BY_A_A => Inst::Out(IOMode::RegisterOffset(Register16::X, A), A),
            OUT_REL_X_BY_B_A => Inst::Out(IOMode::RegisterOffset(Register16::X, B), A),
            OUT_REL_X_BY_C_A => Inst::Out(IOMode::RegisterOffset(Register16::X, C), A),
            OUT_REL_X_BY_D_A => Inst::Out(IOMode::RegisterOffset(Register16::X, D), A),
            OUT_REL_Y_BY_A_A => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), A),
            OUT_REL_Y_BY_B_A => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), A),
            OUT_REL_Y_BY_C_A => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), A),
            OUT_REL_Y_BY_D_A => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), A),

            OUT_PORT_B => Inst::Out(IOMode::Port(stream.next()?), B),
            OUT_REL_X_BY_IMM_B => Inst::Out(IOMode::ConstantOffset(Register16::X, stream.next()?), B),
            OUT_REL_Y_BY_IMM_B => Inst::Out(IOMode::ConstantOffset(Register16::Y, stream.next()?), B),
            OUT_REL_X_BY_A_B => Inst::Out(IOMode::RegisterOffset(Register16::X, A), B),
            OUT_REL_X_BY_B_B => Inst::Out(IOMode::RegisterOffset(Register16::X, B), B),
            OUT_REL_X_BY_C_B => Inst::Out(IOMode::RegisterOffset(Register16::X, C), B),
            OUT_REL_X_BY_D_B => Inst::Out(IOMode::RegisterOffset(Register16::X, D), B),
            OUT_REL_Y_BY_A_B => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), B),
            OUT_REL_Y_BY_B_B => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), B),
            OUT_REL_Y_BY_C_B => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), B),
            OUT_REL_Y_BY_D_B => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), B),

            OUT_PORT_C => Inst::Out(IOMode::Port(stream.next()?), C),
            OUT_REL_X_BY_IMM_C => Inst::Out(IOMode::ConstantOffset(Register16::X, stream.next()?), C),
            OUT_REL_Y_BY_IMM_C => Inst::Out(IOMode::ConstantOffset(Register16::Y, stream.next()?), C),
            OUT_REL_X_BY_A_C => Inst::Out(IOMode::RegisterOffset(Register16::X, A), C),
            OUT_REL_X_BY_B_C => Inst::Out(IOMode::RegisterOffset(Register16::X, B), C),
            OUT_REL_X_BY_C_C => Inst::Out(IOMode::RegisterOffset(Register16::X, C), C),
            OUT_REL_X_BY_D_C => Inst::Out(IOMode::RegisterOffset(Register16::X, D), C),
            OUT_REL_Y_BY_A_C => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), C),
            OUT_REL_Y_BY_B_C => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), C),
            OUT_REL_Y_BY_C_C => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), C),
            OUT_REL_Y_BY_D_C => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), C),

            OUT_PORT_D => Inst::Out(IOMode::Port(stream.next()?), D),
            OUT_REL_X_BY_IMM_D => Inst::Out(IOMode::ConstantOffset(Register16::X, stream.next()?), D),
            OUT_REL_Y_BY_IMM_D => Inst::Out(IOMode::ConstantOffset(Register16::Y, stream.next()?), D),
            OUT_REL_X_BY_A_D => Inst::Out(IOMode::RegisterOffset(Register16::X, A), D),
            OUT_REL_X_BY_B_D => Inst::Out(IOMode::RegisterOffset(Register16::X, B), D),
            OUT_REL_X_BY_C_D => Inst::Out(IOMode::RegisterOffset(Register16::X, C), D),
            OUT_REL_X_BY_D_D => Inst::Out(IOMode::RegisterOffset(Register16::X, D), D),
            OUT_REL_Y_BY_A_D => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), D),
            OUT_REL_Y_BY_B_D => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), D),
            OUT_REL_Y_BY_C_D => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), D),
            OUT_REL_Y_BY_D_D => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), D),

            MV_X_SP => Inst::ReadStackPointer,
            MV_SP_X => Inst::WriteStackPointer,

            MV_X_X => Inst::Move16(Register16::X, Register16::X),
            MV_X_Y => Inst::Move16(Register16::X, Register16::Y),
            MV_X_AB => Inst::Move16FromPair(Register16::X, RegisterPair::Ab),
            MV_X_CD => Inst::Move16FromPair(Register16::X, RegisterPair::Cd),

            MV_Y_X => Inst::Move16(Register16::Y, Register16::X),
            MV_Y_Y => Inst::Move16(Register16::Y, Register16::Y),
            MV_Y_AB => Inst::Move16FromPair(Register16::Y, RegisterPair::Ab),
            MV_Y_CD => Inst::Move16FromPair(Register16::Y, RegisterPair::Cd),
        },
        ExtensionMode::Extended => match Opcode::Extended(byte) {
            Opcode::Normal(_) => panic!(),

            MV_AB_X => Inst::Move16ToPair(RegisterPair::Ab, Register16::X),
            MV_AB_Y => Inst::Move16ToPair(RegisterPair::Ab, Register16::Y),

            MV_CD_X => Inst::Move16ToPair(RegisterPair::Cd, Register16::X),
            MV_CD_Y => Inst::Move16ToPair(RegisterPair::Cd, Register16::Y),

            LD_X_IMM => Inst::Load16Immediate(Register16::X, fetch_word(stream)?),
            LD_Y_IMM => Inst::Load16Immediate(Register16::Y, fetch_word(stream)?),

            LD_X_ABS => Inst::Load16(Register16::X, Mem16::Absolute(fetch_word(stream)?)),
            LD_X_REL_X_BY_IMM => {
                Inst::Load16(Register16::X, Mem16::ConstantOffset(Ptr::X, stream.next()?))
            }
            LD_X_REL_Y_BY_IMM => {
                Inst::Load16(Register16::X, Mem16::ConstantOffset(Ptr::Y, stream.next()?))
            }
            LD_X_REL_SP_BY_IMM => Inst::Load16(
                Register16::X,
                Mem16::ConstantOffset(Ptr::SP, stream.next()?),
            ),

            LD_Y_ABS => Inst::Load16(Register16::Y, Mem16::Absolute(fetch_word(stream)?)),
            LD_Y_REL_X_BY_IMM => {
                Inst::Load16(Register16::Y, Mem16::ConstantOffset(Ptr::X, stream.next()?))
            }
            LD_Y_REL_Y_BY_IMM => {
                Inst::Load16(Register16::Y, Mem16::ConstantOffset(Ptr::Y, stream.next()?))
            }
            LD_Y_REL_SP_BY_IMM => Inst::Load16(
                Register16::Y,
                Mem16::ConstantOffset(Ptr::SP, stream.next()?),
            ),

            ST_ABS_X => Inst::Store16(Mem16::Absolute(fetch_word(stream)?), Register16::X),
            ST_REL_X_BY_IMM_X => Inst::Store16(Mem16::ConstantOffset(Ptr::X, stream.next()?), Register16::X),
            ST_REL_Y_BY_IMM_X => Inst::Store16(Mem16::ConstantOffset(Ptr::Y, stream.next()?), Register16::X),
            ST_REL_SP_BY_IMM_X => Inst::Store16(
                Mem16::ConstantOffset(Ptr::SP, stream.next()?),
                Register16::X,
            ),

            ST_ABS_Y => Inst::Store16(Mem16::Absolute(fetch_word(stream)?), Register16::Y),
            ST_REL_X_BY_IMM_Y => Inst::Store16(Mem16::ConstantOffset(Ptr::X, stream.next()?), Register16::Y),
            ST_REL_Y_BY_IMM_Y => Inst::Store16(Mem16::ConstantOffset(Ptr::Y, stream.next()?), Register16::Y),
            ST_REL_SP_BY_IMM_Y => Inst::Store16(
                Mem16::ConstantOffset(Ptr::SP, stream.next()?),
                Register16::Y,
            ),

            LEA_X_BY_A => Inst::Lea(Ptr::X, LeaMode::Register(A)),
            LEA_X_BY_B => Inst::Lea(Ptr::X, LeaMode::Register(B)),
            LEA_X_BY_C => Inst::Lea(Ptr::X, LeaMode::Register(C)),
            LEA_X_BY_D => Inst::Lea(Ptr::X, LeaMode::Register(D)),
            LEA_X_BY_IMM => Inst::Lea(Ptr::X, LeaMode::Constant(stream.next()?)),

            LEA_Y_BY_A => Inst::Lea(Ptr::Y, LeaMode::Register(A)),
            LEA_Y_BY_B => Inst::Lea(Ptr::Y, LeaMode::Register(B)),
            LEA_Y_BY_C => Inst::Lea(Ptr::Y, LeaMode::Register(C)),
            LEA_Y_BY_D => Inst::Lea(Ptr::Y, LeaMode::Register(D)),
            LEA_Y_BY_IMM => Inst::Lea(Ptr::Y, LeaMode::Constant(stream.next()?)),

            LEA_SP_BY_A => Inst::Lea(Ptr::SP, LeaMode::Register(A)),
            LEA_SP_BY_B => Inst::Lea(Ptr::SP, LeaMode::Register(B)),
            LEA_SP_BY_C => Inst::Lea(Ptr::SP, LeaMode::Register(C)),
            LEA_SP_BY_D => Inst::Lea(Ptr::SP, LeaMode::Register(D)),
            LEA_SP_BY_IMM => Inst::Lea(Ptr::SP, LeaMode::Constant(stream.next()?)),

            INC_X => Inst::Inc16(Register16::X),
            INC_Y => Inst::Inc16(Register16::Y),

            DEC_X => Inst::Dec16(Register16::X),
            DEC_Y => Inst::Dec16(Register16::Y),

            ADDC_A_A => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(A)),
            ADDC_A_B => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(B)),
            ADDC_A_C => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(C)),
            ADDC_A_D => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(D)),

            ADDC_B_A => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(A)),
            ADDC_B_B => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(B)),
            ADDC_B_C => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(C)),
            ADDC_B_D => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(D)),

            ADDC_C_A => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(A)),
            ADDC_C_B => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(B)),
            ADDC_C_C => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(C)),
            ADDC_C_D => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(D)),

            ADDC_D_A => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(A)),
            ADDC_D_B => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(B)),
            ADDC_D_C => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(C)),
            ADDC_D_D => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(D)),

            ADDC_A_IMM => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Constant(stream.next()?)),
            ADDC_B_IMM => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Constant(stream.next()?)),
            ADDC_C_IMM => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Constant(stream.next()?)),
            ADDC_D_IMM => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Constant(stream.next()?)),

            SUBB_A_A => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(A)),
            SUBB_A_B => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(B)),
            SUBB_A_C => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(C)),
            SUBB_A_D => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(D)),

            SUBB_B_A => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(A)),
            SUBB_B_B => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(B)),
            SUBB_B_C => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(C)),
            SUBB_B_D => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(D)),

            SUBB_C_A => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(A)),
            SUBB_C_B => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(B)),
            SUBB_C_C => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(C)),
            SUBB_C_D => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(D)),

            SUBB_D_A => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(A)),
            SUBB_D_B => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(B)),
            SUBB_D_C => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(C)),
            SUBB_D_D => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(D)),

            SUBB_A_IMM => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Constant(stream.next()?)),
            SUBB_B_IMM => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Constant(stream.next()?)),
            SUBB_C_IMM => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Constant(stream.next()?)),
            SUBB_D_IMM => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Constant(stream.next()?)),

            AND_A_A => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(A)),
            AND_A_B => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(B)),
            AND_A_C => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(C)),
            AND_A_D => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(D)),

            AND_B_A => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(A)),
            AND_B_B => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(B)),
            AND_B_C => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(C)),
            AND_B_D => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(D)),

            AND_C_A => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(A)),
            AND_C_B => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(B)),
            AND_C_C => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(C)),
            AND_C_D => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(D)),

            AND_D_A => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(A)),
            AND_D_B => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(B)),
            AND_D_C => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(C)),
            AND_D_D => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(D)),

            AND_A_IMM => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Constant(stream.next()?)),
            AND_B_IMM => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Constant(stream.next()?)),
            AND_C_IMM => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Constant(stream.next()?)),
            AND_D_IMM => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Constant(stream.next()?)),

            OR_A_A => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(A)),
            OR_A_B => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(B)),
            OR_A_C => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(C)),
            OR_A_D => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(D)),

            OR_B_A => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(A)),
            OR_B_B => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(B)),
            OR_B_C => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(C)),
            OR_B_D => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(D)),

            OR_C_A => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(A)),
            OR_C_B => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(B)),
            OR_C_C => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(C)),
            OR_C_D => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(D)),

            OR_D_A => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(A)),
            OR_D_B => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(B)),
            OR_D_C => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(C)),
            OR_D_D => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(D)),

            OR_A_IMM => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Constant(stream.next()?)),
            OR_B_IMM => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Constant(stream.next()?)),
            OR_C_IMM => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Constant(stream.next()?)),
            OR_D_IMM => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Constant(stream.next()?)),

            XOR_A_A => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(A)),
            XOR_A_B => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(B)),
            XOR_A_C => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(C)),
            XOR_A_D => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(D)),

            XOR_B_A => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(A)),
            XOR_B_B => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(B)),
            XOR_B_C => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(C)),
            XOR_B_D => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(D)),

            XOR_C_A => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(A)),
            XOR_C_B => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(B)),
            XOR_C_C => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(C)),
            XOR_C_D => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(D)),

            XOR_D_A => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(A)),
            XOR_D_B => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(B)),
            XOR_D_C => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(C)),
            XOR_D_D => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(D)),

            XOR_A_IMM => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Constant(stream.next()?)),
            XOR_B_IMM => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Constant(stream.next()?)),
            XOR_C_IMM => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Constant(stream.next()?)),
            XOR_D_IMM => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Constant(stream.next()?)),

            SHL_A => Inst::Alu1(Alu1Op::Shl, A),
            SHL_B => Inst::Alu1(Alu1Op::Shl, B),
            SHL_C => Inst::Alu1(Alu1Op::Shl, C),
            SHL_D => Inst::Alu1(Alu1Op::Shl, D),

            SHR_A => Inst::Alu1(Alu1Op::Shr, A),
            SHR_B => Inst::Alu1(Alu1Op::Shr, B),
            SHR_C => Inst::Alu1(Alu1Op::Shr, C),
            SHR_D => Inst::Alu1(Alu1Op::Shr, D),

            ASR_A => Inst::Alu1(Alu1Op::Asr, A),
            ASR_B => Inst::Alu1(Alu1Op::Asr, B),
            ASR_C => Inst::Alu1(Alu1Op::Asr, C),
            ASR_D => Inst::Alu1(Alu1Op::Asr, D),

            NOT_A => Inst::Alu1(Alu1Op::Not, A),
            NOT_B => Inst::Alu1(Alu1Op::Not, B),
            NOT_C => Inst::Alu1(Alu1Op::Not, C),
            NOT_D => Inst::Alu1(Alu1Op::Not, D),

            NEG_A => Inst::Alu1(Alu1Op::Neg, A),
            NEG_B => Inst::Alu1(Alu1Op::Neg, B),
            NEG_C => Inst::Alu1(Alu1Op::Neg, C),
            NEG_D => Inst::Alu1(Alu1Op::Neg, D),

            INC_A => Inst::Alu1(Alu1Op::Inc, A),
            INC_B => Inst::Alu1(Alu1Op::Inc, B),
            INC_C => Inst::Alu1(Alu1Op::Inc, C),
            INC_D => Inst::Alu1(Alu1Op::Inc, D),

            DEC_A => Inst::Alu1(Alu1Op::Dec, A),
            DEC_B => Inst::Alu1(Alu1Op::Dec, B),
            DEC_C => Inst::Alu1(Alu1Op::Dec, C),
            DEC_D => Inst::Alu1(Alu1Op::Dec, D),

            CMP_A_A => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(A)),
            CMP_A_B => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(B)),
            CMP_A_C => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(C)),
            CMP_A_D => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(D)),

            CMP_B_A => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(A)),
            CMP_B_B => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(B)),
            CMP_B_C => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(C)),
            CMP_B_D => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(D)),

            CMP_C_A => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(A)),
            CMP_C_B => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(B)),
            CMP_C_C => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(C)),
            CMP_C_D => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(D)),

            CMP_D_A => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(A)),
            CMP_D_B => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(B)),
            CMP_D_C => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(C)),
            CMP_D_D => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(D)),

            CMP_A_IMM => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Constant(stream.next()?)),
            CMP_B_IMM => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Constant(stream.next()?)),
            CMP_C_IMM => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Constant(stream.next()?)),
            CMP_D_IMM => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Constant(stream.next()?)),

            TEST_A => Inst::Alu1(Alu1Op::Test, A),
            TEST_B => Inst::Alu1(Alu1Op::Test, B),
            TEST_C => Inst::Alu1(Alu1Op::Test, C),
            TEST_D => Inst::Alu1(Alu1Op::Test, D),

            PUSH_A => Inst::Push8(A),
            PUSH_B => Inst::Push8(B),
            PUSH_C => Inst::Push8(C),
            PUSH_D => Inst::Push8(D),

            PUSH_X => Inst::Push16(Register16::X),
            PUSH_Y => Inst::Push16(Register16::Y),

            POP_A => Inst::Pop8(A),
            POP_B => Inst::Pop8(B),
            POP_C => Inst::Pop8(C),
            POP_D => Inst::Pop8(D),

            POP_X => Inst::Pop16(Register16::X),
            POP_Y => Inst::Pop16(Register16::Y),

            CALL_PC_REL => Inst::Call(JumpMode::Relative(stream.next()?)),
            CALL_ABS => Inst::Call(JumpMode::Absolute(fetch_word(stream)?)),
            CALL_X_REL_IMM => Inst::Call(JumpMode::Indirect(Register16::X, stream.next()?)),
            CALL_Y_REL_IMM => Inst::Call(JumpMode::Indirect(Register16::Y, stream.next()?)),
            RET => Inst::Ret,

            SWI => Inst::Swi,
            RETI => Inst::Reti,

            JMP_PC_REL => Inst::Jmp(Condition::Always, JumpMode::Relative(stream.next()?)),
            JMP_ABS => Inst::Jmp(Condition::Always, JumpMode::Absolute(fetch_word(stream)?)),
            JMP_X_REL_IMM => Inst::Jmp(
                Condition::Always,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            JMP_Y_REL_IMM => Inst::Jmp(
                Condition::Always,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_EQ_PC_REL => Inst::Jmp(Condition::Equal, JumpMode::Relative(stream.next()?)),
            BR_EQ_ABS => Inst::Jmp(Condition::Equal, JumpMode::Absolute(fetch_word(stream)?)),
            BR_EQ_X_REL_IMM => Inst::Jmp(
                Condition::Equal,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_EQ_Y_REL_IMM => Inst::Jmp(
                Condition::Equal,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_NE_PC_REL => Inst::Jmp(Condition::NotEqual, JumpMode::Relative(stream.next()?)),
            BR_NE_ABS => Inst::Jmp(Condition::NotEqual, JumpMode::Absolute(fetch_word(stream)?)),
            BR_NE_X_REL_IMM => Inst::Jmp(
                Condition::NotEqual,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_NE_Y_REL_IMM => Inst::Jmp(
                Condition::NotEqual,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_LT_PC_REL => Inst::Jmp(Condition::LessThan, JumpMode::Relative(stream.next()?)),
            BR_LT_ABS => Inst::Jmp(Condition::LessThan, JumpMode::Absolute(fetch_word(stream)?)),
            BR_LT_X_REL_IMM => Inst::Jmp(
                Condition::LessThan,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_LT_Y_REL_IMM => Inst::Jmp(
                Condition::LessThan,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_GT_PC_REL => Inst::Jmp(Condition::GreaterThan, JumpMode::Relative(stream.next()?)),
            BR_GT_ABS => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_GT_X_REL_IMM => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_GT_Y_REL_IMM => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_LE_PC_REL => Inst::Jmp(Condition::LessEqual, JumpMode::Relative(stream.next()?)),
            BR_LE_ABS => Inst::Jmp(Condition::LessEqual, JumpMode::Absolute(fetch_word(stream)?)),
            BR_LE_X_REL_IMM => Inst::Jmp(
                Condition::LessEqual,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_LE_Y_REL_IMM => Inst::Jmp(
                Condition::LessEqual,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_GE_PC_REL => Inst::Jmp(Condition::GreaterEqual, JumpMode::Relative(stream.next()?)),
            BR_GE_ABS => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_GE_X_REL_IMM => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_GE_Y_REL_IMM => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_LTS_PC_REL => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Relative(stream.next()?),
            ),
            BR_LTS_ABS => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_LTS_X_REL_IMM => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_LTS_Y_REL_IMM => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_GTS_PC_REL => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Relative(stream.next()?),
            ),
            BR_GTS_ABS => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_GTS_X_REL_IMM => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_GTS_Y_REL_IMM => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_LES_PC_REL => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Relative(stream.next()?),
            ),
            BR_LES_ABS => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_LES_X_REL_IMM => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_LES_Y_REL_IMM => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),

            BR_GES_PC_REL => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Relative(stream.next()?),
            ),
            BR_GES_ABS => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Absolute(fetch_word(stream)?),
            ),
            BR_GES_X_REL_IMM => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Indirect(Register16::X, stream.next()?),
            ),
            BR_GES_Y_REL_IMM => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Indirect(Register16::Y, stream.next()?),
            ),
        },
    };

    Some(instruction)
}

pub fn encode(inst: Instruction) -> InstructionBytes {
    use Instruction as Inst;
    use Memory8Mode as Mem8;
    use Register8::A;
    use Register8::B;
    use Register8::C;
    use Register8::D;
    use Pointer::X;
    use Pointer::Y;
    use Pointer::SP;

    match inst {
        Inst::Nop => opcode::NOP.encode(),
        Inst::SetCarry => opcode::SET_C.encode(),
        Inst::ClearCarry => opcode::CLR_C.encode(),
        Inst::SetInterruptEnable => opcode::SET_I.encode(),
        Inst::ClearInterruptEnable => opcode::CLR_I.encode(),
        Inst::SetBankEnable => opcode::SET_B.encode(),
        Inst::ClearBankEnable => opcode::CLR_B.encode(),
        Inst::ReadBankRegister => opcode::MV_A_BR.encode(),
        Inst::WriteBankRegister => opcode::MV_BR_A.encode(),
        Inst::Move8(dst, src) => match (dst, src) {
            (A, A) => opcode::MV_A_A.encode(),
            (A, B) => opcode::MV_A_B.encode(),
            (A, C) => opcode::MV_A_C.encode(),
            (A, D) => opcode::MV_A_D.encode(),
            (B, A) => opcode::MV_B_A.encode(),
            (B, B) => opcode::MV_B_B.encode(),
            (B, C) => opcode::MV_B_C.encode(),
            (B, D) => opcode::MV_B_D.encode(),
            (C, A) => opcode::MV_C_A.encode(),
            (C, B) => opcode::MV_C_B.encode(),
            (C, C) => opcode::MV_C_C.encode(),
            (C, D) => opcode::MV_C_D.encode(),
            (D, A) => opcode::MV_D_A.encode(),
            (D, B) => opcode::MV_D_B.encode(),
            (D, C) => opcode::MV_D_C.encode(),
            (D, D) => opcode::MV_D_D.encode(),
        },
        Inst::Load8Immediate(dst, byte) => match dst {
            A => opcode::LD_A_IMM.encode_with_byte(byte),
            B => opcode::LD_B_IMM.encode_with_byte(byte),
            C => opcode::LD_C_IMM.encode_with_byte(byte),
            D => opcode::LD_D_IMM.encode_with_byte(byte),
        },
        Inst::Load8(dst, mode) => match mode {
            Mem8::Absolute(word) => match dst {
                A => opcode::LD_A_ABS.encode_with_word(word),
                B => opcode::LD_B_ABS.encode_with_word(word),
                C => opcode::LD_C_ABS.encode_with_word(word),
                D => opcode::LD_D_ABS.encode_with_word(word),
            },
            Mem8::ConstantOffset(ptr, offset) => match dst {
                A => match ptr {
                    X => opcode::LD_A_REL_X_BY_IMM.encode_with_byte(offset),
                    Y => opcode::LD_A_REL_Y_BY_IMM.encode_with_byte(offset),
                    SP => opcode::LD_A_REL_SP_BY_IMM.encode_with_byte(offset),
                },
                B => match ptr {
                    X => opcode::LD_B_REL_X_BY_IMM.encode_with_byte(offset),
                    Y => opcode::LD_B_REL_Y_BY_IMM.encode_with_byte(offset),
                    SP => opcode::LD_B_REL_SP_BY_IMM.encode_with_byte(offset),
                },
                C => match ptr {
                    X => opcode::LD_C_REL_X_BY_IMM.encode_with_byte(offset),
                    Y => opcode::LD_C_REL_Y_BY_IMM.encode_with_byte(offset),
                    SP => opcode::LD_C_REL_SP_BY_IMM.encode_with_byte(offset),
                },
                D => match ptr {
                    X => opcode::LD_D_REL_X_BY_IMM.encode_with_byte(offset),
                    Y => opcode::LD_D_REL_Y_BY_IMM.encode_with_byte(offset),
                    SP => opcode::LD_D_REL_SP_BY_IMM.encode_with_byte(offset),
                }
            },
            Mem8::RegisterOffset(ptr, reg) => match dst {
                A => match ptr {
                    X => match reg {
                        A => opcode::LD_A_REL_X_BY_A.encode(),
                        B => opcode::LD_A_REL_X_BY_B.encode(),
                        C => opcode::LD_A_REL_X_BY_C.encode(),
                        D => opcode::LD_A_REL_X_BY_D.encode(),
                    },
                    Y => match reg {
                        A => opcode::LD_A_REL_Y_BY_A.encode(),
                        B => opcode::LD_A_REL_Y_BY_B.encode(),
                        C => opcode::LD_A_REL_Y_BY_C.encode(),
                        D => opcode::LD_A_REL_Y_BY_D.encode(),
                    },
                    SP => match reg {
                        A => opcode::LD_A_REL_SP_BY_A.encode(),
                        B => opcode::LD_A_REL_SP_BY_B.encode(),
                        C => opcode::LD_A_REL_SP_BY_C.encode(),
                        D => opcode::LD_A_REL_SP_BY_D.encode(),
                    },
                },
                B => match ptr {
                    X => match reg {
                        A => opcode::LD_B_REL_X_BY_A.encode(),
                        B => opcode::LD_B_REL_X_BY_B.encode(),
                        C => opcode::LD_B_REL_X_BY_C.encode(),
                        D => opcode::LD_B_REL_X_BY_D.encode(),
                    },
                    Y => match reg {
                        A => opcode::LD_B_REL_Y_BY_A.encode(),
                        B => opcode::LD_B_REL_Y_BY_B.encode(),
                        C => opcode::LD_B_REL_Y_BY_C.encode(),
                        D => opcode::LD_B_REL_Y_BY_D.encode(),
                    },
                    SP => match reg {
                        A => opcode::LD_B_REL_SP_BY_A.encode(),
                        B => opcode::LD_B_REL_SP_BY_B.encode(),
                        C => opcode::LD_B_REL_SP_BY_C.encode(),
                        D => opcode::LD_B_REL_SP_BY_D.encode(),
                    },
                },
                C => match ptr {
                    X => match reg {
                        A => opcode::LD_C_REL_X_BY_A.encode(),
                        B => opcode::LD_C_REL_X_BY_B.encode(),
                        C => opcode::LD_C_REL_X_BY_C.encode(),
                        D => opcode::LD_C_REL_X_BY_D.encode(),
                    },
                    Y => match reg {
                        A => opcode::LD_C_REL_Y_BY_A.encode(),
                        B => opcode::LD_C_REL_Y_BY_B.encode(),
                        C => opcode::LD_C_REL_Y_BY_C.encode(),
                        D => opcode::LD_C_REL_Y_BY_D.encode(),
                    },
                    SP => match reg {
                        A => opcode::LD_C_REL_SP_BY_A.encode(),
                        B => opcode::LD_C_REL_SP_BY_B.encode(),
                        C => opcode::LD_C_REL_SP_BY_C.encode(),
                        D => opcode::LD_C_REL_SP_BY_D.encode(),
                    },
                },
                D => match ptr {
                    X => match reg {
                        A => opcode::LD_D_REL_X_BY_A.encode(),
                        B => opcode::LD_D_REL_X_BY_B.encode(),
                        C => opcode::LD_D_REL_X_BY_C.encode(),
                        D => opcode::LD_D_REL_X_BY_D.encode(),
                    },
                    Y => match reg {
                        A => opcode::LD_D_REL_Y_BY_A.encode(),
                        B => opcode::LD_D_REL_Y_BY_B.encode(),
                        C => opcode::LD_D_REL_Y_BY_C.encode(),
                        D => opcode::LD_D_REL_Y_BY_D.encode(),
                    },
                    SP => match reg {
                        A => opcode::LD_D_REL_SP_BY_A.encode(),
                        B => opcode::LD_D_REL_SP_BY_B.encode(),
                        C => opcode::LD_D_REL_SP_BY_C.encode(),
                        D => opcode::LD_D_REL_SP_BY_D.encode(),
                    },
                },
            }
        }
        _ => todo!(),
        // Inst::Store8(mode, src) => match mode {
        //     Mem8::Absolute() => {},
        //     Mem8::ConstantOffset(, ) => {},
        //     Mem8::RegisterOffset(, ) => {}
        // }
    }
}
