use std::ops::Range;

use num_traits::Unsigned;

use crate::cpu::isa::Xlen;
use crate::cpu::CPUError;
use crate::memory::{impl_memory_map, Memory};

pub struct Bus<A: Xlen + Unsigned> {
    mem_map: Vec<(Range<A>, Box<dyn Memory<A>>)>,
}

impl<A: Xlen + Unsigned> Bus<A> {
    pub fn new(mem_map: Vec<(Range<A>, Box<dyn Memory<A>>)>) -> Self {
        let mut end_prev = A::zero();
        for (Range { start, end }, mem) in &mem_map {
            assert!(start < end);
            assert!(start >= &end_prev);
            assert!(mem.size() >= *end - *start);
            end_prev = *end;
        }

        Self { mem_map }
    }
}

impl<A: Xlen + Unsigned> Bus<A> {
    fn map(&self, addr: A) -> Result<(&dyn Memory<A>, &Range<A>), CPUError<A>> {
        for (mapping, mem) in &self.mem_map {
            if mapping.contains(&addr) {
                return Ok((mem.as_ref(), mapping));
            }
        }
        Err(CPUError::AddressNotMapped(addr))
    }

    fn map_mut(&mut self, addr: A) -> Result<(&mut dyn Memory<A>, &Range<A>), CPUError<A>> {
        for (mapping, mem) in &mut self.mem_map {
            if mapping.contains(&addr) {
                return Ok((mem.as_mut(), mapping));
            }
        }
        Err(CPUError::AddressNotMapped(addr))
    }
}

impl<A: Xlen + Unsigned> Memory<A> for Bus<A> {
    fn size(&self) -> A {
        A::max_value()
    }

    impl_memory_map!(self, addr, { self.map(addr)? }, { self.map_mut(addr)? });

    fn get_data(&self, range: Range<A>) -> Result<Vec<u8>, CPUError<A>> {
        let (mem, mapping) = self.map(range.start)?;
        mem.get_data(range.start - mapping.start..range.end - mapping.start)
    }
}
