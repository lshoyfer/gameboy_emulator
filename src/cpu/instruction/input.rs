//! Module containing items for CPU command input management of registers

use crate::cpu::register::{ RegisterU8, RegisterU16 };

/// For 8-bit Operations that have 3 possible variations: register manipulation, immediate-in-memory addressing, HL addressing
pub enum CompoundInputU8 {
    /// Use the given 8-bit register in the operation
    Register(RegisterU8), // x
    /// Use the data immediately following the opcode in memory
    Immediate, // # -- unsure how this is gonna work, first need to handle memory / clock cycle stuff before coming back to it
    /// Use the data that HL points to aka treat HL as an addr and deref it (*HL)
    Address // (HL)
}

/// For 8-bit Operations that have 2 possible variations: register manipulation AND HL addressing -- these DON'T have immediate-in-memory addressing 
pub enum DoubleInputU8 {
    /// Use the given 8-bit register in the operation
    Register(RegisterU8), // x
    /// Use the data that HL points to aka treat HL as an addr and deref it (*HL)
    Address // (HL)
}

// For 8-bit Operations that only take basic register manipulation inputs
pub struct InputU8(pub RegisterU8);

/// For 16-bit Operations that only take basic register manipulation inputs 
/// NOTE: 16-bit Ops only take register inputs
pub struct InputU16(pub RegisterU16);

/// 8-bit Signed Number Input tho tbh idk where it comes from so this may be removed, I gotta figure it out
pub struct InputI8(pub i8);

/// For [`JmpCmd::JP`] which has an input with 3 possible variations
/// 
/// [`JmpCmd::JP`]: super::commands::JmpCmd::JP
pub enum JPInput {
    /// Jump to the address immediately given in the next two bytes of memory
    Direct,
    /// Jump to the address in register HL
    HL,
    /// Conditionally jump to the address immediately given in the next two bytes of memory
    Conditional(JmpCmdCondition)
}

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
