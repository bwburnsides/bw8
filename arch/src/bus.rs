use crate::cpu::*;
use crate::*;

#[derive(Copy, Clone)]
pub enum MemoryAddressKind {
    Code,
    Data,
}

pub struct PhysicalAddress {
    pub bank: Nibble,
    pub base: Address,
}

impl PhysicalAddress {
    pub fn new(bank: Nibble, base: Address) -> Self {
        Self { bank, base }
    }
}

#[derive(Debug)]
pub enum EnvironmentAction {
    Halt,
    Break,
    WriteByte(Byte),
}

pub enum BusResult<T> {
    Data(T),
    Action(EnvironmentAction),
}

pub trait Bus {
    fn memory_read(
        &self,
        privilege: PrivilegeLevel,
        kind: MemoryAddressKind,
        address: PhysicalAddress,
    ) -> Byte;

    fn memory_write(
        &mut self,
        privilege: PrivilegeLevel,
        kind: MemoryAddressKind,
        address: PhysicalAddress,
        data: Byte,
    );

    fn io_read(&mut self, privilege: PrivilegeLevel, address: PhysicalAddress) -> BusResult<Byte>;

    fn io_write(
        &mut self,
        privilege: PrivilegeLevel,
        address: PhysicalAddress,
        data: Byte,
    ) -> BusResult<()>;

    fn is_rst_active(&self) -> bool;

    fn is_nmi_active(&mut self) -> bool;

    fn is_irq_active(&self) -> bool;

    fn is_req_active(&self) -> bool;
}
