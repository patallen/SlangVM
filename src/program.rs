
pub struct Program {
    bytes: Vec<u8>,
    pc: usize,
}


impl Program {
    pub fn new() -> Self {
        Program {
            bytes: Vec::new(),
            pc: 0
        }
    }
    pub fn next_byte(&mut self) -> u8 {
        let val = self.bytes[self.pc];
        self.pc += 1;
        val
    }
    pub fn next_halfword(&mut self) -> u16 {
        let rv = (self.next_byte() as u16) << 8;
        rv | self.next_byte() as u16
    }
    pub fn next_word(&mut self) -> u32 {
        let rv = (self.next_halfword() as u32) << 16;
        rv | self.next_halfword() as u32
    }
    pub fn next_code(&mut self) -> u16 {
        self.next_halfword()
    }
    pub fn jump_to(&mut self, addr: usize) {
        self.pc = addr;
    }
    pub fn jump_relative(&mut self, rel: i32) {
        let na = self.pc as i32 + rel;
        self.jump_to(na as usize);
    }
    pub fn load_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }
    pub fn reset(&mut self) {
        self.pc = 0;
    }
    pub fn current(&self) -> usize{
        self.pc
    }
}

#[test]
fn test_load_bytes() {
    let mut prog = Program::new();
    let bytes: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
    prog.load_bytes(bytes);
    assert!(prog.pc == 0);
    assert!(prog.bytes.len() == 6);
}
#[test]
fn test_next_byte() {
    let mut prog = Program::new();
    let bytes: Vec<u8> = vec![0xFA, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
    prog.load_bytes(bytes);
    let val = prog.next_byte();
    assert!(val == 0xFA);
    assert!(prog.pc == 1);
}

#[test]
fn test_next_halfword() {
    let mut prog = Program::new();
    let bytes: Vec<u8> = vec![0xFA, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
    prog.load_bytes(bytes);
    let val = prog.next_halfword();
    assert!(val == 0xFAFF);
    assert!(prog.pc == 2);
}

#[test]
fn test_next_code() {
    let mut prog = Program::new();
    let mut prog2 = Program::new();
    let bytes: Vec<u8> = vec![0xFA, 0xFF, 0xFF, 0xFF, 0x00, 0x00];
    let bytes2 = bytes.clone();
    prog.load_bytes(bytes);
    prog2.load_bytes(bytes2);
    assert!(prog2.next_code() == prog.next_code());
}

#[test]
fn test_jump_to() {
    let mut prog = Program::new();
    let bytes: Vec<u8> = vec![0xFA, 0xFF, 0xFF, 0xFF, 0x00, 0x01];
    prog.load_bytes(bytes);
    prog.jump_to(5);
    assert!(prog.pc == 5);
    assert!(prog.next_byte() == 0x01);
}

#[test]
fn test_reset() {
    let mut prog = Program::new();
    let bytes: Vec<u8> = vec![0xFA, 0xFF, 0xFF, 0xFF, 0x00, 0x01];
    prog.load_bytes(bytes);
    prog.jump_to(5);
    prog.reset();
    assert!(prog.pc == 0);

}
