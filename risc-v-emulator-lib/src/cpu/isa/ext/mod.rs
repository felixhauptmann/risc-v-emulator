use crate::cpu::CPUError;
use crate::cpu::isa::{Cpu, XlenU};

pub use mul::M;

pub(crate) mod float;
pub(crate) mod mul;

pub(crate) trait IsaExt<XLEN: XlenU, const REG_COUNT: usize, CPU: Cpu<XLEN, REG_COUNT>> {
    const ISA_ID: &'static str;

    fn execute(cpu: &mut CPU, instruction: u32) -> Result<(), CPUError<XLEN>>;
}
