use std::fmt::{Debug, Display, UpperHex};
use std::ops::Range;

use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::{AsPrimitive, NumAssign, PrimInt, Signed, Unsigned, WrappingAdd, WrappingSub};

pub use rv32e::RV32E;
pub use rv32i::RV32I;
pub use rv64i::RV64I;

use crate::cpu::isa::ext::float::FloatExt;
use crate::cpu::{CPUError, RegisterDump};
use crate::memory::Bus;

mod rv32i;

mod rv32e;

mod rv64i;

mod ext;

pub trait XlenU:
    'static
    + Unsigned
    + PrimInt
    + NumAssign
    + WrappingAdd
    + WrappingSub
    + OverflowingAdd
    + UpperHex
    + Display
    + Debug
    + AsPrimitive<Self::Signed>
    + AsPrimitive<usize>
where
    Self::Signed: AsPrimitive<Self>,
{
    const LEN: usize;
    type Signed: XlenI;
}

impl XlenU for u128 {
    const LEN: usize = 128;
    type Signed = i128;
}

impl XlenU for u64 {
    const LEN: usize = 64;
    type Signed = i64;
}

impl XlenU for u32 {
    const LEN: usize = 32;
    type Signed = i32;
}

pub trait XlenI:
    'static
    + Signed
    + PrimInt
    + NumAssign
    + WrappingAdd
    + WrappingSub
    + OverflowingAdd
    + UpperHex
    + Display
    + Debug
    + AsPrimitive<Self::UnSigned>
    + AsPrimitive<usize>
where
    Self::UnSigned: AsPrimitive<Self>,
{
    const LEN: usize;
    type UnSigned: XlenU;
}

impl XlenI for i128 {
    const LEN: usize = 128;
    type UnSigned = u128;
}

impl XlenI for i64 {
    const LEN: usize = 64;
    type UnSigned = u64;
}

impl XlenI for i32 {
    const LEN: usize = 32;
    type UnSigned = u32;
}

pub trait Cpu<XLEN: XlenU, const REG_COUNT: usize> {
    const ISA_ID: &'static str;

    fn new(bus: Bus<XLEN>, dram_mapping: Range<XLEN>, float_ext: Option<FloatExt>) -> Self;
    fn with_code(code: &[u8], float_ext: Option<FloatExt>) -> Self;

    fn isa_id(&self) -> &'static str {
        Self::ISA_ID
    }

    fn cycle(&mut self) -> Result<(), CPUError<XLEN>>;

    fn execute(&mut self, instruction: u32) -> Result<(), CPUError<XLEN>>;

    fn fetch(&self) -> Result<u32, CPUError<XLEN>>;

    fn reset(&mut self);

    fn dump_registers(&self) -> RegisterDump<XLEN, REG_COUNT>;

    fn dump_memory(&self) -> Vec<u8>;
}

// struct RegFile<XLEN: XlenU, const REG_COUNT: usize> {
//     registers: [XLEN; REG_COUNT],
// }
//
// impl<XLEN: XlenU, const REG_COUNT: usize> RegFile<XLEN, REG_COUNT> {
//     fn get(&self, r: usize) -> XLEN {
//         if r == 0 {
//             XLEN::zero()
//         } else {
//             self.registers[r]
//         }
//     }
//
//     fn set(&mut self, r: usize, v: XLEN) {
//         self.registers[r] = v;
//     }
//
//     fn len(&self) -> usize {
//         REG_COUNT
//     }
//
// }
//
// impl<XLEN: XlenU, const REG_COUNT: usize> Default for RegFile<XLEN, REG_COUNT> {
//     fn default() -> Self {
//         Self {
//             registers: [XLEN::zero(); REG_COUNT]
//         }
//     }
// }

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
