use std::fmt::{Debug, Display, Formatter};

use crate::cpu::isa::XlenU;

pub mod isa;
#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum CPUError<A> {
    // TODO handle these errors the way they are supposed to be handled according to RISC-V Spec
    InstructionNotImplemented(u32),
    AddressNotMapped(A),
    InvalidAccessSize(u64),
    Halt,
}

impl<XLEN: XlenU> Display for CPUError<XLEN> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CPUError::InstructionNotImplemented(instruction) => {
                write!(f, "Instruction {instruction:#010x} is not implemented!")
            }
            CPUError::AddressNotMapped(address) => {
                write!(f, "Nothing is mapped to address {address:#018X}!")
            }
            CPUError::InvalidAccessSize(size) => {
                write!(f, "Can not read {size} bits!")
            }
            CPUError::Halt => {
                write!(f, "CPU Halted!")
            }
        }
    }
}

#[derive(PartialEq)]
pub struct RegisterDump<XLEN: XlenU, const REG_COUNT: usize> {
    pc: Option<XLEN>,
    registers: [Option<XLEN>; REG_COUNT],
}

#[cfg(test)]
impl<XLEN: XlenU, const REG_COUNT: usize> RegisterDump<XLEN, REG_COUNT> {
    fn uninitialized() -> Self {
        RegisterDump {
            pc: None,
            registers: [None; REG_COUNT],
        }
    }

    fn apply_mask(&mut self, mask: &RegisterDump<XLEN, REG_COUNT>) {
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

impl<XLEN: XlenU, const REG_COUNT: usize> RegisterDump<XLEN, REG_COUNT> {
    fn new(pc: XLEN, register: &[XLEN; REG_COUNT]) -> Self {
        RegisterDump {
            pc: Some(pc),
            registers: register.map(Some),
        }
    }
}

impl<XLEN: XlenU, const REG_COUNT: usize> Debug for RegisterDump<XLEN, REG_COUNT> {
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
