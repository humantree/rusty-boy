use instructions::{Instruction, Instruction::*};
use std::fmt::{Display, Formatter, Result};

pub fn log_instruction(instruction: Instruction,
                       program_counter: u16,
                       memory: &Vec<u8>) {

    if let Nop     = instruction { return }
    if let Unknown = instruction { return }

    let address = format!("{:#06X}", program_counter - 1);
    print!("{}\t{}", address, instruction);

    if instruction.is_immediate_instruction() {
        let immediate_byte = memory[program_counter as usize];
        print!("\t${}", immediate_byte);
    }

    println!();
}

impl Instruction {
    pub fn is_immediate_instruction(self) -> bool {
        match self {
            AdcImmediate   => true,
            AddImmediate   => true,
            LdHLImmediate  => true,
            LdImmediate(_) => true,
            SbcImmediate   => true,
            SubImmediate   => true,
            _ => false
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Adc(register)         => write!(f, "ADC A,{:?}",   register),
            Add(register)         => write!(f, "ADD A,{:?}",   register),
            Inc(register)         => write!(f, "INC {:?}",     register),
            Sbc(register)         => write!(f, "SBC A,{:?}",   register),
            Sub(register)         => write!(f, "SUB {:?}",     register),
            Ld(register_pair)     => write!(f, "LD A,({:?})",  register_pair),
            AdcHL                 => write!(f, "ADC A,(HL)"),
            AddHL                 => write!(f, "ADD A,(HL)"),
            LdFromInternalRAM     => write!(f, "LD A,(C)"),
            LdToInternalRAM       => write!(f, "LD (C),A"),
            SbcHL                 => write!(f, "SBC A,(HL)"),
            SubHL                 => write!(f, "SUB (HL)"),
            LdFromHL(register)    => write!(f, "LD {:?},(HL)", register),
            LdToHL(register)      => write!(f, "LD (HL),{:?}", register),
            AdcImmediate          => write!(f, "ADC A,d8"),
            AddImmediate          => write!(f, "ADD A,d8"),
            LdHLImmediate         => write!(f, "LD (HL),d8"),
            SbcImmediate          => write!(f, "SBC A,d8"),
            SubImmediate          => write!(f, "SUB A,d8"),
            LdImmediate(register) => write!(f, "LD {:?},d8",   register),
            Nop                   => write!(f, "NOP"),
            Unknown               => write!(f, "NOT IMPLEMENTED"),
        }
    }
}
