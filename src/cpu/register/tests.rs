
use super::*;

#[test]
fn flag_register_conversion() {
    let original = FlagRegister { zero: false, subtract: true, half_carry: true, carry: true };

    let converted = u8::from(original);
    assert_eq!(0b0111_0000, converted);

    let back = FlagRegister::from(converted);
    assert_eq!(format!("{:?}", original), format!("{:?}", back));
}

#[test]
fn two_byte_set_and_get_af() {
    let mut reg = Registers {
        a: 1, b: 2, c: 3, d: 4, e: 5,
        f: FlagRegister { zero: false, subtract: true, half_carry: true, carry: false },
        h: 7, l: 8,
    };

    assert_eq!(0b0000_0001_0110_0000, reg.get_af());
    reg.set_af(0xFFF0);
    assert_eq!(0xFFF0, reg.get_af());
}

#[test]
fn two_byte_set_and_get_bc() {
    let mut reg = Registers {
        a: 1, b: 2, c: 3, d: 4, e: 5,
        f: FlagRegister { zero: false, subtract: true, half_carry: true, carry: false },
        h: 7, l: 8,
    };

    assert_eq!(0b0000_0010_0000_0011, reg.get_bc());
    reg.set_bc(0xFF12);
    assert_eq!(0xFF12, reg.get_bc());
}

#[test]
fn two_byte_set_and_get_de() {
    let mut reg = Registers {
        a: 1, b: 2, c: 3, d: 4, e: 5,
        f: FlagRegister { zero: false, subtract: true, half_carry: true, carry: false },
        h: 7, l: 8,
    };

    assert_eq!(0b0000_0100_0000_0101, reg.get_de());
    reg.set_de(0x31FF);
    assert_eq!(0x31FF, reg.get_de());
}

#[test]
fn two_byte_set_and_get_hl() {
    let mut reg = Registers {
        a: 1, b: 2, c: 3, d: 4, e: 5,
        f: FlagRegister { zero: false, subtract: true, half_carry: true, carry: false },
        h: 7, l: 8,
    };

    assert_eq!(0b0000_0111_0000_1000, reg.get_hl());
    reg.set_hl(0x0AF7);
    assert_eq!(0x0AF7, reg.get_hl());
}

// todo!("Test this on multiple platforms/look into this more.")
#[test]
fn alignment_and_padding_of_registers() {
    assert_eq!(
        std::mem::size_of::<Registers>(), 
        11, 
        "Test has failed for the byte size of Registers which is expected to be 7 bytes (from registers a b c d e l) + 4 bytes 
        (the flag register 'f' which internally keeps a bool (1-byte in memory) for each of the 4 flags) for 11 bytes total"
    );
    assert_eq!(
        std::mem::align_of::<Registers>(),
        1,
        "Test has failed for the alignment of Registers, which imposes some SERIOUS problems due to how loads are implemented with ptr arithmetic.
        If this fails it is likely a platform specific incurred error. The alignment is expected to be 1 byte because each field of
        the Registers struct is 1 byte except for the f-field which holds a FlagRegister struct, which holds 4 bool fields, each a byte, so it itself
        is 1-byte aligned, therefore if all of Registers fields are 1-byte aligned, Registers is 1-byte aligned/there is no padding required."
    );
}