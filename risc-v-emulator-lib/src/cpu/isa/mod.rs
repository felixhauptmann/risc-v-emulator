use std::fmt::{Debug, Display, UpperHex};

use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::{AsPrimitive, NumAssign, PrimInt, Signed, Unsigned, WrappingAdd, WrappingSub};

pub use rv32e::RV32E;
pub use rv32i::RV32I;
pub use rv64i::RV64I;

use crate::cpu::{CPUError, Cpu};

mod rv32i;

mod rv32e;

mod rv64i;

pub trait Xlen:
    'static
    + PrimInt
    + NumAssign
    + WrappingAdd
    + WrappingSub
    + OverflowingAdd
    + UpperHex
    + Display
    + Debug
    + AsPrimitive<usize>
{
}

impl<T> Xlen for T where
    T: 'static
        + PrimInt
        + NumAssign
        + WrappingAdd
        + WrappingSub
        + OverflowingAdd
        + UpperHex
        + Display
        + Debug
        + AsPrimitive<usize>
{
}

pub trait Isa<const REG_COUNT: usize> {
    type XlenU: Xlen + Unsigned + AsPrimitive<Self::XlenI>;
    type XlenI: Xlen + Signed + AsPrimitive<Self::XlenU>;

    const ISA_ID: &'static str;

    const INSN_SIZE: Self::XlenU;

    fn exec<const REG_COUNT_I: usize, I: Isa<REG_COUNT_I>>(
        cpu: &mut Cpu<I, REG_COUNT_I>,
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
        I::XlenU: AsPrimitive<u32>;
}

pub(crate) trait As {
    fn as_t<T: Copy + 'static>(self) -> T
    where
        Self: AsPrimitive<T>;
}

impl<TT> As for TT {
    fn as_t<T: Copy + 'static>(self) -> T
    where
        TT: AsPrimitive<T>,
    {
        self.as_()
    }
}
