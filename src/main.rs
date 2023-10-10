use crate::bus::Bus;
use crate::cpu::CpuRV64I;
use crate::dram::Dram;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;
mod dram;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: risc-v-emulator <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = CpuRV64I::new(Bus::new(Dram::new(code)));

    // start execution
    loop {
        if let Err(e) = cpu.cycle() {
            eprintln!("Error: {e} Dumping registers: {:?}", cpu.dump_registers());
            break;
        }
    }

    Ok(())
}
