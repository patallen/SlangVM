use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

use std::collections::BTreeMap;


#[derive(Debug)]
struct SourceLine {
    text: String,
    lineno: usize,
    parts: Vec<String>
}

impl SourceLine  {
    fn new(line: String, lineno: usize) -> Self {
        let text = line.clone();
        SourceLine {
            text: text,
            lineno: lineno,
            parts: line.split(" ").map(|a| a.to_owned()).collect()
        }
    }
    fn is_label(&self) -> bool {
        self.parts[0].starts_with(".")
    }
    fn is_empty(&self) -> bool {
        self.text == ""
    }
    fn byte_count(&self) -> usize {
        match self.is_label() || self.is_empty() {
            true => 0,
            false => (match_instruction(self.parts[0].to_owned()).byte_count + 1) as usize
        }
    }
}

pub struct Assembler {
    source: Vec<SourceLine>,
    symbols: BTreeMap<String, usize>,
}

impl Assembler {
    pub fn new(filename: &str) -> Self {
        let mut sourcelines: Vec<SourceLine> = vec![];
        for (i, line) in read_lines(filename).to_vec().into_iter().enumerate() {
            sourcelines.push(SourceLine::new(line.to_string(), i + 1));
        }
        Assembler {
            source: sourcelines,
            symbols: BTreeMap::new(),
        }
    }
    pub fn assemble(&mut self) -> Vec<u8> {
        let mut bytecode: Vec<u8> = Vec::new();
        self.record_labels();
        for sl in &self.source {
            let mut bytes = self.parse_line(sl, bytecode.len());
            bytecode.append(&mut bytes);
        }
        bytecode
    }
    fn record_labels(&mut self) {
        let mut bytecount = 0;
        for sl in &self.source {
            if sl.is_label() {
                let label = sl.parts[0].clone();
                self.symbols.insert(label, bytecount);
            }
            bytecount += sl.byte_count();
        }
    }
    fn parse_line(&self, sourceline: &SourceLine, current_len: usize) -> Vec<u8> {
        let byte_count = sourceline.byte_count();
        let addr = current_len + byte_count;
        if byte_count > 0 {
            let action = sourceline.parts[0].clone();
            let act2 = action.clone();
            let instruction = match_instruction(action);
            let mut tail: Vec<u8> = vec!(instruction.code);
            if byte_count == 1 {
                return tail;
            }
            let operand = sourceline.parts[1].clone();
            if operand.starts_with("."){
                if instruction.lbl == true {
                    let mut tmp = match self.symbols.get(&operand) {
                        Some(value) => {
                            if act2.contains("rel") || act2 == "call" {
                                let mut rel: i32 = 0;
                                if act2 == "call" {
                                    rel = *value as i32;
                                } else {
                                    rel = -((addr as i32) - (*value as i32));
                                }
                                val_to_bytes(rel)
                            } else {
                                val_to_bytes(*value as i32)
                            }
                        },
                        None => panic!("{} not a valid label.", operand),
                    };
                    tail.append(&mut tmp);
                } else {
                    panic!("Instruction {:02X} cannot use a label.", instruction.code);
                }
            } else {
                let mut tmp = match operand.parse() {
                    Ok(val) => val_to_bytes(val),
                    Err(_) => panic!("Error, bitch.")
                };
                tail.append(&mut tmp);
            }
            tail
        } else {
            Vec::new()
        }
    }
}
struct ASMCont {
    code: u8,
    byte_count: u8,
    lbl: bool,
}
fn match_instruction(inst: String) -> ASMCont {
    match &*inst.to_lowercase() {
        "noop"        => ASMCont {code: 0x00, byte_count: 0, lbl: false},
        "const"       => ASMCont {code: 0x10, byte_count: 4, lbl: false},
        "load"        => ASMCont {code: 0x11, byte_count: 4, lbl: true},
        "g_load"      => ASMCont {code: 0x12, byte_count: 4, lbl: true},
        "store"       => ASMCont {code: 0x14, byte_count: 4, lbl: true},
        "g_store"     => ASMCont {code: 0x15, byte_count: 4, lbl: true},
        "call"        => ASMCont {code: 0x18, byte_count: 4, lbl: true},
        "dup"         => ASMCont {code: 0x30, byte_count: 0, lbl: false},
        "add"         => ASMCont {code: 0x40, byte_count: 0, lbl: false},
        "sub"         => ASMCont {code: 0x41, byte_count: 0, lbl: false},
        "mul"         => ASMCont {code: 0x42, byte_count: 0, lbl: false},
        "div"         => ASMCont {code: 0x43, byte_count: 0, lbl: false},
        "pow"         => ASMCont {code: 0x44, byte_count: 0, lbl: false},
        "mod"         => ASMCont {code: 0x45, byte_count: 0, lbl: false},
        "shl"         => ASMCont {code: 0x50, byte_count: 0, lbl: false},
        "shr"         => ASMCont {code: 0x51, byte_count: 0, lbl: false},
        "and"         => ASMCont {code: 0x52, byte_count: 0, lbl: false},
        "or"          => ASMCont {code: 0x53, byte_count: 0, lbl: false},
        "xor"         => ASMCont {code: 0x54, byte_count: 0, lbl: false},
        "not"         => ASMCont {code: 0x55, byte_count: 0, lbl: false},
        "cmp_eq"      => ASMCont {code: 0x61, byte_count: 0, lbl: false},
        "cmp_ne"      => ASMCont {code: 0x62, byte_count: 0, lbl: false},
        "cmp_gt"      => ASMCont {code: 0x63, byte_count: 0, lbl: false},
        "cmp_lt"      => ASMCont {code: 0x64, byte_count: 0, lbl: false},
        "jmp_rel"     => ASMCont {code: 0x80, byte_count: 4, lbl: true},
        "jmp_rel_eq"  => ASMCont {code: 0x81, byte_count: 4, lbl: true},
        "jmp_rel_ne"  => ASMCont {code: 0x82, byte_count: 4, lbl: true},
        "jmp_rel_gt"  => ASMCont {code: 0x83, byte_count: 4, lbl: true},
        "jmp_rel_lt"  => ASMCont {code: 0x84, byte_count: 4, lbl: true},
        "jmp"         => ASMCont {code: 0x88, byte_count: 4, lbl: true},
        "ret"         => ASMCont {code: 0xA0, byte_count: 0, lbl: false},
        "print"       => ASMCont {code: 0xE0, byte_count: 0, lbl: false},
        "halt"        => ASMCont {code: 0xF0, byte_count: 0, lbl: false},
        _ => panic!("{} is not a valid instruction", inst)
    }
}

pub fn read_lines(filepath: &str) -> Vec<String>{
    let mut file = File::open(filepath).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    let mut rv: Vec<String> = Vec::new();
    for l in source.lines() {
        let r: Vec<&str> = l.split(";").collect();
        rv.push(r[0].trim().to_string());
    }
    rv
}

fn val_to_bytes(value: i32) -> Vec<u8> {
    let mut rv: Vec<u8> = Vec::new();
    for i in 0..4 {
        rv.push((value >> ((3-i) * 8) & 0xFF) as u8);
    }
    rv
}

