use crate::cpu::isa::{Cpu, XlenU};
use crate::cpu::CPUError;

// use std::mem::ManuallyDrop;
// use crate::cpu::CPUError;
// use crate::cpu::isa::{RV32E, RV32I, RV64I};
//
// trait IsaExt {
//     const ISA_ID: &'static str;
//
//     fn execute_rv32i(&mut self, cpu: RV32I, instruction: u32) -> Result<(), CPUError<u32>>;
//
//     fn execute_rv32e(&mut self, cpu: RV32E, instruction: u32) -> Result<(), CPUError<u32>>;
//
//     fn execute_rv64i(&mut self, cpu: RV64I, instruction: u32) -> Result<(), CPUError<u64>>;
// }
//
// struct F {
//     registers: [f32; 32],
// }
//
// impl IsaExt for F {
//     const ISA_ID: &'static str = "F";
//
//     fn execute_rv32i(&mut self, cpu: RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv32e(&mut self, cpu: RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv64i(&mut self, cpu: RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
//         todo!()
//     }
// }
//
// struct D {
//     registers: [f64; 32],
// }
//
// impl IsaExt for D {
//     const ISA_ID: &'static str = "FD";
//
//     fn execute_rv32i(&mut self, cpu: RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv32e(&mut self, cpu: RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv64i(&mut self, cpu: RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
//         todo!()
//     }
// }
//
// struct Q {
//     registers: [F128; 32],
// }
//
// impl IsaExt for Q {
//     const ISA_ID: &'static str = "FDQ";
//
//     fn execute_rv32i(&mut self, cpu: RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv32e(&mut self, cpu: RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
//         todo!()
//     }
//
//     fn execute_rv64i(&mut self, cpu: RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
//         todo!()
//     }
// }
//
// struct F128(u128);
//
// enum FloatExt {
//     None,
//     F(F),
//     D(D),
//     Q(Q),
// }
//
// fn test() {
//
// }
pub mod float;

trait IsaExt<XLEN: XlenU, const REG_COUNT: usize, CPU: Cpu<XLEN, REG_COUNT>> {
    const ISA_ID: &'static str;

    fn execute(&mut self, cpu: &mut CPU, instruction: u32) -> Result<(), CPUError<XLEN>>;
}
