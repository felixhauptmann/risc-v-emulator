use std::mem;
use std::ops::Range;

use num_traits::{FromBytes, ToBytes, Unsigned};

use crate::cpu::isa::{As, Xlen};
use crate::cpu::CPUError;
use crate::impl_memory;
use crate::memory::Memory;

pub struct Dram<A: Xlen + Unsigned> {
    dram: Vec<u8>,
    size: A,
}

impl<A: Xlen + Unsigned> Dram<A> {
    pub fn with_code(code: &[u8], size: A) -> Dram<A> {
        assert!(size.to_usize().is_some());

        // write code at start of new dram
        let mut dram = vec![0; size.as_t::<usize>()];
        dram.splice(..code.len(), code.iter().copied());

        Self { dram, size }
    }
}

impl<A: Xlen + Unsigned> Dram<A> {
    fn load<T: FromBytes>(&self, addr: A) -> Result<T, CPUError<A>>
    where
        for<'a> <T as FromBytes>::Bytes: TryFrom<&'a [u8]>,
    {
        let bytes =
            self.dram[addr.as_t::<usize>()..addr.as_t::<usize>() + mem::size_of::<T>()].as_ref();

        if let Ok(bytes) = &bytes.try_into() {
            Ok(T::from_le_bytes(bytes))
        } else {
            panic!("Error in {}", line!());
        }
    }

    fn store<T: ToBytes>(&mut self, addr: A, value: T) -> Result<(), CPUError<A>> {
        let bytes = value.to_le_bytes().as_ref().to_owned();
        self.dram.splice(
            addr.as_t::<usize>()..addr.as_t::<usize>() + bytes.len(),
            bytes,
        );

        Ok(())
    }
}

impl<A: Xlen + Unsigned> Memory<A> for Dram<A> {
    fn size(&self) -> A {
        self.size
    }

    impl_memory!(self, addr, value, { self.load(addr) }, {
        self.store(addr, value)
    });

    fn get_data(&self, range: Range<A>) -> Result<Vec<u8>, CPUError<A>> {
        if range.end <= self.size {
            Ok(self.dram[range.start.as_t()..range.end.as_t()].to_vec())
        } else {
            Err(CPUError::AddressNotMapped(range.end))
        }
    }
}
