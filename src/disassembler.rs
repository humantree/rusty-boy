use instructions::{Instruction, Instruction::*};
use std::fmt::{Display, Formatter, Result};

pub fn log_instruction(instruction: Instruction,
                       memory: &Vec<u8>,
                       program_counter: u16) {

    if let NOP     = instruction { return }
    if let Unknown = instruction { return }

    let address = format!("{:#06X}", program_counter.wrapping_sub(1));
    print!("{}\t{}", address, instruction);

    if instruction.is_a16_instruction() {
        let a16 = get_next_two_bytes(memory, program_counter);
        print!(" \t({:#06X})", a16);
    }

    if instruction.is_d8_instruction() {
        let d8 = memory[program_counter as usize];
        print!(" \t{:#04X}", d8);
    }

    if instruction.is_d16_instruction() {
        let d16 = get_next_two_bytes(memory, program_counter);
        print!(" \t{:#06X}", d16);
    }

    println!();
}

fn get_next_two_bytes(memory: &Vec<u8>, program_counter: u16) -> u16 {
    let first_byte = memory[program_counter as usize];
    let second_byte = memory[program_counter.wrapping_add(1) as usize];
    ((second_byte as u16) << 8) + first_byte as u16
}

impl Instruction {
    pub fn is_a16_instruction(self) -> bool {
        match self {
            LD_A_a16 => true,
            LD_a16_A => true,
            _ => false,
        }
    }

    pub fn is_d8_instruction(self) -> bool {
        match self {
            ADC_A_d8   => true,
            ADD_A_d8   => true,
            LD_HL_d8   => true,
            LD_r_d8(_) => true,
            SBC_A_d8   => true,
            SUB_d8     => true,
            _ => false,
        }
    }

    pub fn is_d16_instruction(self) -> bool {
        match self {
            LD_rp_d16(_) => true,
            LD_SP_d16    => true,
            _ => false,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ADC_A_d8      => write!(f, "ADC A,d8"),
            ADC_A_HL      => write!(f, "ADC A,(HL)"),
            ADC_A_r(r)    => write!(f, "ADC A,{:?}",   r),
            ADD_A_d8      => write!(f, "ADD A,d8"),
            ADD_A_HL      => write!(f, "ADD A,(HL)"),
            ADD_A_r(r)    => write!(f, "ADD A,{:?}",   r),
            INC_r(r)      => write!(f, "INC {:?}",     r),
            LD_A_a8       => write!(f, "LD A,(a8)"),
            LD_A_a16      => write!(f, "LD A,(a16)"),
            LD_A_C        => write!(f, "LD A,(C)"),
            LD_A_HLD      => write!(f, "LD A,(HLD)"),
            LD_A_HLI      => write!(f, "LD A,(HLI)"),
            LD_A_rp(rp)   => write!(f, "LD A,({:?})",  rp),
            LD_a8_A       => write!(f, "LD (a8),A"),
            LD_a16_A      => write!(f, "LD (a16),A"),
            LD_C_A        => write!(f, "LD (C),A"),
            LD_HL_d8      => write!(f, "LD (HL),d8"),
            LD_HL_r(r)    => write!(f, "LD (HL),{:?}", r),
            LD_HLD_A      => write!(f, "LD (HLD),A"),
            LD_HLI_A      => write!(f, "LD (HLI),A"),
            LD_r_d8(r)    => write!(f, "LD {:?},d8",   r),
            LD_r_HL(r)    => write!(f, "LD {:?},(HL)", r),
            LD_rp_A(rp)   => write!(f, "LD ({:?}),A",  rp),
            LD_rp_d16(rp) => write!(f, "LD {:?},d16",  rp),
            LD_SP_d16     => write!(f, "LD SP,d16"),
            NOP           => write!(f, "NOP"),
            SBC_A_d8      => write!(f, "SBC A,d8"),
            SBC_A_HL      => write!(f, "SBC A,(HL)"),
            SBC_A_r(r)    => write!(f, "SBC A,{:?}",   r),
            SUB_d8        => write!(f, "SUB A,d8"),
            SUB_HL        => write!(f, "SUB (HL)"),
            SUB_r(r)      => write!(f, "SUB {:?}",     r),
            Unknown       => write!(f, "NOT IMPLEMENTED"),
        }
    }
}
