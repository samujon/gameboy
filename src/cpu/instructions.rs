pub enum instructions{

}

pub enum TargetRegister{
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl TargetRegister{
    fn from_u8(value: u8) -> TargetRegister {
        match value {
            0b000 => TargetRegister::B,
            0b001 => TargetRegister::C,
            0b010 => TargetRegister::D,
            0b011 => TargetRegister::E,
            0b100 => TargetRegister::H,
            0b101 => TargetRegister::L,
            0b110 => TargetRegister::HL,
            0b111 => TargetRegister::A,
            _ => panic!("Invalid register value"),
        }
    }
    
    fn to_u8(&self) -> u8 {
        match self {
            TargetRegister::B => 0b000,
            TargetRegister::C => 0b001,
            TargetRegister::D => 0b010,
            TargetRegister::E => 0b011,
            TargetRegister::H => 0b100,
            TargetRegister::L => 0b101,
            TargetRegister::HL => 0b110,
            TargetRegister::A => 0b111,
        }
    }
}