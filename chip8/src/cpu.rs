#![allow(non_snake_case)]
const MEMORY_SIZE: usize = 4096;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32; 

enum Instruction { 
    ClearScreen, 
    JumpToNNN {addr: u16}, 
    SetRegisterVX {reg: u16, value: u16},
    AddValueToRegisterVX {reg: u16, value: u16},
    SetIndexRegisterI {set_value: u16},
    Draw {x: u8, y: u8, n: u8} 
}

pub struct Display { 
    cleared_flag: bool,
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}

pub struct CPU {
    memory: [u16; MEMORY_SIZE],
    pc: u16,
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
        let nibbles = (
            (opcode >> 12) & 0x000F,
            (opcode >> 8) & 0x000F,
            (opcode >> 4) & 0x000F,
            opcode & 0x000F
        );
        
        let instruction = nibbles.0;  
        let X = nibbles.1;  
        let Y = nibbles.2;
        let N = nibbles.3;
        let NN = opcode & 0x00FF; 
        let NNN = opcode & 0x0FFF; 
        
        match instruction {            
            0xE => {
                println!("Clearing screen");
                self.execute(Instruction::ClearScreen);
            }
            0x1=> {
                println!("Jump to NNN");
                self.execute(Instruction::JumpToNNN { addr: (NNN) });
            }
            0x6 => {
                println!("Set register VX");
                self.execute(Instruction::SetRegisterVX { reg: (X), value: (NN) });
            }
            0x7 => {
                println!("Add value to register VX");
                self.execute(Instruction::AddValueToRegisterVX { reg: (X), value: (NN) });
            }
            0xA => {
                println!("Set index register I");
                self.execute(Instruction::SetIndexRegisterI { set_value: (NNN) });
            }
            0xD => {
                println!("Draw");
                // self.execute(Instruction::Draw { x: (X), y: (Y), n: (N) });
            }

            0 => panic!("No instruction parsed!"),
            _ => panic!("Instruction not implemented!")
            
        };
    }
    
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen =>{
                
            }
            Instruction::JumpToNNN { addr } =>{
                println!("")
            }
            Instruction::SetRegisterVX { reg, value } =>{
                println!("")
            }
            Instruction::AddValueToRegisterVX { reg, value } =>{
                println!("")
            }
            Instruction::SetIndexRegisterI { set_value } =>{
                println!("")
            }
            Instruction::Draw { x, y, n } => {
                println!("")
            }
            
            _ => panic!("Unknown instruction")
            
        }
        
    }
}
