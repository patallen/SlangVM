use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

// pub fn load_bytecode(filepath: &str) -> Vec<u8> {
//     let mut file = File::open(filepath);
//     let mut buffer: Vec<u8> = Vec::new();
//     let bytecode = file.read_to_end(&buffer);
// }

struct ASMCont {
    code: u8,
    byte_count: u8,
}
fn match_instruction(inst: String) -> ASMCont {
    match &*inst.to_lowercase() {
        "noop"        => ASMCont {code: 0x00, byte_count: 0},
        "const"       => ASMCont {code: 0x10, byte_count: 4},
        "load"        => ASMCont {code: 0x11, byte_count: 4},
        "g_load"      => ASMCont {code: 0x12, byte_count: 4},
        "store"       => ASMCont {code: 0x14, byte_count: 4},
        "g_store"     => ASMCont {code: 0x15, byte_count: 4},
        "call"        => ASMCont {code: 0x18, byte_count: 4},
        "add"         => ASMCont {code: 0x40, byte_count: 0},
        "sub"         => ASMCont {code: 0x41, byte_count: 0},
        "mul"         => ASMCont {code: 0x42, byte_count: 0},
        "div"         => ASMCont {code: 0x43, byte_count: 0},
        "pow"         => ASMCont {code: 0x44, byte_count: 0},
        "mod"         => ASMCont {code: 0x45, byte_count: 0},
        "shl"         => ASMCont {code: 0x50, byte_count: 0},
        "shr"         => ASMCont {code: 0x51, byte_count: 0},
        "and"         => ASMCont {code: 0x52, byte_count: 0},
        "or"          => ASMCont {code: 0x53, byte_count: 0},
        "xor"         => ASMCont {code: 0x54, byte_count: 0},
        "not"         => ASMCont {code: 0x55, byte_count: 0},
        "cmp_eq"      => ASMCont {code: 0x61, byte_count: 0},
        "cmp_ne"      => ASMCont {code: 0x62, byte_count: 0},
        "cmp_gt"      => ASMCont {code: 0x63, byte_count: 0},
        "cmp_lt"      => ASMCont {code: 0x64, byte_count: 0},
        "jmp_rel"     => ASMCont {code: 0x80, byte_count: 4},
        "jmp_rel_eq"  => ASMCont {code: 0x81, byte_count: 4},
        "jmp_rel_ne"  => ASMCont {code: 0x82, byte_count: 4},
        "jmp_rel_gt"  => ASMCont {code: 0x83, byte_count: 4},
        "jmp_rel_lt"  => ASMCont {code: 0x84, byte_count: 4},
        "jmp"         => ASMCont {code: 0x88, byte_count: 4},
        "ret"         => ASMCont {code: 0xA0, byte_count: 0},
        "print"       => ASMCont {code: 0xE0, byte_count: 0},
        "halt"        => ASMCont {code: 0xF0, byte_count: 0},
        _ => panic!("{} is not a valid instruction", inst)
    }
}

pub fn assemble_code(filepath: &str) -> Vec<u8> {
    let lines = read_lines(filepath);
    let mut bytes: Vec<u8> = Vec::new();
    for line in lines {
        println!("{}", line);
        bytes.append(&mut parse_line(line))
    }
    bytes
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

fn parse_line(line: String) -> Vec<u8> {
    match &*line {
        "" => Vec::new(),
        _ => {
            println!("Line: {:?}", line);
            let parts: Vec<&str> = line.split(' ').collect();
            println!("Parts: {:?}", parts);
            let ac = match_instruction(parts[0].to_string());
            let mut bytes: Vec<u8> = vec![ac.code];
            if ac.byte_count > 0 {
                let val: i32 = parts[1].parse().unwrap();
                bytes.append(&mut val_to_bytes(val));
            }
            bytes
        }
    }
}

fn val_to_bytes(value: i32) -> Vec<u8> {
    let mut rv: Vec<u8> = Vec::new();
    for i in 0..4 {
        rv.push((value >> ((3-i) * 8) & 0xFF) as u8);
    }
    rv
}

