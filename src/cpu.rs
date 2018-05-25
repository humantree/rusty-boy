use disassembler::log_instruction;
use flags::Flags;
use instructions::Instruction;
use registers::{*, RegisterPair::*};

pub struct Cpu {
    flags: Flags,
    memory: Vec<u8>,
    program_counter: u16,
    registers: Registers,
    stack_pointer: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            flags: Flags::new(),
            memory: Vec::<u8>::new(),
            program_counter: 0,
            registers: Registers::new(),
            stack_pointer: 0,
        }
    }

    pub fn load_memory(&mut self, memory: Vec<u8>) {
        self.memory = memory;
    }

    pub fn run(&mut self) {
        while (self.program_counter as usize) < self.memory.len() {
            let starting_program_counter = self.program_counter;

            let byte = self.get_next_byte();
            let instruction = Instruction::from_byte(byte);

            log_instruction(instruction, &self.memory, self.program_counter);
            self.process_instruction(instruction);

            if self.program_counter < starting_program_counter { break }
        }

        println!("{:?}", self.registers);
        println!("{:?}", self.flags);
    }

    fn byte_for_register_pair(&self, rp: RegisterPair) -> u8 {
        let address = self.registers.pair(rp);
        self.memory[address as usize]
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.memory[self.program_counter as usize];
        self.program_counter = self.program_counter.wrapping_add(1);
        byte
    }

    fn get_next_two_bytes(&mut self) -> u16 {
        let first_byte = self.get_next_byte();
        let second_byte = self.get_next_byte();
        ((second_byte as u16) << 8) + first_byte as u16
    }

    // -------------------------------------------------------------------------

    fn process_instruction(&mut self, instruction: Instruction) {
        use instructions::Instruction::*;

        match instruction {
            ADC_A_d8 => {
                let lhs = self.registers.a;
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, true);
            },

            ADC_A_HL => {
                let lhs = self.registers.a;
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.add(lhs, rhs, true);
            },

            ADC_A_r(r) => {
                let lhs = self.registers.a;
                let rhs = self.registers[r];
                self.registers.a = self.add(lhs, rhs, true);
            },

            ADD_A_d8 => {
                let lhs = self.registers.a;
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, false);
            },

            ADD_A_HL => {
                let lhs = self.registers.a;
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.add(lhs, rhs, false);
            },

            ADD_A_r(r) => {
                let lhs = self.registers.a;
                let rhs = self.registers[r];
                self.registers.a = self.add(lhs, rhs, false);
            },

            INC_r(r) => {
                let lhs = self.registers[r];
                self.registers[r] = self.add_core(lhs, 1, false) as u8;
            },

            LD_A_a8 => {
                let address = internal_ram_address(self.get_next_byte());
                self.registers.a = self.memory[address as usize];
            },

            LD_A_a16 => {
                let address = self.get_next_two_bytes();
                self.registers.a = self.memory[address as usize];
            },

            LD_A_C => {
                let address = internal_ram_address(self.registers.c);
                self.registers.a = self.memory[address as usize];
            },

            LD_A_HLD => {
                let address = self.registers.pair(HL);
                self.registers.a = self.memory[address as usize];
                self.registers.set_pair(HL, address.wrapping_sub(1));
            },

            LD_A_HLI => {
                let address = self.registers.pair(HL);
                self.registers.a = self.memory[address as usize];
                self.registers.set_pair(HL, address.wrapping_add(1));
            },

            LD_A_rp(rp) => {
                self.registers.a = self.byte_for_register_pair(rp);
            },

            LD_a8_A => {
                let address = internal_ram_address(self.get_next_byte());
                self.memory[address as usize] = self.registers.a;
            },

            LD_a16_A => {
                let address = self.get_next_two_bytes();
                self.memory[address as usize] = self.registers.a;
            },

            LD_C_A => {
                let address = internal_ram_address(self.registers.c);
                self.memory[address as usize] = self.registers.a;
            },

            LD_HL_d8 => {
                let address = self.registers.pair(HL);
                self.memory[address as usize] = self.get_next_byte();
            },

            LD_HL_r(r) => {
                let address = self.registers.pair(HL);
                self.memory[address as usize] = self.registers[r];
            },

            LD_HLD_A => {
                let address = self.registers.pair(HL);
                self.memory[address as usize] = self.registers.a;
                self.registers.set_pair(HL, address.wrapping_sub(1));
            },

            LD_HLI_A => {
                let address = self.registers.pair(HL);
                self.memory[address as usize] = self.registers.a;
                self.registers.set_pair(HL, address.wrapping_add(1));
            },

            LD_r_d8(r) => {
                self.registers[r] = self.get_next_byte();
            },


            LD_r_HL(r) => {
                self.registers[r] = self.byte_for_register_pair(HL);
            },

            LD_rp_A(rp) => {
                let address = self.registers.pair(rp);
                self.memory[address as usize] = self.registers.a;
            },

            LD_rp_d16(rp) => {
                let rhs = self.get_next_two_bytes();
                self.registers.set_pair(rp, rhs);
            },

            LD_SP_d16 => {
                self.stack_pointer = self.get_next_two_bytes();
            },

            NOP => (),

            SBC_A_d8 => {
                let lhs = self.registers.a;
                let rhs = self.get_next_byte();
                self.registers.a = self.sub(lhs, rhs, true);
            },

            SBC_A_HL => {
                let lhs = self.registers.a;
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.sub(lhs, rhs, true);
            },

            SBC_A_r(r) => {
                let lhs = self.registers.a;
                let rhs = self.registers[r];
                self.registers.a = self.sub(lhs, rhs, true);
            },

            SUB_d8 => {
                let lhs = self.registers.a;
                let rhs = self.get_next_byte();
                self.registers.a = self.sub(lhs, rhs, false);
            },

            SUB_HL => {
                let lhs = self.registers.a;
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.sub(lhs, rhs, false);
            },

            SUB_r(r) => {
                let lhs = self.registers.a;
                let rhs = self.registers[r];
                self.registers.a = self.sub(lhs, rhs, false);
            },

            Unknown => (),
        }
    }

    // -------------------------------------------------------------------------

    fn add(&mut self, lhs: u8, rhs: u8, carry: bool) -> u8 {
        let result = self.add_core(lhs, rhs, carry);
        self.flags.cy = result > 0xFF;
        result as u8
    }

    fn add_core(&mut self, lhs: u8, rhs: u8, carry: bool) -> u16 {
        let cy: u8 = if carry && self.flags.cy { 1 } else { 0 };
        let result = (lhs as u16) + (rhs as u16) + (cy as u16);
        self.flags.h = ((lhs & 0xF) + (rhs & 0xF) + cy) & 0x10 == 0x10;
        self.flags.n = false;
        self.set_flag_z(result as u8);
        result
    }

    fn sub(&mut self, lhs: u8, rhs: u8, carry: bool) -> u8 {
        let cy: u8 = if carry && self.flags.cy { 1 } else { 0 };
        let result = lhs.wrapping_sub(rhs).wrapping_sub(cy);
        self.flags.cy = rhs as i16 > (lhs as i16) - (cy as i16);
        self.flags.h = ((lhs & 0xF)
            .wrapping_sub(rhs & 0xF)
            .wrapping_sub(cy))
            & 0x10 == 0x10;
        self.flags.n = true;
        self.set_flag_z(result);
        result
    }

    // -------------------------------------------------------------------------

    fn set_flag_z(&mut self, result: u8) {
        self.flags.z = result == 0;
    }
}

fn internal_ram_address(offset: u8) -> u16 {
    0xFF00 + offset as u16
}
