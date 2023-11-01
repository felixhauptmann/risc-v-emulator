use std::ops::Range;

use crate::cpu::isa::ext::float::FloatExt;
use crate::cpu::isa::{Cpu, RV32I};
use crate::cpu::{CPUError, RegisterDump};
use crate::cpu::isa::ext::mul::M;
use crate::memory::{Bus, Dram, Memory};

pub struct RV32E {
    pub(super) pc: u32,
    pub(super) bus: Bus<u32>,
    pub(super) registers: [u32; 16],
    _float_ext: Option<FloatExt>,
    dram_mapping: Range<u32>,
}

impl Cpu<u32, 16> for RV32E {
    const ISA_ID: &'static str = "RV32E";

    fn new(bus: Bus<u32>, dram_mapping: Range<u32>, float_ext: Option<FloatExt>, _mul_ext: Option<M>) -> Self {
        let mut cpu = Self {
            pc: 0,
            bus,
            registers: [0; 16],
            _float_ext: float_ext,
            dram_mapping,
        };

        cpu.reset();

        cpu
    }

    fn with_code(code: &[u8], float_ext: Option<FloatExt>, mul_ext: Option<M>) -> Self {
        const DRAM_BASE: usize = 0x5000_0000;
        const DRAM_SIZE: usize = 1024 * 1024 * 128;

        let dram: Dram<u32> = Dram::<u32>::with_code(code, DRAM_SIZE as u32);

        Self::new(
            Bus::<u32>::new(vec![(
                DRAM_BASE as u32..(DRAM_BASE + DRAM_SIZE) as u32,
                Box::new(dram),
            )]),
            (DRAM_BASE as u32)..(DRAM_BASE + DRAM_SIZE) as u32,
            float_ext,
            mul_ext
        )
    }

    fn cycle(&mut self) -> Result<(), CPUError<u32>> {
        // fetch
        let instruction = self.fetch()?;

        // increment pc
        self.pc += 4;

        // decode and execute
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: u32) -> Result<(), CPUError<u32>> {
        RV32I::exec_rv32e(self, instruction, 4)
    }

    fn fetch(&self) -> Result<u32, CPUError<u32>> {
        self.bus.load_u32(self.pc)
    }

    fn reset(&mut self) {
        self.pc = self.dram_mapping.start;
        self.registers[2] = self.dram_mapping.end;
    }

    fn dump_registers(&self) -> RegisterDump<u32, 16> {
        RegisterDump::new(self.pc, &self.registers)
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.bus.get_data(self.dram_mapping.clone()).unwrap()
    }
}
