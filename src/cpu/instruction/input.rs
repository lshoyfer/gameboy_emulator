//! Module containing items for CPU command input management of registers
//! -- May be mostly gutted since they are just type aliases currently that
//! don't really provide any help and may reduce clarity when destructuring

use crate::cpu::register::{ RegisterU8, RegisterU16 };

/// For 8-bit Operations that have 3 possible variations: register manipulation, immediate-in-memory addressing, HL addressing
pub enum CompoundInputU8 {
    Register(RegisterU8), // x
    Immediate, // # -- unsure how this is gonna work, first need to handle memory / clock cycle stuff before coming back to it
    Address // (HL)
}

// Basic 8-bit Register Input
pub struct InputU8(pub RegisterU8);

/// 16-bit Ops only take register inputs
pub struct InputU16(pub RegisterU16);
/// 8-bit Signed Number Input tho tbh idk where it comes from so this may be removed, I gotta figure it out
pub struct InputI8(pub i8);