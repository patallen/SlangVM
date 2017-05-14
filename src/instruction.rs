use opcode::Opcode;
use stack::Stack;


#[derive(Debug)]
pub struct Instruction {
    pub code: u8,
    pub opcode: Opcode,
    pub value: Option<u32>,
}

impl Instruction {
    pub fn new(code: u8, value: Option<u32>) -> Self {
        Instruction {
            code: code,
            opcode: Opcode::from_value(code),
            value: value,
        }
    }
    pub fn trace(&self, pc: usize, stack: &Stack) {
        let value = match self.value {
            Some(val) => format!("{}", val),
            None => format!(""),
        };
        debug!("{:04X}: {:04X} -> {:?} {}\t{:?}", pc, self.code as i32, self.opcode, value, stack);
    }
}
