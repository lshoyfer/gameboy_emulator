use super::*;
use crate::cpu::register::*;

#[test]
/// tests if Instruction builds the correct SingleBit instructions and BitInputs from an opcode
fn instruction_from_byte_prefixed_single_bit() {
    use RegisterU8::*;

    macro_rules! build_X_instruction {
        ($instruction:path, $bit:expr, $register:expr) => {
            Instruction::SingleBit($instruction(BitInput($bit, DoubleInputU8::Register($register))))
        };
    }
    
    fn n_to_reg_u8(n: u8) -> Option<RegisterU8> {
        match n {
            0 => Some(B),
            1 => Some(C),
            2 => Some(D),
            3 => Some(E),
            4 => Some(H),
            5 => Some(L),
            6 => None,
            7 => Some(A),
            _ => unreachable!("n_to_reg() is only ever passed 0..7 aka a u3")
        }
    }

    let bit_opcodes = 0x40..=0x7F;
    let res_opcodes = 0x80..=0xBF;
    let set_opcodes = 0xC0..=0xFF;

    // a b c d e h l + hl
    let bit_instructions = bit_opcodes.clone()
        .map(|code| {
            // this enumerates the raw table, while the code in Instruction follows an encoding
            let reg_code = (code & 0x0F) % 8;
            let bit = match code {
                0x40..=0x47 => 0,
                0x48..=0x4F => 1,
                0x50..=0x57 => 2,
                0x58..=0x5F => 3,
                0x60..=0x67 => 4,
                0x68..=0x6F => 5,
                0x70..=0x77 => 6,
                0x78..=0x7F => 7,
                _ => unreachable!("BIT opcodes range is 0x40..=0x7F which is fully covered")
            };

            if let Some(reg) = n_to_reg_u8(reg_code) {
                build_X_instruction!(BitCmd::BIT, bit, reg)
            } else {
                Instruction::SingleBit(BitCmd::BIT(BitInput(bit, DoubleInputU8::Address)))
            }
        });

    let res_instructions = res_opcodes.clone()
        .map(|code| {
            // this enumerates the raw table, while the code in Instruction follows an encoding
            let reg_code = (code & 0x0F) % 8;
            let bit = match code {
                0x80..=0x87 => 0,
                0x88..=0x8F => 1,
                0x90..=0x97 => 2,
                0x98..=0x9F => 3,
                0xA0..=0xA7 => 4,
                0xA8..=0xAF => 5,
                0xB0..=0xB7 => 6,
                0xB8..=0xBF => 7,
                _ => unreachable!("RES opcodes range is 0x80..=0xBF which is fully covered")
            };

            if let Some(reg) = n_to_reg_u8(reg_code) {
                build_X_instruction!(BitCmd::RES, bit, reg)
            } else {
                Instruction::SingleBit(BitCmd::RES(BitInput(bit, DoubleInputU8::Address)))
            }
        });

    let set_instructions = set_opcodes.clone()
        .map(|code| {
            // this enumerates the raw table, while the code in Instruction follows an encoding
            let reg_code = (code & 0x0F) % 8;
            let bit = match code {
                0xC0..=0xC7 => 0,
                0xC8..=0xCF => 1,
                0xD0..=0xD7 => 2,
                0xD8..=0xDF => 3,
                0xE0..=0xE7 => 4,
                0xE8..=0xEF => 5,
                0xF0..=0xF7 => 6,
                0xF8..=0xFF => 7,
                _ => unreachable!("RES opcodes range is 0xC0..=0xFF which is fully covered")
            };

            if let Some(reg) = n_to_reg_u8(reg_code) {
                build_X_instruction!(BitCmd::SET, bit, reg)
            } else {
                Instruction::SingleBit(BitCmd::SET(BitInput(bit, DoubleInputU8::Address)))
            }
        });

    let opcodes = bit_opcodes.chain(res_opcodes).chain(set_opcodes);
    let instructions = bit_instructions.chain(res_instructions).chain(set_instructions);

    // use std::fmt::Write;
    // let mut eout = String::new();
    // instructions.for_each(|instruction| {
    //     writeln!(&mut eout, "{}", format!("{:?}", instruction)).unwrap();
    // });
    // eprintln!("{}", eout);
    // panic!("forcing test runner to show me stderr/out");

    let single_bit_opcode_map: std::collections::HashMap<u8, Instruction> = std::iter::zip(opcodes, instructions).collect();

    // use std::fmt::Write;
    // let mut eout = String::new();
    // single_bit_opcode_map.iter().for_each(|(opcode, instruction)| { /* HashMaps print in arbritrary order but yeah looks good */
    //     writeln!(&mut eout, "{:#X} => {}", opcode, format!("{:?}", instruction)).unwrap();
    // });
    // eprintln!("{}", eout);
    // panic!("forcing test runner to show me stderr/out");

    for (opcode, instruction) in single_bit_opcode_map {
        let built_instruction = Instruction::from_byte(opcode, true)
            .expect("Instruction struct should build and identify single bit prefixed opcodes properly");
    

        assert_eq!(
            format!("{:?}", built_instruction),
            format!("{:?}", instruction),
            "left containing built_instruction with {opcode:#X} failed against right containing correct instruction"
        );
    }
}