use crate::cpu::CPUError;

pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: &[u8]) -> Dram {
        // write code at start of new dram
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().copied());

        Self { dram }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, CPUError> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(CPUError::InvalidAccessSize(size)),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), CPUError> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(CPUError::InvalidAccessSize(size)),
        }
    }
}

impl Dram {
    fn load8(&self, addr: u64) -> u64 {
        let index = addr as usize;
        u64::from(self.dram[index])
    }

    fn load16(&self, addr: u64) -> u64 {
        u64::from(u16::from_le_bytes(
            self.dram[addr as usize..addr as usize + 2]
                .try_into()
                .unwrap(),
        ))
    }

    fn load32(&self, addr: u64) -> u64 {
        u64::from(u32::from_le_bytes(
            self.dram[addr as usize..addr as usize + 4]
                .try_into()
                .unwrap(),
        ))
    }

    fn load64(&self, addr: u64) -> u64 {
        u64::from_le_bytes(
            self.dram[addr as usize..addr as usize + 8]
                .try_into()
                .unwrap(),
        )
    }

    fn store8(&mut self, addr: u64, value: u64) {
        self.dram[addr as usize] = value as u8;
    }

    fn store16(&mut self, addr: u64, value: u64) {
        self.dram
            .splice(addr as usize..2, (value as u16).to_le_bytes());
    }

    fn store32(&mut self, addr: u64, value: u64) {
        self.dram
            .splice(addr as usize..4, (value as u32).to_le_bytes());
    }

    fn store64(&mut self, addr: u64, value: u64) {
        self.dram.splice(addr as usize..8, value.to_le_bytes());
    }
}
