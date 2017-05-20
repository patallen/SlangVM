use std::collections::HashMap;

pub mod lexer;

use self::lexer::{Token, Lexer, Directive};
use opcode::Opcode;


pub struct Assembler {
    source: String,
    labels: HashMap<String, HashMap<String, usize>>,
    current_line: usize,
    errors: Vec<String>,
    data: Option<Vec<Token>>,
    code: Option<Vec<Token>>,
    space: Option<Vec<Token>>,
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            labels: HashMap::new(),
            current_line: 0,
            errors: Vec::new(),
            data: None,
            code: None,
            space: None,
        }
    }
    pub fn assemble(&mut self) {
        let source = &self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex();
        self.load_directives(tokens);
    }
    fn load_directives(&mut self, tokens: Vec<Token>) {
        let mut curdir: Option<Directive> = None;
        let mut curvec: Vec<Token> = Vec::new();
        for t in tokens {
            match t {
                Token::Directive(dir) => {
                    match curdir {
                        Some(Directive::Space) => self.space = Some(curvec.clone()),
                        Some(Directive::Code) => self.code = Some(curvec.clone()),
                        Some(Directive::Data) => self.data = Some(curvec.clone()),
                        None => {}
                    };
                    curdir = Some(dir);
                    curvec = Vec::new();
                },
                Token::Eof => {
                    match curdir {
                        Some(Directive::Space) => self.space = Some(curvec.clone()),
                        Some(Directive::Code) => self.code = Some(curvec.clone()),
                        Some(Directive::Data) => self.data = Some(curvec.clone()),
                        None => {}
                    };
                    break
                }
                Token::NewLine => {},
                _ => {
                    match curdir {
                        None => panic!("Tokens must fall within a directive."),
                        Some(_) => curvec.push(t)
                    }
                }
            }
        }
        println!("Code:\n{:?}", self.code);
        println!("Data:\n{:?}", self.data);
    }
}
struct DataChunk {
    label: String,
    values: Vec<f64>
}

struct CodeChunk {
    label: String,
    inner_labels: HashMap<String, usize>,
}

// gather all chunks
// evaluate each chunk

// struct ASMCont {
//     code: u8,
//     byte_count: u8,
//     lbl: bool,
// }
// 
// fn match_instruction(inst: String) -> ASMCont {
//     match &*inst.to_lowercase() {
//         "noop"        => ASMCont {code: 0x00, byte_count: 0, lbl: false},
//         "const"       => ASMCont {code: 0x10, byte_count: 4, lbl: false},
//         "load"        => ASMCont {code: 0x11, byte_count: 4, lbl: true},
//         "g_load"      => ASMCont {code: 0x12, byte_count: 4, lbl: true},
//         "store"       => ASMCont {code: 0x14, byte_count: 4, lbl: true},
//         "g_store"     => ASMCont {code: 0x15, byte_count: 4, lbl: true},
//         "call"        => ASMCont {code: 0x18, byte_count: 4, lbl: true},
//         "dup"         => ASMCont {code: 0x30, byte_count: 0, lbl: false},
//         "swap"        => ASMCont {code: 0x31, byte_count: 0, lbl: false},
//         "add"         => ASMCont {code: 0x40, byte_count: 0, lbl: false},
//         "sub"         => ASMCont {code: 0x41, byte_count: 0, lbl: false},
//         "mul"         => ASMCont {code: 0x42, byte_count: 0, lbl: false},
//         "div"         => ASMCont {code: 0x43, byte_count: 0, lbl: false},
//         "pow"         => ASMCont {code: 0x44, byte_count: 0, lbl: false},
//         "mod"         => ASMCont {code: 0x45, byte_count: 0, lbl: false},
//         "shl"         => ASMCont {code: 0x50, byte_count: 0, lbl: false},
//         "shr"         => ASMCont {code: 0x51, byte_count: 0, lbl: false},
//         "and"         => ASMCont {code: 0x52, byte_count: 0, lbl: false},
//         "or"          => ASMCont {code: 0x53, byte_count: 0, lbl: false},
//         "xor"         => ASMCont {code: 0x54, byte_count: 0, lbl: false},
//         "not"         => ASMCont {code: 0x55, byte_count: 0, lbl: false},
//         "cmp_eq"      => ASMCont {code: 0x61, byte_count: 0, lbl: false},
//         "cmp_ne"      => ASMCont {code: 0x62, byte_count: 0, lbl: false},
//         "cmp_gt"      => ASMCont {code: 0x63, byte_count: 0, lbl: false},
//         "cmp_lt"      => ASMCont {code: 0x64, byte_count: 0, lbl: false},
//         "jmp_rel"     => ASMCont {code: 0x80, byte_count: 4, lbl: true},
//         "jmp_rel_eq"  => ASMCont {code: 0x81, byte_count: 4, lbl: true},
//         "jmp_rel_ne"  => ASMCont {code: 0x82, byte_count: 4, lbl: true},
//         "jmp_rel_gt"  => ASMCont {code: 0x83, byte_count: 4, lbl: true},
//         "jmp_rel_lt"  => ASMCont {code: 0x84, byte_count: 4, lbl: true},
//         "jmp"         => ASMCont {code: 0x88, byte_count: 4, lbl: true},
//         "ret"         => ASMCont {code: 0xA0, byte_count: 0, lbl: false},
//         "print"       => ASMCont {code: 0xE0, byte_count: 0, lbl: false},
//         "halt"        => ASMCont {code: 0xF0, byte_count: 0, lbl: false},
//         _ => panic!("{} is not a valid instruction",  }

#[test]
fn test_new_assembler() {
    let mut assembler = Assembler::new("const 1\nconst2\nadd\nhalt".to_owned());
    assert!(assembler.current_line == 0);
    assert!(assembler.source == "const 1\nconst2\nadd\nhalt");
    assert!(assembler.errors.len() == 0);
    assert!(assembler.labels.len() == 0);
}
