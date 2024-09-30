pub mod bus;
mod cpu;
pub mod trace;

pub use bus::*;
pub use cpu::*;

use refinement::{Predicate, Refinement};

pub struct UpperNibbleClear;

impl Predicate<u8> for UpperNibbleClear {
    fn test(x: &u8) -> bool {
        (x >> 4) == 0
    }
}

pub enum Architectural8 {
    A,
    B,
    C,
    D,
}

pub enum Architectural16 {
    PC,
    SP,
    X,
    Y,
}

pub type Byte = u8;
pub type Address = u16; // TODO: Remove Address and just use Word.
pub type Word = Address;
pub type Nibble = Refinement<u8, UpperNibbleClear>;
