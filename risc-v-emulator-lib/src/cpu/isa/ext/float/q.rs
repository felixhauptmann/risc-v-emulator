// use crate::cpu::isa::ext::float::d::DExt;
// use crate::cpu::isa::ext::float::IsaExt;
// use crate::cpu::isa::{RV32E, RV32I, RV64I};
// use crate::cpu::CPUError;
//
// pub struct QExt {
//     _registers: [F128; 32],
// }
//
// impl QExt {
//     #[inline]
//     pub(crate) fn exec_rv32ifdq(
//         &self,
//         cpu: &mut RV32I,
//         insn: u32,
//         insn_len: u32,
//     ) -> Result<(), CPUError<u32>> {
//         let _ = (cpu, insn, insn_len);
//         todo!()
//     }
//
//     #[inline]
//     pub(crate) fn exec_rv32efdq(
//         &self,
//         cpu: &mut RV32E,
//         insn: u32,
//         insn_len: u32,
//     ) -> Result<(), CPUError<u32>> {
//         let _ = (cpu, insn, insn_len);
//         todo!()
//     }
//
//     #[inline]
//     pub(crate) fn exec_rv64ifdq(
//         &self,
//         cpu: &mut RV64I,
//         insn: u32,
//         insn_len: u64,
//     ) -> Result<(), CPUError<u64>> {
//         let _ = (cpu, insn, insn_len);
//         todo!()
//     }
// }
//
// impl IsaExt<u32, 32, RV32I> for QExt {
//     const ISA_ID: &'static str = "FDQ";
//
//     fn execute(&mut self, cpu: &mut RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
//         if self.exec_rv32ifdq(cpu, instruction, 4).is_err() {
//             DExt::exec_rv32ifdq(self, cpu, instruction, 4)?
//         }
//
//         Ok(())
//     }
// }
//
// impl IsaExt<u32, 16, RV32E> for QExt {
//     const ISA_ID: &'static str = "FDQ";
//
//     fn execute(&mut self, cpu: &mut RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
//         if self.exec_rv32efdq(cpu, instruction, 4).is_err() {
//             DExt::exec_rv32efdq(self, cpu, instruction, 4)?
//         }
//
//         Ok(())
//     }
// }
//
// impl IsaExt<u64, 32, RV64I> for QExt {
//     const ISA_ID: &'static str = "FDQ";
//
//     fn execute(&mut self, cpu: &mut RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
//         if self.exec_rv64ifdq(cpu, instruction, 4).is_err() {
//             DExt::exec_rv64ifdq(self, cpu, instruction, 4)?
//         }
//
//         Ok(())
//     }
// }
//
// struct F128(u128);
