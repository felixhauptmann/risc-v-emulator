use std::ops::Range;

use crate::cpu::isa::XlenU;
use crate::cpu::CPUError;
use crate::memory::{impl_memory_map, Memory};

pub struct Bus<A: XlenU> {
    mem_map: Vec<(Range<A>, Box<dyn Memory<A>>)>,
}

macro_rules! impl_bus {
    ($A:ty) => {
        impl Bus<$A> {
            pub fn new(mem_map: Vec<(Range<$A>, Box<dyn Memory<$A>>)>) -> Self {
                let mut end_prev = 0;
                for (Range { start, end }, mem) in &mem_map {
                    assert!(start < end);
                    assert!(start >= &end_prev);
                    assert!(mem.size() >= *end - *start);
                    end_prev = *end;
                }

                Self { mem_map }
            }
        }

        impl Bus<$A> {
            fn map(&self, addr: $A) -> Result<(&dyn Memory<$A>, &Range<$A>), CPUError<$A>> {
                for (mapping, mem) in &self.mem_map {
                    if mapping.contains(&addr) {
                        return Ok((mem.as_ref(), mapping));
                    }
                }
                Err(CPUError::AddressNotMapped(addr))
            }

            fn map_mut(
                &mut self,
                addr: $A,
            ) -> Result<(&mut dyn Memory<$A>, &Range<$A>), CPUError<$A>> {
                for (mapping, mem) in &mut self.mem_map {
                    if mapping.contains(&addr) {
                        return Ok((mem.as_mut(), mapping));
                    }
                }
                Err(CPUError::AddressNotMapped(addr))
            }
        }

        impl Memory<$A> for Bus<$A> {
            fn size(&self) -> $A {
                <$A>::max_value()
            }

            impl_memory_map!(self, $A, addr, { self.map(addr)? }, { self.map_mut(addr)? });

            fn get_data(&self, range: Range<$A>) -> Result<Vec<u8>, CPUError<$A>> {
                let (mem, mapping) = self.map(range.start)?;
                mem.get_data(range.start - mapping.start..range.end - mapping.start)
            }
        }
    };
}

impl_bus!(u32);
impl_bus!(u64);
impl_bus!(u128);
