use crate::cpu::Cpu64I;
use std::env;
use std::error::Error;
use std::fs::File;
use std::hint::unreachable_unchecked;
use std::io::Read;

mod cpu;
mod ram;

fn main() -> Result<(), Box<dyn Error>> {
    const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

    let mut cpu = Cpu64I::new(MEMORY_SIZE);

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: risc-v-emulator <filename>");
    }
    let mut file = File::open(&args[1])?;
    file.read_to_end(&mut cpu.code)?;

    // start execution

    while cpu.pc < cpu.code.len() as u64 {
        cpu.cycle();
    }

    cpu.dump_registers();

    Ok(())
}
