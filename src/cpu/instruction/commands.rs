use super::input::*;
// todo!("figure out why tf this works if mod input is private and im reexporting from super lmao")

/// Load U8 command set
pub enum LoadU8Cmd {
    LD(LDInputU8),
    LDI(LDIInputU8),
    LDD(LDDInputU8),
}

/// Load U16 command set
pub enum LoadU16Cmd {
    LD(LDInputU16),
    PUSH(InputU16),
    POP(InputU16),
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
    INC(InputU16),
    DEC(InputU16),
    ADDSP,
    LDHLSP
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
    JP(JPInput),
    JR(JmpCmdInput), 
    CALL(JmpCmdInput), // todo!()
    RET(JmpCmdInput), // todo!()
    RETI, // todo!()
    RST(), // todo!()
}