mod input;
mod commands;
mod helper_macros;

use helper_macros::*;

// ---- RE-EXPORTS:
// The intention is to glob import everything from this subtree 
// (instruction and its children) into the cpu.rs module --
// I want the interface to be through the instruction module without
// a deeply nested namespace system -- the module file structure atm is merely
// internal organization/compartmentalization.
// Basically, this is the "instruction-wing" or "instruction-pillar" of the CPU,
// I import from it everything I need to do to handle encoding, decoding, calling, 
// and implementing instructions, which will involve commands and inputs, etc...
pub use input::*;
pub use commands::*;




/// Enumeration of all CPU Instructions that categorizes all instructions into groups/types
/// 
/// Each group/type contains a nested enum of all possible commands for that group/type
/// 
/// Example:
///     An [`Instruction::Control`] can contain a [`CtrCmd`] of variant `NOP` which corresponds to the nop (no-op) CPU instruction
///     or perhaps an `EI` variant which corresponds to the ei CPU instruction which enables interrupts
///
/// I could flatten this for no change in functionality, but I accept this 
/// level of nesting for organizational/reference purposes.
pub enum Instruction {
    Load8Bit(LoadU8Cmd),
    Load16Bit(LoadU16Cmd),
    ArithmeticLogical8Bit(AritLogiU8Cmd),
    ArithmeticLogical16Bit(AritLogiU16Cmd),
    RotateShift(RSCmd), 
    SingleBit(BitCmd),
    Control(CtrCmd),
    Jump(JmpCmd)
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Self> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_not_prefixed(byte)
        }
    }

    /// NOTE, only RotateShift & SingleBit contain prefixed commands
    pub fn from_byte_prefixed(byte: u8) -> Option<Self> {
        match byte {
/* START || Rotate & Shift Commands || START */
            // todo!()
/* END || Rotate & Shift Commands || END */

/* START || Single Bit Operation Commands || START */
            // todo!()
/* END || Single Bit Operation Commands || END */
            _ => None // either unimplemented or unrecognized
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Self> {
        use crate::cpu::register::RegisterU8::{ A, B, C, D, E, H, L };
        use crate::cpu::register::RegisterU16::{ BC, DE, HL, SP };

        /* Comment Guide (note it's not an exhaustive/perfectly enunciated syntax--
            --i.e. use some intuition/critical thinking on some outside the exact format :P) */
        /* INS R,_ | addr | C | xxxx | eq
        where 
            INS := Instruction Name
            R := register the operation is affecting, usually A, sometimes HL, sometimes custom 8 or 16 bit
            _ := input (actually generally just a piece of the opcode, 
            not quite an "input" in the traditional sense), where possible inputs are
                r := a register;
                rr := 16 bit compound register (BC, DE, HL, SP (Stack Pointer))
                n/nn := immediate value
                (HL) := the value at the address in compound 16bit register HL (deref of HL -- *HL)
                dd := signed 8-bit number
            addr := address encoding in hexadecimal (sometimes generic, i.e. having a variable to encode multiple possible inputs)
            C := # clock cycles
            xxxx := registers affected (ordered as znhc - zero, negative, half-carry, carry)
                    letter itself is a calculation, 0 is reset, 1 is set, - is unaffected
            eq := equation demonstrating the operation mathematically
                cy := carry bit
        */
        match byte {
/* TODO STATUS: 8-bit Loads, 16-bit Loads, DAA, Most 16-bit Arithmetic, Rotates & Shifts, Bit Ops, CPU Control, Jumpcommands */
/* START || 8-bit Load Commands || START */
            // todo!()
/* END || 8-bit Load Commands || END */

/* START || 16-bit Load Commands || START */
            // todo!()
/* END || 16-bit Load Commands || END */

/* START || 8bit-Arithmetic/logical Commands || START */
            /* STATUS: All enumerated. All done except for DAA. */
            // ADD A,r | 8x | 4 | z0hc | A=A+r
            0x87 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(A)),
            0x80 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(B)),
            0x81 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(C)),
            0x82 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(D)),
            0x83 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(E)),
            0x84 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(H)),
            0x85 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Register(L)),
            // ADD A,n | C6 nn | 8 | z0hc | A=A+n
            0xC6 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Immediate),
            // ADD A,(HL) | 86 | 8 | z0hc | A=A+(HL)
            0x86 => arithmetic_u8_impl!(AritLogiU8Cmd::ADD, CompoundInputU8::Address),

            // ADC A,r | 8x | 4 | z0hc | A=A+r+cy
            0x8F => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(A)),
            0x88 => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(B)),
            0x89 => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(C)),
            0x8A => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(D)),
            0x8B => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(E)),
            0x8C => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(H)),
            0x8D => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Register(L)),
            // ADC A,n | CE nn | 8 | z0hc | A=A+n+cy
            0xCE => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Immediate),
            // ADC A,(HL) | 8E | 8 | z0hc | A=A+(HL)+cy
            0x8E => arithmetic_u8_impl!(AritLogiU8Cmd::ADC, CompoundInputU8::Address),

            // SUB A,r | 9x | 4 | z1hc | A=A-r
            0x97 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(A)),
            0x90 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(B)),
            0x91 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(C)),
            0x92 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(D)),
            0x93 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(E)),
            0x94 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(H)),
            0x95 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Register(L)),
            // SUB A,n | D6 nn | 8 | z1hc | A=A-n
            0xD6 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Immediate),
            // ADD A,(HL) | 96 | 8 | z1hc | A=A-(HL)
            0x96 => arithmetic_u8_impl!(AritLogiU8Cmd::SUB, CompoundInputU8::Address),

            // SBC A,r | 9x | 4 | z1hc | A=A-r-cy
            0x9F => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(A)),
            0x98 => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(B)),
            0x99 => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(C)),
            0x9A => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(D)),
            0x9B => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(E)),
            0x9C => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(H)),
            0x9D => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Register(L)),
            // SBC A,n | DE nn | 8 | z1hc | A=A-n-cy
            0xDE => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Immediate),
            // SBC A,(HL) | 9E | 8 | z1hc | A=A-(HL)-cy
            0x9E => arithmetic_u8_impl!(AritLogiU8Cmd::SBC, CompoundInputU8::Address),

            // AND A,r | Ax | 4 | z010 | A=A&r
            0xA7 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(A)),
            0xA0 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(B)),
            0xA1 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(C)),
            0xA2 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(D)),
            0xA3 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(E)),
            0xA4 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(H)),
            0xA5 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Register(L)),
            // AND A,n | E6 nn | 8 | z010 | A=A&n
            0xE6 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Immediate),
            // AND A,(HL) | A6 | 8 | z010 | A=A&(HL)
            0xA6 => arithmetic_u8_impl!(AritLogiU8Cmd::AND, CompoundInputU8::Address),

            // XOR A,r | Ax | 4 | z000 | A=A^r
            0xAF => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(A)),
            0xA8 => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(B)),
            0xA9 => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(C)),
            0xAA => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(D)),
            0xAB => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(E)),
            0xAC => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(H)),
            0xAD => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Register(L)),
            // XOR A,n | EE nn | 8 | z000 | A=A^n
            0xEE => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Immediate),
            // XOR A,(HL) | AE | 8 | z000 | A=A^(HL)
            0xAE => arithmetic_u8_impl!(AritLogiU8Cmd::XOR, CompoundInputU8::Address),

            // OR A,r | Bx | 4 | z000 | A=A|r
            0xB7 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(A)),
            0xB0 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(B)),
            0xB1 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(C)),
            0xB2 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(D)),
            0xB3 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(E)),
            0xB4 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(H)),
            0xB5 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Register(L)),
            // OR A,n | F6 nn | 8 | z000 | A=A|n
            0xF6 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Immediate),
            // OR A,(HL) | B6 | 8 | z000 | A=A|(HL)
            0xB6 => arithmetic_u8_impl!(AritLogiU8Cmd::OR, CompoundInputU8::Address),

            // CP A,r | Bx | 4 | z1hc | compare A-r
            0xBF => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(A)),
            0xB8 => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(B)),
            0xB9 => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(C)),
            0xBA => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(D)),
            0xBB => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(E)),
            0xBC => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(H)),
            0xBD => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Register(L)),
            // CP A,n | FE nn | 8 | z1hc | compare A-n
            0xFE => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Immediate),
            // CP A,(HL) | BE | 8 | z1hc | compare A-(HL)
            0xBE => arithmetic_u8_impl!(AritLogiU8Cmd::CP, CompoundInputU8::Address),

            // INC r | xx | 4 | z0h- | r=r+1
            0x3C => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(A)),
            0x04 => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(B)),
            0x0C => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(C)),
            0x14 => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(D)),
            0x1C => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(E)),
            0x24 => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(H)),
            0x2C => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Register(L)),
            // INC (HL) | 34 | 12 | z0h- | (HL)=(HL)+1
            0x34 => arithmetic_u8_impl!(AritLogiU8Cmd::INC, CompoundInputU8::Address),

            // DEC r | xx | 4 | z1h- | r=r-1
            0x3D => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(A)),
            0x05 => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(B)),
            0x0D => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(C)),
            0x15 => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(D)),
            0x1D => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(E)),
            0x25 => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(H)),
            0x2D => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Register(L)),
            // DEC (HL) | 35 | 12 | z1h- | (HL)=(HL)-1
            0x35 => arithmetic_u8_impl!(AritLogiU8Cmd::DEC, CompoundInputU8::Address),
            
            // DAA | 27 | 4 | z-0x | decimal adjust accumulator (A)
            0x27 => todo!("implement DAA on CPU"),
            
            // CPL | 2F | 4 | -11- | A=!A aka A^0xFF
            0x2F => arithmetic_u8_impl!(AritLogiU8Cmd::CPL),
/* END || 8bit-Arithmetic/logical Commands || END */

/* START || 16bit-Arithmetic/logical Commands || START */
            /* STATUS: All enumerated. Most unimplemented. */
            // ADD HL,rr | x9 | 8 | -0hc | HL=HL+rr
            0x09 => arithmetic_u16_impl!(AritLogiU16Cmd::ADDHL, InputU16(BC)),
            0x19 => arithmetic_u16_impl!(AritLogiU16Cmd::ADDHL, InputU16(DE)),
            0x29 => arithmetic_u16_impl!(AritLogiU16Cmd::ADDHL, InputU16(HL)),
            0x39 => arithmetic_u16_impl!(AritLogiU16Cmd::ADDHL, InputU16(SP)),

            // INC rr | x3 | 8 | ---- | rr=rr+1
            0x03 => todo!("Implement 16-bit BC-register increment instruction on CPU"),
            0x13 => todo!("Implement 16-bit DE-register increment instruction on CPU"),
            0x23 => todo!("Implement 16-bit HL-register increment instruction on CPU"),
            0x33 => todo!("Implement 16-bit SP-register increment instruction on CPU"),
            
            // DEC rr | xB | 8 | ---- | rr=rr-1
            0x0B => todo!("Implement 16-bit BC-register decrement instruction on CPU"),
            0x1B => todo!("Implement 16-bit DE-register decrement instruction on CPU"),
            0x2B => todo!("Implement 16-bit HL-register decrement instruction on CPU"),
            0x3B => todo!("Implement 16-bit SP-register decrement instruction on CPU"),

            // ADD SP,dd | E8 | 16 | 00hc | SP=SP +/- dd
            0xE8 => todo!("Implement ADDSP"),

            // LD HL,SP+dd | F8 | 12 | 00hc | HL = SP +/- dd
            0xF8 => todo!("Implement LDHL"),
/* END || 16bit-Arithmetic/logical Commands || END */

/* START || Rotate & Shift Commands || START */
            // todo!()
            // NOTE: MOST of these are prefixed
/* END || Rotate & Shift Commands || END */

/* START || Single Bit Operation Commands || START */
            // NOTE: ALL of these are prefixed
/* END || Single Bit Operation Commands || END */

/* START || CPU Control Commands || START */
            // todo!()
/* END || CPU Control Commands || END */

/* START || Jump Commands || START */
            // todo!()
/* END || Jump Commands || END */

            _ => None // either unimplemented or unrecognized
        }
    }

}
