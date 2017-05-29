mod lexer;

use std::collections::HashMap;
use self::lexer::{Lexer, Token, Directive, LabelType};

enum ASMError {
    Syntax,
    Label,
}

struct TokenHandler {
    tokens: Vec<Token>,
    idx: usize
}

impl TokenHandler {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenHandler{
            tokens: tokens,
            idx: 0,
        }
    }
    pub fn next_line(&mut self) -> Option<Vec<Token>> {
        let mut pass = 0;
        let mut rets = Vec::new();
        loop {
            match self.tokens[self.idx].clone() {
                Token::Eof => match pass {
                    0 => return None,
                    _ => { return Some(rets); }
                },
                Token::NewLine => match pass {
                    0 => { self.idx += 1; return self.next_line()},
                    _ => { return Some(rets); }
                },
                ref token => { rets.push(token.clone()); pass += 1;}
            }
            self.idx += 1;
        }
    }
}

pub struct Assembler {
    current_line: usize,
    current_directive: Option<Directive>,
    current_address: usize,
    current_scope: Option<String>,
    code_symbols: HashMap<String, usize>,
    data_symbols: HashMap<String, usize>,
    tokens: TokenHandler,
    errors: Vec<ASMError>,
    code_length: usize,
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Assembler {
            current_line: 0,
            current_directive: None,
            current_address: 0,
            current_scope: None,
            code_symbols: HashMap::new(),
            data_symbols: HashMap::new(),
            tokens: TokenHandler::new(Lexer::new(&source).lex()),
            errors: Vec::new(),
            code_length: 0,
        }
    }
    pub fn assemble(&mut self) -> Vec<u8> {
        // Here we are simply recording all symbols & making some basic assertions
        loop {
            if let Some(line) = self.tokens.next_line() {
                self.pass_one_line(line)
            } else { break }
        }

        // And here we build each line, ensure that they are valid, resolve symbol
        // references, and record the bytecodes.
        loop {
            if let Some(line) = self.tokens.next_line() {
                self.pass_two_line(line)
            } else { break }
        }
        println!("{:?}", self.code_symbols);
        Vec::new()
    }
    fn pass_two_line(&mut self, line: Vec<Token>) {

    }
    fn pass_one_line(&mut self, line: Vec<Token>) {
        self.current_line += 1;
        for token in line {
            match token {
                Token::Directive(dtype) => { self.current_address = 0; self.current_directive = Some(dtype); },
                Token::Label(ltype, label) => {
                    match (ltype, &self.current_directive) {
                        (LabelType::Local, &Some(Directive::Code)) => {
                            let lbl = match self.current_scope {
                                Some(ref lbl) => lbl,
                                None => {panic!("Local labels must be declared within global scope.")},
                            };
                            self.code_symbols.insert(format!("{}{}", lbl, label), self.current_address);
                        },
                        (LabelType::Global, &Some(Directive::Code)) => {
                            self.current_scope = Some(label.clone());
                            self.code_symbols.insert(label, self.current_address);
                        },
                        (LabelType::Global, &Some(Directive::Data)) => {
                            self.data_symbols.insert(label, self.current_address);
                        }
                        _ => panic!("NOT ALLOWED")
                    }
                },
                Token::Constant(val) => {
                    self.current_address += 4;
                    match self.current_directive {
                        Some(Directive::Code) =>  { self.code_length += 4; },
                        _ => {}
                    }
                }
                Token::Reference(_, label) => { self.current_address += 4; },
                Token::Instruction(inst) => {
                    match self.current_directive {
                        Some(Directive::Code) => { self.current_address += 1; },
                        _ => panic!("Cannot have instructions outside of the '@code' directive.")
                    };
                },
                _ => {}
            }
        }
    }
}
