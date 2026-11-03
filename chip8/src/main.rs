use minifb::{Icon::Buffer, Key, Scale, ScaleMode, Window, WindowOptions};
#[allow(non_snake_case)]

const MEMORY_SIZE: usize = 4096;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;
fn display(buffer: Vec<u32>, window: &mut Window ) {
    window.set_target_fps(60);
    window
        .update_with_buffer(&buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .expect("Could not update window with buffer");
}

fn main() {
    // Program counter - pointer to current instruction memory
    let mut PC: &u16 = &0;
    let I: [u16; 16] = [0; 16];
    let mut stack: [u16; 16] = [0; 16];

    let starting_address: u16 = 200;

    // Other necessary specs
    // 8 bit delay timer

    // 8 bit sound timer

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

    let buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];

    // display(buffer);

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32); 
        (r << 16) | (g << 8) | b
    }

    let mut buffer: Vec<u32> = vec![from_u8_rgb(0, 0, 0); DISPLAY_WIDTH * DISPLAY_HEIGHT];
    for i in 0..20 {
        for j in 0..20 {
            buffer[i * DISPLAY_WIDTH + j] = from_u8_rgb(255,255,255);
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
    
    while window.is_open() && !window.is_key_down(Key::Escape){
        display(buffer.clone(), &mut window);
    }
}
