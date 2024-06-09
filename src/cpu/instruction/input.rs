//! Module containing items for CPU command input management of registers
//! NOTE: It is not entirely perfectly sensical or consistent, as I am messing
//! with different design types/just want to experiment with Rust typing.

use crate::cpu::register::{ RegisterU8, RegisterU16 };

/// For 8-bit Operations that have 3 possible variations: register manipulation, immediate-in-memory addressing, HL addressing
#[cfg_attr(test, derive(Debug))]
pub enum CompoundInputU8 {
    /// Use the given 8-bit register in the operation
    Register(RegisterU8), // x
    /// Use the data immediately following the opcode in memory
    Immediate, // # -- unsure how this is gonna work, first need to handle memory / clock cycle stuff before coming back to it
    /// Use the data that HL points to aka treat HL as an addr and deref it (*HL)
    Address // (HL)
}

/// For 8-bit Operations that have 2 possible variations: register manipulation AND HL addressing -- these DON'T have immediate-in-memory addressing 
#[cfg_attr(test, derive(Debug))]
pub enum DoubleInputU8 {
    /// Use the given 8-bit register in the operation
    Register(RegisterU8), // x
    /// Use the data that HL points to aka treat HL as an addr and deref it (*HL)
    Address // (HL)
}

/// For 8-bit Operations that only take basic register manipulation inputs.
/// This is only used if there is no nesting of Input types (i.e. [`BitInput`] uses
/// [`RegisuterU8`] instead of [`InputU8`])
#[cfg_attr(test, derive(Debug))]
pub struct InputU8(pub RegisterU8);

/// For 16-bit Operations that only take basic register manipulation inputs 
/// NOTE: 16-bit Ops only take register inputs (if they do take any special inputs at all, that is)
#[cfg_attr(test, derive(Debug))]
pub struct InputU16(pub RegisterU16);

/// For [`JmpCmd::JP`] which has an input with 3 possible variations
/// 
/// [`JmpCmd::JP`]: super::JmpCmd::JP
#[cfg_attr(test, derive(Debug))]
pub enum JPInput {
    /// Jump to the address immediately given in the next two bytes of memory
    Direct,
    /// Jump to the address in register HL
    HL,
    /// Conditionally jump to the address immediately given in the next two bytes of memory
    Conditional(JmpCmdCondition)
}

// todo!("consider type alaising JmpCmdInput for diff cmds like you did for loads")

/// For all variants of [`JmpCmd`] with inputs containing 2 possible variations. 
///
/// This excludes [`JmpCmd::JP`] and [`JmpCmd::RST`], which can have
/// inputs with 3 variations and 1 variation respectively.
///
/// NOTE: [`JmpCmd::RETI`] has no input.
/// 
/// [`JmpCmd`]: super::JmpCmd
/// [`JmpCmd::JP`]: super::JmpCmd::JP
/// [`JmpCmd::RST`]: super::JmpCmd::RST
/// [`JmpCmd::RETI`]: super::JmpCmd::RETI
#[cfg_attr(test, derive(Debug))]
pub enum JmpCmdInput {
    /// Has multiple meanings depending on the command.
    /// - [`JmpCmd::JR`], jump to PC + the value in the next byte of memory (relative jump)
    /// - [`JmpCmd::CALL`], todo!()
    /// - [`JmpCmd::RET`], todo!()
    ///
    /// [`JmpCmd::JR`]: super::JmpCmd::JR
    /// [`JmpCmd::CALL`]: super::JmpCmd::CALL
    /// [`JmpCmd::RET`]: super::JmpCmd::RET
    Direct,

    /// The same as the `Direct` variant but only done conditionally to 
    /// specified flags being set or reset
    Conditional(JmpCmdCondition)
}

/// For `Conditional` variants of inputs for Jump commands
#[cfg_attr(test, derive(Debug))]
pub enum JmpCmdCondition {
    /// do Direct operation if zero-flag is reset
    NZ,
    /// do Direct operation if zero-flag is set
    Z,
    /// do Direct operation if carry-flag is reset
    NC,
    /// do Direct operation if carry-flag is set
    C
}

/// For [`LoadU8Cmd::LD`] which is the most versatile CPU command.
/// 
/// Note, in online documentation of this CPU, parentheses around something
/// usually means deref of an address but here I'm using * for deref and
/// parentheses just for visual clarity of the items/relationships
/// involved in these LD inputs and also for normal prose additions.
/// 
/// Also note, 2-byte immediate values are LS-Byte first then MS-Byte in 
/// the memory bus.
/// 
/// [`LoadU8Cmd::LD`]: super::LoadU8Cmd::LD
#[cfg_attr(test, derive(Debug))]
pub enum LDInputU8 {
    /// Load into a (Register) from a (Register)
    RR(RegisterU8, RegisterU8),
    /// Load into a (Register) from an (Immediate) 1-byte value
    RI(RegisterU8),
    /// Load into a (Register) from (*HL)
    RHL(RegisterU8),
    /// Load into (*HL) from a (Register)
    HLR(RegisterU8),
    /// Load into (*HL) from an (Immediate) 1-byte value
    HLI,
    /// Load into register (A) from (*BC)
    ABC,
    /// Load into register (A) from (*DE)
    ADE,
    /// Load into register (A) from (*nn) (deref of 2-byte immediate value)
    AII,
    /// Load into (*BC) from register (A)
    BCA,
    /// Load into (*DE) from register (A)
    DEA,
    /// Load into (*nn) (deref of 2-byte immediate value) from register (A)
    IIA, 

// todo!("io-ports aren't yet implemented/designed/considered")
// START IO-ports
    /// Load into register (A) from (IO-port N) (0xFFF0+n)
    ReadIoN,
    /// Load into (IO-port N) (0xFFF0+n) from register (A)
    WriteIoN,

    /// Load into register (A) from (IO-port C) (0xFFF0+C)
    ReadIoC,
    /// Load into (IO-port C) (0xFFF0+C) from register (A)
    WriteIoC
// END IO-ports
}

/// For [`LoadU8Cmd::LDI`] and [`LoadU8Cmd::LDD`] which have identical input variants
/// 
/// [`LoadU8Cmd::LDI`]: super::LoadU8Cmd::LDI
/// [`LoadU8Cmd::LDD`]: super::LoadU8Cmd::LDD
#[cfg_attr(test, derive(Debug))]
pub enum LDIncDecInputU8 {
    /// Load into (*HL) from register (A) and increment (LDI) or decrement (LDD) register (HL) 
    HLA,
    /// Load into register (A) from (*HL) and increment (LDI) or decrement (LDD) register (HL) 
    AHL
}

/// Type alias for LDI of [`LDIncDecInputU8`]
pub type LDIInputU8 = LDIncDecInputU8;
/// Type alias for LDD of [`LDIncDecInputU8`]
pub type LDDInputU8 = LDIncDecInputU8;

/// For [`LoadU16Cmd::LD`].
/// 
/// Note, in online documentation of this CPU, parentheses around something
/// usually means deref of an address but here I'm using * for deref and
/// parentheses just for visual clarity of the items/relationships
/// involved in these LD inputs and also for normal prose additions.
/// 
/// Also note, 2-byte immediate values are LS-Byte first then MS-Byte in 
/// the memory bus.
/// 
/// [`LoadU16Cmd::LD`]: super::LoadU16Cmd::LD
#[cfg_attr(test, derive(Debug))]
pub enum LDInputU16 {
    /// Load into (*rr) from (*nn)
    RRNN(RegisterU16),
    /// Load into direct SP from direct HL; (note the lack of deref/parenthesis)
    SPHL
}

/// Type alias for a byte -- signals which bit the BitCmd should operate on
pub type U3 = u8;

/// The input type for [`BitCmd`]. The [`BitOperator`] signals which bit for the command
/// to operate on. The [`DoubleInputU8`] signals which register's (or *(HL)'s) bits to operate on.
#[cfg_attr(test, derive(Debug))]
pub struct BitInput(pub U3, pub DoubleInputU8);

impl BitInput {
    pub fn from_opcode(opcode: u8) -> Self {
        eprintln!("FROM OPCODE RECEIVING: {opcode:#b}");
        let reg_code = opcode & 0b0000_0111;
        let bit_index = (opcode & 0b0011_1000) >> 3;
        dbg!(bit_index);
        eprintln!("{bit_index:#b}");
        // let cmd_code = (opcode & 0b1100_0000) >> 6; /* I'll let macros do this since for better or worse that is how i have done it thus far */
        
        let reg_input = match reg_code {
            0 => DoubleInputU8::Register(RegisterU8::B),
            1 => DoubleInputU8::Register(RegisterU8::C),
            2 => DoubleInputU8::Register(RegisterU8::D),
            3 => DoubleInputU8::Register(RegisterU8::E),
            4 => DoubleInputU8::Register(RegisterU8::H),
            5 => DoubleInputU8::Register(RegisterU8::L),
            6 => DoubleInputU8::Address,
            7 => DoubleInputU8::Register(RegisterU8::A),
            _ => unreachable!("Match statement enumerates all possible values for a 3-bit number")
        };

        Self(bit_index, reg_input)
    }
}