use super::input::*;

/// Load U8 command set
pub enum LoadU8Cmd {
    LD, // todo!("Implement & also figure out input structure -- this one has a lot of variants"),
    LDI, // todo!("Implement & also figure out input structure")
    LDD, // todo!("Implement & also figure out input structure")
}

/// Load U16 command set
pub enum LoadU16Cmd {
    LD, // todo!()
    PUSH, // todo!()
    POP, // todo!()
}

/// Arithmetical/Logical U8 command set
pub enum AritLogiU8Cmd {
    ADD(CompoundInputU8), 
    ADC(CompoundInputU8), 
    SUB(CompoundInputU8), 
    SBC(CompoundInputU8), 
    AND(CompoundInputU8), 
    XOR(CompoundInputU8), 
    OR(CompoundInputU8), 
    CP(CompoundInputU8), 
        INC(CompoundInputU8), // todo!("Immediate variant input is unused, consider changing type/system")
        DEC(CompoundInputU8), // todo!("Immediate variant input is unused, consider changing type/system")
        DAA, // todo!("Implement")
    CPL,
}

/// Arithmetical/Logical U16 command set
pub enum AritLogiU16Cmd {
    ADDHL(InputU16),
    INC(InputU16), // todo!()
    DEC(InputU16), // todo!()
    ADDSP(InputI8), // todo!()
    LD(InputI8), // todo!()
}

/// Rotate & Shift command set
pub enum RSCmd {
    RLCA, // 07 - this one is incorrectly spelled "RRLA" on the Rust Guidebook
    RLA,
    RRCA,
    RRA,
    RLC(DoubleInputU8), // todo!("(HL) implementation")
    RL(DoubleInputU8), // todo!("(HL) implementation")
    RRC(DoubleInputU8), // todo!("(HL) implementation")
    RR(DoubleInputU8), // todo!("(HL) implementation")
    SLA(DoubleInputU8), // todo!("(HL) implementation")
    SWAP(DoubleInputU8), // todo!("(HL) implementation")
    SRA(DoubleInputU8), // todo!("(HL) implementation")
    SRL(DoubleInputU8), // todo!("(HL) implementation")
}

/// Single Bit Operation command set
pub enum BitCmd {
    // todo!("Figure all this shit out in terms of arg passing. I believe I already implemented in the CPU a general handler for these tho.")
    BIT(u8, InputU8), // todo!()
    RES(u8, InputU8), // todo!()
    SET(u8, InputU8), // todo!()
}

// CPU Control command set
pub enum CtrCmd {
    // todo!("Implement")
    CCF,
    SCF,
    NOP, // todo!()
    HALT, // todo!()
    STOP, // todo!()
    DI, // todo!()
    EI, // todo!()
}

pub enum JmpCmd {
    // todo!("Figure out inputs & implement")
    JP(), // todo!()
    JR(), // todo!()
    CALL(), // todo!()
    RET(), // todo!()
    RETI, // todo!()
    RST(), // todo!()
}