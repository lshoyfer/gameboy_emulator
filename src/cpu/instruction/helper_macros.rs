//! This module contains helper macros specifically used in the 
//! implementation of the [Instruction] enum of the  [`crate::cpu::instruction`] module.
//! 
//! The motivation behind these is for clarity, organization, and code deduplication.
//! 
//! [Instruction]: crate::cpu::instruction::Instruction


/// Meta-macro that makes an implementation macro for a specified Instruction variant.
/// The implementation macro created takes a path to a nested Cmd enum relevant
/// to the given top level Instruction variant and an input for that cmd enum.
/// When invoked, the implementation macro will correctly expand to satisfy the return
/// value of [`Instruction::from_byte`] and its inner helper methods. The meta-macro
/// also exports its defined implementation macro from this module.
/// 
/// [`Instruction::from_byte`]: crate::cpu::instruction::Instruction::from_byte
macro_rules! make_macro {
    ($macro_name:ident, $instruction:path) => {
        macro_rules! $macro_name {
            ($cmd:path, $input:expr) => {
                Some(
                    $instruction(
                        $cmd($input)
                    )
                )
            };
        }

        pub(super) use $macro_name;
    }
}

/// Two armed version of make_macro that adds an arm to support inputless commands 
macro_rules! make_macro_with_no_input {
    ($macro_name:ident, $instruction:path) => {
        macro_rules! $macro_name {
            ($cmd:path, $input:expr) => {
                Some(
                    $instruction(
                        $cmd($input)
                    )
                )
            };
            ($no_input_cmd:expr) => {
                Some(
                    $instruction(
                        $no_input_cmd
                    )
                )
            };
        }
        
        pub(super) use $macro_name;
    }
}

// Instantiating the implementation macros' definitions
make_macro_with_no_input!(arithmetic_u8_impl, Instruction::ArithmeticLogical8Bit);
make_macro_with_no_input!(arithmetic_u16_impl, Instruction::ArithmeticLogical16Bit);
make_macro!(jump_impl, Instruction::Jump);
make_macro!(load_u8_impl, Instruction::Load8Bit);
make_macro!(load_u16_impl, Instruction::Load16Bit);