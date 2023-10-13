use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use risc_v_emulator_lib::bus::Bus;
use risc_v_emulator_lib::cpu::CpuRV64I;
use risc_v_emulator_lib::dram::Dram;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2, "Usage: risc-v-emulator <filename>");

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = CpuRV64I::new(Bus::new(Dram::with_code(&code)));

    // start execution
    loop {
        if let Err(e) = cpu.cycle() {
            eprintln!("Error: {e} Dumping registers: {:?}", cpu.dump_registers());
            break;
        }
    }

    Ok(())
}
