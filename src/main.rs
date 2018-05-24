mod cpu;
mod disassembler;
mod flags;
mod instructions;
mod registers;

use cpu::Cpu;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("../gameboy/blargg-gb-tests/cpu_instrs/cpu_instrs.gb");
    let mut file = File::open(path)
        .expect("Unable to open file.");

    let mut memory = Vec::new();
    file.read_to_end(&mut memory)
        .expect("Unable to read file.");

    let mut cpu = Cpu::new();
    cpu.load_memory(memory);
    cpu.run();
}

// http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
// https://stackoverflow.com/a/22034331/1133560
