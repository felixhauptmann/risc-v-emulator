use std::fmt::{Debug, Display, Formatter};

use num_traits::{AsPrimitive, Zero};

use crate::bus::{Bus, DRAM_BASE};
use crate::cpu::isa::{As, Isa};
use crate::dram::DRAM_SIZE;

pub mod isa;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum CPUError {
    // TODO handle these errors the way they are supposed to be handled according to RISC-V Spec
    InstructionNotImplemented(u32),
    AddressNotMapped(u64),
    InvalidAccessSize(u64),
}

impl Display for CPUError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CPUError::InstructionNotImplemented(opcode) => {
                write!(f, "Opcode {opcode:#010x} is not implemented!")
            }
            CPUError::AddressNotMapped(address) => {
                write!(f, "Nothing is mapped to address {address:#018x}!")
            }
            CPUError::InvalidAccessSize(size) => {
                write!(f, "Can not read {size} bits!")
            }
        }
    }
}

#[derive(PartialEq)]
pub struct RegisterDump<I: Isa<REG_COUNT>, const REG_COUNT: usize> {
    pc: Option<I::XlenU>,
    registers: [Option<I::XlenU>; REG_COUNT],
}

#[cfg(test)]
impl<I: Isa<REG_COUNT>, const REG_COUNT: usize> RegisterDump<I, REG_COUNT> {
    fn uninitialized() -> Self {
        RegisterDump {
            pc: None,
            registers: [None; REG_COUNT],
        }
    }

    fn apply_mask(&mut self, mask: &RegisterDump<I, REG_COUNT>) {
        if mask.pc.is_none() {
            self.pc = None;
        }

        for (i, v) in mask.registers.iter().enumerate() {
            if v.is_none() {
                self.registers[i] = None;
            }
        }
    }
}

impl<I: Isa<REG_COUNT>, const REG_COUNT: usize> RegisterDump<I, REG_COUNT> {
    fn new(pc: I::XlenU, register: &[I::XlenU; REG_COUNT]) -> Self {
        RegisterDump {
            pc: Some(pc),
            registers: register.map(Some),
        }
    }
}

impl<I: Isa<REG_COUNT>, const REG_COUNT: usize> Debug for RegisterDump<I, REG_COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const ABI: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];

        writeln!(f, "--------- Register Dump ---------")?;
        let pc = match self.pc {
            None => "?".to_string(),
            Some(pc) => {
                format!("{pc:#010X}")
            }
        };
        writeln!(f, "pc: {pc}\n")?;

        writeln!(f, "reg abi   base16     base10")?;
        for (i, reg) in self.registers.iter().enumerate() {
            let (v_hex, v) = match reg {
                None => ("?".to_string(), "?".to_string()),
                Some(v) => (format!("{v:#018X}"), v.to_string()),
            };
            writeln!(f, "x{i:<#2} {:<#4}: {v_hex} {v}", ABI[i])?;
        }
        writeln!(f, "------------ bye :) -------------")
    }
}

pub struct Cpu<I: Isa<REG_COUNT>, const REG_COUNT: usize> {
    pub(crate) pc: I::XlenU,
    pub(crate) bus: Bus,
    pub(crate) registers: [I::XlenU; REG_COUNT],
}

impl<I: Isa<REG_COUNT>, const REG_COUNT: usize> Cpu<I, REG_COUNT> {
    pub fn get_isa_id(&self) -> &'static str {
        I::ISA_ID
    }
}

impl<I: Isa<REG_COUNT>, const REG_COUNT: usize> Cpu<I, REG_COUNT>
where
    bool: AsPrimitive<I::XlenU>,

    u8: AsPrimitive<I::XlenU>,
    i8: AsPrimitive<I::XlenU>,
    u16: AsPrimitive<I::XlenU>,
    u32: AsPrimitive<I::XlenU>,
    i32: AsPrimitive<<I as Isa<REG_COUNT>>::XlenU>,

    i8: AsPrimitive<I::XlenI>,
    i16: AsPrimitive<I::XlenI>,
    u32: AsPrimitive<I::XlenI>,
    i32: AsPrimitive<I::XlenI>,

    I::XlenU: AsPrimitive<u8>,
    I::XlenU: AsPrimitive<u16>,
    I::XlenU: AsPrimitive<u32>,

    usize: AsPrimitive<I::XlenU>,
{
    pub fn new(bus: Bus) -> Cpu<I, REG_COUNT> {
        let mut cpu = Self {
            pc: DRAM_BASE.as_t::<I::XlenU>(),
            bus,
            registers: [I::XlenU::zero(); REG_COUNT],
        };

        cpu.registers[2] = (DRAM_BASE + DRAM_SIZE).as_t::<I::XlenU>();

        cpu
    }

    pub fn cycle(&mut self) -> Result<(), CPUError> {
        // fetch
        let instruction = self.fetch()?;

        // increment pc
        self.pc += I::INSN_SIZE;

        // decode and execute
        self.execute(instruction)
    }

    pub fn reset(&mut self) {
        self.pc = DRAM_BASE.as_t::<I::XlenU>()
    }

    pub fn dump_registers(&self) -> RegisterDump<I, REG_COUNT> {
        RegisterDump::new(self.pc, &self.registers)
    }

    pub fn dump_memory(&self) -> &Vec<u8> {
        self.bus.get_mem().get_data()
    }

    fn fetch(&self) -> Result<u32, CPUError> {
        self.bus.load::<u32>(self.pc.as_t::<usize>())
    }

    fn execute(&mut self, instruction: u32) -> Result<(), CPUError> {
        I::exec(self, instruction)
    }
}
