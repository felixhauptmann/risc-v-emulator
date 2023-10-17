use num_traits::AsPrimitive;

use crate::cpu::isa::rv32i::RV32I;
use crate::cpu::isa::Isa;
use crate::cpu::{CPUError, Cpu};

pub struct RV64I(());

impl Isa<32> for RV64I {
    type XlenU = u64;
    type XlenI = i64;
    const ISA_ID: &'static str = "RV64I";
    const INSN_SIZE: Self::XlenU = 4;

    fn exec<const REG_COUNT: usize, I: Isa<REG_COUNT>>(
        cpu: &mut Cpu<I, REG_COUNT>,
        instruction: u32,
    ) -> Result<(), CPUError<I::XlenU>>
    where
        bool: AsPrimitive<I::XlenU>,
        u8: AsPrimitive<I::XlenU>,
        i8: AsPrimitive<I::XlenU>,
        u16: AsPrimitive<I::XlenU>,
        u32: AsPrimitive<I::XlenU>,
        i32: AsPrimitive<I::XlenU>,
        i8: AsPrimitive<I::XlenI>,
        i16: AsPrimitive<I::XlenI>,
        u32: AsPrimitive<I::XlenI>,
        i32: AsPrimitive<I::XlenI>,
        I::XlenU: AsPrimitive<u8>,
        I::XlenU: AsPrimitive<u16>,
        I::XlenU: AsPrimitive<u32>,
    {
        let res = RV32I::exec(cpu, instruction);

        if let Err(CPUError::InstructionNotImplemented(_)) = res {
            todo!("")
        } else {
            res
        }
    }
}
