pub struct Cpu64I {
    registers: [u64; 32],
    pub(crate) pc: u64,
    pub(crate) code: Vec<u8>, // TODO remove (this has to be replaced by implementation via bus and dram module)
}

impl Cpu64I {
    pub fn new(mem_size: u64) -> Self {
        let mut cpu = Cpu64I {
            registers: [0; 32],
            pc: 0,
            code: vec![0; mem_size as usize],
        };

        cpu.registers[2] = mem_size;

        cpu
    }

    pub fn dump_registers(&self) {
        /*
           pc:     0x12abc

           x0:     0x00
           x1:     0x00
        */
        println!("pc:\t{:#010x}\n", self.pc);
        for (i, reg) in self.registers.iter().enumerate() {
            println!(
                "x{i}:\t{reg:#010x} {reg}{}",
                if i == 2 { " (SP)" } else { "" }
            );
        }
    }

    pub fn cycle(&mut self) {
        // set x0 to 0
        self.registers[0] = 0;

        // fetch
        let instruction = self.fetch();

        // increment pc
        self.pc += 4;

        // decode
        // execute
        self.execute(instruction);
    }
}

impl Cpu64I {
    fn fetch(&self) -> u32 {
        // Read 32-bit instruction from memory (little endian)
        let ins = &self.code[self.pc as usize..self.pc as usize + 4];
        u32::from_le_bytes(ins.try_into().unwrap())
    }

    fn execute(&mut self, instruction: u32) {
        // Decode an instruction and execute it.

        let opcode = instruction & 0x7f;
        let rd = ((instruction >> 7) & 0x1f) as usize;
        let funct3 = ((instruction >> 12) & 0x7) as usize;
        let rs1 = ((instruction >> 15) & 0x1f) as usize;
        let rs2 = ((instruction >> 20) & 0x1f) as usize;

        let imm = ((instruction & 0xfff00000) as i32 as i64 >> 20) as u64; // sign extended immediate

        match opcode {
            0x00 => {}
            0x13 => {
                // addi
                self.registers[rd] = self.registers[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.registers[rd] = self.registers[rs1].wrapping_add(self.registers[rs2]);
            }
            _ => {
                self.dump_registers();
                panic!("Not Implemented!")
            }
        }
    }
}
