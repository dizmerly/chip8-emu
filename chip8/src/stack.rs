use minifb::Key::Delete;

const MAX_SIZE: usize = 16; 

// Stack implementation
// array of 16, 16 bit unsigned integers
// size in usize
pub struct Stack {
    pub items: [u16; 16],
    pub size: usize,
}

impl Stack {
    // Pushes item to the back of the stack
    //
    // Doesn't return anything
    pub fn push(&mut self, value: u16) {
        if self.size >= MAX_SIZE {
            panic!("Can't push more than 16 items onto the stack!"); 
        }
        self.items[self.size] = value;
        self.size += 1;
        return;
    }
    // Pops from the top of the stack
    //
    // Returns what it pops as a u32
    fn pop(&mut self) -> Option<u16> {
        if self.size < 1 {
            return None; 
        }
        let popped_item = self.items[self.size - 1]; 
        self.items[self.size - 1] = 0; 
        self.size -= 1;
        return Some(popped_item);
    }
}
