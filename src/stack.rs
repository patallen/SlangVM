use std::fmt;
use std::collections::BTreeMap;


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


pub struct CallFrame {
    pub ret: usize,
    locals: BTreeMap<usize, u32>,
}

impl CallFrame {
    pub fn new(ret: usize) -> Self {
        Self {
            ret: ret,
            locals: BTreeMap::new(),
        }
    }
    pub fn set_local(&mut self, addr: usize, value: u32) {
        self.locals.insert(addr, value);
    }
    pub fn get_local(&mut self, addr: usize) -> u32 {
        *self.locals.get(&addr).unwrap()
    }
}

pub struct CallStack {
    frames: Vec<CallFrame>,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }
    pub fn push(&mut self, sf: CallFrame) {
        self.frames.push(sf);
    }
    pub fn pop(&mut self) -> CallFrame {
        self.frames.pop().unwrap()
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
