// Stack implementation
// array of 16, 16 bit unsigned integers
// size in usize
pub struct Stack {
    items: [u16; 16],
    size: usize,
}

impl Stack {
    // Pushes item to the back of the stack
    //
    // Doesn't return anything
    pub fn push(&mut self, value: u16) {
        self.items[self.size] = value;
        self.size += 1;
        return;
    }
    // Pops from the top of the stack
    //
    // Returns what it pops as a u32
    fn pop(&mut self) {
        return;
    }
}
