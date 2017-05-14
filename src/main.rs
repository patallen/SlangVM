#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;

mod vm;
mod stack;
mod opcode;
mod program;
mod assembler;
mod instruction;


use assembler::Assembler;
use vm::VirtualMachine;


fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut assembler = Assembler::new(filename);
    let bytes = assembler.assemble();
    let mut vm = VirtualMachine::new(bytes);
    vm.run();
}

