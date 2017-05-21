use std::collections::HashMap;

pub mod lexer;

use self::lexer::{Token, Lexer, Directive, LabelType};


pub struct Assembler {
    source: String,
    globals: HashMap<String, usize>,
    bytecode: Vec<u8>,
    current_line: usize,
    errors: Vec<String>,
    directives: HashMap<Directive, Vec<Token>>,
}

#[derive(Debug)]
struct GlobalSection<'g> {
    bytes_size: Option<usize>,
    tokens: Vec<&'g Token>,
    locals: HashMap<String, usize>,
    bytecode: Vec<u8>,
}

impl<'g> GlobalSection<'g> {
    pub fn new() -> GlobalSection<'g> {
        Self {
            bytes_size: None,
            tokens: Vec::new(),
            locals: HashMap::new(),
            bytecode: Vec::new(),
        }
    }
    fn record_local_info(&mut self) {
        let mut count: usize = 0;
        let tokens = &self.tokens;
        for token in tokens {
            match token {
                &&Token::Label(LabelType::Local, ref label) => {
                    self.locals.insert(label.clone(), count);
                },
                &&Token::Reference(_, _) | &&Token::Constant(_) => { count += 4; },
                &&Token::Instruction(_) => { count += 1; },
                _ => {}
            }
        }
        self.bytes_size = Some(count)
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
    pub fn assemble(&mut self) -> Vec<u8> {
        let source = &self.source.clone();
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex();

        self.load_directives(tokens);
        self.handle_data_section();
        let mut datasize = 6;
        for token in self.directives.get(&Directive::Data).unwrap() {
            if let &Token::Constant(_) = token { datasize += 4 };
        }
        let mut secvec: Vec<GlobalSection> = Vec::new();
        let sections = self.make_global_sections();
        let mut globals = HashMap::new();
        for (label, mut section) in sections {
            globals.insert(label, datasize);
            section.record_local_info();
            datasize += section.bytes_size.unwrap();
            secvec.push(section);
        }
        for (k, v) in self.globals.clone() {
            globals.insert(k, v + 6);
        }
        let mut bytecode: Vec<u8> = Vec::new();
        bytecode.push(match_instruction("jmp".to_owned()));
        if let Some(addr) = globals.get("._entry") {
            bytecode.append(&mut to_bytes_32(*addr as i64));
        } else { panic!("SlangASM requires a global entry point '._entry'")}
        bytecode.push(0);
        bytecode.append(&mut self.bytecode.clone());
        for section in secvec {
            let mut bytecodes = Vec::new();
            for tok in &section.tokens {
                match tok {
                    &&Token::Reference(LabelType::Global, ref label) => {
                        if let Some(addr) = globals.get(&label.clone()) {
                            bytecodes.append(&mut to_bytes_32(*addr as i64))
                        } else { panic!("{} is not a known label.", label)}
                    },
                    &&Token::Reference(LabelType::Local, ref label) => {
                        if let Some(addr) = section.locals.get(&label.clone()) {
                            bytecodes.append(&mut to_bytes_32(*addr as i64))
                        } else { panic!("{} is not a known local label.", label)}
                    },
                    &&Token::Constant(value) => {bytecodes.append(&mut to_bytes_32(value))},
                    &&Token::Instruction(ref inst) => {
                        bytecodes.push(match_instruction(inst.clone()));
                    },
                    _ => {}
                }
            }
            bytecode.append(&mut bytecodes);
        }
        bytecode
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
    fn make_global_sections<'a>(&'a self) -> HashMap<String, GlobalSection> {
        let tokens = match self.directives.get(&Directive::Code) {
            Some(token_vec) => token_vec,
            None => panic!("NO TOKENS IN @CODE!!!!"),
        };
        let mut current_label: Option<String> = None;
        let mut section = GlobalSection::new();
        let mut sections: HashMap<String, GlobalSection> = HashMap::new();
        for token in tokens {
            match token {
                &Token::Label(LabelType::Global, ref label) => {
                    match current_label.clone() {
                        None => { current_label = Some(label.clone()); },
                        Some(clabel) => {
                            sections.insert(clabel, section);
                            section = GlobalSection::new();
                            current_label = Some(label.clone());
                        }
                    };
                },
                &Token::Comment(_) => {},
                _ => { section.tokens.push(token); }
            }
        }
        sections.insert(current_label.unwrap().clone(), section);
        sections
    }
}

fn match_instruction(inst: String) -> u8 {
    match &*inst.to_lowercase() {
        "noop"        => 0x00,
        "const"       => 0x10,
        "load"        => 0x11,
        "g_load"      => 0x12,
        "store"       => 0x14,
        "g_store"     => 0x15,
        "call"        => 0x18,
        "dup"         => 0x30,
        "swap"        => 0x31,
        "add"         => 0x40,
        "sub"         => 0x41,
        "mul"         => 0x42,
        "div"         => 0x43,
        "pow"         => 0x44,
        "mod"         => 0x45,
        "shl"         => 0x50,
        "shr"         => 0x51,
        "and"         => 0x52,
        "or"          => 0x53,
        "xor"         => 0x54,
        "not"         => 0x55,
        "cmp_eq"      => 0x61,
        "cmp_ne"      => 0x62,
        "cmp_gt"      => 0x63,
        "cmp_lt"      => 0x64,
        "jmp_rel"     => 0x80,
        "jmp_rel_eq"  => 0x81,
        "jmp_rel_ne"  => 0x82,
        "jmp_rel_gt"  => 0x83,
        "jmp_rel_lt"  => 0x84,
        "jmp"         => 0x88,
        "ret"         => 0xA0,
        "print"       => 0xE0,
        "halt"        => 0xF0,
        _ => panic!("{} is not a valid instruction", inst),
    }
}
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
