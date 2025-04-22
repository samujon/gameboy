pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    sp: u16,
    pc: u16,
}

pub enum CombinedRegisters{
    AF,
    BC,
    DE,
    HL
}

const ZERO: u8 = 0b10000000;
const SUBTRACTION: u8 = 0b01000000;
const HALF_CARRY: u8 = 0b00100000;
const CARRY: u8 = 0b00010000;

fn combine(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

fn set(mut high: u8,mut low: u8, value: u16) {
    high = ((value & 0xFF00) >> 8) as u8;
    low = (value & 0x00FF) as u8;
}

impl Registers {
    fn get_wide_registers(&self, registers: CombinedRegisters) -> u16{
        match registers{
            CombinedRegisters::AF => combine(self.a, self.f),
            CombinedRegisters::BC => combine(self.b, self.c),
            CombinedRegisters::DE => combine(self.d, self.e),
            CombinedRegisters::HL => combine(self.h, self.l)
        }
    }

    fn set_wide_registers(&mut self, registers: CombinedRegisters, value: u16){
        match registers{
            CombinedRegisters::AF => set(self.a, self.f, value),
            CombinedRegisters::BC => set(self.b, self.c, value),
            CombinedRegisters::DE => set(self.d, self.e, value),
            CombinedRegisters::HL => set(self.h, self.l, value),
        }
    }
}