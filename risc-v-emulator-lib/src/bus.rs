use crate::cpu::CPUError;
use crate::dram::{Dram, DRAM_SIZE};
use num_traits::{FromBytes, ToBytes};

pub const DRAM_BASE: usize = 0x8000_0000;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(dram: Dram) -> Self {
        assert!((DRAM_BASE + DRAM_SIZE) < u32::MAX as usize);
        Self { dram }
    }
}

impl Bus {
    pub(crate) fn load<T: FromBytes>(&self, addr: usize) -> Result<T, CPUError>
    where
        for<'a> <T as FromBytes>::Bytes: TryFrom<&'a [u8]>,
    {
        if (DRAM_BASE..DRAM_BASE + DRAM_SIZE).contains(&addr) {
            return Ok(self.dram.load::<T>(addr - DRAM_BASE));
        }
        Err(CPUError::AddressNotMapped(addr as u64))
    }

    pub(crate) fn store<T: ToBytes>(&mut self, addr: usize, value: T) -> Result<(), CPUError> {
        if (DRAM_BASE..DRAM_BASE + DRAM_SIZE).contains(&addr) {
            return Ok(self.dram.store::<T>(addr - DRAM_BASE, value));
        }
        Err(CPUError::AddressNotMapped(addr as u64))
    }

    pub(crate) fn get_mem(&self) -> &Dram {
        &self.dram
    }
}
