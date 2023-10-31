use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::{env, fs};

use risc_v_emulator_lib::cpu::isa::RV32I;
use risc_v_emulator_lib::cpu::isa::{Cpu, XlenU, RV32E, RV64I};
use risc_v_emulator_lib::cpu::CPUError;

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 3, "Usage: risc-v-emulator <isa> <filename>");

    let mut file = File::open(&args[2])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    match args[1].to_uppercase().as_str() {
        "RV32I" => run(RV32I::with_code(&code, None)),
        "RV32E" => run(RV32E::with_code(&code, None)),
        "RV64I" => {
            run(RV64I::with_code(&code, None));
        }
        _ => {}
    }

    Ok(())
}

fn run<XLEN: XlenU, const REG_COUNT: usize, CPU: Cpu<XLEN, REG_COUNT>>(mut cpu: CPU) {
    let mut cycles = 0;
    let t_start = Instant::now();

    // start execution
    loop {
        cycles += 1;

        match cpu.cycle() {
            Err(CPUError::Halt) => {
                println!("{}", CPUError::<XLEN>::Halt);
                break;
            }
            Err(e) => {
                eprintln!("Error: {e} Dumping registers:\n{:?}", cpu.dump_registers());
                break;
            }
            _ => {}
        }
    }

    let elapsed = t_start.elapsed().as_nanos();
    let ns_per_cycle = elapsed / cycles;
    let freq = 1. / ns_per_cycle as f32;

    println!(
        "Cycles: {cycles} | Elapsed: {} | t/cycle: {} ns | {} GHz",
        human_time(elapsed),
        ns_per_cycle,
        freq
    );
    println!("Writing memory dump...");

    fs::write("mem.dump", cpu.dump_memory()).expect("Could not write memory dump!");
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
