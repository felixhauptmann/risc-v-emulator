use crate::cpu::CPUError;
use crate::dram::{Dram, DRAM_SIZE};

pub const DRAM_BASE: u64 = 0x8000_0000;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(dram: Dram) -> Self {
        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, CPUError> {
        if (DRAM_BASE..DRAM_BASE + DRAM_SIZE).contains(&addr) {
            return self.dram.load(addr - DRAM_BASE, size);
        }
        Err(CPUError::AddressNotMapped(addr))
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), CPUError> {
        if (DRAM_BASE..DRAM_BASE + DRAM_SIZE).contains(&addr) {
            return self.dram.store(addr - DRAM_BASE, size, value);
        }
        Err(CPUError::AddressNotMapped(addr))
    }

    #[cfg(test)]
    pub fn get_mem(&self) -> &Dram {
        &self.dram
    }
}
