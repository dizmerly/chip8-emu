#![allow(non_snake_case)]
use minifb::{
    Icon::Buffer,
    Key::{self, Key2},
    MouseMode::Pass,
    Scale, ScaleMode, Window, WindowOptions,
};
mod stack;
use crate::stack::Stack;
mod cpu; 
use crate::cpu::CPU; 

const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const KEYPAD: [Key; 16] = [
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
];

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn draw_pixel(x_coordinate: u32, y_coordinate: u32) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];

    // position x coordinate
    // position y coordinate
    buffer[y_coordinate as usize * DISPLAY_WIDTH + x_coordinate as usize] =
        from_u8_rgb(255, 255, 255);
    return buffer;
}

fn main() {
    // Program counter - pointer to current instruction memory
    let cpu = CPU::new(); 
    let I: [u16; 16] = [0; 16];
    let mut stack = Stack {
        items: [0; 16],
        size: 0,
    };
    // Stack pointer
    let mut SP: u8 = 0;

    let starting_address: u16 = 200;

    // Other necessary specs
    // 8 bit delay timer
    let mut delay_timer: usize = 0;
    // 8 bit sound timer
    let mut sound_timer: usize = 0;
    // 16, 8 bit varaible registers
    let V0: [u8; 16] = [0; 16];
    let V1: [u8; 16] = [0; 16];
    let V2: [u8; 16] = [0; 16];
    let V3: [u8; 16] = [0; 16];
    let V4: [u8; 16] = [0; 16];
    let V5: [u8; 16] = [0; 16];
    let V6: [u8; 16] = [0; 16];
    let V7: [u8; 16] = [0; 16];
    let V8: [u8; 16] = [0; 16];
    let V9: [u8; 16] = [0; 16];
    let VA: [u8; 16] = [0; 16];
    let VB: [u8; 16] = [0; 16];
    let VC: [u8; 16] = [0; 16];
    let VD: [u8; 16] = [0; 16];
    let VE: [u8; 16] = [0; 16];
    // Flag register
    let VF: [u8; 16] = [0; 16];

    let mut buffer: Vec<u32> = vec![from_u8_rgb(0, 0, 0); DISPLAY_WIDTH * DISPLAY_HEIGHT];
    for i in 0..20 {
        for j in 0..20 {
            buffer[i * DISPLAY_WIDTH + j] = from_u8_rgb(255, 255, 255);
        }
    }

    let mut options = WindowOptions::default();
    options.scale = Scale::X8;
    let mut window = match Window::new("CHIP-8 Emulator", DISPLAY_WIDTH, DISPLAY_HEIGHT, options) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    
    let mut pos = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // fetch instruction from PC (2 bytes)
         
        // Decode instruction from PC
        
        // Execute instruction



        
        let buffer = draw_pixel(pos * 2, pos);
        window.set_target_fps(60);
        window
            .update_with_buffer(&buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .expect("Could not update window with buffer");
        pos += 1;
    }
}
