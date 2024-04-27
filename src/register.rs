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

// comments are the CPU opcode
pub enum Instruction {
    ADD(RegisterU8), // 8X
    ADDHL(RegisterU16), // X9
    ADC(RegisterU8), // 8X
    SUB(RegisterU8), // 9X
    SBC(RegisterU8), // 9X
    AND(RegisterU8), // AX
    OR(RegisterU8), // BX
    XOR(RegisterU8), // AX
    CP(RegisterU8), // BX
    INC(RegisterU8), // XX
    DEC(RegisterU8), // XX
    CCF, // 3F
    SCF, // 37
    RRA, // 1F
    RLA, // 17
    RRCA, // 0F
    RLCA, // 07 - this one is incorrectly spelled "RRLA" on the Rust Guidebook
    CPL, // 2F
    BIT(u8, RegisterU8), // CB XX
    RES(u8, RegisterU8), // CB XX
    SET(u8, RegisterU8), // CB XX
    SRL(RegisterU8), // CB 3x
    RR(RegisterU8), // CB 1x
    RL(RegisterU8), // CB 1x
    RRC(RegisterU8), // CB 0x
    RLC(RegisterU8), // CB 0x
    SRA(RegisterU8), // CB 2x
    SLA(RegisterU8), // CB 2x
    SWAP(RegisterU8), // CB 3x
}

pub enum RegisterU8 {
    A, B, C, D, E, H, L,
}

pub enum RegisterU16 {
    BC, DE, HL,
    SP // TODO (stack pointer)
}

pub struct CPU {
    registers: Registers
} 

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.add(self.registers.a),
                    RegisterU8::B => self.registers.a = self.add(self.registers.b),
                    RegisterU8::C => self.registers.a = self.add(self.registers.c),
                    RegisterU8::D => self.registers.a = self.add(self.registers.d),
                    RegisterU8::E => self.registers.a = self.add(self.registers.e),
                    RegisterU8::H => self.registers.a = self.add(self.registers.h),
                    RegisterU8::L => self.registers.a = self.add(self.registers.l),
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    RegisterU16::BC => {
                        let sum = self.add_u16(self.registers.get_bc());
                        self.registers.set_hl(sum);
                    },
                    RegisterU16::DE => {
                        let sum = self.add_u16(self.registers.get_de());
                        self.registers.set_hl(sum);
                    }
                    RegisterU16::HL => {
                        let sum = self.add_u16(self.registers.get_hl());
                        self.registers.set_hl(sum);
                    }
                    RegisterU16::SP => todo!("Stack Pointer Implementation")
                }
            }
            Instruction::ADC(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.adc(self.registers.a),
                    RegisterU8::B => self.registers.a = self.adc(self.registers.b),
                    RegisterU8::C => self.registers.a = self.adc(self.registers.c),
                    RegisterU8::D => self.registers.a = self.adc(self.registers.d),
                    RegisterU8::E => self.registers.a = self.adc(self.registers.e),
                    RegisterU8::H => self.registers.a = self.adc(self.registers.h),
                    RegisterU8::L => self.registers.a = self.adc(self.registers.l),
                }
            }
            Instruction::SUB(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.sub(self.registers.a),
                    RegisterU8::B => self.registers.a = self.sub(self.registers.b),
                    RegisterU8::C => self.registers.a = self.sub(self.registers.c),
                    RegisterU8::D => self.registers.a = self.sub(self.registers.d),
                    RegisterU8::E => self.registers.a = self.sub(self.registers.e),
                    RegisterU8::H => self.registers.a = self.sub(self.registers.h),
                    RegisterU8::L => self.registers.a = self.sub(self.registers.l),
                }
            }
            Instruction::SBC(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.sbc(self.registers.a),
                    RegisterU8::B => self.registers.a = self.sbc(self.registers.b),
                    RegisterU8::C => self.registers.a = self.sbc(self.registers.c),
                    RegisterU8::D => self.registers.a = self.sbc(self.registers.d),
                    RegisterU8::E => self.registers.a = self.sbc(self.registers.e),
                    RegisterU8::H => self.registers.a = self.sbc(self.registers.h),
                    RegisterU8::L => self.registers.a = self.sbc(self.registers.l),
                }
            }
            Instruction::AND(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.and(self.registers.a),
                    RegisterU8::B => self.registers.a = self.and(self.registers.b),
                    RegisterU8::C => self.registers.a = self.and(self.registers.c),
                    RegisterU8::D => self.registers.a = self.and(self.registers.d),
                    RegisterU8::E => self.registers.a = self.and(self.registers.e),
                    RegisterU8::H => self.registers.a = self.and(self.registers.h),
                    RegisterU8::L => self.registers.a = self.and(self.registers.l),
                }
            }
            Instruction::OR(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.or(self.registers.a),
                    RegisterU8::B => self.registers.a = self.or(self.registers.b),
                    RegisterU8::C => self.registers.a = self.or(self.registers.c),
                    RegisterU8::D => self.registers.a = self.or(self.registers.d),
                    RegisterU8::E => self.registers.a = self.or(self.registers.e),
                    RegisterU8::H => self.registers.a = self.or(self.registers.h),
                    RegisterU8::L => self.registers.a = self.or(self.registers.l),
                }
            }
            Instruction::XOR(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.xor(self.registers.a),
                    RegisterU8::B => self.registers.a = self.xor(self.registers.b),
                    RegisterU8::C => self.registers.a = self.xor(self.registers.c),
                    RegisterU8::D => self.registers.a = self.xor(self.registers.d),
                    RegisterU8::E => self.registers.a = self.xor(self.registers.e),
                    RegisterU8::H => self.registers.a = self.xor(self.registers.h),
                    RegisterU8::L => self.registers.a = self.xor(self.registers.l),
                }
            }
            Instruction::CP(target) => {
                match target {
                    RegisterU8::A => self.cp(self.registers.a),
                    RegisterU8::B => self.cp(self.registers.b),
                    RegisterU8::C => self.cp(self.registers.c),
                    RegisterU8::D => self.cp(self.registers.d),
                    RegisterU8::E => self.cp(self.registers.e),
                    RegisterU8::H => self.cp(self.registers.h),
                    RegisterU8::L => self.cp(self.registers.l),
                }
            }
            Instruction::INC(target) => {
                match target {
                    RegisterU8::A => { 
                        self.registers.a = self.registers.a.wrapping_add(1);
                        self.inc_flags(self.registers.a);
                    }
                    RegisterU8::B => { 
                        self.registers.b = self.registers.b.wrapping_add(1);
                        self.inc_flags(self.registers.b);
                    }
                    RegisterU8::C => { 
                        self.registers.c = self.registers.c.wrapping_add(1);
                        self.inc_flags(self.registers.c);
                    }
                    RegisterU8::D => { 
                        self.registers.d = self.registers.d.wrapping_add(1);
                        self.inc_flags(self.registers.d);
                    }
                    RegisterU8::E => { 
                        self.registers.e = self.registers.e.wrapping_add(1);
                        self.inc_flags(self.registers.e);
                    }
                    RegisterU8::H => { 
                        self.registers.h = self.registers.h.wrapping_add(1);
                        self.inc_flags(self.registers.h);
                    }
                    RegisterU8::L => { 
                        self.registers.l = self.registers.l.wrapping_add(1);
                        self.inc_flags(self.registers.l);
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    RegisterU8::A => { 
                        self.registers.a = self.registers.a.wrapping_sub(1);
                        self.dec_flags(self.registers.a);
                    }
                    RegisterU8::B => { 
                        self.registers.b = self.registers.b.wrapping_sub(1);
                        self.dec_flags(self.registers.b);
                    }
                    RegisterU8::C => { 
                        self.registers.c = self.registers.c.wrapping_sub(1);
                        self.dec_flags(self.registers.c);
                    }
                    RegisterU8::D => { 
                        self.registers.d = self.registers.d.wrapping_sub(1);
                        self.dec_flags(self.registers.d);
                    }
                    RegisterU8::E => { 
                        self.registers.e = self.registers.e.wrapping_sub(1);
                        self.dec_flags(self.registers.e);
                    }
                    RegisterU8::H => { 
                        self.registers.h = self.registers.h.wrapping_sub(1);
                        self.dec_flags(self.registers.h);
                    }
                    RegisterU8::L => { 
                        self.registers.l = self.registers.l.wrapping_sub(1);
                        self.dec_flags(self.registers.l);
                    }
                }
            }
            Instruction::CCF => {
                // zero flag not affected
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
            }
            Instruction::SCF => {
                // zero flag not affected
                self.registers.f.subtract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
            }
            Instruction::RRA => self.registers.a = self.rr(self.registers.a),
            Instruction::RLA => self.registers.a = self.rl(self.registers.a),
            Instruction::RRCA => self.registers.a = self.rrc(self.registers.a),
            Instruction::RLCA => self.registers.a = self.rlc(self.registers.a),
            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                // zero flag is not affected
                self.registers.f.subtract = true;
                self.registers.f.half_carry = true;
                // carry flag is not affected
            }
            Instruction::BIT(bit, target) => {
                match target {
                    RegisterU8::A => self.bit(bit, self.registers.a),
                    RegisterU8::B => self.bit(bit, self.registers.b),
                    RegisterU8::C => self.bit(bit, self.registers.c),
                    RegisterU8::D => self.bit(bit, self.registers.d),
                    RegisterU8::E => self.bit(bit, self.registers.e),
                    RegisterU8::H => self.bit(bit, self.registers.h),
                    RegisterU8::L => self.bit(bit, self.registers.l),
                }
            }
            Instruction::RES(bit, target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.res(bit, self.registers.a),
                    RegisterU8::B => self.registers.b = self.res(bit, self.registers.b),
                    RegisterU8::C => self.registers.c = self.res(bit, self.registers.c),
                    RegisterU8::D => self.registers.d = self.res(bit, self.registers.d),
                    RegisterU8::E => self.registers.e = self.res(bit, self.registers.e),
                    RegisterU8::H => self.registers.h = self.res(bit, self.registers.h),
                    RegisterU8::L => self.registers.l = self.res(bit, self.registers.l),
                }
            }
            Instruction::SET(bit, target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.set(bit, self.registers.a),
                    RegisterU8::B => self.registers.b = self.set(bit, self.registers.b),
                    RegisterU8::C => self.registers.c = self.set(bit, self.registers.c),
                    RegisterU8::D => self.registers.d = self.set(bit, self.registers.d),
                    RegisterU8::E => self.registers.e = self.set(bit, self.registers.e),
                    RegisterU8::H => self.registers.h = self.set(bit, self.registers.h),
                    RegisterU8::L => self.registers.l = self.set(bit, self.registers.l),
                }
            }
            Instruction::SRL(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.srl(self.registers.a),
                    RegisterU8::B => self.registers.b = self.srl(self.registers.b),
                    RegisterU8::C => self.registers.c = self.srl(self.registers.c),
                    RegisterU8::D => self.registers.d = self.srl(self.registers.d),
                    RegisterU8::E => self.registers.e = self.srl(self.registers.e),
                    RegisterU8::H => self.registers.h = self.srl(self.registers.h),
                    RegisterU8::L => self.registers.l = self.srl(self.registers.l),
                }
            }
            Instruction::RR(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.rr(self.registers.a),
                    RegisterU8::B => self.registers.b = self.rr(self.registers.b),
                    RegisterU8::C => self.registers.c = self.rr(self.registers.c),
                    RegisterU8::D => self.registers.d = self.rr(self.registers.d),
                    RegisterU8::E => self.registers.e = self.rr(self.registers.e),
                    RegisterU8::H => self.registers.h = self.rr(self.registers.h),
                    RegisterU8::L => self.registers.l = self.rr(self.registers.l),
                }
            }
            Instruction::RL(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.rl(self.registers.a),
                    RegisterU8::B => self.registers.b = self.rl(self.registers.b),
                    RegisterU8::C => self.registers.c = self.rl(self.registers.c),
                    RegisterU8::D => self.registers.d = self.rl(self.registers.d),
                    RegisterU8::E => self.registers.e = self.rl(self.registers.e),
                    RegisterU8::H => self.registers.h = self.rl(self.registers.h),
                    RegisterU8::L => self.registers.l = self.rl(self.registers.l),
                }
            }
            Instruction::RRC(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.rrc(self.registers.a),
                    RegisterU8::B => self.registers.b = self.rrc(self.registers.b),
                    RegisterU8::C => self.registers.c = self.rrc(self.registers.c),
                    RegisterU8::D => self.registers.d = self.rrc(self.registers.d),
                    RegisterU8::E => self.registers.e = self.rrc(self.registers.e),
                    RegisterU8::H => self.registers.h = self.rrc(self.registers.h),
                    RegisterU8::L => self.registers.l = self.rrc(self.registers.l),
                }
            }
            Instruction::RLC(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.rlc(self.registers.a),
                    RegisterU8::B => self.registers.b = self.rlc(self.registers.b),
                    RegisterU8::C => self.registers.c = self.rlc(self.registers.c),
                    RegisterU8::D => self.registers.d = self.rlc(self.registers.d),
                    RegisterU8::E => self.registers.e = self.rlc(self.registers.e),
                    RegisterU8::H => self.registers.h = self.rlc(self.registers.h),
                    RegisterU8::L => self.registers.l = self.rlc(self.registers.l),
                }
            }
            Instruction::SRA(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.sra(self.registers.a),
                    RegisterU8::B => self.registers.b = self.sra(self.registers.b),
                    RegisterU8::C => self.registers.c = self.sra(self.registers.c),
                    RegisterU8::D => self.registers.d = self.sra(self.registers.d),
                    RegisterU8::E => self.registers.e = self.sra(self.registers.e),
                    RegisterU8::H => self.registers.h = self.sra(self.registers.h),
                    RegisterU8::L => self.registers.l = self.sra(self.registers.l),
                }
            }
            Instruction::SLA(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.sla(self.registers.a),
                    RegisterU8::B => self.registers.b = self.sla(self.registers.b),
                    RegisterU8::C => self.registers.c = self.sla(self.registers.c),
                    RegisterU8::D => self.registers.d = self.sla(self.registers.d),
                    RegisterU8::E => self.registers.e = self.sla(self.registers.e),
                    RegisterU8::H => self.registers.h = self.sla(self.registers.h),
                    RegisterU8::L => self.registers.l = self.sla(self.registers.l),
                }
            }
            Instruction::SWAP(target) => {
                match target {
                    RegisterU8::A => self.registers.a = self.swap(self.registers.a),
                    RegisterU8::B => self.registers.b = self.swap(self.registers.b),
                    RegisterU8::C => self.registers.c = self.swap(self.registers.c),
                    RegisterU8::D => self.registers.d = self.swap(self.registers.d),
                    RegisterU8::E => self.registers.e = self.swap(self.registers.e),
                    RegisterU8::H => self.registers.h = self.swap(self.registers.h),
                    RegisterU8::L => self.registers.l = self.swap(self.registers.l),
                }
            }
            // _ => todo!("implement all CPU Instructions")
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (sum, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = did_overflow;

        sum
    }

    fn add_u16(&mut self, value: u16) -> u16 {
        let (sum, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        /* the LR-CPU has a 4bit ALU, so I believe adding from least to most significant would
           mean the half_carry reflects the upper byte, checking @ bit 11 & 12. */
        self.registers.f.half_carry = ((self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF)) & 0x1000 == 0x1000;
        self.registers.f.carry = did_overflow;

        sum
    }

    fn adc(&mut self, value: u8) -> u8 {
        // I believe the CPU only cares about the end result of all internally compound ops like this,
        // so if that is true, the following logic/implementation should be correct
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (value, value_did_overflow) = value.overflowing_add(carry);
        let (sum, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = value_did_overflow || did_overflow;

        sum
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (diff, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = diff == 0;
        self.registers.f.subtract = true;
        // "Set if no borrow from bit 4."
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        // "Set if no borrow."
        self.registers.f.carry = !did_overflow;

        diff
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (value, value_did_overflow) = value.overflowing_add(carry);
        let (diff, did_overflow) = self.registers.a.overflowing_sub(value);
        
        self.registers.f.zero = diff == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        self.registers.f.carry = !(value_did_overflow || did_overflow);
        
        diff
    }

    fn and(&mut self, value: u8) -> u8 {
        let res = self.registers.a & value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;

        res
    }

    fn or(&mut self, value: u8) -> u8 {
        let res = self.registers.a | value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn xor(&mut self, value: u8) -> u8 {
        let res = self.registers.a ^ value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn cp(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // "Set if no borrow from bit 4."
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        // "Set FOR no borrow."
        self.registers.f.carry = self.registers.a < value;
    }

    // sets the proper flags after an inc operation
    fn inc_flags(&mut self, value: u8) {
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0; 
        // carry flag is """not affected""" apparently ¯\_(ツ)_/¯ 
    }

    // sets the proper flags after a dec operation
    fn dec_flags(&mut self, value: u8) {
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value.wrapping_add(1) & 0xF) > 0;
        // carry flag is not affected
    }

    fn bit(&mut self, bit: u8, value: u8) {
        self.registers.f.zero = (value & (0b1 << bit)) == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        // carry flag is not affected
    }

    fn res(&mut self, bit: u8, value: u8) -> u8 {
        if (value & (0b1 << bit)) == (0b1 << bit) {
            value - (0b1 << bit)
        } else {
            value
        }
        // all flags are not affected
    }

    fn set(&mut self, bit: u8, value: u8) -> u8 {
        if (value & (0b1 << bit)) != (0b1 << bit) {
            value + (0b1 << bit)
        } else {
            value
        }
        // all flags are not affected
    }

    fn srl(&mut self, value: u8) -> u8 {
        let res = value >> 1;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;

        res
    }

    // through the carry
    fn rr(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | ((self.registers.f.carry as u8) << 7);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
        
        res
    }

    // through the carry
    fn rl(&mut self, value: u8) -> u8 {
        let res = (value << 1) | (self.registers.f.carry as u8);

        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;
    
        res
    }

    // circular
    fn rrc(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | ((value & 0b1) << 7);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
    
        res
    }

    // circular
    fn rlc(&mut self, value: u8) -> u8 {
        let res = (value << 1) | (value & (0b1 << 7)) >> 7;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;
    
        res
    }

    fn sra(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | (value & (0b1 << 7));
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
    
        res
    }

    fn sla(&mut self, value: u8) -> u8 {
        let res = value << 1;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;

        res
    }

    fn swap(&mut self, value: u8) -> u8 {
        let res = ((value & 0xF0) >> 4) | ((value & 0x0F) << 4);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }
}