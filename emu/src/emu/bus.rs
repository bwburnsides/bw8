use crate::emu::uart::Uart;
use arch::{self, Address, Byte};

pub struct Bw8Bus {
    memory: [Byte; 0x1_0000],
    framebuffer: [Byte; 28 * 1024],
    _uart: Uart,
    // pub vga: Vga, // TODO: Temporary. Combine Bw8Bus and Bw8.
    pending_rst: bool,
    pending_nmi: bool,
    pending_irq: bool,
}

impl Bw8Bus {
    pub fn new(binary_path: String) -> Self {
        // TODO: Make this code not the worst thing you've ever seen.
        let bytes = std::fs::read(binary_path).unwrap();
        let mut memory: [Byte; 0x1_0000] = [0x00; 0x1_0000];
        for (idx, byte) in bytes.into_iter().enumerate() {
            memory[idx] = byte;
        }

        Self {
            memory,
            framebuffer: [0x0; 28 * 1024],
            _uart: Uart::new(),
            // vga: Vga::new(),
            pending_rst: false,
            pending_nmi: false,
            pending_irq: false,
        }
    }

    pub fn reset(&mut self) {
        self.pending_rst = false;
        self.pending_nmi = false;
        self.pending_irq = false;
    }

    pub fn inspect_memory(&self, address: Address) -> Byte {
        self.memory[address as usize]
    }

    pub fn inspect_framebuffer(&self, address: Address) -> Byte {
        self.framebuffer[address as usize]
    }

    pub fn set_reset(&mut self, state: bool) {
        self.pending_rst = state
    }

    pub fn set_irq(&mut self, state: bool) {
        self.pending_irq = state
    }

    pub fn set_nmi(&mut self, state: bool) {
        self.pending_nmi = state
    }
}

impl arch::Bus for Bw8Bus {
    fn memory_read(
        &self,
        _privilege: arch::PrivilegeLevel,
        _kind: arch::MemoryAddressKind,
        address: arch::PhysicalAddress,
    ) -> arch::Byte {
        self.memory[address.base as usize]
    }

    fn memory_write(
        &mut self,
        _privilege: arch::PrivilegeLevel,
        _kind: arch::MemoryAddressKind,
        address: arch::PhysicalAddress,
        data: arch::Byte,
    ) {
        if address.base >= 0x8000 {
            self.memory[address.base as usize] = data;
        }
    }

    fn io_read(
        &mut self,
        _privilege: arch::PrivilegeLevel,
        _address: arch::PhysicalAddress,
    ) -> arch::BusResult<arch::Byte> {
        arch::BusResult::Data(0)
    }

    fn io_write(
        &mut self,
        _privilege: arch::PrivilegeLevel,
        address: arch::PhysicalAddress,
        data: arch::Byte,
    ) -> arch::BusResult<()> {
        match address.base {
            0x01 => arch::BusResult::Action(arch::EnvironmentAction::WriteByte(data)),
            0x02 => {
                self.pending_irq = false;
                arch::BusResult::Data(())
            }
            0x03 => {
                println!("Reached breakpoint!");
                arch::BusResult::Action(arch::EnvironmentAction::Break)
            }
            0x8000..=0xEFFF => {
                let offset = address.base - 0x8000;
                self.framebuffer[offset as usize] = data;
                arch::BusResult::Data(())
            }
            _ => arch::BusResult::Data(()),
        }
    }

    fn is_rst_active(&self) -> bool {
        self.pending_rst
    }

    fn is_nmi_active(&mut self) -> bool {
        let rv = self.pending_nmi;
        self.pending_nmi = false;
        rv
    }

    fn is_irq_active(&self) -> bool {
        self.pending_irq
    }

    fn is_req_active(&self) -> bool {
        false
    }
}
