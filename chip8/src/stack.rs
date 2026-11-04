struct Stack {
    items: [u16; 16],
    size: usize,
}

fn push(value: u16, mut stack: Stack) {
    stack.items[stack.size] = value;
    stack.size += 1; 
    return; 
    
}
fn pop(stack: Stack) { 
    return; 
}

