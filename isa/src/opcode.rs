use crate::{Byte, Word, InstructionBytes};

#[derive(PartialEq)]
pub enum Opcode {
    Normal(Byte),
    Extended(Byte),
}

impl Opcode {
    pub fn encode_with_byte(&self, byte: Byte) -> InstructionBytes {
        self.encode_with(InstructionBytes::One(byte))
    }

    pub fn encode_with_word(&self, word: Word) -> InstructionBytes {
        let low = (word & 255) as Byte;
        let high = (word >> 8) as Byte;

        self.encode_with(InstructionBytes::Two(low, high))
    }

    pub fn encode_with(&self, bytes: InstructionBytes) -> InstructionBytes {
        match self {
            Self::Normal(byte) => match bytes {
                InstructionBytes::One(operand) => InstructionBytes::Two(*byte, operand),
                InstructionBytes::Two(low, high) => InstructionBytes::Three(*byte, low, high),
                InstructionBytes::Three(operand, low, high) => {
                    InstructionBytes::Four(*byte, operand, low, high)
                }
                InstructionBytes::Four(..) => panic!(),
            },
            Self::Extended(byte) => match bytes {
                InstructionBytes::One(operand) => {
                    EXT.encode_with(InstructionBytes::Two(*byte, operand))
                }
                InstructionBytes::Two(low, high) => {
                    EXT.encode_with(InstructionBytes::Three(*byte, low, high))
                }
                InstructionBytes::Three(operand, low, high) => {
                    EXT.encode_with(InstructionBytes::Four(*byte, operand, low, high))
                }
                InstructionBytes::Four(..) => panic!(),
            },
        }
    }

    pub fn encode(&self) -> InstructionBytes {
        match self {
            Self::Normal(byte) => InstructionBytes::One(*byte),
            Self::Extended(byte) => EXT.encode_with(InstructionBytes::One(*byte)),
        }
    }
}

pub const NOP: Opcode = Opcode::Normal(0x00);

pub const EXT: Opcode = Opcode::Normal(0x01);

pub const SET_C: Opcode = Opcode::Normal(0x02);
pub const CLR_C: Opcode = Opcode::Normal(0x03);

pub const SET_I: Opcode = Opcode::Normal(0x04);
pub const CLR_I: Opcode = Opcode::Normal(0x05);

pub const SET_B: Opcode = Opcode::Normal(0x06);
pub const CLR_B: Opcode = Opcode::Normal(0x07);

pub const MV_A_BR: Opcode = Opcode::Normal(0x08);
pub const MV_BR_A: Opcode = Opcode::Normal(0x09);

pub const MV_A_A: Opcode = Opcode::Normal(0x0a);
pub const MV_A_B: Opcode = Opcode::Normal(0x0b);
pub const MV_A_C: Opcode = Opcode::Normal(0x0c);
pub const MV_A_D: Opcode = Opcode::Normal(0x0d);
pub const MV_B_A: Opcode = Opcode::Normal(0x0e);
pub const MV_B_B: Opcode = Opcode::Normal(0x0f);
pub const MV_B_C: Opcode = Opcode::Normal(0x10);
pub const MV_B_D: Opcode = Opcode::Normal(0x11);
pub const MV_C_A: Opcode = Opcode::Normal(0x12);
pub const MV_C_B: Opcode = Opcode::Normal(0x13);
pub const MV_C_C: Opcode = Opcode::Normal(0x14);
pub const MV_C_D: Opcode = Opcode::Normal(0x15);
pub const MV_D_A: Opcode = Opcode::Normal(0x16);
pub const MV_D_B: Opcode = Opcode::Normal(0x17);
pub const MV_D_C: Opcode = Opcode::Normal(0x18);
pub const MV_D_D: Opcode = Opcode::Normal(0x19);

pub const LD_A_IMM: Opcode = Opcode::Normal(0x1a);
pub const LD_B_IMM: Opcode = Opcode::Normal(0x1b);
pub const LD_C_IMM: Opcode = Opcode::Normal(0x1c);
pub const LD_D_IMM: Opcode = Opcode::Normal(0x1d);

pub const LD_A_ABS: Opcode = Opcode::Normal(0x1e);
pub const LD_A_REL_X_BY_IMM: Opcode = Opcode::Normal(0x1f);
pub const LD_A_REL_Y_BY_IMM: Opcode = Opcode::Normal(0x20);
pub const LD_A_REL_SP_BY_IMM: Opcode = Opcode::Normal(0x21);
pub const LD_A_REL_X_BY_A: Opcode = Opcode::Normal(0x22);
pub const LD_A_REL_X_BY_B: Opcode = Opcode::Normal(0x23);
pub const LD_A_REL_X_BY_C: Opcode = Opcode::Normal(0x24);
pub const LD_A_REL_X_BY_D: Opcode = Opcode::Normal(0x25);
pub const LD_A_REL_Y_BY_A: Opcode = Opcode::Normal(0x26);
pub const LD_A_REL_Y_BY_B: Opcode = Opcode::Normal(0x27);
pub const LD_A_REL_Y_BY_C: Opcode = Opcode::Normal(0x28);
pub const LD_A_REL_Y_BY_D: Opcode = Opcode::Normal(0x29);
pub const LD_A_REL_SP_BY_A: Opcode = Opcode::Normal(0x2a);
pub const LD_A_REL_SP_BY_B: Opcode = Opcode::Normal(0x2b);
pub const LD_A_REL_SP_BY_C: Opcode = Opcode::Normal(0x2c);
pub const LD_A_REL_SP_BY_D: Opcode = Opcode::Normal(0x2d);

pub const LD_B_ABS: Opcode = Opcode::Normal(0x2e);
pub const LD_B_REL_X_BY_IMM: Opcode = Opcode::Normal(0x2f);
pub const LD_B_REL_Y_BY_IMM: Opcode = Opcode::Normal(0x30);
pub const LD_B_REL_SP_BY_IMM: Opcode = Opcode::Normal(0x31);
pub const LD_B_REL_X_BY_A: Opcode = Opcode::Normal(0x32);
pub const LD_B_REL_X_BY_B: Opcode = Opcode::Normal(0x33);
pub const LD_B_REL_X_BY_C: Opcode = Opcode::Normal(0x34);
pub const LD_B_REL_X_BY_D: Opcode = Opcode::Normal(0x35);
pub const LD_B_REL_Y_BY_A: Opcode = Opcode::Normal(0x36);
pub const LD_B_REL_Y_BY_B: Opcode = Opcode::Normal(0x37);
pub const LD_B_REL_Y_BY_C: Opcode = Opcode::Normal(0x38);
pub const LD_B_REL_Y_BY_D: Opcode = Opcode::Normal(0x39);
pub const LD_B_REL_SP_BY_A: Opcode = Opcode::Normal(0x3a);
pub const LD_B_REL_SP_BY_B: Opcode = Opcode::Normal(0x3b);
pub const LD_B_REL_SP_BY_C: Opcode = Opcode::Normal(0x3c);
pub const LD_B_REL_SP_BY_D: Opcode = Opcode::Normal(0x3d);

pub const LD_C_ABS: Opcode = Opcode::Normal(0x3e);
pub const LD_C_REL_X_BY_IMM: Opcode = Opcode::Normal(0x3f);
pub const LD_C_REL_Y_BY_IMM: Opcode = Opcode::Normal(0x40);
pub const LD_C_REL_SP_BY_IMM: Opcode = Opcode::Normal(0x41);
pub const LD_C_REL_X_BY_A: Opcode = Opcode::Normal(0x42);
pub const LD_C_REL_X_BY_B: Opcode = Opcode::Normal(0x43);
pub const LD_C_REL_X_BY_C: Opcode = Opcode::Normal(0x44);
pub const LD_C_REL_X_BY_D: Opcode = Opcode::Normal(0x45);
pub const LD_C_REL_Y_BY_A: Opcode = Opcode::Normal(0x46);
pub const LD_C_REL_Y_BY_B: Opcode = Opcode::Normal(0x47);
pub const LD_C_REL_Y_BY_C: Opcode = Opcode::Normal(0x48);
pub const LD_C_REL_Y_BY_D: Opcode = Opcode::Normal(0x49);
pub const LD_C_REL_SP_BY_A: Opcode = Opcode::Normal(0x4a);
pub const LD_C_REL_SP_BY_B: Opcode = Opcode::Normal(0x4b);
pub const LD_C_REL_SP_BY_C: Opcode = Opcode::Normal(0x4c);
pub const LD_C_REL_SP_BY_D: Opcode = Opcode::Normal(0x4d);

pub const LD_D_ABS: Opcode = Opcode::Normal(0x4e);
pub const LD_D_REL_X_BY_IMM: Opcode = Opcode::Normal(0x4f);
pub const LD_D_REL_Y_BY_IMM: Opcode = Opcode::Normal(0x50);
pub const LD_D_REL_SP_BY_IMM: Opcode = Opcode::Normal(0x51);
pub const LD_D_REL_X_BY_A: Opcode = Opcode::Normal(0x52);
pub const LD_D_REL_X_BY_B: Opcode = Opcode::Normal(0x53);
pub const LD_D_REL_X_BY_C: Opcode = Opcode::Normal(0x54);
pub const LD_D_REL_X_BY_D: Opcode = Opcode::Normal(0x55);
pub const LD_D_REL_Y_BY_A: Opcode = Opcode::Normal(0x56);
pub const LD_D_REL_Y_BY_B: Opcode = Opcode::Normal(0x57);
pub const LD_D_REL_Y_BY_C: Opcode = Opcode::Normal(0x58);
pub const LD_D_REL_Y_BY_D: Opcode = Opcode::Normal(0x59);
pub const LD_D_REL_SP_BY_A: Opcode = Opcode::Normal(0x5a);
pub const LD_D_REL_SP_BY_B: Opcode = Opcode::Normal(0x5b);
pub const LD_D_REL_SP_BY_C: Opcode = Opcode::Normal(0x5c);
pub const LD_D_REL_SP_BY_D: Opcode = Opcode::Normal(0x5d);

pub const ST_ABS_A: Opcode = Opcode::Normal(0x5e);
pub const ST_REL_X_BY_IMM_A: Opcode = Opcode::Normal(0x5f);
pub const ST_REL_Y_BY_IMM_A: Opcode = Opcode::Normal(0x60);
pub const ST_REL_SP_BY_IMM_A: Opcode = Opcode::Normal(0x61);
pub const ST_REL_X_BY_A_A: Opcode = Opcode::Normal(0x62);
pub const ST_REL_X_BY_B_A: Opcode = Opcode::Normal(0x63);
pub const ST_REL_X_BY_C_A: Opcode = Opcode::Normal(0x64);
pub const ST_REL_X_BY_D_A: Opcode = Opcode::Normal(0x65);
pub const ST_REL_Y_BY_A_A: Opcode = Opcode::Normal(0x66);
pub const ST_REL_Y_BY_B_A: Opcode = Opcode::Normal(0x67);
pub const ST_REL_Y_BY_C_A: Opcode = Opcode::Normal(0x68);
pub const ST_REL_Y_BY_D_A: Opcode = Opcode::Normal(0x69);
pub const ST_REL_SP_BY_A_A: Opcode = Opcode::Normal(0x6a);
pub const ST_REL_SP_BY_B_A: Opcode = Opcode::Normal(0x6b);
pub const ST_REL_SP_BY_C_A: Opcode = Opcode::Normal(0x6c);
pub const ST_REL_SP_BY_D_A: Opcode = Opcode::Normal(0x6d);

pub const ST_ABS_B: Opcode = Opcode::Normal(0x6e);
pub const ST_REL_X_BY_IMM_B: Opcode = Opcode::Normal(0x6f);
pub const ST_REL_Y_BY_IMM_B: Opcode = Opcode::Normal(0x70);
pub const ST_REL_SP_BY_IMM_B: Opcode = Opcode::Normal(0x71);
pub const ST_REL_X_BY_A_B: Opcode = Opcode::Normal(0x72);
pub const ST_REL_X_BY_B_B: Opcode = Opcode::Normal(0x73);
pub const ST_REL_X_BY_C_B: Opcode = Opcode::Normal(0x74);
pub const ST_REL_X_BY_D_B: Opcode = Opcode::Normal(0x75);
pub const ST_REL_Y_BY_A_B: Opcode = Opcode::Normal(0x76);
pub const ST_REL_Y_BY_B_B: Opcode = Opcode::Normal(0x77);
pub const ST_REL_Y_BY_C_B: Opcode = Opcode::Normal(0x78);
pub const ST_REL_Y_BY_D_B: Opcode = Opcode::Normal(0x79);
pub const ST_REL_SP_BY_A_B: Opcode = Opcode::Normal(0x7a);
pub const ST_REL_SP_BY_B_B: Opcode = Opcode::Normal(0x7b);
pub const ST_REL_SP_BY_C_B: Opcode = Opcode::Normal(0x7c);
pub const ST_REL_SP_BY_D_B: Opcode = Opcode::Normal(0x7d);

pub const ST_ABS_C: Opcode = Opcode::Normal(0x7e);
pub const ST_REL_X_BY_IMM_C: Opcode = Opcode::Normal(0x7f);
pub const ST_REL_Y_BY_IMM_C: Opcode = Opcode::Normal(0x80);
pub const ST_REL_SP_BY_IMM_C: Opcode = Opcode::Normal(0x81);
pub const ST_REL_X_BY_A_C: Opcode = Opcode::Normal(0x82);
pub const ST_REL_X_BY_B_C: Opcode = Opcode::Normal(0x83);
pub const ST_REL_X_BY_C_C: Opcode = Opcode::Normal(0x84);
pub const ST_REL_X_BY_D_C: Opcode = Opcode::Normal(0x85);
pub const ST_REL_Y_BY_A_C: Opcode = Opcode::Normal(0x86);
pub const ST_REL_Y_BY_B_C: Opcode = Opcode::Normal(0x87);
pub const ST_REL_Y_BY_C_C: Opcode = Opcode::Normal(0x88);
pub const ST_REL_Y_BY_D_C: Opcode = Opcode::Normal(0x89);
pub const ST_REL_SP_BY_A_C: Opcode = Opcode::Normal(0x8a);
pub const ST_REL_SP_BY_B_C: Opcode = Opcode::Normal(0x8b);
pub const ST_REL_SP_BY_C_C: Opcode = Opcode::Normal(0x8c);
pub const ST_REL_SP_BY_D_C: Opcode = Opcode::Normal(0x8d);

pub const ST_ABS_D: Opcode = Opcode::Normal(0x8e);
pub const ST_REL_X_BY_IMM_D: Opcode = Opcode::Normal(0x8f);
pub const ST_REL_Y_BY_IMM_D: Opcode = Opcode::Normal(0x90);
pub const ST_REL_SP_BY_IMM_D: Opcode = Opcode::Normal(0x91);
pub const ST_REL_X_BY_A_D: Opcode = Opcode::Normal(0x92);
pub const ST_REL_X_BY_B_D: Opcode = Opcode::Normal(0x93);
pub const ST_REL_X_BY_C_D: Opcode = Opcode::Normal(0x94);
pub const ST_REL_X_BY_D_D: Opcode = Opcode::Normal(0x95);
pub const ST_REL_Y_BY_A_D: Opcode = Opcode::Normal(0x96);
pub const ST_REL_Y_BY_B_D: Opcode = Opcode::Normal(0x97);
pub const ST_REL_Y_BY_C_D: Opcode = Opcode::Normal(0x98);
pub const ST_REL_Y_BY_D_D: Opcode = Opcode::Normal(0x99);
pub const ST_REL_SP_BY_A_D: Opcode = Opcode::Normal(0x9a);
pub const ST_REL_SP_BY_B_D: Opcode = Opcode::Normal(0x9b);
pub const ST_REL_SP_BY_C_D: Opcode = Opcode::Normal(0x9c);
pub const ST_REL_SP_BY_D_D: Opcode = Opcode::Normal(0x9d);

pub const IN_A_PORT: Opcode = Opcode::Normal(0x9e);
pub const IN_A_REL_X_BY_IMM: Opcode = Opcode::Normal(0x9f);
pub const IN_A_REL_Y_BY_IMM: Opcode = Opcode::Normal(0xa0);
pub const IN_A_REL_X_BY_A: Opcode = Opcode::Normal(0xa1);
pub const IN_A_REL_X_BY_B: Opcode = Opcode::Normal(0xa2);
pub const IN_A_REL_X_BY_C: Opcode = Opcode::Normal(0xa3);
pub const IN_A_REL_X_BY_D: Opcode = Opcode::Normal(0xa4);
pub const IN_A_REL_Y_BY_A: Opcode = Opcode::Normal(0xa5);
pub const IN_A_REL_Y_BY_B: Opcode = Opcode::Normal(0xa6);
pub const IN_A_REL_Y_BY_C: Opcode = Opcode::Normal(0xa7);
pub const IN_A_REL_Y_BY_D: Opcode = Opcode::Normal(0xa8);

pub const IN_B_PORT: Opcode = Opcode::Normal(0xa9);
pub const IN_B_REL_X_BY_IMM: Opcode = Opcode::Normal(0xaa);
pub const IN_B_REL_Y_BY_IMM: Opcode = Opcode::Normal(0xab);
pub const IN_B_REL_X_BY_A: Opcode = Opcode::Normal(0xac);
pub const IN_B_REL_X_BY_B: Opcode = Opcode::Normal(0xad);
pub const IN_B_REL_X_BY_C: Opcode = Opcode::Normal(0xae);
pub const IN_B_REL_X_BY_D: Opcode = Opcode::Normal(0xaf);
pub const IN_B_REL_Y_BY_A: Opcode = Opcode::Normal(0xb0);
pub const IN_B_REL_Y_BY_B: Opcode = Opcode::Normal(0xb1);
pub const IN_B_REL_Y_BY_C: Opcode = Opcode::Normal(0xb2);
pub const IN_B_REL_Y_BY_D: Opcode = Opcode::Normal(0xb3);

pub const IN_C_PORT: Opcode = Opcode::Normal(0xb4);
pub const IN_C_REL_X_BY_IMM: Opcode = Opcode::Normal(0xb5);
pub const IN_C_REL_Y_BY_IMM: Opcode = Opcode::Normal(0xb6);
pub const IN_C_REL_X_BY_A: Opcode = Opcode::Normal(0xb7);
pub const IN_C_REL_X_BY_B: Opcode = Opcode::Normal(0xb8);
pub const IN_C_REL_X_BY_C: Opcode = Opcode::Normal(0xb9);
pub const IN_C_REL_X_BY_D: Opcode = Opcode::Normal(0xba);
pub const IN_C_REL_Y_BY_A: Opcode = Opcode::Normal(0xbb);
pub const IN_C_REL_Y_BY_B: Opcode = Opcode::Normal(0xbc);
pub const IN_C_REL_Y_BY_C: Opcode = Opcode::Normal(0xbd);
pub const IN_C_REL_Y_BY_D: Opcode = Opcode::Normal(0xbe);

pub const IN_D_PORT: Opcode = Opcode::Normal(0xbf);
pub const IN_D_REL_X_BY_IMM: Opcode = Opcode::Normal(0xc0);
pub const IN_D_REL_Y_BY_IMM: Opcode = Opcode::Normal(0xc1);
pub const IN_D_REL_X_BY_A: Opcode = Opcode::Normal(0xc2);
pub const IN_D_REL_X_BY_B: Opcode = Opcode::Normal(0xc3);
pub const IN_D_REL_X_BY_C: Opcode = Opcode::Normal(0xc4);
pub const IN_D_REL_X_BY_D: Opcode = Opcode::Normal(0xc5);
pub const IN_D_REL_Y_BY_A: Opcode = Opcode::Normal(0xc6);
pub const IN_D_REL_Y_BY_B: Opcode = Opcode::Normal(0xc7);
pub const IN_D_REL_Y_BY_C: Opcode = Opcode::Normal(0xc8);
pub const IN_D_REL_Y_BY_D: Opcode = Opcode::Normal(0xc9);

pub const OUT_PORT_A: Opcode = Opcode::Normal(0xca);
pub const OUT_REL_X_BY_IMM_A: Opcode = Opcode::Normal(0xcb);
pub const OUT_REL_Y_BY_IMM_A: Opcode = Opcode::Normal(0xcc);
pub const OUT_REL_X_BY_A_A: Opcode = Opcode::Normal(0xcd);
pub const OUT_REL_X_BY_B_A: Opcode = Opcode::Normal(0xce);
pub const OUT_REL_X_BY_C_A: Opcode = Opcode::Normal(0xcf);
pub const OUT_REL_X_BY_D_A: Opcode = Opcode::Normal(0xd0);
pub const OUT_REL_Y_BY_A_A: Opcode = Opcode::Normal(0xd1);
pub const OUT_REL_Y_BY_B_A: Opcode = Opcode::Normal(0xd2);
pub const OUT_REL_Y_BY_C_A: Opcode = Opcode::Normal(0xd3);
pub const OUT_REL_Y_BY_D_A: Opcode = Opcode::Normal(0xd4);

pub const OUT_PORT_B: Opcode = Opcode::Normal(0xd5);
pub const OUT_REL_X_BY_IMM_B: Opcode = Opcode::Normal(0xd6);
pub const OUT_REL_Y_BY_IMM_B: Opcode = Opcode::Normal(0xd7);
pub const OUT_REL_X_BY_A_B: Opcode = Opcode::Normal(0xd8);
pub const OUT_REL_X_BY_B_B: Opcode = Opcode::Normal(0xd9);
pub const OUT_REL_X_BY_C_B: Opcode = Opcode::Normal(0xda);
pub const OUT_REL_X_BY_D_B: Opcode = Opcode::Normal(0xdb);
pub const OUT_REL_Y_BY_A_B: Opcode = Opcode::Normal(0xdc);
pub const OUT_REL_Y_BY_B_B: Opcode = Opcode::Normal(0xdd);
pub const OUT_REL_Y_BY_C_B: Opcode = Opcode::Normal(0xde);
pub const OUT_REL_Y_BY_D_B: Opcode = Opcode::Normal(0xdf);

pub const OUT_PORT_C: Opcode = Opcode::Normal(0xe0);
pub const OUT_REL_X_BY_IMM_C: Opcode = Opcode::Normal(0xe1);
pub const OUT_REL_Y_BY_IMM_C: Opcode = Opcode::Normal(0xe2);
pub const OUT_REL_X_BY_A_C: Opcode = Opcode::Normal(0xe3);
pub const OUT_REL_X_BY_B_C: Opcode = Opcode::Normal(0xe4);
pub const OUT_REL_X_BY_C_C: Opcode = Opcode::Normal(0xe5);
pub const OUT_REL_X_BY_D_C: Opcode = Opcode::Normal(0xe6);
pub const OUT_REL_Y_BY_A_C: Opcode = Opcode::Normal(0xe7);
pub const OUT_REL_Y_BY_B_C: Opcode = Opcode::Normal(0xe8);
pub const OUT_REL_Y_BY_C_C: Opcode = Opcode::Normal(0xe9);
pub const OUT_REL_Y_BY_D_C: Opcode = Opcode::Normal(0xea);

pub const OUT_PORT_D: Opcode = Opcode::Normal(0xeb);
pub const OUT_REL_X_BY_IMM_D: Opcode = Opcode::Normal(0xec);
pub const OUT_REL_Y_BY_IMM_D: Opcode = Opcode::Normal(0xed);
pub const OUT_REL_X_BY_A_D: Opcode = Opcode::Normal(0xee);
pub const OUT_REL_X_BY_B_D: Opcode = Opcode::Normal(0xef);
pub const OUT_REL_X_BY_C_D: Opcode = Opcode::Normal(0xf0);
pub const OUT_REL_X_BY_D_D: Opcode = Opcode::Normal(0xf1);
pub const OUT_REL_Y_BY_A_D: Opcode = Opcode::Normal(0xf2);
pub const OUT_REL_Y_BY_B_D: Opcode = Opcode::Normal(0xf3);
pub const OUT_REL_Y_BY_C_D: Opcode = Opcode::Normal(0xf4);
pub const OUT_REL_Y_BY_D_D: Opcode = Opcode::Normal(0xf5);

pub const MV_X_SP: Opcode = Opcode::Normal(0xf6);
pub const MV_SP_X: Opcode = Opcode::Normal(0xf7);

pub const MV_X_X: Opcode = Opcode::Normal(0xf8);
pub const MV_X_Y: Opcode = Opcode::Normal(0xf9);
pub const MV_X_AB: Opcode = Opcode::Normal(0xfa);
pub const MV_X_CD: Opcode = Opcode::Normal(0xfb);

pub const MV_Y_X: Opcode = Opcode::Normal(0xfc);
pub const MV_Y_Y: Opcode = Opcode::Normal(0xfd);
pub const MV_Y_AB: Opcode = Opcode::Normal(0xfe);
pub const MV_Y_CD: Opcode = Opcode::Normal(0xff);

pub const MV_AB_X: Opcode = Opcode::Extended(0x00);
pub const MV_AB_Y: Opcode = Opcode::Extended(0x01);

pub const MV_CD_X: Opcode = Opcode::Extended(0x02);
pub const MV_CD_Y: Opcode = Opcode::Extended(0x03);

pub const LD_X_IMM: Opcode = Opcode::Extended(0x04);
pub const LD_Y_IMM: Opcode = Opcode::Extended(0x05);

pub const LD_X_ABS: Opcode = Opcode::Extended(0x06);
pub const LD_X_REL_X_BY_IMM: Opcode = Opcode::Extended(0x07);
pub const LD_X_REL_Y_BY_IMM: Opcode = Opcode::Extended(0x08);
pub const LD_X_REL_SP_BY_IMM: Opcode = Opcode::Extended(0x09);

pub const LD_Y_ABS: Opcode = Opcode::Extended(0x0a);
pub const LD_Y_REL_X_BY_IMM: Opcode = Opcode::Extended(0x0b);
pub const LD_Y_REL_Y_BY_IMM: Opcode = Opcode::Extended(0x0c);
pub const LD_Y_REL_SP_BY_IMM: Opcode = Opcode::Extended(0x0d);

pub const ST_ABS_X: Opcode = Opcode::Extended(0x0e);
pub const ST_REL_X_BY_IMM_X: Opcode = Opcode::Extended(0x0f);
pub const ST_REL_Y_BY_IMM_X: Opcode = Opcode::Extended(0x10);
pub const ST_REL_SP_BY_IMM_X: Opcode = Opcode::Extended(0x11);

pub const ST_ABS_Y: Opcode = Opcode::Extended(0x12);
pub const ST_REL_X_BY_IMM_Y: Opcode = Opcode::Extended(0x13);
pub const ST_REL_Y_BY_IMM_Y: Opcode = Opcode::Extended(0x14);
pub const ST_REL_SP_BY_IMM_Y: Opcode = Opcode::Extended(0x15);

pub const LEA_X_BY_A: Opcode = Opcode::Extended(0x16);
pub const LEA_X_BY_B: Opcode = Opcode::Extended(0x17);
pub const LEA_X_BY_C: Opcode = Opcode::Extended(0x18);
pub const LEA_X_BY_D: Opcode = Opcode::Extended(0x19);
pub const LEA_X_BY_IMM: Opcode = Opcode::Extended(0x1a);

pub const LEA_Y_BY_A: Opcode = Opcode::Extended(0x1b);
pub const LEA_Y_BY_B: Opcode = Opcode::Extended(0x1c);
pub const LEA_Y_BY_C: Opcode = Opcode::Extended(0x1d);
pub const LEA_Y_BY_D: Opcode = Opcode::Extended(0x1e);
pub const LEA_Y_BY_IMM: Opcode = Opcode::Extended(0x1f);

pub const LEA_SP_BY_A: Opcode = Opcode::Extended(0x20);
pub const LEA_SP_BY_B: Opcode = Opcode::Extended(0x21);
pub const LEA_SP_BY_C: Opcode = Opcode::Extended(0x22);
pub const LEA_SP_BY_D: Opcode = Opcode::Extended(0x23);
pub const LEA_SP_BY_IMM: Opcode = Opcode::Extended(0x24);

pub const INC_X: Opcode = Opcode::Extended(0x25);
pub const INC_Y: Opcode = Opcode::Extended(0x26);

pub const DEC_X: Opcode = Opcode::Extended(0x27);
pub const DEC_Y: Opcode = Opcode::Extended(0x28);

pub const ADDC_A_A: Opcode = Opcode::Extended(0x29);
pub const ADDC_A_B: Opcode = Opcode::Extended(0x2a);
pub const ADDC_A_C: Opcode = Opcode::Extended(0x2b);
pub const ADDC_A_D: Opcode = Opcode::Extended(0x2c);

pub const ADDC_B_A: Opcode = Opcode::Extended(0x2d);
pub const ADDC_B_B: Opcode = Opcode::Extended(0x2e);
pub const ADDC_B_C: Opcode = Opcode::Extended(0x2f);
pub const ADDC_B_D: Opcode = Opcode::Extended(0x30);

pub const ADDC_C_A: Opcode = Opcode::Extended(0x31);
pub const ADDC_C_B: Opcode = Opcode::Extended(0x32);
pub const ADDC_C_C: Opcode = Opcode::Extended(0x33);
pub const ADDC_C_D: Opcode = Opcode::Extended(0x34);

pub const ADDC_D_B: Opcode = Opcode::Extended(0x36);
pub const ADDC_D_A: Opcode = Opcode::Extended(0x35);
pub const ADDC_D_C: Opcode = Opcode::Extended(0x37);
pub const ADDC_D_D: Opcode = Opcode::Extended(0x38);

pub const ADDC_A_IMM: Opcode = Opcode::Extended(0x39);
pub const ADDC_B_IMM: Opcode = Opcode::Extended(0x3a);
pub const ADDC_C_IMM: Opcode = Opcode::Extended(0x3b);
pub const ADDC_D_IMM: Opcode = Opcode::Extended(0x3c);

pub const SUBB_A_A: Opcode = Opcode::Extended(0x3d);
pub const SUBB_A_B: Opcode = Opcode::Extended(0x3e);
pub const SUBB_A_C: Opcode = Opcode::Extended(0x3f);
pub const SUBB_A_D: Opcode = Opcode::Extended(0x40);

pub const SUBB_B_A: Opcode = Opcode::Extended(0x41);
pub const SUBB_B_B: Opcode = Opcode::Extended(0x42);
pub const SUBB_B_C: Opcode = Opcode::Extended(0x43);
pub const SUBB_B_D: Opcode = Opcode::Extended(0x44);

pub const SUBB_C_A: Opcode = Opcode::Extended(0x45);
pub const SUBB_C_B: Opcode = Opcode::Extended(0x46);
pub const SUBB_C_C: Opcode = Opcode::Extended(0x47);
pub const SUBB_C_D: Opcode = Opcode::Extended(0x48);

pub const SUBB_D_A: Opcode = Opcode::Extended(0x49);
pub const SUBB_D_B: Opcode = Opcode::Extended(0x4a);
pub const SUBB_D_C: Opcode = Opcode::Extended(0x4b);
pub const SUBB_D_D: Opcode = Opcode::Extended(0x4c);

pub const SUBB_A_IMM: Opcode = Opcode::Extended(0x4d);
pub const SUBB_B_IMM: Opcode = Opcode::Extended(0x4e);
pub const SUBB_C_IMM: Opcode = Opcode::Extended(0x4f);
pub const SUBB_D_IMM: Opcode = Opcode::Extended(0x50);

pub const AND_A_A: Opcode = Opcode::Extended(0x51);
pub const AND_A_B: Opcode = Opcode::Extended(0x52);
pub const AND_A_C: Opcode = Opcode::Extended(0x53);
pub const AND_A_D: Opcode = Opcode::Extended(0x54);

pub const AND_B_A: Opcode = Opcode::Extended(0x55);
pub const AND_B_B: Opcode = Opcode::Extended(0x56);
pub const AND_B_C: Opcode = Opcode::Extended(0x57);
pub const AND_B_D: Opcode = Opcode::Extended(0x58);

pub const AND_C_A: Opcode = Opcode::Extended(0x59);
pub const AND_C_B: Opcode = Opcode::Extended(0x5a);
pub const AND_C_C: Opcode = Opcode::Extended(0x5b);
pub const AND_C_D: Opcode = Opcode::Extended(0x5c);

pub const AND_D_A: Opcode = Opcode::Extended(0x5d);
pub const AND_D_B: Opcode = Opcode::Extended(0x5e);
pub const AND_D_C: Opcode = Opcode::Extended(0x5f);
pub const AND_D_D: Opcode = Opcode::Extended(0x60);

pub const AND_A_IMM: Opcode = Opcode::Extended(0x61);
pub const AND_B_IMM: Opcode = Opcode::Extended(0x62);
pub const AND_C_IMM: Opcode = Opcode::Extended(0x63);
pub const AND_D_IMM: Opcode = Opcode::Extended(0x64);

pub const OR_A_A: Opcode = Opcode::Extended(0x65);
pub const OR_A_B: Opcode = Opcode::Extended(0x66);
pub const OR_A_C: Opcode = Opcode::Extended(0x67);
pub const OR_A_D: Opcode = Opcode::Extended(0x68);

pub const OR_B_A: Opcode = Opcode::Extended(0x69);
pub const OR_B_B: Opcode = Opcode::Extended(0x6a);
pub const OR_B_C: Opcode = Opcode::Extended(0x6b);
pub const OR_B_D: Opcode = Opcode::Extended(0x6c);

pub const OR_C_A: Opcode = Opcode::Extended(0x6d);
pub const OR_C_B: Opcode = Opcode::Extended(0x6e);
pub const OR_C_C: Opcode = Opcode::Extended(0x6f);
pub const OR_C_D: Opcode = Opcode::Extended(0x70);

pub const OR_D_A: Opcode = Opcode::Extended(0x71);
pub const OR_D_B: Opcode = Opcode::Extended(0x72);
pub const OR_D_C: Opcode = Opcode::Extended(0x73);
pub const OR_D_D: Opcode = Opcode::Extended(0x74);

pub const OR_A_IMM: Opcode = Opcode::Extended(0x75);
pub const OR_B_IMM: Opcode = Opcode::Extended(0x76);
pub const OR_C_IMM: Opcode = Opcode::Extended(0x77);
pub const OR_D_IMM: Opcode = Opcode::Extended(0x78);

pub const XOR_A_A: Opcode = Opcode::Extended(0x79);
pub const XOR_A_B: Opcode = Opcode::Extended(0x7a);
pub const XOR_A_C: Opcode = Opcode::Extended(0x7b);
pub const XOR_A_D: Opcode = Opcode::Extended(0x7c);

pub const XOR_B_A: Opcode = Opcode::Extended(0x7d);
pub const XOR_B_B: Opcode = Opcode::Extended(0x7e);
pub const XOR_B_C: Opcode = Opcode::Extended(0x7f);
pub const XOR_B_D: Opcode = Opcode::Extended(0x80);

pub const XOR_C_A: Opcode = Opcode::Extended(0x81);
pub const XOR_C_B: Opcode = Opcode::Extended(0x82);
pub const XOR_C_C: Opcode = Opcode::Extended(0x83);
pub const XOR_C_D: Opcode = Opcode::Extended(0x84);

pub const XOR_D_A: Opcode = Opcode::Extended(0x85);
pub const XOR_D_B: Opcode = Opcode::Extended(0x86);
pub const XOR_D_C: Opcode = Opcode::Extended(0x87);
pub const XOR_D_D: Opcode = Opcode::Extended(0x88);

pub const XOR_A_IMM: Opcode = Opcode::Extended(0x89);
pub const XOR_B_IMM: Opcode = Opcode::Extended(0x8a);
pub const XOR_C_IMM: Opcode = Opcode::Extended(0x8b);
pub const XOR_D_IMM: Opcode = Opcode::Extended(0x8c);

pub const SHL_A: Opcode = Opcode::Extended(0x8d);
pub const SHL_B: Opcode = Opcode::Extended(0x8e);
pub const SHL_C: Opcode = Opcode::Extended(0x8f);
pub const SHL_D: Opcode = Opcode::Extended(0x90);

pub const SHR_A: Opcode = Opcode::Extended(0x91);
pub const SHR_B: Opcode = Opcode::Extended(0x92);
pub const SHR_C: Opcode = Opcode::Extended(0x93);
pub const SHR_D: Opcode = Opcode::Extended(0x94);

pub const ASR_A: Opcode = Opcode::Extended(0x95);
pub const ASR_B: Opcode = Opcode::Extended(0x96);
pub const ASR_C: Opcode = Opcode::Extended(0x97);
pub const ASR_D: Opcode = Opcode::Extended(0x98);

pub const NOT_A: Opcode = Opcode::Extended(0x99);
pub const NOT_B: Opcode = Opcode::Extended(0x9a);
pub const NOT_C: Opcode = Opcode::Extended(0x9b);
pub const NOT_D: Opcode = Opcode::Extended(0x9c);

pub const NEG_A: Opcode = Opcode::Extended(0x9d);
pub const NEG_B: Opcode = Opcode::Extended(0x9e);
pub const NEG_C: Opcode = Opcode::Extended(0x9f);
pub const NEG_D: Opcode = Opcode::Extended(0xa0);

pub const INC_A: Opcode = Opcode::Extended(0xa1);
pub const INC_B: Opcode = Opcode::Extended(0xa2);
pub const INC_C: Opcode = Opcode::Extended(0xa3);
pub const INC_D: Opcode = Opcode::Extended(0xa4);

pub const DEC_A: Opcode = Opcode::Extended(0xa5);
pub const DEC_B: Opcode = Opcode::Extended(0xa6);
pub const DEC_C: Opcode = Opcode::Extended(0xa7);
pub const DEC_D: Opcode = Opcode::Extended(0xa8);

pub const CMP_A_A: Opcode = Opcode::Extended(0xa9);
pub const CMP_A_B: Opcode = Opcode::Extended(0xaa);
pub const CMP_A_C: Opcode = Opcode::Extended(0xab);
pub const CMP_A_D: Opcode = Opcode::Extended(0xac);

pub const CMP_B_A: Opcode = Opcode::Extended(0xad);
pub const CMP_B_B: Opcode = Opcode::Extended(0xae);
pub const CMP_B_C: Opcode = Opcode::Extended(0xaf);
pub const CMP_B_D: Opcode = Opcode::Extended(0xb0);

pub const CMP_C_A: Opcode = Opcode::Extended(0xb1);
pub const CMP_C_B: Opcode = Opcode::Extended(0xb2);
pub const CMP_C_C: Opcode = Opcode::Extended(0xb3);
pub const CMP_C_D: Opcode = Opcode::Extended(0xb4);

pub const CMP_D_A: Opcode = Opcode::Extended(0xb5);
pub const CMP_D_B: Opcode = Opcode::Extended(0xb6);
pub const CMP_D_C: Opcode = Opcode::Extended(0xb7);
pub const CMP_D_D: Opcode = Opcode::Extended(0xb8);

pub const CMP_A_IMM: Opcode = Opcode::Extended(0xb9);
pub const CMP_B_IMM: Opcode = Opcode::Extended(0xba);
pub const CMP_C_IMM: Opcode = Opcode::Extended(0xbb);
pub const CMP_D_IMM: Opcode = Opcode::Extended(0xbc);

pub const TEST_A: Opcode = Opcode::Extended(0xbd);
pub const TEST_B: Opcode = Opcode::Extended(0xbe);
pub const TEST_C: Opcode = Opcode::Extended(0xbf);
pub const TEST_D: Opcode = Opcode::Extended(0xc0);

pub const PUSH_A: Opcode = Opcode::Extended(0xc1);
pub const PUSH_B: Opcode = Opcode::Extended(0xc2);
pub const PUSH_C: Opcode = Opcode::Extended(0xc3);
pub const PUSH_D: Opcode = Opcode::Extended(0xc4);

pub const PUSH_X: Opcode = Opcode::Extended(0xc5);
pub const PUSH_Y: Opcode = Opcode::Extended(0xc6);

pub const POP_A: Opcode = Opcode::Extended(0xc7);
pub const POP_B: Opcode = Opcode::Extended(0xc8);
pub const POP_C: Opcode = Opcode::Extended(0xc9);
pub const POP_D: Opcode = Opcode::Extended(0xca);

pub const POP_X: Opcode = Opcode::Extended(0xcb);
pub const POP_Y: Opcode = Opcode::Extended(0xcc);

pub const CALL_PC_REL: Opcode = Opcode::Extended(0xcd);
pub const CALL_ABS: Opcode = Opcode::Extended(0xce);
pub const CALL_X_REL_IMM: Opcode = Opcode::Extended(0xcf);
pub const CALL_Y_REL_IMM: Opcode = Opcode::Extended(0xd0);
pub const RET: Opcode = Opcode::Extended(0xd1);

pub const SWI: Opcode = Opcode::Extended(0xd2);
pub const RETI: Opcode = Opcode::Extended(0xd3);

pub const JMP_PC_REL: Opcode = Opcode::Extended(0xd4);
pub const JMP_ABS: Opcode = Opcode::Extended(0xd5);
pub const JMP_X_REL_IMM: Opcode = Opcode::Extended(0xd6);
pub const JMP_Y_REL_IMM: Opcode = Opcode::Extended(0xd7);

pub const BR_EQ_PC_REL: Opcode = Opcode::Extended(0xd8);
pub const BR_EQ_ABS: Opcode = Opcode::Extended(0xd9);
pub const BR_EQ_X_REL_IMM: Opcode = Opcode::Extended(0xda);
pub const BR_EQ_Y_REL_IMM: Opcode = Opcode::Extended(0xdb);

pub const BR_NE_PC_REL: Opcode = Opcode::Extended(0xdc);
pub const BR_NE_ABS: Opcode = Opcode::Extended(0xdd);
pub const BR_NE_X_REL_IMM: Opcode = Opcode::Extended(0xde);
pub const BR_NE_Y_REL_IMM: Opcode = Opcode::Extended(0xdf);

pub const BR_LT_PC_REL: Opcode = Opcode::Extended(0xe0);
pub const BR_LT_ABS: Opcode = Opcode::Extended(0xe1);
pub const BR_LT_X_REL_IMM: Opcode = Opcode::Extended(0xe2);
pub const BR_LT_Y_REL_IMM: Opcode = Opcode::Extended(0xe3);

pub const BR_GT_PC_REL: Opcode = Opcode::Extended(0xe4);
pub const BR_GT_ABS: Opcode = Opcode::Extended(0xe5);
pub const BR_GT_X_REL_IMM: Opcode = Opcode::Extended(0xe6);
pub const BR_GT_Y_REL_IMM: Opcode = Opcode::Extended(0xe7);

pub const BR_LE_PC_REL: Opcode = Opcode::Extended(0xe8);
pub const BR_LE_ABS: Opcode = Opcode::Extended(0xe9);
pub const BR_LE_X_REL_IMM: Opcode = Opcode::Extended(0xea);
pub const BR_LE_Y_REL_IMM: Opcode = Opcode::Extended(0xeb);

pub const BR_GE_PC_REL: Opcode = Opcode::Extended(0xec);
pub const BR_GE_ABS: Opcode = Opcode::Extended(0xed);
pub const BR_GE_X_REL_IMM: Opcode = Opcode::Extended(0xee);
pub const BR_GE_Y_REL_IMM: Opcode = Opcode::Extended(0xef);

pub const BR_LTS_PC_REL: Opcode = Opcode::Extended(0xf0);
pub const BR_LTS_ABS: Opcode = Opcode::Extended(0xf1);
pub const BR_LTS_X_REL_IMM: Opcode = Opcode::Extended(0xf2);
pub const BR_LTS_Y_REL_IMM: Opcode = Opcode::Extended(0xf3);

pub const BR_GTS_PC_REL: Opcode = Opcode::Extended(0xf4);
pub const BR_GTS_ABS: Opcode = Opcode::Extended(0xf5);
pub const BR_GTS_X_REL_IMM: Opcode = Opcode::Extended(0xf6);
pub const BR_GTS_Y_REL_IMM: Opcode = Opcode::Extended(0xf7);

pub const BR_LES_PC_REL: Opcode = Opcode::Extended(0xf8);
pub const BR_LES_ABS: Opcode = Opcode::Extended(0xf9);
pub const BR_LES_X_REL_IMM: Opcode = Opcode::Extended(0xfa);
pub const BR_LES_Y_REL_IMM: Opcode = Opcode::Extended(0xfb);

pub const BR_GES_PC_REL: Opcode = Opcode::Extended(0xfc);
pub const BR_GES_ABS: Opcode = Opcode::Extended(0xfd);
pub const BR_GES_X_REL_IMM: Opcode = Opcode::Extended(0xfe);
pub const BR_GES_Y_REL_IMM: Opcode = Opcode::Extended(0xff);
