use crate::cpu::isa::rv32i::RV32I;
use crate::cpu::isa::Isa;
use crate::cpu::{CPUError, Cpu};
use num_traits::AsPrimitive;

pub struct RV32E;

impl Isa<16> for RV32E {
    type XlenU = u32;
    type XlenI = i32;

    const ISA_ID: &'static str = "RV32E";
    const INSN_SIZE: Self::XlenU = 4;
    const REG_COUNT: usize = 16;

    fn exec<const REG_COUNT: usize, I: Isa<REG_COUNT>>(
        cpu: &mut Cpu<I, REG_COUNT>,
        instruction: u32,
    ) -> Result<(), CPUError>
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
        RV32I::exec(cpu, instruction)
    }
}
