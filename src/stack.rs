use std::fmt;


pub struct Stack {
    space: Vec<u32>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            space: Vec::new()
        }
    }
    pub fn push(&mut self, st: u32) {
        self.space.push(st);
    }
    pub fn pop(&mut self) -> u32 {
        self.space.pop().unwrap()
    }
    pub fn peek(&mut self) -> u32 {
        *self.space.last().unwrap()
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.space)
    }
}


#[test]
fn test_stack_init() {
    let stack = Stack::new();
    assert!(stack.space.len() == 0);
}

#[test]
fn test_stack_push() {
    let mut stack = Stack::new();
    stack.push(10);
    assert!(stack.space.len() == 1);
    assert!(stack.space[0] == 10);
}

#[test]
fn test_stack_pull() {
    let mut stack = Stack::new();
    stack.space.push(111);
    stack.space.push(222);
    assert!(stack.pop() + stack.pop() == 333);
    assert!(stack.space.len() == 0);
}

#[test]
fn test_stack_peek() {
    let mut stack = Stack::new();
    stack.space.push(111);
    stack.space.push(222);
    assert!(stack.peek() == 222);
    assert!(stack.space.len() == 2);
    assert!(stack.space[1] == 222)
}
