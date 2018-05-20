use self::Instruction::*;
use registers::{Register, Register::*};

pub enum Instruction {
    Add(Register),
    Inc(Register),
    Sub(Register),
    AddHL,
    SubHL,
    AddImmediate,
    SubImmediate,
    Nop,
    Unknown,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x00 => Nop,
            0x04 => Inc(B),
            0x0C => Inc(C),
            0x14 => Inc(D),
            0x1C => Inc(E),
            0x24 => Inc(H),
            0x2C => Inc(L),
            0x3C => Inc(A),
            0x80 => Add(B),
            0x81 => Add(C),
            0x82 => Add(D),
            0x83 => Add(E),
            0x84 => Add(H),
            0x85 => Add(L),
            0x86 => AddHL,
            0x87 => Add(A),
            0x90 => Sub(B),
            0x91 => Sub(C),
            0x92 => Sub(D),
            0x93 => Sub(E),
            0x94 => Sub(H),
            0x95 => Sub(L),
            0x96 => SubHL,
            0x97 => Sub(A),
            0xC6 => AddImmediate,
            0xD6 => SubImmediate,
            _ => Unknown,
        }
    }
}
