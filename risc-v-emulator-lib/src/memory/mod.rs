use std::ops::Range;

pub use bus::Bus;
pub use dram::Dram;

use crate::cpu::CPUError;

mod bus;
mod dram;

pub trait Memory<A> {
    fn size(&self) -> A;

    fn load_u8(&self, addr: A) -> Result<u8, CPUError<A>>;
    fn load_u16(&self, addr: A) -> Result<u16, CPUError<A>>;
    fn load_u32(&self, addr: A) -> Result<u32, CPUError<A>>;
    fn load_u64(&self, addr: A) -> Result<u64, CPUError<A>>;
    fn load_u128(&self, addr: A) -> Result<u128, CPUError<A>>;

    fn load_i8(&self, addr: A) -> Result<i8, CPUError<A>>;
    fn load_i16(&self, addr: A) -> Result<i16, CPUError<A>>;
    fn load_i32(&self, addr: A) -> Result<i32, CPUError<A>>;
    fn load_i64(&self, addr: A) -> Result<i64, CPUError<A>>;
    fn load_i128(&self, addr: A) -> Result<i128, CPUError<A>>;

    fn store_u8(&mut self, addr: A, value: u8) -> Result<(), CPUError<A>>;
    fn store_u16(&mut self, addr: A, value: u16) -> Result<(), CPUError<A>>;
    fn store_u32(&mut self, addr: A, value: u32) -> Result<(), CPUError<A>>;
    fn store_u64(&mut self, addr: A, value: u64) -> Result<(), CPUError<A>>;
    fn store_u128(&mut self, addr: A, value: u128) -> Result<(), CPUError<A>>;

    fn store_i8(&mut self, addr: A, value: i8) -> Result<(), CPUError<A>>;
    fn store_i16(&mut self, addr: A, value: i16) -> Result<(), CPUError<A>>;
    fn store_i32(&mut self, addr: A, value: i32) -> Result<(), CPUError<A>>;
    fn store_i64(&mut self, addr: A, value: i64) -> Result<(), CPUError<A>>;
    fn store_i128(&mut self, addr: A, value: i128) -> Result<(), CPUError<A>>;

    fn get_data(&self, range: Range<A>) -> Result<Vec<u8>, CPUError<A>>;
}

#[macro_export]
macro_rules! impl_memory {
    ($self:ident, $addr:ident, $value:ident, $load:block, $store:block) => {
        fn load_u8(&$self, $addr: A) -> Result<u8, CPUError<A>> $load
        fn load_u16(&$self, $addr: A) -> Result<u16, CPUError<A>> $load
        fn load_u32(&$self, $addr: A) -> Result<u32, CPUError<A>> $load
        fn load_u64(&$self, $addr: A) -> Result<u64, CPUError<A>> $load
        fn load_u128(&$self, $addr: A) -> Result<u128, CPUError<A>> $load

        fn load_i8(&$self, $addr: A) -> Result<i8, CPUError<A>> $load
        fn load_i16(&$self, $addr: A) -> Result<i16, CPUError<A>> $load
        fn load_i32(&$self, $addr: A) -> Result<i32, CPUError<A>> $load
        fn load_i64(&$self, $addr: A) -> Result<i64, CPUError<A>> $load
        fn load_i128(&$self, $addr: A) -> Result<i128, CPUError<A>> $load

        fn store_u8(&mut $self, $addr: A, $value: u8) -> Result<(), CPUError<A>> $store
        fn store_u16(&mut $self, $addr: A, $value: u16) -> Result<(), CPUError<A>> $store
        fn store_u32(&mut $self, $addr: A, $value: u32) -> Result<(), CPUError<A>> $store
        fn store_u64(&mut $self, $addr: A, $value: u64) -> Result<(), CPUError<A>> $store
        fn store_u128(&mut $self, $addr: A, $value: u128) -> Result<(), CPUError<A>> $store

        fn store_i8(&mut $self, $addr: A, $value: i8) -> Result<(), CPUError<A>> $store
        fn store_i16(&mut $self, $addr: A, $value: i16) -> Result<(), CPUError<A>> $store
        fn store_i32(&mut $self, $addr: A, $value: i32) -> Result<(), CPUError<A>> $store
        fn store_i64(&mut $self, $addr: A, $value: i64) -> Result<(), CPUError<A>> $store
        fn store_i128(&mut $self, $addr: A, $value: i128) -> Result<(), CPUError<A>> $store
    };
}

#[macro_export]
macro_rules! impl_memory_map {
    ($self:ident, $addr:ident, $map:block, $map_mut:block) => {
        fn load_u8(&$self, $addr: A) -> Result<u8, CPUError<A>> { let (mem, mapping) = $map; mem.load_u8($addr - mapping.start) }
        fn load_u16(&$self, $addr: A) -> Result<u16, CPUError<A>> { let (mem, mapping) = $map; mem.load_u16($addr - mapping.start) }
        fn load_u32(&$self, $addr: A) -> Result<u32, CPUError<A>> { let (mem, mapping) = $map; mem.load_u32($addr - mapping.start) }
        fn load_u64(&$self, $addr: A) -> Result<u64, CPUError<A>> { let (mem, mapping) = $map; mem.load_u64($addr - mapping.start) }
        fn load_u128(&$self, $addr: A) -> Result<u128, CPUError<A>> { let (mem, mapping) = $map; mem.load_u128($addr - mapping.start) }

        fn load_i8(&$self, $addr: A) -> Result<i8, CPUError<A>> { let (mem, mapping) = $map; mem.load_i8($addr - mapping.start) }
        fn load_i16(&$self, $addr: A) -> Result<i16, CPUError<A>> { let (mem, mapping) = $map; mem.load_i16($addr - mapping.start) }
        fn load_i32(&$self, $addr: A) -> Result<i32, CPUError<A>> { let (mem, mapping) = $map; mem.load_i32($addr - mapping.start) }
        fn load_i64(&$self, $addr: A) -> Result<i64, CPUError<A>> { let (mem, mapping) = $map; mem.load_i64($addr - mapping.start) }
        fn load_i128(&$self, $addr: A) -> Result<i128, CPUError<A>> { let (mem, mapping) = $map; mem.load_i128($addr - mapping.start) }

        fn store_u8(&mut $self, $addr: A, value: u8) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_u8($addr - mapping.start, value)}
        fn store_u16(&mut $self, $addr: A, value: u16) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_u16($addr - mapping.start, value)}
        fn store_u32(&mut $self, $addr: A, value: u32) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_u32($addr - mapping.start, value)}
        fn store_u64(&mut $self, $addr: A, value: u64) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_u64($addr - mapping.start, value)}
        fn store_u128(&mut $self, $addr: A, value: u128) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_u128($addr - mapping.start, value)}

        fn store_i8(&mut $self, $addr: A, value: i8) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_i8($addr - mapping.start, value)}
        fn store_i16(&mut $self, $addr: A, value: i16) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_i16($addr - mapping.start, value)}
        fn store_i32(&mut $self, $addr: A, value: i32) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_i32($addr - mapping.start, value)}
        fn store_i64(&mut $self, $addr: A, value: i64) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_i64($addr - mapping.start, value)}
        fn store_i128(&mut $self, $addr: A, value: i128) -> Result<(), CPUError<A>> {let (mem, mapping) = $map_mut; mem.store_i128($addr - mapping.start, value)}
    };
}
