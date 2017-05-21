use std;

use stack::{Stack, CallStack, CallFrame};
use opcode::Opcode;
use program::Program;
use instruction::Instruction;


pub struct VirtualMachine {
    stack: Stack,
    callstack: CallStack,
    program: Program,
    mem: [u32; 0xFFFF],
    current_frame: CallFrame
}


impl VirtualMachine {
    pub fn new(source: Vec<u8>) -> Self {
        let mut program = Program::new();
        program.load_bytes(source);
        return VirtualMachine {
            stack: Stack::new(),
            callstack: CallStack::new(),
            program: program,
            mem: [0; 0xFFFF],
            current_frame: CallFrame::new(0),
        }
    }
    pub fn run(&mut self) {
        loop {
            let instr = self.fetch_instruction();
            self.handle_instruction(instr);
        }
    }
    fn fetch_instruction(&mut self) -> Instruction {
        let pc = self.program.current();
        let base = self.program.next_byte();
        let value = match base >> 4 {
            1 | 8 => Some(self.program.next_word()),
            _ => None
        };
        let instruction = Instruction::new(base, value);
        instruction.trace(pc, &self.stack);
        instruction
    }
    fn handle_instruction(&mut self, instr: Instruction) {
        match instr.opcode {
            Opcode::Noop     => {},
            Opcode::Const    => self.load_const(instr.value.unwrap()),
            Opcode::Load     => self.load_local(instr.value.unwrap()),
            Opcode::GLoad    => self.load_global(instr.value.unwrap()),
            Opcode::Store    => self.store_local(instr.value.unwrap()),
            Opcode::GStore   => self.store_global(instr.value.unwrap()),
            Opcode::Call     => self.call(instr.value.unwrap()),
            Opcode::Dup      => self.dup(),
            Opcode::Swap     => self.swap(),
            Opcode::Add      => self.add(),
            Opcode::Sub      => self.sub(),
            Opcode::Mul      => self.mul(),
            Opcode::Div      => self.div(),
            Opcode::Pow      => self.pow(),
            Opcode::Mod      => self.modulo(),
            Opcode::Shl      => self.bit_shl(),
            Opcode::Shr      => self.bit_shr(),
            Opcode::And      => self.bit_and(),
            Opcode::Or       => self.bit_or(),
            Opcode::Xor      => self.bit_xor(),
            Opcode::Not      => self.bit_not(),
            Opcode::CmpEq    => self.cmp_eq(),
            Opcode::CmpNe    => self.cmp_ne(),
            Opcode::CmpGt    => self.cmp_gt(),
            Opcode::CmpLt    => self.cmp_lt(),
            Opcode::RelJmp   => self.rel_jmp(instr.value.unwrap()),
            Opcode::RelJmpEq => self.rel_jmp_eq(instr.value.unwrap()),
            Opcode::RelJmpNe => self.rel_jmp_ne(instr.value.unwrap()),
            Opcode::RelJmpGt => self.rel_jmp_gt(instr.value.unwrap()),
            Opcode::RelJmpLt => self.rel_jmp_lt(instr.value.unwrap()),
            Opcode::Jmp      => self.jmp(instr.value.unwrap()),
            Opcode::JmpNZ    => self.jmp_nz(instr.value.unwrap()),
            Opcode::Ret      => self.ret(),
            Opcode::Print    => self.print(),
            Opcode::Halt     => self.halt(),
        }
    }
    fn jmp_nz(&mut self, value: u32) {
        let addr = value as usize;
        if self.stack.pop() != 0 { self.program.jump_to(addr); }
    }
    fn load_const(&mut self, value: u32) {
        self.stack.push(value);
    }
    fn load_global(&mut self, addr: u32) {
        let value = self.mem[addr as usize];
        self.stack.push(value);
    }
    fn load_local(&mut self, addr: u32) {
        let value = self.current_frame.get_local(addr as usize);
        self.stack.push(value);
    }
    fn store_global(&mut self, addr: u32) {
        let value = self.stack.pop();
        self.mem[addr as usize] = value;
    }
    fn store_local(&mut self, addr: u32) {
        let value = self.stack.pop();
        self.current_frame.set_local(addr as usize, value);
    }
    fn call(&mut self, addr: u32) {
        let pc = self.program.current();
        self.callstack.push(CallFrame::new(pc));
        self.program.jump_to(addr as usize);
    }
    fn ret(&mut self) {
        let ret = self.callstack.pop().ret;
        self.program.jump_to(ret);
    }
    fn add(&mut self) {
        let s1 = self.stack.pop() as u32;
        let s2 = self.stack.pop() as u32;
        self.stack.push((s1.wrapping_add(s2)) as u32);
    }
    fn sub(&mut self) {
        let s2 = self.stack.pop() as f32;
        let s1 = self.stack.pop() as f32;
        self.stack.push((s1 - s2) as u32);
    }
    fn mul(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s1 * s2) as u32);
    }
    fn div(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s2 / s1) as u32);
    }
    fn pow(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s1.powf(s2)) as u32);
    }
    fn modulo(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s1 % s2) as u32);
    }
    fn bit_shl(&mut self) {
        let s = self.stack.pop() << 1;
        self.stack.push(s);
    }
    fn bit_shr(&mut self) {
        let s = self.stack.pop() >> 1;
        self.stack.push(s);
    }
    fn bit_and(&mut self) {
        let s = self.stack.pop() & self.stack.pop();
        self.stack.push(s);
    }
    fn bit_or(&mut self) {
        let s = self.stack.pop() | self.stack.pop();
        self.stack.push(s);
    }
    fn bit_xor(&mut self) {
        let s = self.stack.pop() ^ self.stack.pop();
        self.stack.push(s);
    }
    fn bit_not(&mut self) {
        let s = self.stack.pop();
        self.stack.push(!s);
    }
    fn cmp_eq(&mut self) {
        let eq = self.stack.pop() == self.stack.pop();
        self.stack.push(eq as u32);
    }
    fn cmp_ne(&mut self) {
        let ne = self.stack.pop() != self.stack.pop();
        self.stack.push(ne as u32);
    }
    fn cmp_gt(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s1 > s2) as u32);
    }
    fn cmp_lt(&mut self) {
        let s1 = self.stack.pop() as f32;
        let s2 = self.stack.pop() as f32;
        self.stack.push((s1 < s2) as u32);
    }
    fn rel_jmp(&mut self, addr: u32) {
        let na = addr as i32;
        self.program.jump_relative(na);
    }
    fn rel_jmp_eq(&mut self, addr: u32) {
        if self.stack.pop() == self.stack.pop() {
            self.program.jump_relative(addr as i32);
        }
    }
    fn rel_jmp_ne(&mut self, addr: u32) {
        if self.stack.pop() != self.stack.pop() {
            self.program.jump_relative(addr as i32);
        }
    }
    fn rel_jmp_gt(&mut self, addr: u32) {
        if self.stack.pop() > self.stack.pop() {
            self.program.jump_relative(addr as i32);
        }
    }
    fn rel_jmp_lt(&mut self, addr: u32) {
        if self.stack.pop() < self.stack.pop() {
            self.program.jump_relative(addr as i32);
        }
    }
    fn jmp(&mut self, addr: u32) {
        self.program.jump_to(addr as usize)
    }
    fn halt(&mut self) {
        std::process::exit(1);
    }
    fn print(&mut self) {
        let s = self.stack.peek() as f32;
        println!("{}", s);
    }
    fn dup(&mut self) {
        let todupe = self.stack.peek();
        self.stack.push(todupe);
    }
    fn swap(&mut self) {
        let s1 = self.stack.pop();
        let s2 = self.stack.pop();
        self.stack.push(s1);
        self.stack.push(s2);
    }
}

#[test]
fn test_vm_new() {
    let mut vm = VirtualMachine::new(vec![0xFF, 0xFF]);
    assert!(vm.program.next_byte() == 0xFF);
}

#[test]
fn test_fetch_instruction() {
    let mut vm = VirtualMachine::new(vec![0x10, 0xFF, 0xFF, 0xFF, 0xFF]);
    let inst = vm.fetch_instruction();
    assert!(inst.code == 0x10);
    assert!(inst.value.unwrap() == 0xFFFFFFFF);
}

#[test]
fn test_swap_instruction() {
    let mut vm = VirtualMachine::new(vec![]);
    vm.stack.push(0x01);
    vm.stack.push(0x02);
    vm.swap();

    assert!(vm.stack.pop() == 1);
    assert!(vm.stack.pop() == 2);
}
