use modular_bitfield::{bitfield, BitfieldSpecifier};

#[derive(BitfieldSpecifier)]
#[bits = 4]
pub enum DataBusAssert {
    A,
    B,
    C,
    D,
    Memory, // This is a Memory READ
    Alu,
    Bank,
    Status,
    Temp1,
    Temp2,
}

#[derive(BitfieldSpecifier)]
#[bits = 4]
pub enum DataBusLoad {
    A,
    B,
    C,
    D,
    Memory, // This is a Memory WRITE
    Bank,
    Status,
    Temp1,
    Temp2,
}

#[derive(BitfieldSpecifier)]
#[bits = 3]
pub enum AddressBusAssert {
    ProgramCounter,
    StackPointer,
    X,
    Y,
    IOAddress,
}

#[derive(BitfieldSpecifier)]
#[bits = 4]
pub enum AluOperation {
    Nop,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Asr,
}

#[derive(BitfieldSpecifier)]
#[bits = 5]
pub enum TransferBusAssert {
    A_A,
    A_B,
    A_C,
    A_D,
    B_A,
    B_B,
    B_C,
    B_D,
}

#[bitfield(bits = 32, filled = false)]
pub struct ControlWord {
    data_assert: DataBusAssert,
    data_load: DataBusLoad,
    address_assert: AddressBusAssert,
    alu_op: AluOperation,
    xfer_assert: TransferBusAssert,
}

fn main() {
    println!("Hello, world!");
}
