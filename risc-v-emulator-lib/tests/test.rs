use risc_v_emulator_lib::cpu::isa::{Cpu, RV32E};
use risc_v_emulator_lib::cpu::isa::{RV32I, RV64I};

#[test]
fn test_rv32i() {
    let cpu_rv32i = RV32I::with_code(&[], None);
    assert_eq!("RV32I", cpu_rv32i.isa_id());
}

#[test]
fn test_rv32e() {
    let cpu_rv32e = RV32E::with_code(&[], None);
    assert_eq!("RV32E", cpu_rv32e.isa_id());
}

#[test]
fn test_rv64i() {
    let cpu_rv64i = RV64I::with_code(&[], None);
    assert_eq!("RV64I", cpu_rv64i.isa_id());
}
