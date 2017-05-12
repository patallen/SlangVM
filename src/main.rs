use std::env;

mod vm;
mod stack;
mod program;
mod assembler;
mod instruction;

use assembler::assemble_code;
use vm::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Filename: {}", filename);
    let bytes = assemble_code(&*filename);
    println!("Bytes: {:?}", bytes);
    let mut vm = VirtualMachine::new(bytes);
    vm.run();
}

