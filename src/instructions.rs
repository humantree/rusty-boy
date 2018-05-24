use self::Instruction::*;
use registers::*;

#[derive(Clone, Copy)]
pub enum Instruction {
    Adc(Register),
    Add(Register),
    Inc(Register),
    Sbc(Register),
    Sub(Register),
    Ld(RegisterPair),
    AdcHL,
    AddHL,
    LdFromInternalRAM,
    LdToInternalRAM,
    SbcHL,
    SubHL,
    LdFromHL(Register),
    LdToHL(Register),
    AdcImmediate,
    AddImmediate,
    LdFromInternalRAMImmediate,
    LdHLImmediate,
    LdToInternalRAMImmediate,
    SbcImmediate,
    SubImmediate,
    LdImmediate(Register),
    LdFromRAMImmediate16,
    LdToRAMImmediate16,
    Nop,
    Unknown,
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        use registers::{Register::*, RegisterPair::*};

        match byte {
            0x00 => Nop,
            0x04 => Inc(B),
            0x06 => LdImmediate(B),
            0x0A => Ld(BC),
            0x0C => Inc(C),
            0x0E => LdImmediate(C),
            0x14 => Inc(D),
            0x16 => LdImmediate(D),
            0x1A => Ld(DE),
            0x1C => Inc(E),
            0x1E => LdImmediate(E),
            0x24 => Inc(H),
            0x26 => LdImmediate(H),
            0x2C => Inc(L),
            0x2E => LdImmediate(L),
            0x36 => LdHLImmediate,
            0x3C => Inc(A),
            0x3E => LdImmediate(A),
            0x46 => LdFromHL(B),
            0x4E => LdFromHL(C),
            0x56 => LdFromHL(D),
            0x5E => LdFromHL(E),
            0x66 => LdFromHL(H),
            0x6E => LdFromHL(L),
            0x70 => LdToHL(B),
            0x71 => LdToHL(C),
            0x72 => LdToHL(D),
            0x73 => LdToHL(E),
            0x74 => LdToHL(H),
            0x75 => LdToHL(L),
            0x77 => LdToHL(A),
            0x7E => LdFromHL(A),
            0x80 => Add(B),
            0x81 => Add(C),
            0x82 => Add(D),
            0x83 => Add(E),
            0x84 => Add(H),
            0x85 => Add(L),
            0x86 => AddHL,
            0x87 => Add(A),
            0x88 => Adc(B),
            0x89 => Adc(C),
            0x8A => Adc(D),
            0x8B => Adc(E),
            0x8C => Adc(H),
            0x8D => Adc(L),
            0x8E => AdcHL,
            0x8F => Adc(A),
            0x90 => Sub(B),
            0x91 => Sub(C),
            0x92 => Sub(D),
            0x93 => Sub(E),
            0x94 => Sub(H),
            0x95 => Sub(L),
            0x96 => SubHL,
            0x97 => Sub(A),
            0x98 => Sbc(B),
            0x99 => Sbc(C),
            0x9A => Sbc(D),
            0x9B => Sbc(E),
            0x9C => Sbc(H),
            0x9D => Sbc(L),
            0x9E => SbcHL,
            0x9F => Sbc(A),
            0xC6 => AddImmediate,
            0xCE => AdcImmediate,
            0xD6 => SubImmediate,
            0xDE => SbcImmediate,
            0xE0 => LdToInternalRAMImmediate,
            0xE2 => LdToInternalRAM,
            0xEA => LdToRAMImmediate16,
            0xF0 => LdFromInternalRAMImmediate,
            0xF2 => LdFromInternalRAM,
            0xFA => LdFromRAMImmediate16,
            _ => Unknown,
        }
    }
}
