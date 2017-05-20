extern crate itertools;

#[macro_use]
extern crate log;
extern crate log4rs;
extern crate regex;

use std::env;

mod vm;
mod stack;
mod opcode;
mod program;
mod compiler;
mod assembler;
mod instruction;


use assembler::Assembler;
use assembler::lexer::Lexer;
use compiler::lex::{Tokenizer, shunting_yard};
use vm::VirtualMachine;


use std::fs::File;
use std::io::prelude::*;


fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut source = String::new();
    let mut file = File::open(&filename).unwrap();
    file.read_to_string(&mut source).unwrap();
    let mut lexer = Lexer::new(source);
    let lexed = lexer.lex();
    println!("{:?}", lexed);
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let mut assembler = Assembler::new(filename);
    // let bytes = assembler.assemble();
    // let mut vm = VirtualMachine::new(bytes);
    // vm.run();
}

