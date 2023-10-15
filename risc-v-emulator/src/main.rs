use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::{env, fs};

use risc_v_emulator_lib::bus::Bus;
use risc_v_emulator_lib::cpu::CpuRV32I;
use risc_v_emulator_lib::dram::Dram;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2, "Usage: risc-v-emulator <filename>");

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = CpuRV32I::new(Bus::new(Dram::with_code(&code)));

    let mut cycles = 0;
    let t_start = Instant::now();

    // start execution
    loop {
        cycles += 1;
        if let Err(e) = cpu.cycle() {
            eprintln!("Error: {e} Dumping registers:\n{:?}", cpu.dump_registers());
            break;
        }
    }

    let elapsed = t_start.elapsed().as_nanos();
    let ns_per_cycle = elapsed / cycles;
    let freq = 1. / ns_per_cycle as f32;

    println!(
        "Cycles: {cycles} | Elapsed: {} | t/cycle: {} | {} GHz",
        human_time(elapsed),
        human_time(ns_per_cycle),
        freq
    );
    println!("Writing memory dump...");

    fs::write("mem.dump", cpu.dump_memory()).expect("Could not write memory dump!");

    Ok(())
}

fn human_time(mut d: u128) -> String {
    for unit in ["ns", "µs", "ms", "s", "m"] {
        if d < 1000 {
            return format!("{d} {unit}");
        }

        d /= 1000;
    }

    format!("{d} h")
}
