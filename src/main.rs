#[macro_use]
extern crate log;
extern crate log4rs;

use std::env;

mod vm;
mod lex;
mod stack;
mod opcode;
mod program;
mod assembler;
mod instruction;


use assembler::Assembler;
use lex::{Tokenizer, shunting_yard};
use vm::VirtualMachine;


fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // let source = "1 + 2 * (0 + 1)".to_owned();
    // let mut lexer = lex::Lexer::new(source);
    // lexer.lex();
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let mut assembler = Assembler::new(filename);
    // let bytes = assembler.assemble();
    // let mut vm = VirtualMachine::new(bytes);
    // vm.run();
    let mut tokenizer = Tokenizer::new("a + b + 123 / ur_mum");
    let tokens = tokenizer.tokenize();
    println!("Tokens: {:?}", tokens);
    println!("Shunting: {:?}", shunting_yard(tokens))
}

