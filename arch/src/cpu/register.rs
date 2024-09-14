use std::ops::{Index, IndexMut};

use crate::*;

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
pub enum Pointer {
    X,
    Y,
    SP,
}

#[derive(Default)]
pub struct RegisterFile {
    gpr8: [Byte; 4],
    gpr16: [Address; 3],
}

impl Index<Register8> for RegisterFile {
    type Output = Byte;

    fn index(&self, index: Register8) -> &Self::Output {
        let index = match index {
            Register8::A => 0,
            Register8::B => 1,
            Register8::C => 2,
            Register8::D => 3,
        };

        &self.gpr8[index]
    }
}

impl IndexMut<Register8> for RegisterFile {
    fn index_mut(&mut self, index: Register8) -> &mut Self::Output {
        let index = match index {
            Register8::A => 0,
            Register8::B => 1,
            Register8::C => 2,
            Register8::D => 3,
        };

        &mut self.gpr8[index]
    }
}

impl Index<Register16> for RegisterFile {
    type Output = Address;

    fn index(&self, index: Register16) -> &Self::Output {
        let index = match index {
            Register16::X => 0,
            Register16::Y => 1,
        };

        &self.gpr16[index]
    }
}

impl IndexMut<Register16> for RegisterFile {
    fn index_mut(&mut self, index: Register16) -> &mut Self::Output {
        let index = match index {
            Register16::X => 0,
            Register16::Y => 1,
        };

        &mut self.gpr16[index]
    }
}

impl Index<Pointer> for RegisterFile {
    type Output = Address;

    fn index(&self, index: Pointer) -> &Self::Output {
        let index = match index {
            Pointer::X => 0,
            Pointer::Y => 1,
            Pointer::SP => 2,
        };

        &self.gpr16[index]
    }
}

impl IndexMut<Pointer> for RegisterFile {
    fn index_mut(&mut self, index: Pointer) -> &mut Self::Output {
        let index = match index {
            Pointer::X => 0,
            Pointer::Y => 1,
            Pointer::SP => 2,
        };

        &mut self.gpr16[index]
    }
}
