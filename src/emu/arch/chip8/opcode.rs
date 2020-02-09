#[derive(Debug)]
pub struct Opcode {
    pub value: u16,
}

impl Opcode {
    pub fn first(&self) -> u16 {
        (self.value & 0xF000) >> 12
    }

    pub fn nn(&self) -> u16 {
        self.value & 0xFF
    }

    pub fn nnn(&self) -> u16 {
        self.value & 0xFFF
    }

    pub fn x(&self) -> u16 {
        (self.value & 0xF00) >> 8
    }

    pub fn y(&self) -> u16 {
        (self.value & 0xF0) >> 4
    }

    pub fn last(&self) -> u16 {
        self.value & 0xF
    }
}
