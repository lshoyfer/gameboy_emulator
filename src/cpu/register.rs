#[cfg(test)]
mod tests;

// VERY IMPORTANT NOTE:
// The invariants here are due to the unsafe pointer arithmetic I do inside [`super::CPU::execute`] for loads.
// They must remain in place unless I change the arithmetic to some very large match branches

// non-spec (for this codebase) INVARIANT: must be repr(C) for reliable ordering of fields to their definition order
#[cfg_attr(test, derive(Debug))]
#[repr(C)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    // non-spec (for this codebase) INVARIANT: packed at the end purposely; is 4-bytes; it should be UNTOUCHED in the ptr load code
    pub f: FlagRegister, 
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


#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug))]
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

/*  non-spec (for this codebase) INVARIANTS: 
        - F is not introduced here or if it is it must be at the end of the enum variant list to make its usize value == 7
        - the enum variants either be listed in the order A,B,C,D,E,H,L or their numerical values be explicitly defined to be 
            0,1,2,3,4,5,6 respectively in order for use in casts and ptr arithmetic
*/

/// Consists of the major seven 8-bit registers (F is notably excluded which is what it is but it is never going to be matched in a match arm so...)
#[cfg_attr(test, derive(Debug))]
pub enum RegisterU8 {
    A, B, C, D, E, H, L,
}

/// Consists of the major 5 compound/16-bit registers including the stack pointer
#[cfg_attr(test, derive(Debug))]
pub enum RegisterU16 {
    BC, DE, HL, SP, AF
}