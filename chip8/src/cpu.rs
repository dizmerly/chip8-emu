#![allow(non_snake_case)]
const MEMORY_SIZE: usize = 4096;

pub struct CPU {
    pub memory: [u16; MEMORY_SIZE],
    pub pc: u16,
}

impl CPU {

    pub fn new() -> Self {
        Self { 
            memory: [0; MEMORY_SIZE], 
            pc: 0x200, 
        }
    }

    // Takes in 2 bytes of memory instruction
    // 
    // Increments program counter by 2. 
    pub fn fetch(&mut self) -> Option<u16> {
        if self.pc >= MEMORY_SIZE as u16 - 1 {
            return None;
        }
        let opcode = self.memory[self.pc as usize] << 8 | self.memory[(self.pc + 1) as usize] << 8;
        self.pc += 2; 
        Some(opcode)
    }

    pub fn decode(&mut self, opcode: u16) {
        let instruction = (opcode >> 12) & 0x000F;  
        let x = (opcode >> 8) & 0x000F;  
        let y = (opcode >> 4) & 0x000F;

        
    }

}
