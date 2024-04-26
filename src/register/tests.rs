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