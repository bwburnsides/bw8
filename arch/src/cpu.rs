mod instruction;
mod register;

pub use instruction::Instruction;
pub use register::{Pointer, Register16, Register8, RegisterFile};

use crate::*;
use bus::*;
use instruction::*;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum PrivilegeLevel {
    #[default]
    Kernel,
    User,
}

#[derive(Default)]
pub struct Status {
    pub carry: bool,
    pub zero: bool,
    pub overflow: bool,
    pub negative: bool,
    pub irq_enable: bool,
    pub bank_enable: bool,
    pub privilege_level: PrivilegeLevel,
    pub nmi_active: bool,
}

impl Status {
    pub fn to_byte(&self) -> Byte {
        let mut byte = 0b0000_0000;

        if self.carry {
            byte |= 0b0000_0001;
        };
        if self.zero {
            byte |= 0b0000_0010;
        };
        if self.overflow {
            byte |= 0b0000_0100;
        };
        if self.negative {
            byte |= 0b0000_1000;
        };
        if self.irq_enable {
            byte |= 0b0001_0000;
        };
        if self.bank_enable {
            byte |= 0b0010_0000;
        };
        byte |= match self.privilege_level {
            PrivilegeLevel::User => 0b0100_0000,
            PrivilegeLevel::Kernel => 0b0000_0000,
        };
        if self.nmi_active {
            byte |= 0b1000_0000;
        }

        byte
    }

    pub fn from_byte(byte: Byte) -> Self {
        let mut this = Self::default();

        this.carry = (byte & 0b0000_0001) != 0;

        this.zero = (byte & 0b0000_0010) != 0;

        this.overflow = (byte & 0b0000_0100) != 0;

        this.negative = (byte & 0b0000_1000) != 0;

        this.irq_enable = (byte & 0b0001_0000) != 0;

        this.bank_enable = (byte & 0b0010_0000) != 0;

        if (byte & 0b0100_0000) != 0 {
            this.privilege_level = PrivilegeLevel::User;
        };

        this.nmi_active = (byte & 0b1000_0000) != 0;

        this
    }

    pub fn condition(&self, condition: Condition) -> bool {
        match condition {
            Condition::Always => true,
            Condition::Equal => self.zero,
            Condition::NotEqual => !self.zero,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReachedBreakpoint {
    Did,
    DidNot,
}

#[derive(Debug)]
pub enum ExecutionResult {
    Instruction(Instruction),
    Action(Instruction, EnvironmentAction),
}

#[derive(PartialEq)]
enum InterruptKind {
    Nmi,
    Irq,
    Swi,
}

enum CycleKind {
    Reset,
    Instruction,
    BusStall,
    Interrupt(InterruptKind),
}

pub struct CpuState {
    bank_register: Nibble,
    program_counter: Address,
    registers: RegisterFile,
    status: Status,
}

pub struct Cpu<'a, B> {
    state: &'a mut CpuState,
    bus: &'a mut B,
}

impl<'a, B: Bus> Cpu<'a, B> {
    const RST_VECTOR: Address = 0x0000;
    const NMI_VECTOR: Address = 0x0004;
    const IRQ_VECTOR: Address = 0x0008;
    const SWI_VECTOR: Address = 0x000C;

    pub fn reset(&mut self) {
        self.state.bank_register = Nibble::new(0).unwrap();
        self.state.program_counter = Self::RST_VECTOR;
        self.state.registers = RegisterFile::default();
        self.state.status = Status::default();
    }

    fn service_interrupt(&mut self, kind: InterruptKind) {
        // 1. Write Program Counter to Stack
        // 2. Write Status Register to Stack
        // 3. Jump to Interrupt Vector
        // 4. Disable IRQs
        // 5. Enable Privilege

        self.push_word(self.state.program_counter);
        self.push_byte(self.state.status.to_byte());

        self.state.status.irq_enable = false;
        self.state.status.privilege_level = PrivilegeLevel::Kernel;

        if kind == InterruptKind::Nmi {
            self.state.status.nmi_active = true;
        }

        self.state.program_counter = match kind {
            InterruptKind::Irq => Self::IRQ_VECTOR,
            InterruptKind::Nmi => Self::NMI_VECTOR,
            InterruptKind::Swi => Self::SWI_VECTOR,
        }
    }

    fn push_byte(&mut self, data: Byte) {
        self.memory_write(
            MemoryAddressKind::Data,
            self.state.registers[Pointer::SP],
            data,
        );
        self.state.registers[Pointer::SP] = decrement_word(self.state.registers[Pointer::SP]);
    }

    fn pop_byte(&mut self) -> Byte {
        let sp = self.state.registers[Pointer::SP];
        self.state.registers[Pointer::SP] = increment_word(sp);
        self.memory_read(MemoryAddressKind::Data, self.state.registers[Pointer::SP])
    }

    fn push_word(&mut self, data: Address) {
        let (high, low) = split_bytes(data);
        self.push_byte(high);
        self.push_byte(low);
    }

    fn pop_word(&mut self) -> Address {
        let low = self.pop_byte();
        let high = self.pop_byte();
        concatenate(high, low)
    }

    fn memory_read(&self, kind: MemoryAddressKind, address: Address) -> Byte {
        self.bus.memory_read(
            self.state.status.privilege_level,
            kind,
            PhysicalAddress::new(self.effective_bank_address(kind), address),
        )
    }

    fn memory_write(&mut self, kind: MemoryAddressKind, address: Address, data: Byte) {
        self.bus.memory_write(
            self.state.status.privilege_level,
            kind,
            PhysicalAddress::new(self.effective_bank_address(kind), address),
            data,
        )
    }

    fn io_read(&mut self, address: Address) -> BusResult<Byte> {
        self.bus.io_read(
            self.state.status.privilege_level,
            PhysicalAddress::new(
                self.effective_bank_address(MemoryAddressKind::Data),
                address,
            ),
        )
    }

    fn io_write(&mut self, address: Address, data: Byte) -> BusResult<()> {
        self.bus.io_write(
            self.state.status.privilege_level,
            PhysicalAddress::new(
                self.effective_bank_address(MemoryAddressKind::Data),
                address,
            ),
            data,
        )
    }

    fn execute(&mut self) -> ExecutionResult {
        use Instruction as Inst;
        use Memory16Mode as Mem16;
        use Memory8Mode as Mem8;
        use Pointer as Ptr;

        let instruction = self.decode();

        match instruction {
            Inst::Nop => {}
            Inst::SetCarry => self.state.status.carry = true,
            Inst::ClearCarry => self.state.status.carry = false,
            Inst::SetInterruptEnable => self.state.status.irq_enable = true,
            Inst::ClearInterruptEnable => self.state.status.irq_enable = false,
            Inst::SetBankEnable => self.state.status.bank_enable = true,
            Inst::ClearBankEnable => self.state.status.bank_enable = false,
            Inst::ReadBankRegister => {
                self.state.registers[Register8::A] = self.state.bank_register.as_inner()
            }
            Inst::WriteBankRegister => {
                self.state.bank_register =
                    Nibble::new(self.state.registers[Register8::A] & 0b0000_1111).unwrap()
            }
            Inst::Move8(dst, src) => self.state.registers[dst] = self.state.registers[src],
            Inst::Load8Immediate(dst, imm) => self.state.registers[dst] = imm,
            Inst::Load8(dst, mode) => {
                self.state.registers[dst] = match mode {
                    Mem8::Absolute(addr) => self.memory_read(MemoryAddressKind::Data, addr),
                    Mem8::ConstantOffset(ptr, offset) => self.memory_read(
                        MemoryAddressKind::Data,
                        address_with_offset(self.state.registers[ptr], offset),
                    ),
                    Mem8::RegisterOffset(ptr, offset) => self.memory_read(
                        MemoryAddressKind::Data,
                        address_with_offset(
                            self.state.registers[ptr],
                            self.state.registers[offset],
                        ),
                    ),
                }
            }
            Inst::Store8(mode, src) => {
                let address = match mode {
                    Mem8::Absolute(addr) => addr,
                    Mem8::ConstantOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], offset)
                    }
                    Mem8::RegisterOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], self.state.registers[offset])
                    }
                };
                self.memory_write(MemoryAddressKind::Data, address, self.state.registers[src]);
            }
            Inst::In(dst, mode) => {
                self.state.registers[dst] = match mode {
                    IOMode::Port(port) => match self.io_read(port as u16) {
                        BusResult::Data(data) => data,
                        BusResult::Action(action) => {
                            return ExecutionResult::Action(instruction, action)
                        }
                    },
                    IOMode::ConstantOffset(ptr, offset) => {
                        match self.io_read(address_with_offset(self.state.registers[ptr], offset)) {
                            BusResult::Action(action) => {
                                return ExecutionResult::Action(instruction, action)
                            }
                            BusResult::Data(data) => data,
                        }
                    }
                    IOMode::RegisterOffset(ptr, offset) => {
                        match self.io_read(address_with_offset(
                            self.state.registers[ptr],
                            self.state.registers[offset],
                        )) {
                            BusResult::Action(action) => {
                                return ExecutionResult::Action(instruction, action)
                            }
                            BusResult::Data(data) => data,
                        }
                    }
                }
            }
            Inst::Out(mode, src) => {
                let address = match mode {
                    IOMode::Port(port) => port as Address,
                    IOMode::ConstantOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], offset)
                    }
                    IOMode::RegisterOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], self.state.registers[offset])
                    }
                };
                match self.io_write(address, self.state.registers[src]) {
                    BusResult::Action(action) => {
                        return ExecutionResult::Action(instruction, action)
                    }
                    BusResult::Data(_) => {}
                };
            }
            Inst::ReadStackPointer => self.state.registers[Ptr::X] = self.state.registers[Ptr::SP],
            Inst::WriteStackPointer => self.state.registers[Ptr::SP] = self.state.registers[Ptr::X],
            Inst::Move16(dst, src) => self.state.registers[dst] = self.state.registers[src],
            Inst::Move16FromPair(dst, src) => {
                let pair = match src {
                    RegisterPair::Ab => concatenate(
                        self.state.registers[Register8::A],
                        self.state.registers[Register8::B],
                    ),
                    RegisterPair::Cd => concatenate(
                        self.state.registers[Register8::C],
                        self.state.registers[Register8::D],
                    ),
                };
                self.state.registers[dst] = pair;
            }
            Inst::Move16ToPair(dst, src) => {
                let (high, low) = split_bytes(self.state.registers[src]);

                let (reg_high, reg_low) = match dst {
                    RegisterPair::Ab => (Register8::A, Register8::B),
                    RegisterPair::Cd => (Register8::C, Register8::D),
                };

                self.state.registers[reg_high] = high;
                self.state.registers[reg_low] = low;
            }
            Inst::Load16Immediate(dst, word) => self.state.registers[dst] = word,
            Inst::Load16(dst, mode) => {
                let effective_address_low = match mode {
                    Mem16::Absolute(addr) => addr,
                    Mem16::ConstantOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], offset)
                    }
                };

                let low = self.memory_read(MemoryAddressKind::Data, effective_address_low);
                let high = self.memory_read(
                    MemoryAddressKind::Data,
                    increment_word(effective_address_low),
                );

                self.state.registers[dst] = concatenate(high, low);
            }
            Inst::Store16(mode, src) => {
                let effective_address_low = match mode {
                    Mem16::Absolute(addr) => addr,
                    Mem16::ConstantOffset(ptr, offset) => {
                        address_with_offset(self.state.registers[ptr], offset)
                    }
                };

                let (high, low) = split_bytes(self.state.registers[src]);

                self.memory_write(MemoryAddressKind::Data, effective_address_low, low);
                self.memory_write(
                    MemoryAddressKind::Data,
                    increment_word(effective_address_low),
                    high,
                );
            }
            Inst::Lea(ptr, LeaMode::Constant(offset)) => {
                self.state.registers[ptr] = address_with_offset(self.state.registers[ptr], offset)
            }
            Inst::Lea(ptr, LeaMode::Register(offset)) => {
                self.state.registers[ptr] =
                    address_with_offset(self.state.registers[ptr], self.state.registers[offset])
            }
            Inst::Inc16(dst) => {
                self.state.registers[dst] = increment_word(self.state.registers[dst])
            }
            Inst::Dec16(dst) => {
                self.state.registers[dst] = decrement_word(self.state.registers[dst])
            }
            Inst::Alu2(op, left, right) => {
                let lhs = self.state.registers[left];
                let rhs = match right {
                    Alu2OpMode::Constant(val) => val,
                    Alu2OpMode::Register(reg) => self.state.registers[reg],
                };

                let result = match op {
                    Alu2Op::Addc => lhs + rhs + (if self.state.status.carry { 1 } else { 0 }),
                    Alu2Op::Subb => lhs - rhs - (if !self.state.status.carry { 1 } else { 0 }),
                    Alu2Op::And => lhs & rhs,
                    Alu2Op::Or => lhs | rhs,
                    Alu2Op::Xor => lhs ^ rhs,
                    Alu2Op::Cmp => wrapping_subtract(lhs, rhs),
                };

                if op != Alu2Op::Cmp {
                    self.state.registers[left] = result;
                }

                self.state.status.zero = result == 0;
            }
            Inst::Alu1(op, left) => {
                let lhs = self.state.registers[left];

                let result = match op {
                    Alu1Op::Shl => lhs << 1,
                    Alu1Op::Shr => lhs << 1,
                    Alu1Op::Asr => ((lhs as i8) << 1) as u8,
                    Alu1Op::Not => !lhs,
                    Alu1Op::Neg => (-(lhs as i8)) as u8,
                    Alu1Op::Inc => increment_byte(lhs),
                    Alu1Op::Dec => decrement_byte(lhs),
                    Alu1Op::Test => lhs & lhs,
                };

                if op != Alu1Op::Test {
                    self.state.registers[left] = result;
                }
            }
            Inst::Push8(op) => self.push_byte(self.state.registers[op]),
            Inst::Push16(op) => self.push_word(self.state.registers[op]),
            Inst::Pop8(op) => self.state.registers[op] = self.pop_byte(),
            Inst::Pop16(op) => self.state.registers[op] = self.pop_word(),
            Inst::Call(mode) => {
                self.push_word(self.state.program_counter);

                match mode {
                    JumpMode::Relative(offset) => {
                        self.state.program_counter =
                            address_with_offset(self.state.program_counter, offset)
                    }
                    JumpMode::Absolute(addr) => self.state.program_counter = addr,
                    JumpMode::Indirect(base, offset) => {
                        self.state.program_counter =
                            address_with_offset(self.state.registers[base], offset)
                    }
                }
            }
            Inst::Ret => self.state.program_counter = self.pop_word(),
            Inst::Swi => self.service_interrupt(InterruptKind::Swi),
            Inst::Reti => {
                self.state.status.privilege_level = PrivilegeLevel::User;
                self.state.status = Status::from_byte(self.pop_byte());
                self.state.program_counter = self.pop_word();
            }
            Inst::Jmp(condition, mode) => {
                let target = match mode {
                    JumpMode::Relative(offset) => {
                        address_with_offset(self.state.program_counter, offset)
                    }
                    JumpMode::Absolute(addr) => addr,
                    JumpMode::Indirect(base, offset) => {
                        address_with_offset(self.state.registers[base], offset)
                    }
                };
                if self.state.status.condition(condition) {
                    self.state.program_counter = target;
                }
            }
        }

        ExecutionResult::Instruction(instruction)
    }

    fn fetch_byte(&mut self) -> Byte {
        let address = self.state.program_counter;
        self.state.program_counter = increment_word(self.state.program_counter);

        self.memory_read(MemoryAddressKind::Code, address)
    }

    fn fetch_word(&mut self) -> Address {
        let low = self.memory_read(MemoryAddressKind::Code, self.state.program_counter);
        self.state.program_counter = increment_word(self.state.program_counter);

        let high = self.memory_read(MemoryAddressKind::Code, self.state.program_counter);
        self.state.program_counter = increment_word(self.state.program_counter);

        concatenate(high, low)
    }

    fn decode(&mut self) -> Instruction {
        use Instruction as Inst;
        use Memory8Mode as Mem8;
        use Pointer as Ptr;
        use Pointer::SP;
        use Register8::A;
        use Register8::B;
        use Register8::C;
        use Register8::D;

        match self.fetch_byte() {
            0x00 => Inst::Nop,
            0x01 => self.decode_extended(),

            0x02 => Inst::SetCarry,
            0x03 => Inst::ClearCarry,

            0x04 => Inst::SetInterruptEnable,
            0x05 => Inst::ClearInterruptEnable,

            0x06 => Inst::SetBankEnable,
            0x07 => Inst::ClearBankEnable,

            0x08 => Inst::ReadBankRegister,
            0x09 => Inst::WriteBankRegister,

            0x0A => Inst::Move8(A, A),
            0x0B => Inst::Move8(A, B),
            0x0C => Inst::Move8(A, C),
            0x0D => Inst::Move8(A, D),
            0x0E => Inst::Move8(B, A),
            0x0F => Inst::Move8(B, B),
            0x10 => Inst::Move8(B, C),
            0x11 => Inst::Move8(B, D),
            0x12 => Inst::Move8(C, A),
            0x13 => Inst::Move8(C, B),
            0x14 => Inst::Move8(C, C),
            0x15 => Inst::Move8(C, D),
            0x16 => Inst::Move8(D, A),
            0x17 => Inst::Move8(D, B),
            0x18 => Inst::Move8(D, C),
            0x19 => Inst::Move8(D, D),

            0x1A => Inst::Load8Immediate(A, self.fetch_byte()),
            0x1B => Inst::Load8Immediate(B, self.fetch_byte()),
            0x1C => Inst::Load8Immediate(C, self.fetch_byte()),
            0x1D => Inst::Load8Immediate(D, self.fetch_byte()),

            0x1E => Inst::Load8(A, Mem8::Absolute(self.fetch_word())),
            0x1F => Inst::Load8(A, Mem8::ConstantOffset(Ptr::X, self.fetch_byte())),
            0x20 => Inst::Load8(A, Mem8::ConstantOffset(Ptr::Y, self.fetch_byte())),
            0x21 => Inst::Load8(A, Mem8::ConstantOffset(SP, self.fetch_byte())),
            0x22 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, A)),
            0x23 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, B)),
            0x24 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, C)),
            0x25 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::X, D)),
            0x26 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, A)),
            0x27 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, B)),
            0x28 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, C)),
            0x29 => Inst::Load8(A, Mem8::RegisterOffset(Ptr::Y, D)),
            0x2A => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, A)),
            0x2B => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, B)),
            0x2C => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, C)),
            0x2D => Inst::Load8(A, Mem8::RegisterOffset(Ptr::SP, D)),

            0x2E => Inst::Load8(B, Mem8::Absolute(self.fetch_word())),
            0x2F => Inst::Load8(B, Mem8::ConstantOffset(Ptr::X, self.fetch_byte())),
            0x30 => Inst::Load8(B, Mem8::ConstantOffset(Ptr::Y, self.fetch_byte())),
            0x31 => Inst::Load8(B, Mem8::ConstantOffset(Ptr::SP, self.fetch_byte())),
            0x32 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, A)),
            0x33 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, B)),
            0x34 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, C)),
            0x35 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::X, D)),
            0x36 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, A)),
            0x37 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, B)),
            0x38 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, C)),
            0x39 => Inst::Load8(B, Mem8::RegisterOffset(Ptr::Y, D)),
            0x3A => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, A)),
            0x3B => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, B)),
            0x3C => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, C)),
            0x3D => Inst::Load8(B, Mem8::RegisterOffset(Ptr::SP, D)),

            0x3E => Inst::Load8(C, Mem8::Absolute(self.fetch_word())),
            0x3F => Inst::Load8(C, Mem8::ConstantOffset(Ptr::X, self.fetch_byte())),
            0x40 => Inst::Load8(C, Mem8::ConstantOffset(Ptr::Y, self.fetch_byte())),
            0x41 => Inst::Load8(C, Mem8::ConstantOffset(Ptr::SP, self.fetch_byte())),
            0x42 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, A)),
            0x43 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, B)),
            0x44 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, C)),
            0x45 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::X, D)),
            0x46 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, A)),
            0x47 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, B)),
            0x48 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, C)),
            0x49 => Inst::Load8(C, Mem8::RegisterOffset(Ptr::Y, D)),
            0x4A => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, A)),
            0x4B => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, B)),
            0x4C => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, C)),
            0x4D => Inst::Load8(C, Mem8::RegisterOffset(Ptr::SP, D)),

            0x4E => Inst::Load8(D, Mem8::Absolute(self.fetch_word())),
            0x4F => Inst::Load8(D, Mem8::ConstantOffset(Ptr::X, self.fetch_byte())),
            0x50 => Inst::Load8(D, Mem8::ConstantOffset(Ptr::Y, self.fetch_byte())),
            0x51 => Inst::Load8(D, Mem8::ConstantOffset(Ptr::SP, self.fetch_byte())),
            0x52 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, A)),
            0x53 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, B)),
            0x54 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, C)),
            0x55 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::X, D)),
            0x56 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, A)),
            0x57 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, B)),
            0x58 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, C)),
            0x59 => Inst::Load8(D, Mem8::RegisterOffset(Ptr::Y, D)),
            0x5A => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, A)),
            0x5B => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, B)),
            0x5C => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, C)),
            0x5D => Inst::Load8(D, Mem8::RegisterOffset(Ptr::SP, D)),

            0x5E => Inst::Store8(Mem8::Absolute(self.fetch_word()), A),
            0x5F => Inst::Store8(Mem8::ConstantOffset(Ptr::X, self.fetch_byte()), A),
            0x60 => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, self.fetch_byte()), A),
            0x61 => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, self.fetch_byte()), A),
            0x62 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), A),
            0x63 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), A),
            0x64 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), A),
            0x65 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), A),
            0x66 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), A),
            0x67 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), A),
            0x68 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), A),
            0x69 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), A),
            0x6A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), A),
            0x6B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), A),
            0x6C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), A),
            0x6D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), A),

            0x6E => Inst::Store8(Mem8::Absolute(self.fetch_word()), B),
            0x6F => Inst::Store8(Mem8::ConstantOffset(Ptr::X, self.fetch_byte()), B),
            0x70 => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, self.fetch_byte()), B),
            0x71 => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, self.fetch_byte()), B),
            0x72 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), B),
            0x73 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), B),
            0x74 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), B),
            0x75 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), B),
            0x76 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), B),
            0x77 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), B),
            0x78 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), B),
            0x79 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), B),
            0x7A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), B),
            0x7B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), B),
            0x7C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), B),
            0x7D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), B),

            0x7E => Inst::Store8(Mem8::Absolute(self.fetch_word()), C),
            0x7F => Inst::Store8(Mem8::ConstantOffset(Ptr::X, self.fetch_byte()), C),
            0x80 => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, self.fetch_byte()), C),
            0x81 => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, self.fetch_byte()), C),
            0x82 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), C),
            0x83 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), C),
            0x84 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), C),
            0x85 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), C),
            0x86 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), C),
            0x87 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), C),
            0x88 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), C),
            0x89 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), C),
            0x8A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), C),
            0x8B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), C),
            0x8C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), C),
            0x8D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), C),

            0x8E => Inst::Store8(Mem8::Absolute(self.fetch_word()), D),
            0x8F => Inst::Store8(Mem8::ConstantOffset(Ptr::X, self.fetch_byte()), D),
            0x90 => Inst::Store8(Mem8::ConstantOffset(Ptr::Y, self.fetch_byte()), D),
            0x91 => Inst::Store8(Mem8::ConstantOffset(Ptr::SP, self.fetch_byte()), D),
            0x92 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, A), D),
            0x93 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, B), D),
            0x94 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, C), D),
            0x95 => Inst::Store8(Mem8::RegisterOffset(Ptr::X, D), D),
            0x96 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, A), D),
            0x97 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, B), D),
            0x98 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, C), D),
            0x99 => Inst::Store8(Mem8::RegisterOffset(Ptr::Y, D), D),
            0x9A => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, A), D),
            0x9B => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, B), D),
            0x9C => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, C), D),
            0x9D => Inst::Store8(Mem8::RegisterOffset(Ptr::SP, D), D),

            0x9E => Inst::In(A, IOMode::Port(self.fetch_byte())),
            0x9F => Inst::In(A, IOMode::ConstantOffset(Register16::X, self.fetch_byte())),
            0xA0 => Inst::In(A, IOMode::ConstantOffset(Register16::Y, self.fetch_byte())),
            0xA1 => Inst::In(A, IOMode::RegisterOffset(Register16::X, A)),
            0xA2 => Inst::In(A, IOMode::RegisterOffset(Register16::X, B)),
            0xA3 => Inst::In(A, IOMode::RegisterOffset(Register16::X, C)),
            0xA4 => Inst::In(A, IOMode::RegisterOffset(Register16::X, D)),
            0xA5 => Inst::In(A, IOMode::RegisterOffset(Register16::Y, A)),
            0xA6 => Inst::In(A, IOMode::RegisterOffset(Register16::Y, B)),
            0xA7 => Inst::In(A, IOMode::RegisterOffset(Register16::Y, C)),
            0xA8 => Inst::In(A, IOMode::RegisterOffset(Register16::Y, D)),

            0xA9 => Inst::In(B, IOMode::Port(self.fetch_byte())),
            0xAA => Inst::In(B, IOMode::ConstantOffset(Register16::X, self.fetch_byte())),
            0xAB => Inst::In(B, IOMode::ConstantOffset(Register16::Y, self.fetch_byte())),
            0xAC => Inst::In(B, IOMode::RegisterOffset(Register16::X, A)),
            0xAD => Inst::In(B, IOMode::RegisterOffset(Register16::X, B)),
            0xAE => Inst::In(B, IOMode::RegisterOffset(Register16::X, C)),
            0xAF => Inst::In(B, IOMode::RegisterOffset(Register16::X, D)),
            0xB0 => Inst::In(B, IOMode::RegisterOffset(Register16::Y, A)),
            0xB1 => Inst::In(B, IOMode::RegisterOffset(Register16::Y, B)),
            0xB2 => Inst::In(B, IOMode::RegisterOffset(Register16::Y, C)),
            0xB3 => Inst::In(B, IOMode::RegisterOffset(Register16::Y, D)),

            0xB4 => Inst::In(C, IOMode::Port(self.fetch_byte())),
            0xB5 => Inst::In(C, IOMode::ConstantOffset(Register16::X, self.fetch_byte())),
            0xB6 => Inst::In(C, IOMode::ConstantOffset(Register16::Y, self.fetch_byte())),
            0xB7 => Inst::In(C, IOMode::RegisterOffset(Register16::X, A)),
            0xB8 => Inst::In(C, IOMode::RegisterOffset(Register16::X, B)),
            0xB9 => Inst::In(C, IOMode::RegisterOffset(Register16::X, C)),
            0xBA => Inst::In(C, IOMode::RegisterOffset(Register16::X, D)),
            0xBB => Inst::In(C, IOMode::RegisterOffset(Register16::Y, A)),
            0xBC => Inst::In(C, IOMode::RegisterOffset(Register16::Y, B)),
            0xBD => Inst::In(C, IOMode::RegisterOffset(Register16::Y, C)),
            0xBE => Inst::In(C, IOMode::RegisterOffset(Register16::Y, D)),

            0xBF => Inst::In(D, IOMode::Port(self.fetch_byte())),
            0xC0 => Inst::In(D, IOMode::ConstantOffset(Register16::X, self.fetch_byte())),
            0xC1 => Inst::In(D, IOMode::ConstantOffset(Register16::Y, self.fetch_byte())),
            0xC2 => Inst::In(D, IOMode::RegisterOffset(Register16::X, A)),
            0xC3 => Inst::In(D, IOMode::RegisterOffset(Register16::X, B)),
            0xC4 => Inst::In(D, IOMode::RegisterOffset(Register16::X, C)),
            0xC5 => Inst::In(D, IOMode::RegisterOffset(Register16::X, D)),
            0xC6 => Inst::In(D, IOMode::RegisterOffset(Register16::Y, A)),
            0xC7 => Inst::In(D, IOMode::RegisterOffset(Register16::Y, B)),
            0xC8 => Inst::In(D, IOMode::RegisterOffset(Register16::Y, C)),
            0xC9 => Inst::In(D, IOMode::RegisterOffset(Register16::Y, D)),

            0xCA => Inst::Out(IOMode::Port(self.fetch_byte()), A),
            0xCB => Inst::Out(IOMode::ConstantOffset(Register16::X, self.fetch_byte()), A),
            0xCC => Inst::Out(IOMode::ConstantOffset(Register16::Y, self.fetch_byte()), A),
            0xCD => Inst::Out(IOMode::RegisterOffset(Register16::X, A), A),
            0xCE => Inst::Out(IOMode::RegisterOffset(Register16::X, B), A),
            0xCF => Inst::Out(IOMode::RegisterOffset(Register16::X, C), A),
            0xD0 => Inst::Out(IOMode::RegisterOffset(Register16::X, D), A),
            0xD1 => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), A),
            0xD2 => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), A),
            0xD3 => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), A),
            0xD4 => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), A),

            0xD5 => Inst::Out(IOMode::Port(self.fetch_byte()), B),
            0xD6 => Inst::Out(IOMode::ConstantOffset(Register16::X, self.fetch_byte()), B),
            0xD7 => Inst::Out(IOMode::ConstantOffset(Register16::Y, self.fetch_byte()), B),
            0xD8 => Inst::Out(IOMode::RegisterOffset(Register16::X, A), B),
            0xD9 => Inst::Out(IOMode::RegisterOffset(Register16::X, B), B),
            0xDA => Inst::Out(IOMode::RegisterOffset(Register16::X, C), B),
            0xDB => Inst::Out(IOMode::RegisterOffset(Register16::X, D), B),
            0xDC => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), B),
            0xDD => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), B),
            0xDE => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), B),
            0xDF => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), B),

            0xE0 => Inst::Out(IOMode::Port(self.fetch_byte()), C),
            0xE1 => Inst::Out(IOMode::ConstantOffset(Register16::X, self.fetch_byte()), C),
            0xE2 => Inst::Out(IOMode::ConstantOffset(Register16::Y, self.fetch_byte()), C),
            0xE3 => Inst::Out(IOMode::RegisterOffset(Register16::X, A), C),
            0xE4 => Inst::Out(IOMode::RegisterOffset(Register16::X, B), C),
            0xE5 => Inst::Out(IOMode::RegisterOffset(Register16::X, C), C),
            0xE6 => Inst::Out(IOMode::RegisterOffset(Register16::X, D), C),
            0xE7 => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), C),
            0xE8 => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), C),
            0xE9 => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), C),
            0xEA => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), C),

            0xEB => Inst::Out(IOMode::Port(self.fetch_byte()), D),
            0xEC => Inst::Out(IOMode::ConstantOffset(Register16::X, self.fetch_byte()), D),
            0xED => Inst::Out(IOMode::ConstantOffset(Register16::Y, self.fetch_byte()), D),
            0xEE => Inst::Out(IOMode::RegisterOffset(Register16::X, A), D),
            0xEF => Inst::Out(IOMode::RegisterOffset(Register16::X, B), D),
            0xF0 => Inst::Out(IOMode::RegisterOffset(Register16::X, C), D),
            0xF1 => Inst::Out(IOMode::RegisterOffset(Register16::X, D), D),
            0xF2 => Inst::Out(IOMode::RegisterOffset(Register16::Y, A), D),
            0xF3 => Inst::Out(IOMode::RegisterOffset(Register16::Y, B), D),
            0xF4 => Inst::Out(IOMode::RegisterOffset(Register16::Y, C), D),
            0xF5 => Inst::Out(IOMode::RegisterOffset(Register16::Y, D), D),

            0xF6 => Inst::ReadStackPointer,
            0xF7 => Inst::WriteStackPointer,

            0xF8 => Inst::Move16(Register16::X, Register16::X),
            0xF9 => Inst::Move16(Register16::X, Register16::Y),
            0xFA => Inst::Move16FromPair(Register16::X, RegisterPair::Ab),
            0xFB => Inst::Move16FromPair(Register16::X, RegisterPair::Cd),

            0xFC => Inst::Move16(Register16::Y, Register16::X),
            0xFD => Inst::Move16(Register16::Y, Register16::Y),
            0xFE => Inst::Move16FromPair(Register16::Y, RegisterPair::Ab),
            0xFF => Inst::Move16FromPair(Register16::Y, RegisterPair::Cd),
        }
    }

    fn decode_extended(&mut self) -> Instruction {
        use Instruction as Inst;
        use Memory16Mode as Mem16;
        use Pointer as Ptr;
        use Register8::A;
        use Register8::B;
        use Register8::C;
        use Register8::D;

        match self.fetch_byte() {
            0x00 => Inst::Move16ToPair(RegisterPair::Ab, Register16::X),
            0x01 => Inst::Move16ToPair(RegisterPair::Ab, Register16::Y),

            0x02 => Inst::Move16ToPair(RegisterPair::Cd, Register16::X),
            0x03 => Inst::Move16ToPair(RegisterPair::Cd, Register16::Y),

            0x04 => Inst::Load16Immediate(Register16::X, self.fetch_word()),
            0x05 => Inst::Load16Immediate(Register16::Y, self.fetch_word()),

            0x06 => Inst::Load16(Register16::X, Mem16::Absolute(self.fetch_word())),
            0x07 => Inst::Load16(
                Register16::X,
                Mem16::ConstantOffset(Ptr::X, self.fetch_byte()),
            ),
            0x08 => Inst::Load16(
                Register16::X,
                Mem16::ConstantOffset(Ptr::Y, self.fetch_byte()),
            ),
            0x09 => Inst::Load16(
                Register16::X,
                Mem16::ConstantOffset(Ptr::SP, self.fetch_byte()),
            ),

            0x0A => Inst::Load16(Register16::Y, Mem16::Absolute(self.fetch_word())),
            0x0B => Inst::Load16(
                Register16::Y,
                Mem16::ConstantOffset(Ptr::X, self.fetch_byte()),
            ),
            0x0C => Inst::Load16(
                Register16::Y,
                Mem16::ConstantOffset(Ptr::Y, self.fetch_byte()),
            ),
            0x0D => Inst::Load16(
                Register16::Y,
                Mem16::ConstantOffset(Ptr::SP, self.fetch_byte()),
            ),

            0x0E => Inst::Store16(Mem16::Absolute(self.fetch_word()), Register16::X),
            0x0F => Inst::Store16(
                Mem16::ConstantOffset(Ptr::X, self.fetch_byte()),
                Register16::X,
            ),
            0x10 => Inst::Store16(
                Mem16::ConstantOffset(Ptr::Y, self.fetch_byte()),
                Register16::X,
            ),
            0x11 => Inst::Store16(
                Mem16::ConstantOffset(Ptr::SP, self.fetch_byte()),
                Register16::X,
            ),

            0x12 => Inst::Store16(Mem16::Absolute(self.fetch_word()), Register16::Y),
            0x13 => Inst::Store16(
                Mem16::ConstantOffset(Ptr::X, self.fetch_byte()),
                Register16::Y,
            ),
            0x14 => Inst::Store16(
                Mem16::ConstantOffset(Ptr::Y, self.fetch_byte()),
                Register16::Y,
            ),
            0x15 => Inst::Store16(
                Mem16::ConstantOffset(Ptr::SP, self.fetch_byte()),
                Register16::Y,
            ),

            0x16 => Inst::Lea(Ptr::X, LeaMode::Register(A)),
            0x17 => Inst::Lea(Ptr::X, LeaMode::Register(B)),
            0x18 => Inst::Lea(Ptr::X, LeaMode::Register(C)),
            0x19 => Inst::Lea(Ptr::X, LeaMode::Register(D)),
            0x1A => Inst::Lea(Ptr::X, LeaMode::Constant(self.fetch_byte())),

            0x1B => Inst::Lea(Ptr::Y, LeaMode::Register(A)),
            0x1C => Inst::Lea(Ptr::Y, LeaMode::Register(B)),
            0x1D => Inst::Lea(Ptr::Y, LeaMode::Register(C)),
            0x1E => Inst::Lea(Ptr::Y, LeaMode::Register(D)),
            0x1F => Inst::Lea(Ptr::Y, LeaMode::Constant(self.fetch_byte())),

            0x20 => Inst::Lea(Ptr::SP, LeaMode::Register(A)),
            0x21 => Inst::Lea(Ptr::SP, LeaMode::Register(B)),
            0x22 => Inst::Lea(Ptr::SP, LeaMode::Register(C)),
            0x23 => Inst::Lea(Ptr::SP, LeaMode::Register(D)),
            0x24 => Inst::Lea(Ptr::SP, LeaMode::Constant(self.fetch_byte())),

            0x25 => Inst::Inc16(Register16::X),
            0x26 => Inst::Inc16(Register16::Y),

            0x27 => Inst::Dec16(Register16::X),
            0x28 => Inst::Dec16(Register16::Y),

            0x29 => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(A)),
            0x2A => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(B)),
            0x2B => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(C)),
            0x2C => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Register(D)),

            0x2D => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(A)),
            0x2E => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(B)),
            0x2F => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(C)),
            0x30 => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Register(D)),

            0x31 => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(A)),
            0x32 => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(B)),
            0x33 => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(C)),
            0x34 => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Register(D)),

            0x35 => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(A)),
            0x36 => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(B)),
            0x37 => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(C)),
            0x38 => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Register(D)),

            0x39 => Inst::Alu2(Alu2Op::Addc, A, Alu2OpMode::Constant(self.fetch_byte())),
            0x3A => Inst::Alu2(Alu2Op::Addc, B, Alu2OpMode::Constant(self.fetch_byte())),
            0x3B => Inst::Alu2(Alu2Op::Addc, C, Alu2OpMode::Constant(self.fetch_byte())),
            0x3C => Inst::Alu2(Alu2Op::Addc, D, Alu2OpMode::Constant(self.fetch_byte())),

            0x3D => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(A)),
            0x3E => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(B)),
            0x3F => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(C)),
            0x40 => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Register(D)),

            0x41 => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(A)),
            0x42 => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(B)),
            0x43 => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(C)),
            0x44 => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Register(D)),

            0x45 => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(A)),
            0x46 => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(B)),
            0x47 => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(C)),
            0x48 => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Register(D)),

            0x49 => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(A)),
            0x4A => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(B)),
            0x4B => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(C)),
            0x4C => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Register(D)),

            0x4D => Inst::Alu2(Alu2Op::Subb, A, Alu2OpMode::Constant(self.fetch_byte())),
            0x4E => Inst::Alu2(Alu2Op::Subb, B, Alu2OpMode::Constant(self.fetch_byte())),
            0x4F => Inst::Alu2(Alu2Op::Subb, C, Alu2OpMode::Constant(self.fetch_byte())),
            0x50 => Inst::Alu2(Alu2Op::Subb, D, Alu2OpMode::Constant(self.fetch_byte())),

            0x51 => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(A)),
            0x52 => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(B)),
            0x53 => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(C)),
            0x54 => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Register(D)),

            0x55 => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(A)),
            0x56 => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(B)),
            0x57 => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(C)),
            0x58 => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Register(D)),

            0x59 => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(A)),
            0x5A => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(B)),
            0x5B => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(C)),
            0x5C => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Register(D)),

            0x5D => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(A)),
            0x5E => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(B)),
            0x5F => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(C)),
            0x60 => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Register(D)),

            0x61 => Inst::Alu2(Alu2Op::And, A, Alu2OpMode::Constant(self.fetch_byte())),
            0x62 => Inst::Alu2(Alu2Op::And, B, Alu2OpMode::Constant(self.fetch_byte())),
            0x63 => Inst::Alu2(Alu2Op::And, C, Alu2OpMode::Constant(self.fetch_byte())),
            0x64 => Inst::Alu2(Alu2Op::And, D, Alu2OpMode::Constant(self.fetch_byte())),

            0x65 => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(A)),
            0x66 => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(B)),
            0x67 => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(C)),
            0x68 => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Register(D)),

            0x69 => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(A)),
            0x6A => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(B)),
            0x6B => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(C)),
            0x6C => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Register(D)),

            0x6D => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(A)),
            0x6E => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(B)),
            0x6F => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(C)),
            0x70 => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Register(D)),

            0x71 => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(A)),
            0x72 => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(B)),
            0x73 => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(C)),
            0x74 => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Register(D)),

            0x75 => Inst::Alu2(Alu2Op::Or, A, Alu2OpMode::Constant(self.fetch_byte())),
            0x76 => Inst::Alu2(Alu2Op::Or, B, Alu2OpMode::Constant(self.fetch_byte())),
            0x77 => Inst::Alu2(Alu2Op::Or, C, Alu2OpMode::Constant(self.fetch_byte())),
            0x78 => Inst::Alu2(Alu2Op::Or, D, Alu2OpMode::Constant(self.fetch_byte())),

            0x79 => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(A)),
            0x7A => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(B)),
            0x7B => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(C)),
            0x7C => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Register(D)),

            0x7D => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(A)),
            0x7E => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(B)),
            0x7F => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(C)),
            0x80 => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Register(D)),

            0x81 => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(A)),
            0x82 => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(B)),
            0x83 => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(C)),
            0x84 => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Register(D)),

            0x85 => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(A)),
            0x86 => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(B)),
            0x87 => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(C)),
            0x88 => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Register(D)),

            0x89 => Inst::Alu2(Alu2Op::Xor, A, Alu2OpMode::Constant(self.fetch_byte())),
            0x8A => Inst::Alu2(Alu2Op::Xor, B, Alu2OpMode::Constant(self.fetch_byte())),
            0x8B => Inst::Alu2(Alu2Op::Xor, C, Alu2OpMode::Constant(self.fetch_byte())),
            0x8C => Inst::Alu2(Alu2Op::Xor, D, Alu2OpMode::Constant(self.fetch_byte())),

            0x8D => Inst::Alu1(Alu1Op::Shl, A),
            0x8E => Inst::Alu1(Alu1Op::Shl, B),
            0x8F => Inst::Alu1(Alu1Op::Shl, C),
            0x90 => Inst::Alu1(Alu1Op::Shl, D),

            0x91 => Inst::Alu1(Alu1Op::Shr, A),
            0x92 => Inst::Alu1(Alu1Op::Shr, B),
            0x93 => Inst::Alu1(Alu1Op::Shr, C),
            0x94 => Inst::Alu1(Alu1Op::Shr, D),

            0x95 => Inst::Alu1(Alu1Op::Asr, A),
            0x96 => Inst::Alu1(Alu1Op::Asr, B),
            0x97 => Inst::Alu1(Alu1Op::Asr, C),
            0x98 => Inst::Alu1(Alu1Op::Asr, D),

            0x99 => Inst::Alu1(Alu1Op::Not, A),
            0x9A => Inst::Alu1(Alu1Op::Not, B),
            0x9B => Inst::Alu1(Alu1Op::Not, C),
            0x9C => Inst::Alu1(Alu1Op::Not, D),

            0x9D => Inst::Alu1(Alu1Op::Neg, A),
            0x9E => Inst::Alu1(Alu1Op::Neg, B),
            0x9F => Inst::Alu1(Alu1Op::Neg, C),
            0xA0 => Inst::Alu1(Alu1Op::Neg, D),

            0xA1 => Inst::Alu1(Alu1Op::Inc, A),
            0xA2 => Inst::Alu1(Alu1Op::Inc, B),
            0xA3 => Inst::Alu1(Alu1Op::Inc, C),
            0xA4 => Inst::Alu1(Alu1Op::Inc, D),

            0xA5 => Inst::Alu1(Alu1Op::Dec, A),
            0xA6 => Inst::Alu1(Alu1Op::Dec, B),
            0xA7 => Inst::Alu1(Alu1Op::Dec, C),
            0xA8 => Inst::Alu1(Alu1Op::Dec, D),

            0xA9 => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(A)),
            0xAA => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(B)),
            0xAB => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(C)),
            0xAC => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Register(D)),

            0xAD => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(A)),
            0xAE => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(B)),
            0xAF => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(C)),
            0xB0 => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Register(D)),

            0xB1 => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(A)),
            0xB2 => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(B)),
            0xB3 => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(C)),
            0xB4 => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Register(D)),

            0xB5 => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(A)),
            0xB6 => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(B)),
            0xB7 => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(C)),
            0xB8 => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Register(D)),

            0xB9 => Inst::Alu2(Alu2Op::Cmp, A, Alu2OpMode::Constant(self.fetch_byte())),
            0xBA => Inst::Alu2(Alu2Op::Cmp, B, Alu2OpMode::Constant(self.fetch_byte())),
            0xBB => Inst::Alu2(Alu2Op::Cmp, C, Alu2OpMode::Constant(self.fetch_byte())),
            0xBC => Inst::Alu2(Alu2Op::Cmp, D, Alu2OpMode::Constant(self.fetch_byte())),

            0xBD => Inst::Alu1(Alu1Op::Test, A),
            0xBE => Inst::Alu1(Alu1Op::Test, B),
            0xBF => Inst::Alu1(Alu1Op::Test, C),
            0xC0 => Inst::Alu1(Alu1Op::Test, D),

            0xC1 => Inst::Push8(A),
            0xC2 => Inst::Push8(B),
            0xC3 => Inst::Push8(C),
            0xC4 => Inst::Push8(D),

            0xC5 => Inst::Push16(Register16::X),
            0xC6 => Inst::Push16(Register16::Y),

            0xC7 => Inst::Pop8(A),
            0xC8 => Inst::Pop8(B),
            0xC9 => Inst::Pop8(C),
            0xCA => Inst::Pop8(D),

            0xCB => Inst::Pop16(Register16::X),
            0xCC => Inst::Pop16(Register16::Y),

            0xCD => Inst::Call(JumpMode::Relative(self.fetch_byte())),
            0xCE => Inst::Call(JumpMode::Absolute(self.fetch_word())),
            0xCF => Inst::Call(JumpMode::Indirect(Register16::X, self.fetch_byte())),
            0xD0 => Inst::Call(JumpMode::Indirect(Register16::Y, self.fetch_byte())),
            0xD1 => Inst::Ret,

            0xD2 => Inst::Swi,
            0xD3 => Inst::Reti,

            0xD4 => Inst::Jmp(Condition::Always, JumpMode::Relative(self.fetch_byte())),
            0xD5 => Inst::Jmp(Condition::Always, JumpMode::Absolute(self.fetch_word())),
            0xD6 => Inst::Jmp(
                Condition::Always,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xD7 => Inst::Jmp(
                Condition::Always,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xD8 => Inst::Jmp(Condition::Equal, JumpMode::Relative(self.fetch_byte())),
            0xD9 => Inst::Jmp(Condition::Equal, JumpMode::Absolute(self.fetch_word())),
            0xDA => Inst::Jmp(
                Condition::Equal,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xDB => Inst::Jmp(
                Condition::Equal,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xDC => Inst::Jmp(Condition::NotEqual, JumpMode::Relative(self.fetch_byte())),
            0xDD => Inst::Jmp(Condition::NotEqual, JumpMode::Absolute(self.fetch_word())),
            0xDE => Inst::Jmp(
                Condition::NotEqual,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xDF => Inst::Jmp(
                Condition::NotEqual,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xE0 => Inst::Jmp(Condition::LessThan, JumpMode::Relative(self.fetch_byte())),
            0xE1 => Inst::Jmp(Condition::LessThan, JumpMode::Absolute(self.fetch_word())),
            0xE2 => Inst::Jmp(
                Condition::LessThan,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xE3 => Inst::Jmp(
                Condition::LessThan,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xE4 => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xE5 => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xE6 => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xE7 => Inst::Jmp(
                Condition::GreaterThan,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xE8 => Inst::Jmp(Condition::LessEqual, JumpMode::Relative(self.fetch_byte())),
            0xE9 => Inst::Jmp(Condition::LessEqual, JumpMode::Absolute(self.fetch_word())),
            0xEA => Inst::Jmp(
                Condition::LessEqual,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xEB => Inst::Jmp(
                Condition::LessEqual,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xEC => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xED => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xEE => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xEF => Inst::Jmp(
                Condition::GreaterEqual,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xF0 => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xF1 => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xF2 => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xF3 => Inst::Jmp(
                Condition::LessThanSigned,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xF4 => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xF5 => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xF6 => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xF7 => Inst::Jmp(
                Condition::GreaterThanSigned,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xF8 => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xF9 => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xFA => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xFB => Inst::Jmp(
                Condition::LessEqualSigned,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),

            0xFC => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Relative(self.fetch_byte()),
            ),
            0xFD => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Absolute(self.fetch_word()),
            ),
            0xFE => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Indirect(Register16::X, self.fetch_byte()),
            ),
            0xFF => Inst::Jmp(
                Condition::GreaterEqualSigned,
                JumpMode::Indirect(Register16::Y, self.fetch_byte()),
            ),
        }
    }

    fn effective_bank_address(&self, kind: MemoryAddressKind) -> Nibble {
        // A physical address (PADDR) is formed by prepending a 4-bit bank address to a 16-bit virtual address (VADDR).
        // The bank_enable flag in the status register is used to enable kernel accesses of user memory.
        // In user mode, the bank register is *always* used.
        // In kernel mode, the bank register is MUXd out for 0 during code accesses.
        // In typical operation, the bank register is also MUXd out for 0 during data accesses.
        // However if the bank_enable flag is set, then the stored bank register value is used to make data accesses.

        let kernel_bank = Nibble::new(0).unwrap();

        match self.state.status.privilege_level {
            PrivilegeLevel::User => self.state.bank_register,
            PrivilegeLevel::Kernel => match kind {
                MemoryAddressKind::Code => kernel_bank,
                MemoryAddressKind::Data => match self.state.status.bank_enable {
                    true => self.state.bank_register,
                    false => kernel_bank,
                },
            },
        }
    }

    fn next_cycle_kind(&mut self) -> CycleKind {
        if self.bus.is_rst_active() {
            return CycleKind::Reset;
        }

        if self.bus.is_nmi_active() {
            return CycleKind::Interrupt(InterruptKind::Nmi);
        }

        if self.bus.is_irq_active() {
            return CycleKind::Interrupt(InterruptKind::Irq);
        }

        if self.bus.is_req_active() {
            return CycleKind::BusStall;
        }

        CycleKind::Instruction
    }
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            bank_register: Nibble::new(0).unwrap(),
            program_counter: 0,
            registers: RegisterFile::default(),
            status: Status::default(),
        }
    }

    pub fn run<B: Bus>(&mut self, bus: &mut B, cycles: usize) -> (trace::Trace, ReachedBreakpoint) {
        let mut trace = trace::Trace::new();

        let mut cpu = Cpu {
            state: self,
            bus: bus,
        };

        for _ in 0..cycles {
            match cpu.next_cycle_kind() {
                CycleKind::BusStall => continue,
                CycleKind::Interrupt(InterruptKind::Irq) => {
                    if cpu.state.status.irq_enable {
                        // TODO: Servicing interrupt should be execution result option
                        cpu.service_interrupt(InterruptKind::Irq);
                    } else {
                        match cpu.execute() {
                            ExecutionResult::Instruction(inst) => trace.add(&inst),
                            ExecutionResult::Action(inst, action) => {
                                trace.add(&inst);
                                match action {
                                    EnvironmentAction::Halt => break,
                                    EnvironmentAction::Break => {
                                        return (trace, ReachedBreakpoint::Did)
                                    }
                                    EnvironmentAction::WriteByte(val) => print!("{}", val as char),
                                }
                            }
                        }
                    }
                }
                CycleKind::Interrupt(InterruptKind::Nmi) => {
                    if cpu.state.status.nmi_active {
                        println!("Received nested NMI; system resetting.");
                        cpu.reset(); // TODO: Should reset entire system not just CPU
                    } else {
                        cpu.service_interrupt(InterruptKind::Nmi);
                    }
                }
                CycleKind::Interrupt(kind) => cpu.service_interrupt(kind),
                CycleKind::Reset => cpu.reset(),
                CycleKind::Instruction => match cpu.execute() {
                    ExecutionResult::Instruction(inst) => trace.add(&inst),
                    ExecutionResult::Action(inst, action) => {
                        trace.add(&inst);
                        match action {
                            EnvironmentAction::Halt => break,
                            EnvironmentAction::Break => return (trace, ReachedBreakpoint::Did),
                            EnvironmentAction::WriteByte(val) => print!("{}", val as char),
                        }
                    }
                },
            }
        }

        (trace, ReachedBreakpoint::DidNot)
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn br(&self) -> &Nibble {
        &self.bank_register
    }
}

impl std::ops::Index<Architectural8> for CpuState {
    type Output = Byte;

    fn index(&self, index: Architectural8) -> &Self::Output {
        match index {
            Architectural8::A => &self.registers[Register8::A],
            Architectural8::B => &self.registers[Register8::B],
            Architectural8::C => &self.registers[Register8::C],
            Architectural8::D => &self.registers[Register8::D],
        }
    }
}

impl std::ops::IndexMut<Architectural8> for CpuState {
    fn index_mut(&mut self, index: Architectural8) -> &mut Self::Output {
        match index {
            Architectural8::A => &mut self.registers[Register8::A],
            Architectural8::B => &mut self.registers[Register8::B],
            Architectural8::C => &mut self.registers[Register8::C],
            Architectural8::D => &mut self.registers[Register8::D],
        }
    }
}

impl std::ops::Index<Architectural16> for CpuState {
    type Output = Word;

    fn index(&self, index: Architectural16) -> &Self::Output {
        match index {
            Architectural16::PC => &self.program_counter,
            Architectural16::SP => &self.registers[Pointer::SP],
            Architectural16::X => &self.registers[Pointer::X],
            Architectural16::Y => &self.registers[Pointer::Y],
        }
    }
}

impl std::ops::IndexMut<Architectural16> for CpuState {
    fn index_mut(&mut self, index: Architectural16) -> &mut Self::Output {
        match index {
            Architectural16::PC => &mut self.program_counter,
            Architectural16::SP => &mut self.registers[Pointer::SP],
            Architectural16::X => &mut self.registers[Pointer::X],
            Architectural16::Y => &mut self.registers[Pointer::Y],
        }
    }
}

#[inline]
const fn split_bytes(word: Address) -> (Byte, Byte) {
    let low = (word & 0x00FF) as u8;
    let high = ((word & 0xFF00) >> 8) as u8;
    (high, low)
}

#[inline]
const fn concatenate(high: Byte, low: Byte) -> Address {
    (high as Address) << 8 | (low as Address)
}

#[inline]
const fn address_with_offset(address: Address, offset: Byte) -> Address {
    ((address as i32) + (offset as i32)) as u16
}

#[inline]
const fn increment_byte(byte: Byte) -> Byte {
    if byte == 0xFF {
        0x00
    } else {
        byte + 1
    }
}

#[inline]
const fn decrement_byte(byte: Byte) -> Byte {
    if byte == 0x00 {
        0xFF
    } else {
        byte - 1
    }
}

#[inline]
const fn increment_word(word: Address) -> Address {
    if word == 0xFFFF {
        0x0000
    } else {
        word + 1
    }
}

#[inline]
const fn decrement_word(word: Address) -> Address {
    if word == 0x0000 {
        0xFFFF
    } else {
        word - 1
    }
}

#[inline]
const fn wrapping_subtract(left: Byte, right: Byte) -> Byte {
    if right > left {
        right - left
    } else {
        left - right
    }
}
