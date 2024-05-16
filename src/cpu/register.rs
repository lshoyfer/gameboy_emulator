#[cfg(test)]
mod tests;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
}

// af, bc, de, hl
impl Registers {
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = ((value & 0xFF) as u8).into();
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

}


#[derive(Debug, Copy, Clone)]
pub struct FlagRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const FLAG_REGISTER_ZERO_BIT_POSITION: u8 = 7;
const FLAG_REGISTER_SUBTRACT_BIT_POSITION: u8 = 6;
const FLAG_REGISTER_HALFCARRY_BIT_POSITION: u8 = 5;
const FLAG_REGISTER_CARRY_BIT_POSITION: u8 = 4;


impl From<FlagRegister> for u8 {
    fn from(value: FlagRegister) -> Self {
        (if value.zero { 0b1 } else { 0b0 } << FLAG_REGISTER_ZERO_BIT_POSITION |
        if value.subtract { 0b1 } else { 0b0 } << FLAG_REGISTER_SUBTRACT_BIT_POSITION |
        if value.half_carry { 0b1 } else { 0b0 } << FLAG_REGISTER_HALFCARRY_BIT_POSITION |
        if value.carry { 0b1 } else { 0b0 } << FLAG_REGISTER_CARRY_BIT_POSITION)
    }
}

impl From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        Self {
            zero: 1 == value >> FLAG_REGISTER_ZERO_BIT_POSITION,
            subtract: 1 == (value >> FLAG_REGISTER_SUBTRACT_BIT_POSITION) & 0b1,
            half_carry: 1 == (value >> FLAG_REGISTER_HALFCARRY_BIT_POSITION) & 0b1,
            carry: 1 == (value >> FLAG_REGISTER_CARRY_BIT_POSITION) & 0b1,
        }
    }
}

pub enum RegisterU8 {
    A, B, C, D, E, H, L,
}

pub enum RegisterU16 {
    BC, DE, HL,
    SP // TODO (stack pointer)
}