use disassembler::log_instruction;
use flags::Flags;
use instructions::Instruction;
use registers::{*, RegisterPair::*};

pub struct Cpu {
    flags: Flags,
    memory: Vec<u8>,
    program_counter: u16,
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            flags: Flags::new(),
            memory: Vec::<u8>::new(),
            program_counter: 0,
            registers: Registers::new(),
        }
    }

    pub fn load_memory(&mut self, memory: Vec<u8>) {
        self.memory = memory;
    }

    pub fn run(&mut self) {
        while (self.program_counter as usize) < self.memory.len() {
            let byte = self.get_next_byte();
            let instruction = Instruction::from_byte(byte);

            log_instruction(instruction, self.program_counter, &self.memory);
            self.process_instruction(instruction);

            if self.program_counter == 0x00 { break }
        }

        println!("{:?}", self.registers);
        println!("{:?}", self.flags);
    }

    fn byte_for_register_pair(&self, register_pair: RegisterPair) -> u8 {
        let address = self.registers.pair(register_pair);
        self.memory[address as usize]
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.memory[self.program_counter as usize];
        self.program_counter = self.program_counter.wrapping_add(1);
        byte
    }

    fn set_byte_at_hl(&mut self, byte: u8) {
        let address = self.registers.pair(HL);
        self.memory[address as usize] = byte;
    }

    // -------------------------------------------------------------------------

    fn process_instruction(&mut self, instruction: Instruction) {
        use instructions::Instruction::*;

        let lhs = self.registers.a;

        match instruction {
            Adc(register) => {
                let rhs = self.registers[register];
                self.registers.a = self.add(lhs, rhs, true);
            },

            Add(register) => {
                let rhs = self.registers[register];
                self.registers.a = self.add(lhs, rhs, false);
            },

            Inc(register) => {
                let lhs = self.registers[register];
                self.registers[register] = self.add_core(lhs, 1, false) as u8;
            },

            Sbc(register) => {
                let rhs = self.registers[register];
                self.registers.a = self.sub(lhs, rhs, true);
            },

            Sub(register) => {
                let rhs = self.registers[register];
                self.registers.a = self.sub(lhs, rhs, false);
            },

            Ld(register_pair) => {
                self.registers.a = self.byte_for_register_pair(register_pair);
            },

            AdcHL => {
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.add(lhs, rhs, true);
            },

            AddHL => {
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.add(lhs, rhs, false);
            },

            SbcHL => {
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.sub(lhs, rhs, true);
            },

            SubHL => {
                let rhs = self.byte_for_register_pair(HL);
                self.registers.a = self.sub(lhs, rhs, false);
            },

            LdFromHL(register) => {
                self.registers[register] = self.byte_for_register_pair(HL);
            },

            LdToHL(register) => {
                let byte = self.registers[register];
                self.set_byte_at_hl(byte);
            }

            AdcImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, true);
            },

            AddImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, false);
            },

            LdHLImmediate => {
                let byte = self.get_next_byte();
                self.set_byte_at_hl(byte);
            }

            SbcImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.sub(lhs, rhs, true);
            },

            SubImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.sub(lhs, rhs, false);
            },

            LdImmediate(register) => {
                self.registers[register] = self.get_next_byte();
            }

            Nop => (),
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
        self.flags.cy = rhs as i8 > (lhs as i8) - (cy as i8);
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
