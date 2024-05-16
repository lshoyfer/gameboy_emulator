//! This module contains helper macros specifically used in the 
//! implementation of the [Instruction] enum of the  [`crate::cpu::instruction`] module
//! 
//! [Instruction]: crate::cpu::instruction::InstructionType

macro_rules! arithmetic_u8_impl {
    ($cmd:path, $input:expr) => {
        Some(
            Instruction::ArithmeticLogical8Bit(
                $cmd($input)
            )
        )
    };
    ($argless_cmd:expr) => {
        Some(
            Instruction::ArithmeticLogical8Bit(
                $argless_cmd
            )
        )
    }
}

macro_rules! arithmetic_u16_impl {
    ($cmd:path, $input:expr) => {
        Some(
            Instruction::ArithmeticLogical16Bit(
                $cmd($input)
            )
        )
    };
    ($argless_cmd:expr) => {
        Some(
            Instruction::ArithmeticLogical16Bit(
                $argless_cmd
            )
        )
    }
}

pub(super) use arithmetic_u8_impl;
pub(super) use arithmetic_u16_impl;