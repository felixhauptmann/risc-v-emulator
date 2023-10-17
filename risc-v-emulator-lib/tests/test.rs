use risc_v_emulator_lib::cpu::isa::{RV32E, RV32I, RV64I};
use risc_v_emulator_lib::cpu::Cpu;

#[test]
fn test_rv32i() {
    let cpu_rv32i: Cpu<RV32I, 32> = Cpu::with_code(&[]);
    assert_eq!("RV32I", cpu_rv32i.get_isa_id());
}

#[test]
fn test_rv32e() {
    let cpu_rv32e: Cpu<RV32E, 16> = Cpu::with_code(&[]);
    assert_eq!("RV32E", cpu_rv32e.get_isa_id());
}

#[test]
fn test_rv64i() {
    let cpu_rv64i: Cpu<RV64I, 32> = Cpu::with_code(&[]);
    assert_eq!("RV64I", cpu_rv64i.get_isa_id());
}
