use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug)]
pub enum Register { A, B, C, D, E, H, L }

#[derive(Clone, Copy, Debug)]
pub enum RegisterPair { BC, DE, HL }

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn pair(&self, register_pair: RegisterPair) -> u16 {
        use self::RegisterPair::*;

        match register_pair {
            BC => ((self.b as u16) << 8) + (self.c as u16),
            DE => ((self.d as u16) << 8) + (self.e as u16),
            HL => ((self.h as u16) << 8) + (self.l as u16),
        }
    }
}

impl Index<Register> for Registers {
    type Output = u8;

    fn index(&self, register: Register) -> &u8 {
        use self::Register::*;

        match register {
            A => &self.a,
            B => &self.b,
            C => &self.c,
            D => &self.d,
            E => &self.e,
            H => &self.h,
            L => &self.l,
        }
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut u8 {
        use self::Register::*;

        match register {
            A => &mut self.a,
            B => &mut self.b,
            C => &mut self.c,
            D => &mut self.d,
            E => &mut self.e,
            H => &mut self.h,
            L => &mut self.l,
        }
    }
}
