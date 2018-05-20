use disassembler::log_instruction;
use flags::Flags;
use instructions::Instruction;
use registers::*;

pub struct Cpu {
    flags: Flags,
    program_counter: u16,
    registers: Registers,
    rom: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            flags: Flags::new(),
            program_counter: 0,
            registers: Registers::new(),
            rom: Vec::<u8>::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }

    pub fn run(&mut self) {
        while (self.program_counter as usize) < self.rom.len() {
            let byte = self.get_next_byte();
            let instruction = Instruction::from_byte(byte);

            log_instruction(&instruction, &self.program_counter, &self.rom);
            self.process_instruction(instruction);

            if self.program_counter == 0x00 { break }
        }

        println!("{:?}", self.registers);
        println!("{:?}", self.flags);
    }

    fn byte_for_register_pair(&self, register_pair: &RegisterPair) -> u8 {
        let address = self.registers.pair(register_pair);
        self.rom[address as usize]
    }

    fn get_next_byte(&mut self) -> u8 {
        let byte = self.rom[self.program_counter as usize];
        self.program_counter = self.program_counter.wrapping_add(1);
        byte
    }

    // -------------------------------------------------------------------------

    fn process_instruction(&mut self, instruction: Instruction) {
        use instructions::Instruction::*;
        use registers::RegisterPair::*;

        let lhs = self.registers.a;

        match instruction {
            Adc(ref register) => {
                let rhs = self.registers[register];
                self.registers.a = self.add(lhs, rhs, true);
            },

            Add(ref register) => {
                let rhs = self.registers[register];
                self.registers.a = self.add(lhs, rhs, false);
            },

            Inc(ref register) => {
                let lhs = self.registers[register];
                self.registers[register] = self.add_core(lhs, 1, false) as u8;
            },

            Sub(ref register) => {
                let rhs = self.registers[register];
                self.registers.a = self.sub(lhs, rhs);
            },

            AdcHL => {
                let rhs = self.byte_for_register_pair(&HL);
                self.registers.a = self.add(lhs, rhs, true);
            },

            AddHL => {
                let rhs = self.byte_for_register_pair(&HL);
                self.registers.a = self.add(lhs, rhs, false);
            },

            SubHL => {
                let rhs = self.byte_for_register_pair(&HL);
                self.registers.a = self.sub(lhs, rhs);
            },

            AdcImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, true);
            },

            AddImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.add(lhs, rhs, false);
            },

            SubImmediate => {
                let rhs = self.get_next_byte();
                self.registers.a = self.sub(lhs, rhs);
            },

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

    fn sub(&mut self, lhs: u8, rhs: u8) -> u8 {
        let result = lhs.wrapping_sub(rhs);
        self.flags.cy = rhs > lhs;
        self.flags.h = ((lhs & 0xF).wrapping_sub(rhs & 0xF)) & 0x10 == 0x10;
        self.flags.n = true;
        self.set_flag_z(result);
        result
    }

    // -------------------------------------------------------------------------

    fn set_flag_z(&mut self, result: u8) {
        self.flags.z = result == 0;
    }
}
