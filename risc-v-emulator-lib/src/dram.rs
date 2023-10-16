use num_traits::{FromBytes, ToBytes};
use std::mem;

pub const DRAM_SIZE: usize = 1024 * 1024 * 128;

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn with_code(code: &[u8]) -> Dram {
        // write code at start of new dram
        let mut dram = vec![0; DRAM_SIZE];
        dram.splice(..code.len(), code.iter().copied());

        Self { dram }
    }
}

impl Dram {
    pub(crate) fn load<T: FromBytes>(&self, addr: usize) -> T
    where
        for<'a> <T as FromBytes>::Bytes: TryFrom<&'a [u8]>,
    {
        let bytes = self.dram[addr..addr + mem::size_of::<T>()].as_ref();

        if let Ok(bytes) = &bytes.try_into() {
            T::from_le_bytes(bytes)
        } else {
            panic!("Error in {}", line!())
        }
    }

    pub(crate) fn store<T: ToBytes>(&mut self, addr: usize, value: T) {
        let bytes = value.to_le_bytes().as_ref().to_owned();
        self.dram.splice(addr..addr + bytes.len(), bytes);
    }

    pub(crate) fn get_data(&self) -> &Vec<u8> {
        &self.dram
    }
}
