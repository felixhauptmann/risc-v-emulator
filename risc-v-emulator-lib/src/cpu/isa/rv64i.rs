use std::ops::Range;

use crate::cpu::isa::ext::float::FloatExt;
use crate::cpu::isa::{Cpu, RV32I};
use crate::cpu::{CPUError, RegisterDump};
use crate::memory::{Bus, Dram, Memory};

macro_rules! impl_rv64i_exec {
    ($XLENU:ty, $XLENI:ty, $self:ident, $instruction:ident) => {{
        use std::ops::{Shl, Shr};

        let rd = (($instruction >> 7) & 0x1F) as usize;
        let rs1 = (($instruction >> 15) & 0x1F) as usize;
        let rs2 = (($instruction >> 20) & 0x1F) as usize;

        let opcode = $instruction & 0x7F; // opcode [6:0]

        let funct3 = (($instruction >> 12) & 0x7) as usize; // [14:12]
        let funct7 = (($instruction >> 25) & 0x7F) as usize; // [31:25]

        match opcode {
            0b000_0011 => {
                let imm = (($instruction & 0xFFF0_0000) as i32 >> 20) as $XLENI as $XLENU; // sign extended immediate [31:20]
                let address = $self.registers[rs1].overflowing_add(imm).0;

                $self.registers[rd] = match funct3 {
                    0b110 => $self.bus.load_u32(address)? as $XLENU,           // LWU
                    0b011 => $self.bus.load_i32(address)? as $XLENI as $XLENU, // LD
                    _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                };
            },
            0b010_0011 if funct3 == 0b011 => { // SD
                let imm = (($instruction & 0xFE00_0000) as i32 >> 20) as $XLENI as $XLENU
                | ($instruction & 0xF80) as $XLENU >> 7; // sign extended immediate [31:25][11:7]
                let address = $self.registers[rs1].overflowing_add(imm).0;

                $self.bus.store_u64(address, $self.registers[rs2] as u64)?
            }
            0b001_0011 => {
                let shamt = ($instruction & 0x3F00_000) >> 20;
                let funct6 = ($instruction) >> 26;
                $self.registers[rd] = match (funct6, funct3) {
                    // SLLI (logical left shift)
                    (0b000_000, 0b001) => $self.registers[rs1].shl(shamt),
                    // SRLI (logical right shift)
                    (0b000_000, 0b101) => $self.registers[rs1].shr(shamt),
                    // SRAI (arithmetic right shift)
                    (0b010_000, 0b101) => ($self.registers[rs1] as $XLENI).shr(shamt) as $XLENU,
                    _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                };
            }
            0b001_1011 => {
                let imm = (($instruction & 0xFFF0_0000) as i32 >> 20)
                    as i32
                    as u32; // sign extended immediate [31:20]
                $self.registers[rd] = match funct3 {
                    0b000 => ($self.registers[rs1] as u32).wrapping_add(imm) as $XLENU, // ADDIW
                    0b001 if funct7 == 0b0000000 && $instruction & 0x2000000 == 0 => ($self.registers[rs1] as u32).shl(imm & 0x1F) as $XLENU, // SLLIW
                    0b101 if $instruction & 0x2000000 == 0 => match funct7 {
                        0b000_0000 => ($self.registers[rs1] as u32).shr(imm & 0x1F) as $XLENU,           // SRLIW
                        0b010_0000 => ($self.registers[rs1] as i32).shr(imm & 0x1F) as i32 as $XLENU, // SRAIW
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                    _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                };
            },
            0b011_1011 => $self.registers[rd] = match funct3 {
                0b000 => match funct7 {
                    0b000_0000 => ($self.registers[rs1] as u32).wrapping_add($self.registers[rs2] as u32) as i32 as $XLENI as $XLENU, // ADDW
                    0b010_0000 => ($self.registers[rs1] as u32).wrapping_sub($self.registers[rs2] as u32) as i32 as $XLENI as $XLENU, // SUBW
                    _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                }
                0b001 if funct7 == 0b000_0000 => ($self.registers[rs1] as u32).shl(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize) as $XLENU, // SLLW
                0b101 => match funct7 {
                    0b000_0000 => ($self.registers[rs1] as u32).shr(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize) as $XLENU, // SRLW
                    0b010_0000 => ($self.registers[rs1] as i32).shr(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize) as $XLENU, // SRAW
                    _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                }
                _ => return Err(CPUError::InstructionNotImplemented($instruction)),
            },
            _ => return Err(CPUError::InstructionNotImplemented($instruction)),
        }

        Ok(())
    }};
}
pub(crate) use impl_rv64i_exec;

pub struct RV64I {
    pub(super) pc: u64,
    pub(super) bus: Bus<u64>,
    pub(super) registers: [u64; 32],
    _float_ext: Option<FloatExt>,
    dram_mapping: Range<u64>,
}

impl RV64I {
    fn exec_rv64i(&mut self, insn: u32) -> Result<(), CPUError<u64>> {
        impl_rv64i_exec!(u64, i64, self, insn)
    }
}

impl Cpu<u64, 32> for RV64I {
    const ISA_ID: &'static str = "RV64I";

    fn new(bus: Bus<u64>, dram_mapping: Range<u64>, float_ext: Option<FloatExt>) -> Self {
        let mut cpu = Self {
            pc: 0,
            bus,
            registers: [0; 32],
            _float_ext: float_ext,
            dram_mapping,
        };

        cpu.reset();

        cpu
    }

    fn with_code(code: &[u8], float_ext: Option<FloatExt>) -> Self {
        const DRAM_BASE: usize = 0x5000_0000;
        const DRAM_SIZE: usize = 1024 * 1024 * 128;

        let dram: Dram<u64> = Dram::<u64>::with_code(code, DRAM_SIZE as u64);

        Self::new(
            Bus::<u64>::new(vec![(
                DRAM_BASE as u64..(DRAM_BASE + DRAM_SIZE) as u64,
                Box::new(dram),
            )]),
            (DRAM_BASE as u64)..(DRAM_BASE + DRAM_SIZE) as u64,
            float_ext,
        )
    }

    fn cycle(&mut self) -> Result<(), CPUError<u64>> {
        // fetch
        let instruction = self.fetch()?;

        // increment pc
        self.pc += 4;

        // decode and execute
        self.execute(instruction)
    }

    fn execute(&mut self, instruction: u32) -> Result<(), CPUError<u64>> {
        if self.exec_rv64i(instruction).is_err() {
            RV32I::exec_rv64i(self, instruction, 4)?
        }

        Ok(())
    }

    fn fetch(&self) -> Result<u32, CPUError<u64>> {
        self.bus.load_u32(self.pc)
    }

    fn reset(&mut self) {
        self.pc = self.dram_mapping.start;
        self.registers[2] = self.dram_mapping.end;
    }

    fn dump_registers(&self) -> RegisterDump<u64, 32> {
        RegisterDump::new(self.pc, &self.registers)
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.bus.get_data(self.dram_mapping.clone()).unwrap()
    }
}
