mod lexer;

use std::collections::HashMap;
use self::lexer::{Lexer, Token};

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
    symbols: HashMap<String, usize>,
    current_line: usize,
    current_token: usize,
    tokens: TokenHandler,
    errors: Vec<ASMError>
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Assembler {
            symbols: HashMap::new(),
            current_line: 0,
            current_token: 0,
            tokens: TokenHandler::new(Lexer::new(&source).lex()),
            errors: Vec::new()
        }
    }
    pub fn assemble(&mut self) -> Vec<u8> {
        loop {
            match self.tokens.next_line() {
                None => break,
                Some(line) => println!("{:?}", line)
            }
        }
        Vec::new()
    }
}
