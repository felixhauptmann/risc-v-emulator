use crate::cpu::isa::As;
use crate::cpu::isa::Isa;
use crate::cpu::{CPUError, Cpu};
use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::{AsPrimitive, Bounded, PrimInt, WrappingAdd, WrappingSub, Zero};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

#[derive(PartialEq)]
pub struct RV32I;

impl Isa<32> for RV32I {
    type XlenU = u32;
    type XlenI = i32;
    const ISA_ID: &'static str = "RV32I";

    const INSN_SIZE: Self::XlenU = 4;
    const REG_COUNT: usize = 32;

    fn exec<const REG_COUNT: usize, I: Isa<REG_COUNT>>(
        cpu: &mut Cpu<I, REG_COUNT>,
        instruction: u32,
    ) -> Result<(), CPUError>
    where
        bool: AsPrimitive<I::XlenU>,
        u8: AsPrimitive<I::XlenU>,
        i8: AsPrimitive<I::XlenU>,
        u16: AsPrimitive<I::XlenU>,
        u32: AsPrimitive<I::XlenU>,
        i32: AsPrimitive<I::XlenU>,
        i8: AsPrimitive<I::XlenI>,
        i16: AsPrimitive<I::XlenI>,
        u32: AsPrimitive<I::XlenI>,
        i32: AsPrimitive<I::XlenI>,
        I::XlenU: AsPrimitive<u8>,
        I::XlenU: AsPrimitive<u16>,
        I::XlenU: AsPrimitive<u32>,
    {
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;

        let opcode = instruction & 0x7F; // opcode [6:0]

        let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
        let funct7 = ((instruction >> 25) & 0x7F) as usize; // [31:25]

        // set x0 to 0 to emulate x0 hardwired to all zeroes
        cpu.registers[0] = I::XlenU::zero();

        // decode and execute instruction
        match opcode {
            _ if instruction == 0xFFFF_FFFF => {
                println!(
                    "ISA: {} bits={} insn_size={} reg_count={}",
                    I::ISA_ID,
                    I::XlenU::max_value().count_ones(),
                    I::INSN_SIZE.as_t::<usize>(),
                    I::REG_COUNT
                )
            }
            // LUI
            0b011_0111 => {
                let imm = (instruction & 0xFFFF_F000).as_t::<I::XlenU>(); // TODO sign extend for 64bit?
                cpu.registers[rd] = imm;
            }
            // AUIPC
            0b001_0111 => {
                let imm = (instruction & 0xFFFF_F000).as_t::<I::XlenU>(); // TODO sign extend for 64bit?
                cpu.registers[rd] = cpu.pc + imm - I::INSN_SIZE;
            }
            // JAL
            0b110_1111 => {
                // [31][19:12][20][30:21]0  ins
                //  20  19 12  11  10  1    target
                let imm = ((instruction & 0x8000_0000) as i32 >> 11).as_t::<I::XlenI>().as_t::<I::XlenU>() // [31] -> [20]
                    | (instruction & 0xF_F000).as_t::<I::XlenU>() // [12:19] -> [12:19]
                    | (instruction & 0x10_0000).as_t::<I::XlenU>() >> 9
                    | (instruction & 0x7FE0_0000).as_t::<I::XlenU>() >> 20;

                cpu.registers[rd] = cpu.pc;
                cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0;
            }
            // JALR
            0b110_0111 if funct3 == 0b000 => {
                let imm = ((instruction & 0xFFF0_0000) as i32 >> 20)
                    .as_t::<I::XlenI>()
                    .as_t::<I::XlenU>(); // sign extended immediate [31:20]

                cpu.registers[rd] = cpu.pc;
                cpu.pc = cpu.registers[rs1].overflowing_add(&imm).0 & (I::XlenU::max_value() << 1);
            }
            // BRANCH
            0b110_0011 => {
                // [31][7][30:25][11:8]0  ins
                //  12  11 10  5  4  1    target
                let imm = ((instruction & 0x8000_0000) as i32 >> 19)
                    .as_t::<I::XlenI>()
                    .as_t::<I::XlenU>()
                    | ((instruction & 0x80).as_t::<I::XlenU>()) << 4
                    | (instruction & 0x7E00_0000).as_t::<I::XlenU>() >> 20
                    | (instruction & 0xF00).as_t::<I::XlenU>() >> 7;

                let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
                match funct3 {
                    // BEQ
                    0b000 => {
                        if cpu.registers[rs1] == cpu.registers[rs2] {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    // BNE
                    0b001 => {
                        if cpu.registers[rs1] != cpu.registers[rs2] {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    // BLT
                    0b100 => {
                        if cpu.registers[rs1].as_t::<I::XlenI>()
                            < cpu.registers[rs2].as_t::<I::XlenI>()
                        {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    // BGE
                    0b101 => {
                        if cpu.registers[rs1].as_t::<I::XlenI>()
                            >= cpu.registers[rs2].as_t::<I::XlenI>()
                        {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    // BLTU
                    0b110 => {
                        if cpu.registers[rs1] < cpu.registers[rs2] {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    // BGEU
                    0b111 => {
                        if cpu.registers[rs1] >= cpu.registers[rs2] {
                            cpu.pc = (cpu.pc - I::INSN_SIZE).overflowing_add(&imm).0
                        }
                    }
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            // LOAD
            0b000_0011 => {
                let imm = ((instruction & 0xFFF0_0000) as i32 >> 20)
                    .as_t::<I::XlenI>()
                    .as_t::<I::XlenU>(); // sign extended immediate [31:20]
                let address = cpu.registers[rs1].overflowing_add(&imm).0;

                cpu.registers[rd] = match funct3 {
                    // LB
                    0b000 => cpu
                        .bus
                        .load::<i8>(address.as_t::<usize>())?
                        .as_t::<I::XlenI>()
                        .as_t::<I::XlenU>(),
                    // LH
                    0b001 => cpu
                        .bus
                        .load::<i16>(address.as_t::<usize>())?
                        .as_t::<I::XlenI>()
                        .as_t::<I::XlenU>(),
                    // LW
                    0b010 => cpu
                        .bus
                        .load::<i32>(address.as_t::<usize>())?
                        .as_t::<I::XlenI>()
                        .as_t::<I::XlenU>(),
                    // LBU
                    0b100 => cpu
                        .bus
                        .load::<u8>(address.as_t::<usize>())?
                        .as_t::<I::XlenU>(),
                    // LHU
                    0b101 => cpu
                        .bus
                        .load::<u16>(address.as_t::<usize>())?
                        .as_t::<I::XlenU>(),
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            // STORE
            0b010_0011 => {
                let imm = ((instruction & 0xFE00_0000) as i32 >> 20)
                    .as_t::<I::XlenI>()
                    .as_t::<I::XlenU>()
                    | (instruction & 0xF80).as_t::<I::XlenU>() >> 7; // sign extended immediate [31:25][11:7]
                let address = cpu.registers[rs1].overflowing_add(&imm).0;

                match funct3 {
                    // SB
                    0b000 => cpu
                        .bus
                        .store::<u8>(address.as_t::<usize>(), cpu.registers[rs2].as_t::<u8>())?,
                    // SH
                    0b001 => cpu
                        .bus
                        .store::<u16>(address.as_t::<usize>(), cpu.registers[rs2].as_t::<u16>())?,
                    // SW
                    0b010 => cpu
                        .bus
                        .store::<u32>(address.as_t::<usize>(), cpu.registers[rs2].as_t::<u32>())?,
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            // OP-IMM
            0b001_0011 => {
                let imm = ((instruction & 0xFFF0_0000) as i32 >> 20)
                    .as_t::<I::XlenI>()
                    .as_t::<I::XlenU>(); // sign extended immediate [31:20]

                cpu.registers[rd] = match (funct7, funct3) {
                    // ADDI
                    (_, 0b000) => cpu.registers[rs1].wrapping_add(&imm),
                    // SLTI
                    (_, 0b010) => cpu.registers[rs1]
                        .as_t::<I::XlenI>()
                        .lt(&imm.as_t::<I::XlenI>())
                        .as_t::<I::XlenU>(),
                    // SLTIU
                    (_, 0b011) => cpu.registers[rs1].lt(&imm).as_t::<I::XlenU>(),
                    // XORI
                    (_, 0b100) => cpu.registers[rs1].bitxor(imm),
                    // ORI
                    (_, 0b110) => cpu.registers[rs1].bitor(imm),
                    // ANDI
                    (_, 0b111) => cpu.registers[rs1].bitand(imm),
                    // SLLI (logical left shift)
                    (0b000_0000, 0b001) => cpu.registers[rs1].shl(rs2),
                    // SRLI (logical right shift)
                    (0b000_0000, 0b101) => cpu.registers[rs1].shr(rs2),
                    // SRAI (arithmetic right shift)
                    (0b010_0000, 0b101) => cpu.registers[rs1]
                        .as_t::<I::XlenI>()
                        .shr(rs2)
                        .as_t::<I::XlenU>(),
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            // OP
            0b011_0011 => {
                cpu.registers[rd] = match (funct7, funct3) {
                    // ADD
                    (0b000_0000, 0b000) => cpu.registers[rs1].wrapping_add(&cpu.registers[rs2]),
                    // SUB
                    (0b010_0000, 0b000) => cpu.registers[rs1].wrapping_sub(&cpu.registers[rs2]),
                    // SLL (logical left shift)
                    (0b000_0000, 0b001) => cpu.registers[rs1]
                        .shl((cpu.registers[rs2] & 0b1_1111.as_t::<I::XlenU>()).as_t::<usize>()),
                    // SLT (rs1 < rs2 signed)
                    (0b000_0000, 0b010) => cpu.registers[rs1]
                        .as_t::<I::XlenI>()
                        .lt(&cpu.registers[rs2].as_t::<I::XlenI>())
                        .as_t::<I::XlenU>(),
                    // SLTU (rs1 < rs2 unsigned)
                    (0b000_0000, 0b011) => (cpu.registers[rs1])
                        .lt(&(cpu.registers[rs2]))
                        .as_t::<I::XlenU>(),
                    // XOR
                    (0b000_0000, 0b100) => cpu.registers[rs1].bitxor(cpu.registers[rs2]),
                    // SRL (logical right shift)
                    (0b000_0000, 0b101) => cpu.registers[rs1]
                        .shr((cpu.registers[rs2] & 0b1_1111.as_t::<I::XlenU>()).as_t::<usize>()),
                    // SRA (arithmetic right shift)
                    (0b010_0000, 0b101) => cpu.registers[rs1]
                        .as_t::<I::XlenI>()
                        .shr((cpu.registers[rs2] & 0b1_1111.as_t::<I::XlenU>()).as_t::<usize>())
                        .as_t::<I::XlenU>(),
                    // OR
                    (0b000_0000, 0b110) => cpu.registers[rs1].bitor(cpu.registers[rs2]),
                    // AND
                    (0b000_0000, 0b111) => cpu.registers[rs1].bitand(cpu.registers[rs2]),
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            // MISC_MEM
            0b000_1111 if funct3 == 0b000 => todo!("FENCE (RV32I"),
            // SYSTEM
            0b111_0011 => {
                let bits_31_20 = ((instruction >> 20) & 0xFFF) as usize;
                match (bits_31_20, rs1, funct3, rd) {
                    // ECALL
                    (0b0000_0000_0000, 0b0_0000, 0b000, 0b0_0000) => todo!("ECALL (RV32I"),
                    // EBREAK
                    (0b0000_0000_0001, 0b0_0000, 0b000, 0b0_0000) => todo!("EBREAK (RV32I"),
                    _ => return Err(CPUError::InstructionNotImplemented(instruction)),
                }
            }
            _ => return Err(CPUError::InstructionNotImplemented(instruction)),
        }

        Ok(())
    }
}
