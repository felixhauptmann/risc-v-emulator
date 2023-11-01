use crate::cpu::CPUError;
use crate::cpu::isa::ext::IsaExt;
use crate::cpu::isa::RV32I;

pub struct M;

impl IsaExt<u32, 32, RV32I> for M {
    const ISA_ID: &'static str = "M";

    fn execute(cpu: &mut RV32I, instruction: u32) -> Result<(), CPUError<u32>> {

        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;

        let opcode = instruction & 0x7F; // opcode [6:0]

        let funct3 = ((instruction >> 12) & 0x7) as usize; // [14:12]
        let funct7 = ((instruction >> 25) & 0x7F) as usize; // [31:25]

        if opcode == 0b011_0011 && funct7 == 0b000_0001 {
            cpu.registers[rd] = match funct3 {
                0b000 => {
                    cpu.registers[rs1].overflowing_mul(cpu.registers[rs2]).0
                }
                0b011 => {
                    ((cpu.registers[rs1] as u64 * cpu.registers[rs2] as u64) >> 32) as u32
                }
                _ => return Err(CPUError::InstructionNotImplemented(instruction))
            }
        } else {
            return Err(CPUError::InstructionNotImplemented(instruction))
        }
        println!("M did the trick");
        Ok(())
    }
}