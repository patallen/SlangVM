#[derive(Debug)]
pub enum Opcode {
    Noop     = 0x00,
    Const    = 0x10,
    Load     = 0x11,
    GLoad    = 0x12,
    Store    = 0x14,
    GStore   = 0x15,
    Call     = 0x18,
    Dup      = 0x30,
    Swap     = 0x31,
    Add      = 0x40,
    Sub      = 0x41,
    Mul      = 0x42,
    Div      = 0x43,
    Pow      = 0x44,
    Mod      = 0x45,
    Shl      = 0x50,
    Shr      = 0x51,
    And      = 0x52,
    Or       = 0x53,
    Xor      = 0x54,
    Not      = 0x55,
    CmpEq    = 0x61,
    CmpNe    = 0x62,
    CmpGt    = 0x63,
    CmpLt    = 0x64,
    RelJmp   = 0x80,
    RelJmpEq = 0x81,
    RelJmpNe = 0x82,
    RelJmpGt = 0x83,
    RelJmpLt = 0x84,
    Jmp      = 0x88,
    JmpNZ    = 0x89,
    Ret      = 0xA0,
    Print    = 0xE0,
    Halt     = 0xF0,
}

impl Opcode {
    pub fn from_value(value: u8) -> Self{
        match value {
            0x00 => Opcode::Noop,
            0x10 => Opcode::Const,
            0x11 => Opcode::Load,
            0x12 => Opcode::GLoad,
            0x14 => Opcode::Store,
            0x15 => Opcode::GStore,
            0x18 => Opcode::Call,
            0x30 => Opcode::Dup,
            0x31 => Opcode::Swap,
            0x40 => Opcode::Add,
            0x41 => Opcode::Sub,
            0x42 => Opcode::Mul,
            0x43 => Opcode::Div,
            0x44 => Opcode::Pow,
            0x45 => Opcode::Mod,
            0x50 => Opcode::Shl,
            0x51 => Opcode::Shr,
            0x52 => Opcode::And,
            0x53 => Opcode::Or,
            0x54 => Opcode::Xor,
            0x55 => Opcode::Not,
            0x61 => Opcode::CmpEq,
            0x62 => Opcode::CmpNe,
            0x63 => Opcode::CmpGt,
            0x64 => Opcode::CmpLt,
            0x80 => Opcode::RelJmp,
            0x81 => Opcode::RelJmpEq,
            0x82 => Opcode::RelJmpNe,
            0x83 => Opcode::RelJmpGt,
            0x84 => Opcode::RelJmpLt,
            0x88 => Opcode::Jmp,
            0x89 => Opcode::JmpNZ,
            0xA0 => Opcode::Ret,
            0xE0 => Opcode::Print,
            0xF0 => Opcode::Halt,
            _ => panic!("{:04X} is not a valid opcode.", value)
        }
    }
}
