use crate::cpu::isa::ext::float::d::DExt;
use crate::cpu::isa::ext::IsaExt;
use crate::cpu::isa::{RV32E, RV32I, RV64I};
use crate::cpu::CPUError;

pub struct FExt {
    _registers: [f32; 32],
}

impl FExt {
    #[inline]
    fn exec_rv32if(
        &mut self,
        cpu: &mut RV32I,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }

    #[inline]
    fn exec_rv32ef(
        &mut self,
        cpu: &mut RV32E,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }

    #[inline]
    fn exec_rv64if(
        &mut self,
        cpu: &mut RV64I,
        insn: u32,
        insn_len: u64,
    ) -> Result<(), CPUError<u64>> {
        let _ = (cpu, insn, insn_len);
        Ok(())
    }
}

impl FExt {
    pub(crate) fn exec_rv32ifd(
        ext: &mut DExt,
        cpu: &mut RV32I,
        insn: u32,
        insn_len: i32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }

    pub(crate) fn exec_rv32efd(
        ext: &mut DExt,
        cpu: &mut RV32E,
        insn: u32,
        insn_len: i32,
    ) -> Result<(), CPUError<u32>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }

    pub(crate) fn exec_rv64ifd(
        ext: &mut DExt,
        cpu: &mut RV64I,
        insn: u32,
        insn_len: i64,
    ) -> Result<(), CPUError<u64>> {
        let _ = (ext, cpu, insn, insn_len);
        todo!()
    }
}

impl IsaExt<u32, 32, RV32I> for FExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV32I, instruction: u32) -> Result<(), CPUError<u32>> {
        self.exec_rv32if(cpu, instruction, 4)
    }
}

impl IsaExt<u32, 16, RV32E> for FExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV32E, instruction: u32) -> Result<(), CPUError<u32>> {
        self.exec_rv32ef(cpu, instruction, 4)
    }
}

impl IsaExt<u64, 32, RV64I> for FExt {
    const ISA_ID: &'static str = "F";

    fn execute(&mut self, cpu: &mut RV64I, instruction: u32) -> Result<(), CPUError<u64>> {
        self.exec_rv64if(cpu, instruction, 4)
    }
}
