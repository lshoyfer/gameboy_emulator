//! Module containing items for CPU command input management of registers

use crate::cpu::register::{ RegisterU8, RegisterU16 };

/// For 8-bit Operations that have 3 possible variations: register manipulation, immediate-in-memory addressing, HL addressing
pub enum CompoundInputU8 {
    Register(RegisterU8), // x
    Immediate, // # -- unsure how this is gonna work, first need to handle memory / clock cycle stuff before coming back to it
    Address // (HL)
}

/// For 8-bit Operations that have 2 possible variations: register manipulation AND HL addressing -- these DON'T have immediate-in-memory addressing 
pub enum DoubleInputU8 {
    Register(RegisterU8), // x
    Address // (HL)
}

// For 8-bit Operations that only take basic register manipulation inputs
pub struct InputU8(pub RegisterU8);

/// For 16-bit Operations that only take basic register manipulation inputs 
/// NOTE: 16-bit Ops only take register inputs
pub struct InputU16(pub RegisterU16);

/// 8-bit Signed Number Input tho tbh idk where it comes from so this may be removed, I gotta figure it out
pub struct InputI8(pub i8);