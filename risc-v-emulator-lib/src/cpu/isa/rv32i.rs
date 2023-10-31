use std::ops::Range;

use crate::cpu::isa::ext::float::FloatExt;
use crate::cpu::isa::{Cpu, RV32E, RV64I};
use crate::cpu::{CPUError, RegisterDump};
use crate::memory::{Bus, Dram, Memory};

macro_rules! impl_rv32i_exec {
    ($XLENU:ty, $XLENI:ty, $self:ident, $instruction:ident, $INSN_SIZE:expr) => {
        {
            use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

            let rd = (($instruction >> 7) & 0x1F) as usize;
            let rs1 = (($instruction >> 15) & 0x1F) as usize;
            let rs2 = (($instruction >> 20) & 0x1F) as usize;

            let opcode = $instruction & 0x7F; // opcode [6:0]

            let funct3 = (($instruction >> 12) & 0x7) as usize; // [14:12]
            let funct7 = (($instruction >> 25) & 0x7F) as usize; // [31:25]

            // set x0 to 0 to emulate x0 hardwired to all zeroes
            $self.registers[0] = 0;

            // decode and execute $instruction
            match opcode {
                _ if $instruction == 0xFFFF_FFFF => {
                    return Err(CPUError::Halt)
                }
                _ if $instruction == 0xFFFF_FFFE => {
                    println!(
                        "CPU info: ISA: {} bits={} reg_count={}",
                        $self.isa_id(),
                        <$XLENU>::BITS,
                        $self.registers.len()
                    );
                    println!("Dumping registers:\n{:?}", $self.dump_registers());
                }
                _ if $instruction == 0xFFFF_FFFD => {
                    let char = $self.registers[10] as u8 as char;
                    print!("{char}");
                }
                // LUI
                0b011_0111 => {
                    let imm = ($instruction & 0xFFFF_F000) as $XLENU;
                    $self.registers[rd] = imm;
                }
                // AUIPC
                0b001_0111 => {
                    let imm = ($instruction & 0xFFFF_F000) as $XLENU;
                    $self.registers[rd] = $self.pc + imm - $INSN_SIZE;
                }
                // JAL
                0b110_1111 => {
                    // [31][19:12][20][30:21]0  ins
                    //  20  19 12  11  10  1    target
                    let imm = (($instruction & 0x8000_0000) as i32 >> 11) as $XLENI as $XLENU // [31] -> [20]
                        | ($instruction & 0xF_F000) as $XLENU // [12:19] -> [12:19]
                        | ($instruction & 0x10_0000) as $XLENU >> 9
                        | ($instruction & 0x7FE0_0000) as $XLENU >> 20;

                    $self.registers[rd] = $self.pc;
                    $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0;
                }
                // JALR
                0b110_0111 if funct3 == 0b000 => {
                    let imm = (($instruction & 0xFFF0_0000) as i32 >> 20)
                        as $XLENI
                        as $XLENU; // sign extended immediate [31:20]

                    $self.registers[rd] = $self.pc;
                    $self.pc = $self.registers[rs1].overflowing_add(imm).0 & (<$XLENU>::max_value() << 1);
                }
                // BRANCH
                0b110_0011 => {
                    // [31][7][30:25][11:8]0  ins
                    //  12  11 10  5  4  1    target
                    let imm = (($instruction & 0x8000_0000) as i32 >> 19)
                        as $XLENI
                        as $XLENU
                        | (($instruction & 0x80) as $XLENU) << 4
                        | ($instruction & 0x7E00_0000) as $XLENU >> 20
                        | ($instruction & 0xF00) as $XLENU >> 7;

                    let funct3 = (($instruction >> 12) & 0x7) as usize; // [14:12]
                    match funct3 {
                        // BEQ
                        0b000 => {
                            if $self.registers[rs1] == $self.registers[rs2] {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        // BNE
                        0b001 => {
                            if $self.registers[rs1] != $self.registers[rs2] {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        // BLT
                        0b100 => {
                            if $self.registers[rs1] as $XLENI
                                < $self.registers[rs2] as $XLENI
                            {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        // BGE
                        0b101 => {
                            if $self.registers[rs1] as $XLENI
                                >= $self.registers[rs2] as $XLENI
                            {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        // BLTU
                        0b110 => {
                            if $self.registers[rs1] < $self.registers[rs2] {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        // BGEU
                        0b111 => {
                            if $self.registers[rs1] >= $self.registers[rs2] {
                                $self.pc = ($self.pc - $INSN_SIZE).overflowing_add(imm).0
                            }
                        }
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                // LOAD
                0b000_0011 => {
                    let imm = (($instruction & 0xFFF0_0000) as i32 >> 20)
                         as $XLENI
                         as $XLENU; // sign extended immediate [31:20]
                    let address = $self.registers[rs1].overflowing_add(imm).0;

                    $self.registers[rd] = match funct3 {
                        // LB
                        0b000 => $self.bus.load_i8(address)? as $XLENI as $XLENU,
                        // LH
                        0b001 => $self.bus.load_i16(address)? as $XLENI as $XLENU,
                        // LW
                        0b010 => $self.bus.load_i32(address)? as $XLENI as $XLENU,
                        // LBU
                        0b100 => $self.bus.load_u8(address)? as $XLENU,
                        // LHU
                        0b101 => $self.bus.load_u16(address)? as $XLENU,
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                // STORE
                0b010_0011 => {
                    let imm = (($instruction & 0xFE00_0000) as i32 >> 20) as $XLENI as $XLENU
                    | ($instruction & 0xF80) as $XLENU >> 7; // sign extended immediate [31:25][11:7]
                    let address = $self.registers[rs1].overflowing_add(imm).0;

                    match funct3 {
                        // SB
                        0b000 => $self.bus.store_u8(address, $self.registers[rs2] as u8)?,
                        // SH
                        0b001 => $self.bus.store_u16(address, $self.registers[rs2] as u16)?,
                        // SW
                        0b010 => $self.bus.store_u32(address, $self.registers[rs2] as u32)?,
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                // OP-IMM
                0b001_0011 => {
                    let imm = (($instruction & 0xFFF0_0000) as i32 >> 20)
                        as $XLENI
                        as $XLENU; // sign extended immediate [31:20]

                    $self.registers[rd] = match (funct7, funct3) {
                        // ADDI
                        (_, 0b000) => $self.registers[rs1].wrapping_add(imm),
                        // SLTI
                        (_, 0b010) => ($self.registers[rs1] as $XLENI).lt(&(imm as $XLENI)) as $XLENU,
                        // SLTIU
                        (_, 0b011) => $self.registers[rs1].lt(&imm) as $XLENU,
                        // XORI
                        (_, 0b100) => $self.registers[rs1].bitxor(imm),
                        // ORI
                        (_, 0b110) => $self.registers[rs1].bitor(imm),
                        // ANDI
                        (_, 0b111) => $self.registers[rs1].bitand(imm),
                        // SLLI (logical left shift)
                        (0b000_0000, 0b001) => $self.registers[rs1].shl(imm & 0x1F),
                        // SRLI (logical right shift)
                        (0b000_0000, 0b101) => $self.registers[rs1].shr(imm & 0x1F),
                        // SRAI (arithmetic right shift)
                        (0b010_0000, 0b101) => ($self.registers[rs1] as $XLENI).shr(imm & 0x1F) as $XLENU,
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                // OP
                0b011_0011 => {
                    $self.registers[rd] = match (funct7, funct3) {
                        // ADD
                        (0b000_0000, 0b000) => $self.registers[rs1].wrapping_add($self.registers[rs2]),
                        // SUB
                        (0b010_0000, 0b000) => $self.registers[rs1].wrapping_sub($self.registers[rs2]),
                        // SLL (logical left shift)
                        (0b000_0000, 0b001) => $self.registers[rs1]
                            .shl(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize),
                        // SLT (rs1 < rs2 signed)
                        (0b000_0000, 0b010) => ($self.registers[rs1]
                            as $XLENI)
                            .lt(&($self.registers[rs2] as $XLENI))
                            as $XLENU,
                        // SLTU (rs1 < rs2 unsigned)
                        (0b000_0000, 0b011) => ($self.registers[rs1])
                            .lt(&($self.registers[rs2]))
                            as $XLENU,
                        // XOR
                        (0b000_0000, 0b100) => $self.registers[rs1].bitxor($self.registers[rs2]),
                        // SRL (logical right shift)
                        (0b000_0000, 0b101) => $self.registers[rs1]
                            .shr(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize),
                        // SRA (arithmetic right shift)
                        (0b010_0000, 0b101) => ($self.registers[rs1] as $XLENI).shr(($self.registers[rs2] & 0b1_1111 as $XLENU) as usize) as $XLENU,
                        // OR
                        (0b000_0000, 0b110) => $self.registers[rs1].bitor($self.registers[rs2]),
                        // AND
                        (0b000_0000, 0b111) => $self.registers[rs1].bitand($self.registers[rs2]),
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                // MISC_MEM
                0b000_1111 if funct3 == 0b000 => todo!("FENCE (RV32I"),
                // SYSTEM
                0b111_0011 => {
                    let bits_31_20 = (($instruction >> 20) & 0xFFF) as usize;
                    match (bits_31_20, rs1, funct3, rd) {
                        // ECALL
                        (0b0000_0000_0000, 0b0_0000, 0b000, 0b0_0000) => todo!("ECALL (RV32I"),
                        // EBREAK
                        (0b0000_0000_0001, 0b0_0000, 0b000, 0b0_0000) => todo!("EBREAK (RV32I"),
                        _ => return Err(CPUError::InstructionNotImplemented($instruction)),
                    }
                }
                _ => return Err(CPUError::InstructionNotImplemented($instruction)),
            }

            Ok(())
        }
    };
}

pub struct RV32I {
    pc: u32,
    bus: Bus<u32>,
    registers: [u32; 32],
    _float_ext: Option<FloatExt>,
    dram_mapping: Range<u32>,
}

impl RV32I {
    #[inline]
    fn exec_rv32i(cpu: &mut Self, insn: u32, insn_len: u32) -> Result<(), CPUError<u32>> {
        impl_rv32i_exec!(u32, i32, cpu, insn, insn_len)
    }

    #[inline]
    pub(super) fn exec_rv32e(
        cpu: &mut RV32E,
        insn: u32,
        insn_len: u32,
    ) -> Result<(), CPUError<u32>> {
        impl_rv32i_exec!(u32, i32, cpu, insn, insn_len)
    }

    #[inline]
    pub(super) fn exec_rv64i(
        cpu: &mut RV64I,
        insn: u32,
        insn_len: u64,
    ) -> Result<(), CPUError<u64>> {
        impl_rv32i_exec!(u64, i64, cpu, insn, insn_len)
    }
}

impl Cpu<u32, 32> for RV32I {
    const ISA_ID: &'static str = "RV32I";

    fn new(bus: Bus<u32>, dram_mapping: Range<u32>, float_ext: Option<FloatExt>) -> Self {
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

        let dram: Dram<u32> = Dram::<u32>::with_code(code, DRAM_SIZE as u32);

        Self::new(
            Bus::<u32>::new(vec![(
                DRAM_BASE as u32..(DRAM_BASE + DRAM_SIZE) as u32,
                Box::new(dram),
            )]),
            (DRAM_BASE as u32)..(DRAM_BASE + DRAM_SIZE) as u32,
            float_ext,
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
        Self::exec_rv32i(self, instruction, 4)
    }

    fn fetch(&self) -> Result<u32, CPUError<u32>> {
        self.bus.load_u32(self.pc)
    }

    fn reset(&mut self) {
        self.pc = self.dram_mapping.start;
        self.registers[2] = self.dram_mapping.end; // set sp to
    }

    fn dump_registers(&self) -> RegisterDump<u32, 32> {
        RegisterDump::new(self.pc, &self.registers)
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.bus.get_data(self.dram_mapping.clone()).unwrap()
    }
}
