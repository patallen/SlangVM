use std::env;

mod vm;
mod stack;
mod program;
mod assembler;
mod instruction;


use assembler::Assembler;
use vm::VirtualMachine;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut assembler = Assembler::new(filename);
    let bytes = assembler.assemble();
    let mut vm = VirtualMachine::new(bytes);
    vm.run();
}

