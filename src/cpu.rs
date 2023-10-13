use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

use crate::bus::{Bus, DRAM_BASE};
use crate::cpu::CPUError::InstructionNotImplemented;
use crate::dram::DRAM_SIZE;

pub struct CpuRV64I {
    registers: [u64; 32],
    pub(crate) pc: u64,
    bus: Bus,
}

impl CpuRV64I {
    pub fn new(bus: Bus) -> Self {
        let mut cpu = CpuRV64I {
            registers: [0; 32],
            pc: DRAM_BASE,
            bus,
        };

        cpu.registers[2] = DRAM_BASE + DRAM_SIZE;

        cpu
    }

    pub fn dump_registers(&self) -> RegisterDump {
        RegisterDump::new(self.pc, &self.registers)
    }

    pub fn cycle(&mut self) -> Result<(), CPUError> {
        // fetch
        let instruction = self.fetch()?;

        // increment pc
        self.pc += 4;

        // decode
        // execute
        self.execute(instruction as u32)?;

        Ok(())
    }
}

impl CpuRV64I {
    fn fetch(&self) -> Result<u64, CPUError> {
        self.bus.load(self.pc, 32)
    }

    fn execute(&mut self, instruction: u32) -> Result<(), CPUError> {
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;

        let opcode = instruction & 0x7f; // opcode [6:0]

        let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
        let funct7 = ((instruction >> 25) & 0x7F) as usize; // [31:25]

        // set x0 to 0 to emulate x0 hardwired to all zeroes
        self.registers[0] = 0;

        // decode and execute instruction
        match opcode {
            // LUI
            0b011_0111 => {
                let imm = (instruction & 0xFFFFF000) as u64; // TODO sign extend for 64bit?
                self.registers[rd] = imm;
            }
            // AUIPC
            0b001_0111 => {
                let imm = (instruction & 0xFFFFF000) as u64; // TODO sign extend for 64bit?
                self.registers[rd] = self.pc + imm - 4;
            }
            // JAL
            0b110_1111 => {
                // [31][19:12][20][30:21]0  ins
                //  20  19 12  11  10  1    target
                let imm = ((instruction & 0x80000000) as i32 >> 11) as u64
                    | (instruction & 0xFF000) as u64
                    | (instruction & 0x100000) as u64 >> 9
                    | (instruction & 0x7FE00000) as u64 >> 20;

                self.registers[rd] = self.pc;
                self.pc = (self.pc - 4).overflowing_add(dbg!(imm)).0;
            }
            // JALR
            0b110_0111 if funct3 == 0b000 => {
                let imm = i64::from((instruction & 0xfff_00000) as i32 >> 20) as u64; // sign extended immediate [31:20]

                self.registers[rd] = self.pc;
                self.pc = self.registers[rs1].overflowing_add(imm).0 & 0xFFFFFFFFFFFFFFFE;
            }
            // BRANCH
            0b110_0011 => {
                // [31][7][30:25][11:8]0  ins
                //  12  11 10  5  4  1    target
                let imm = ((instruction & 80000000) as i32 >> 19) as u64
                    | ((instruction & 0x80) as u64) << 4
                    | (instruction & 0x7E000000) as u64 >> 20
                    | (instruction & 0xF00) as u64 >> 7;

                let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
                match funct3 {
                    // BEQ
                    0b000 => {
                        if self.registers[rs1] == self.registers[rs2] {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    // BNE
                    0b001 => {
                        if self.registers[rs1] != self.registers[rs2] {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    // BLT
                    0b100 => {
                        if (self.registers[rs1] as i64) < self.registers[rs2] as i64 {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    // BGE
                    0b101 => {
                        if self.registers[rs1] as i64 >= self.registers[rs2] as i64 {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    // BLTU
                    0b110 => {
                        if self.registers[rs1] < self.registers[rs2] {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    // BGEU
                    0b111 => {
                        if self.registers[rs1] >= self.registers[rs2] {
                            self.pc = (self.pc - 4).overflowing_add(imm).0
                        }
                    }
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // LOAD
            0b000_0011 => {
                let imm = i64::from((instruction & 0xfff_00000) as i32 >> 20) as u64; // sign extended immediate [31:20]
                let address = self.registers[rs1].overflowing_add(imm).0;

                self.registers[rd] = match funct3 {
                    // LB
                    0b000 => ((self.bus.load(address, 8)? as u8) as i8) as u64,
                    // LH
                    0b001 => ((self.bus.load(address, 16)? as u16) as i16) as u64,
                    // LW
                    0b010 => ((self.bus.load(dbg!(address), 32)? as u32) as i32) as u64,
                    // LBU
                    0b100 => self.bus.load(address, 8)?,
                    // LHU
                    0b101 => self.bus.load(address, 16)?,
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // STORE
            0b010_0011 => {
                let imm = i64::from((instruction & 0xFE00_0000) as i32 >> 20) as u64
                    | (instruction & 0xF80) as u64 >> 7; // sign extended immediate [31:25][11:7]
                let address = dbg!(self.registers[rs1]).overflowing_add(dbg!(imm)).0;

                match funct3 {
                    // SB
                    0b000 => self.bus.store(address, 8, self.registers[rs2])?,
                    // SH
                    0b001 => self.bus.store(address, 16, self.registers[rs2])?,
                    // SW
                    0b010 => self.bus.store(address, 32, self.registers[rs2])?,
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // OP-IMM
            0b001_0011 => {
                let imm = i64::from((instruction & 0xfff_00000) as i32 >> 20) as u64; // sign extended immediate [31:20]

                self.registers[rd] = match (funct7, funct3) {
                    // ADDI
                    (_, 0b000) => self.registers[rs1].wrapping_add(imm),
                    // SLTI
                    (_, 0b010) => u64::from((self.registers[rs1] as i64).lt(&(imm as i64))),
                    // SLTIU
                    (_, 0b011) => u64::from(self.registers[rs1].lt(&imm)),
                    // XORI
                    (_, 0b100) => self.registers[rs1].bitxor(imm),
                    // ORI
                    (_, 0b110) => self.registers[rs1].bitor(imm),
                    // ANDI
                    (_, 0b111) => self.registers[rs1].bitand(imm),
                    // SLLI (logical left shift)
                    (0b000_0000, 0b001) => self.registers[rs1].shl(rs2),
                    // SRLI (logical right shift)
                    (0b000_0000, 0b101) => self.registers[rs1].shr(rs2),
                    // SRAI (arithmetic right shift)
                    (0b010_0000, 0b101) => (self.registers[rs1] as i64).shr(rs2) as u64,
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // OP
            0b011_0011 => {
                self.registers[rd] = match (funct7, funct3) {
                    // ADD
                    (0b000_0000, 0b000) => self.registers[rs1].wrapping_add(self.registers[rs2]),
                    // SUB
                    (0b010_0000, 0b000) => self.registers[rs1].wrapping_sub(self.registers[rs2]),
                    // SLL (logical left shift)
                    (0b000_0000, 0b001) => self.registers[rs1].shl(self.registers[rs2] & 0b11111),
                    // SLT (rs1 < rs2 signed)
                    (0b000_0000, 0b010) => {
                        u64::from((self.registers[rs1] as i64).lt(&(self.registers[rs2] as i64)))
                    }
                    // SLTU (rs1 < rs2 unsigned)
                    (0b000_0000, 0b011) => {
                        u64::from((self.registers[rs1]).lt(&(self.registers[rs2])))
                    }
                    // XOR
                    (0b000_0000, 0b100) => self.registers[rs1].bitxor(self.registers[rs2]),
                    // SRL (logical right shift)
                    (0b000_0000, 0b101) => self.registers[rs1].shr(self.registers[rs2] & 0b11111),
                    // SRA (arithmetic right shift)
                    (0b010_0000, 0b101) => {
                        (self.registers[rs1] as i64).shr(self.registers[rs2] & 0b11111) as u64
                    }
                    // OR
                    (0b000_0000, 0b110) => self.registers[rs1].bitor(self.registers[rs2]),
                    // AND
                    (0b000_0000, 0b111) => self.registers[rs1].bitand(self.registers[rs2]),
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // MISC_MEM
            0b000_1111 if funct3 == 0b000 => todo!("FENCE (RV32I"),
            // SYSTEM
            0b111_0011 => {
                let bits_31_20 = ((instruction >> 20) & 0xFFF) as usize;
                match (bits_31_20, rs1, funct3, rd) {
                    // ECALL
                    (0b0000_0000_0000, 0b00000, 0b000, 0b00000) => todo!("ECALL (RV32I"),
                    // EBREAK
                    (0b0000_0000_0001, 0b00000, 0b000, 0b00000) => todo!("EBREAK (RV32I"),
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            _ => return Err(InstructionNotImplemented(instruction)),
        }

        Ok(())
    }
}

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
            InstructionNotImplemented(opcode) => {
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
pub struct RegisterDump {
    pc: Option<u64>,
    registers: [Option<u64>; 32],
}

#[cfg(test)]
impl RegisterDump {
    fn uninitialized() -> Self {
        RegisterDump {
            pc: None,
            registers: [None; 32],
        }
    }

    fn apply_mask(&mut self, mask: &RegisterDump) {
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

impl RegisterDump {
    fn new(pc: u64, register: &[u64; 32]) -> Self {
        RegisterDump {
            pc: Some(pc),
            registers: register.map(Some),
        }
    }
}

impl Debug for RegisterDump {
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

#[cfg(test)]
mod test {
    use std::fs;
    use std::num::ParseIntError;
    use std::str::FromStr;
    use std::time::SystemTime;

    use crate::bus::Bus;
    use crate::cpu::{CpuRV64I, RegisterDump};
    use crate::dram::Dram;

    #[test]
    fn test_cmp_casting() {
        assert_eq!(1, u64::from(69.lt(&420)));
        assert_eq!(0, u64::from(69.lt(&42)));
    }

    #[test]
    fn test_unsigned_signed_add() {
        let a: u64 = 100;
        let b: i64 = -5;
        let (c, _) = a.overflowing_add(b as u64);
        assert_eq!(95, c)
    }

    // TODO parse at compile time
    fn parse_testcase(testcase: &str) -> RegisterDump {
        fn parse_u64(s: &str) -> Result<u64, ParseIntError> {
            if let Some(s) = s.strip_prefix("0x") {
                u64::from_str_radix(s, 16)
            } else if let Some(s) = s.strip_prefix("0o") {
                u64::from_str_radix(s, 8)
            } else if let Some(s) = s.strip_prefix("0b") {
                u64::from_str_radix(s, 2)
            } else if let Some(s) = s.strip_prefix('-') {
                u64::from_str(s).map(|v| -i128::from(v) as u64)
            } else {
                u64::from_str(s)
            }
        }

        let comments = testcase
            .lines()
            .enumerate()
            .filter_map(|(line_n, l)| l.split_once('#').map(|(_, comment)| (line_n, comment)));

        let definitions = comments
            .clone()
            .filter_map(|(line_n, c)| c.split_once('=').map(|(k, v)| (line_n, k.trim(), v.trim())));

        let mut expected_regs = RegisterDump::uninitialized();

        for (line, k, v) in definitions {
            match k.strip_prefix('x') {
                Some(reg) => {
                    expected_regs.registers[usize::from_str(reg)
                        .unwrap_or_else(|_| panic!("Could not parse key {k} in line {line}"))] =
                        Some(parse_u64(v).unwrap_or_else(|e| {
                            panic!("Could not parse value {v} in line {line}: {e}")
                        }));
                }
                None => match k {
                    "pc" => {
                        expected_regs.pc = Some(parse_u64(v).unwrap_or_else(|e| {
                            panic!("Could not parse value {v} in line {line}: {e}")
                        }));
                    }
                    _ => {
                        panic!("Could not parse key {k} in line {line}")
                    }
                },
            }
        }

        expected_regs
    }

    /// test runner for instruction tests
    fn execute_insn_test(name: &str, testcase: &str, binary: &[u8]) {
        let mut cpu = CpuRV64I::new(Bus::new(Dram::with_code(binary)));

        loop {
            // were currently just waiting for the cpu to run into empty memory
            if cpu.cycle().is_err() {
                break;
            }
        }

        let mut actual_regs = cpu.dump_registers();

        let expected_regs = parse_testcase(testcase);

        actual_regs.apply_mask(&expected_regs);

        if actual_regs != expected_regs {
            let data = cpu.bus.get_mem().get_data();

            fs::write(
                format!(
                    "memdump_dram_test_{name}_{}.dump",
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Could not calculate current timestamp!")
                        .as_secs()
                ),
                data,
            )
            .expect("Could not write memory dump!");
            assert_eq!(
                actual_regs, expected_regs,
                "Actual Register values are not matching expected values defined in {name}.S!",
            );
        }
    }
    // include tests generated via build.rs
    include!(concat!(env!("OUT_DIR"), "/tests_insn.rs"));
}
