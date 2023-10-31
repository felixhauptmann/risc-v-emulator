use crate::cpu::isa::ext::float::f::FExt;
use crate::cpu::isa::ext::float::q::QExt;
use crate::cpu::isa::ext::float::IsaExt;
use crate::cpu::isa::{RV32E, RV32I, RV64I};
use crate::cpu::CPUError;

pub struct DExt {
    _registers: [f64; 32],
}

impl DExt {
    #[inline]
    fn exec_rv32ifd(
        &mut self,
        cpu: &mut RV32I,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }

    #[inline]
    fn exec_rv32efd(
        &mut self,
        cpu: &mut RV32E,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }

    #[inline]
    fn exec_rv64ifd(
        &mut self,
        cpu: &mut RV64I,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u64>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }
}

impl DExt {
    #[inline]
    pub(crate) fn exec_rv32ifdq(
        ext: &mut QExt,
        cpu: &mut RV32I,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }

    #[inline]
    pub(crate) fn exec_rv32efdq(
        ext: &mut QExt,
        cpu: &mut RV32E,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }

    #[inline]
    pub(crate) fn exec_rv64ifdq(
        ext: &mut QExt,
        cpu: &mut RV64I,
        insn: u32,
        insn_len: u64,
    ) -> Result<(), CPUError<u64>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }
}

impl IsaExt<u32, 32, RV32I> for DExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
        if self.exec_rv32ifd(cpu, instruction, 4).is_err() {
            FExt::exec_rv32ifd(self, cpu, instruction, 4)?
        }

        Ok(())
    }
}

impl IsaExt<u32, 16, RV32E> for DExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
        if self.exec_rv32efd(cpu, instruction, 4).is_err() {
            FExt::exec_rv32efd(self, cpu, instruction, 4)?
        }

        Ok(())
    }
}

impl IsaExt<u64, 32, RV64I> for DExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
        if self.exec_rv64ifd(cpu, instruction, 4).is_err() {
            FExt::exec_rv64ifd(self, cpu, instruction, 4)?
        }

        Ok(())
    }
}
