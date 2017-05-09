const REG_A: u32 = 0x00000001;
const REG_B: u32 = 0x00000002;
const REG_C: u32 = 0x00000003;
const REG_D: u32 = 0x00000004;
const REG_E: u32 = 0x00000005;

struct Registers {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
}
impl Registers {
    fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
        }
    }
    fn get_ref(&mut self, value: u32) -> &mut u32 {
        match value {
            0x2 => &mut self.a,
            0x3 => &mut self.b,
            0x4 => &mut self.c,
            0x5 => &mut self.d,
            0x6 => &mut self.e,
            _ => panic!("{{:08X} is not a valid register value.}")
        }
    }
}
