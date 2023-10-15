use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use risc_v_emulator_lib::bus::Bus;
use risc_v_emulator_lib::cpu::{CpuRV32I};
use risc_v_emulator_lib::dram::Dram;

fn cycle_loop(c: &mut Criterion) {
    const CODE: &[u8] = include_bytes!("loop.bin");

    let mut cpu = CpuRV32I::new(Bus::new(Dram::with_code(criterion::black_box(CODE))));

    c.bench_function("infinite loop cycle time", |b| b.iter(|| criterion::black_box(cpu.cycle())));
}

fn cycle_fib(c: &mut Criterion) {
    const CODE: &[u8] = include_bytes!("../../../binaries/programs/fib.bin");

    let mut cpu = CpuRV32I::new(Bus::new(Dram::with_code(criterion::black_box(CODE))));
    c.bench_function("fibonacci calculation", |b| {
        b.iter(|| {
            loop {
                if let Err(e) = criterion::black_box(cpu.cycle()) {
                    criterion::black_box(e);
                    break;
                }
            }
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(20));
    targets = cycle_loop, cycle_fib
);

criterion_main!(benches);