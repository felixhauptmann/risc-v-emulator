use crate::bus::{Bus, DRAM_BASE};
use crate::cpu::CPUError::InstructionNotImplemented;
use crate::dram::DRAM_SIZE;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

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
            0b0110111 => todo!("LUI (RV32I)"),
            // AUIPC
            0b0010111 => todo!("AUIPC (RV32I)"),
            // JAL
            0b1101111 => todo!("JAL (RV32I)"),
            // JALR
            0b1100111 if funct3 == 0b000 => todo!("JALR (RV32I"),
            // BRANCH
            0b1100011 => {
                let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
                match funct3 {
                    // BEQ
                    0b000 => todo!("BEQ (RV32I"),
                    // BNE
                    0b001 => todo!("BNE (RV32I"),
                    // BLT
                    0b100 => todo!("BLT (RV32I"),
                    // BGE
                    0b101 => todo!("BGE (RV32I"),
                    // BLTU
                    0b110 => todo!("BLTU (RV32I"),
                    // BGEU
                    0b111 => todo!("BGEU (RV32I"),
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // LOAD
            0b0000011 => match funct3 {
                // LB
                0b000 => todo!("LB (RV32I"),
                // LH
                0b001 => todo!("LH (RV32I"),
                // LW
                0b010 => todo!("LW (RV32I"),
                // LBU
                0b100 => todo!("LBU (RV32I"),
                // LHU
                0b101 => todo!("LHU (RV32I"),
                _ => return Err(InstructionNotImplemented(instruction)),
            },
            // STORE
            0b0100011 => match funct3 {
                // SB
                0b000 => todo!("SB (RV32I"),
                // SH
                0b001 => todo!("SH (RV32I"),
                // SW
                0b010 => todo!("SW (RV32I"),
                _ => return Err(InstructionNotImplemented(instruction)),
            },
            // OP-IMM
            0b0010011 => {
                let imm = ((instruction & 0xfff00000) as i32 as i64 >> 20) as u64; // sign extended immediate

                match (funct7, funct3) {
                    // ADDI
                    (_, 0b000) => self.registers[rd] = self.registers[rs1].wrapping_add(imm),
                    // SLTI
                    (_, 0b010) => todo!("SLTI (RV32I"),
                    // SLTIU
                    (_, 0b011) => todo!("SLTIU (RV32I"),
                    // XORI
                    (_, 0b100) => todo!("XORI (RV32I"),
                    // ORI
                    (_, 0b110) => todo!("ORI (RV32I"),
                    // ANDI
                    (_, 0b111) => todo!("ANDI (RV32I"),
                    // SLLI
                    (0b0000000, 0b001) => todo!("SLLI (RV32I"),
                    // SRLI
                    (0b0000000, 0b101) => todo!("SRLI (RV32I"),
                    // SRAI
                    (0b0100000, 0b101) => todo!("SRAI (RV32I"),

                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // OP
            0b0110011 => {
                self.registers[rd] = match (funct7, funct3) {
                    // ADD
                    (0b0000000, 0b000) => self.registers[rs1].wrapping_add(self.registers[rs2]),
                    // SUB
                    (0b0100000, 0b000) => self.registers[rs1].wrapping_sub(self.registers[rs2]),
                    // SLL (logical left shift)
                    (0b0000000, 0b001) => self.registers[rs1].shl(self.registers[rs2] & 0b11111),
                    // SLT (rs1 < rs2 signed)
                    (0b0000000, 0b010) => {
                        (self.registers[rs1] as i64).lt(&(self.registers[rs2] as i64)) as u64
                    }
                    // SLTU (rs1 < rs2 unsigned)
                    (0b0000000, 0b011) => (self.registers[rs1]).lt(&(self.registers[rs2])) as u64,
                    // XOR
                    (0b0000000, 0b100) => self.registers[rs1].bitxor(self.registers[rs2]),
                    // SRL (logical right shift)
                    (0b0000000, 0b101) => self.registers[rs1].shr(self.registers[rs2] & 0b11111),
                    // SRA (arithmetic right shift)
                    (0b0100000, 0b101) => {
                        (self.registers[rs1] as i64).shr(self.registers[rs2] & 0b11111) as u64
                    }
                    // OR
                    (0b0000000, 0b110) => self.registers[rs1].bitor(self.registers[rs2]),
                    // AND
                    (0b0000000, 0b111) => self.registers[rs1].bitand(self.registers[rs2]),
                    _ => return Err(InstructionNotImplemented(instruction)),
                }
            }
            // MISC_MEM
            0b0001111 if funct3 == 0b000 => todo!("FENCE (RV32I"),
            // SYSTEM
            0b1110011 => {
                let bits_31_20 = ((instruction >> 20) & 0xFFF) as usize;
                match (bits_31_20, rs1, funct3, rd) {
                    // ECALL
                    (0b000000000000, 0b00000, 0b000, 0b00000) => todo!("ECALL (RV32I"),
                    // EBREAK
                    (0b000000000001, 0b00000, 0b000, 0b00000) => todo!("EBREAK (RV32I"),
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
                Some(v) => (format!("{v:#010X}"), v.to_string()),
            };
            writeln!(f, "x{i:<#2} {:<#4}: {v_hex} {v}", ABI[i])?;
        }
        writeln!(f, "------------ bye :) -------------")
    }
}

#[cfg(test)]
mod test {
    use crate::bus::Bus;
    use crate::cpu::{CpuRV64I, RegisterDump};
    use crate::dram::Dram;
    use std::fmt::format;
    use std::str::FromStr;

    #[test]
    fn test_cmp_casting() {
        assert_eq!(1, 69.lt(&420) as u64);
        assert_eq!(0, 69.lt(&42) as u64);
    }

    // TODO parse at compile time
    fn parse_expected(testcase: &str) -> (RegisterDump, Option<String>) {
        let comments = testcase
            .lines()
            .enumerate()
            .filter_map(|(line_n, l)| l.split_once('#').map(|(_, comment)| (line_n, comment)));

        let definitions = comments
            .clone()
            .filter_map(|(line_n, c)| c.split_once('=').map(|(k, v)| (line_n, k.trim(), v.trim())));

        let meta = comments
            .filter_map(|(line_n, c)| c.split_once(':').map(|(k, v)| (line_n, k.trim(), v.trim())));

        let mut expected = RegisterDump::uninitialized();

        for (line, k, v) in definitions {
            match k.strip_prefix('x') {
                Some(reg) => {
                    expected.registers[usize::from_str(reg)
                        .unwrap_or_else(|_| panic!("Could not parse key {k} in line {line}"))] =
                        Some(
                            u64::from_str(v).unwrap_or_else(|_| {
                                panic!("Could not parse value {v} in line {line}")
                            }),
                        )
                }
                None => match k {
                    "pc" => {
                        expected.pc =
                            Some(u64::from_str(v).unwrap_or_else(|_| {
                                panic!("Could not parse value {v} in line {line}")
                            }))
                    }
                    _ => {
                        panic!("Could not parse key {k} in line {line}")
                    }
                },
            }
        }

        let mut comment = None;
        for (line, k, v) in meta {
            if k == "comment" {
                comment = Some(v.to_string());
            }
        }

        (expected, comment)
    }

    /// test runner for instruction tests
    fn execute_insn_test(name: &str, testcase: &str, binary: &[u8]) {
        let (expected, comment) = parse_expected(testcase);
        let comment = comment.map_or("".to_string(), |c| format!(" [{c}]"));

        let mut cpu = CpuRV64I::new(Bus::new(Dram::new(binary.to_vec())));

        loop {
            if let Err(e) = cpu.cycle() {
                eprintln!("Error: {e} Dumping registers:");
                break;
            }
        }

        let mut actual = cpu.dump_registers();

        actual.apply_mask(&expected);

        assert_eq!(
            expected, actual,
            "Actual Register values are not matching expected values defined in {name}!{comment}",
        )
    }
    // include tests generated via build.rs
    include!(concat!(env!("OUT_DIR"), "/tests_insn.rs"));
}
