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
                r := a register
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
            // LD r,r | xx | 4 | ---- | r=r
                /* A,r | 7x */
            0x7F => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, A)),
            0x78 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, B)),
            0x79 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, C)),
            0x7A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, D)),
            0x7B => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, E)),
            0x7C => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, H)),
            0x7D => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(A, L)),
                /* B, r | 4x */
            0x40 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, B)),
            0x41 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, C)),
            0x42 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, D)),
            0x43 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, E)),
            0x44 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, H)),
            0x45 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(B, L)),
                /* C, r | 4x */
            0x48 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, B)),
            0x49 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, C)),
            0x4A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, D)),
            0x4B => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, E)),
            0x4C => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, H)),
            0x4D => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(C, L)),
                /* D, r | 5x */
            0x50 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, B)),
            0x51 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, C)),
            0x52 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, D)),
            0x53 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, E)),
            0x54 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, H)),
            0x55 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(D, L)),
                /* E, r | 5x */
            0x58 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, B)),
            0x59 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, C)),
            0x5A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, D)),
            0x5B => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, E)),
            0x5C => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, H)),
            0x5D => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(E, L)),
                /* H, r | 6x */
            0x60 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, B)),
            0x61 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, C)),
            0x62 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, D)),
            0x63 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, E)),
            0x64 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, H)),
            0x65 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(H, L)),
                /* L, r | 6x */
            0x68 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, B)),
            0x69 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, C)),
            0x6A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, D)),
            0x6B => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, E)),
            0x6C => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, H)),
            0x6D => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RR(L, L)),
            // LD r,n | xx nn | 8 | ---- | r=n
            0x3E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(A)),
            0x06 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(B)),
            0x0E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(C)),
            0x16 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(D)),
            0x1E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(E)),
            0x26 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(H)),
            0x2E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RI(L)),
            // LD r,(HL) | xx | 8 | ---- | r=(HL)
            0x7E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(A)),
            0x46 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(B)),
            0x4E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(C)),
            0x56 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(D)),
            0x5E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(E)),
            0x66 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(H)),
            0x6E => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::RHL(L)),
            // LD (HL),r | 7x | 8 | ---- | (HL)=r
            0x70 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(B)),
            0x71 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(C)),
            0x72 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(D)),
            0x73 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(E)),
            0x74 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(H)),
            0x75 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLR(L)),
            // LD (HL),n | 36 nn | 12 | ---- | (HL)=n
            0x36 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::HLI),
            // LD A,(BC) | 0A | 8 | ---- | A=(BC)
            0x0A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::ABC),
            // LD A,(DE) | 1A | 8 | ---- | A=(DE)
            0x1A => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::ADE),
            // LD A,(nn) | 1A | 16 | ---- | A=(nn)
            0xFA => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::AII),
            // LD (BC),A | 02 | 8 | ---- | (BC)=A
            0x02 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::BCA),
            // LD (DE),A | 12 | 8 | ---- | (DE)=A
            0x12 => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::DEA),
            // LD (nn),A | EA | 16 | ---- | (nn)=A
            0xEA => load_u8_impl!(LoadU8Cmd::LD, LDInputU8::IIA),
        // todo!("IO-Port Loads")

            // LDI (HL),A | 22 | 8 | ---- | (HL)=A; HL=HL+1
            0x22 => load_u8_impl!(LoadU8Cmd::LDI, LDIInputU8::HLA),
            // LDI A,(HL) | 2A | 8 | ---- | A=(HL); HL=HL+1
            0x2A => load_u8_impl!(LoadU8Cmd::LDI, LDIInputU8::AHL),

            // LDD (HL),A | 32 | 8 | ---- | (HL)=A; HL=HL-1
            0x32 => load_u8_impl!(LoadU8Cmd::LDD, LDDInputU8::HLA),
            // LDD A,(HL) | 3A | 8 | ---- | A=(HL); HL=HL-1
            0x3A => load_u8_impl!(LoadU8Cmd::LDD, LDDInputU8::AHL),


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
            // JP nn | C3 nn nn | 16 | ---- | PC=nn
            0xC3 => jump_impl!(JmpCmd::JP, JPInput::Direct),
            // JP HL | E9 | 4 | ---- | PC=HL
            0xE9 => jump_impl!(JmpCmd::JP, JPInput::HL),
            // JP f,nn | xx nn nn | 16/12 | ---- | conditional
            0xC2 => jump_impl!(JmpCmd::JP, JPInput::Conditional(JmpCmdCondition::NZ)),
            0xCA => jump_impl!(JmpCmd::JP, JPInput::Conditional(JmpCmdCondition::Z)),
            0xD2 => jump_impl!(JmpCmd::JP, JPInput::Conditional(JmpCmdCondition::NC)),
            0xDA => jump_impl!(JmpCmd::JP, JPInput::Conditional(JmpCmdCondition::C)),

            // JR 

        
        
/* END || Jump Commands || END */

            _ => None // either unimplemented or unrecognized
        }
    }

}
