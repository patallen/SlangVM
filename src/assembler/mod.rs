use std::collections::HashMap;

pub mod lexer;

use self::lexer::{Token, Lexer, Directive, LabelType};
use opcode::Opcode;


pub struct Assembler {
    source: String,
    globals: HashMap<String, usize>,
    bytecode: Vec<u8>,
    current_line: usize,
    errors: Vec<String>,
    directives: HashMap<Directive, Vec<Token>>,
}

struct GlobalSection {
    locals: HashMap<String, usize>,
    bytecode: Vec<u8>
}
impl GlobalSection {
    pub fn new() -> GlobalSection {
        Self {
            locals: HashMap::new(),
            bytecode: Vec::new(),
        }
    }
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            globals: HashMap::new(),
            bytecode: Vec::new(),
            current_line: 0,
            errors: Vec::new(),
            directives: HashMap::new(),
        }
    }
    pub fn assemble(&mut self) {
        let source = &self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex();
        self.load_directives(tokens);
        self.handle_data_section();
    }
    fn handle_data_section(&mut self) {
        if let Some(directive) = self.directives.get(&Directive::Data) {
            let mut total_bytes = 0;
            let mut newline = true;
            for token in directive {
                match token {
                    &Token::NewLine => { newline = true },
                    &Token::Reference(_, _) => { panic!("References my not appear in the data section.") },
                    &Token::Label(LabelType::Global, ref label) => {
                        if !newline { panic!("Global labels must be the first on the line.")};
                        self.globals.insert(label.clone(), total_bytes);
                    },
                    &Token::Constant(val) => {
                        let mut bytes = to_bytes_32(val);
                        self.bytecode.append(&mut bytes);
                        total_bytes += 4;
                    }
                    _ => panic!("NOPE")
                }
            }
        }
    }
    fn load_directives(&mut self, tokens: Vec<Token>) {
        let mut curdir: Option<Directive> = None;
        let mut curvec: Vec<Token> = Vec::new();
        for t in tokens {
            match t {
                Token::NewLine | Token::Comment(_) => {},
                Token::Directive(dir) => {
                    match curdir {
                        Some(directive) => { self.directives.insert(directive, curvec.clone()); },
                        None => {}
                    };
                    curdir = Some(dir);
                    curvec = Vec::new();
                },
                Token::Eof => {
                    match curdir {
                        Some(directive) => { self.directives.insert(directive, curvec.clone()); },
                        None => {}
                    };
                    break
                },
                _ => {
                    match curdir {
                        Some(_) => curvec.push(t),
                        None => panic!("Tokens must fall within a directive."),
                    }
                }
            }
        }
    }
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
fn to_bytes_32(value: i64) -> Vec<u8> {
    let val = (value & 0xFFFFFFFF) as i32;
    let mut bytes: Vec<u8> = Vec::new();
    for n in 0..4 {
        let shift = (3 - n) * 8;
        bytes.push((val >> shift & 0xFF) as u8);
    }
    bytes
}

#[test]
fn test_new_assembler() {
    let assembler = Assembler::new("const 1\nconst2\nadd\nhalt".to_owned());
    assert!(assembler.current_line == 0);
    assert!(assembler.source == "const 1\nconst2\nadd\nhalt");
    assert!(assembler.errors.len() == 0);
    assert!(assembler.globals.len() == 0);
}

#[test]
fn test_to_bytes_32() {
    let value = 999999;
    let exp: Vec<u8> = vec![0, 15, 66, 63];
    let res = to_bytes_32(value);
    assert!(res == exp);
}
