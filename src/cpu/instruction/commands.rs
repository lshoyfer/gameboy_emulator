use super::input::*;
// todo!("figure out why tf this works if mod input is private and im reexporting from super lmao")

/// Load U8 command set
#[cfg_attr(test, derive(Debug))]
pub enum LoadU8Cmd {
    LD(LDInputU8),
    LDI(LDIInputU8),
    LDD(LDDInputU8),
}

/// Load U16 command set
#[cfg_attr(test, derive(Debug))]
pub enum LoadU16Cmd {
    LD(LDInputU16),
    PUSH(InputU16),
    POP(InputU16),
}

/// Arithmetical/Logical u8 command set
#[cfg_attr(test, derive(Debug))]
pub enum AritLogiU8Cmd {
    ADD(CompoundInputU8), 
    ADC(CompoundInputU8), 
    SUB(CompoundInputU8), 
    SBC(CompoundInputU8), 
    AND(CompoundInputU8), 
    XOR(CompoundInputU8), 
    OR(CompoundInputU8), 
    CP(CompoundInputU8), 
    INC(DoubleInputU8),
    DEC(DoubleInputU8),
    DAA,
    CPL,
}

/// Arithmetical/Logical U16 command set
#[cfg_attr(test, derive(Debug))]
pub enum AritLogiU16Cmd {
    ADDHL(InputU16),
    INC(InputU16),
    DEC(InputU16),
    ADDSP,
    LDHLSP
}

/// Rotate & Shift command set
#[cfg_attr(test, derive(Debug))]
pub enum RSCmd {
    RLCA, // 07 - this one is incorrectly spelled "RRLA" on the Rust Guidebook
    RLA,
    RRCA,
    RRA,
    RLC(DoubleInputU8),
    RL(DoubleInputU8),
    RRC(DoubleInputU8),
    RR(DoubleInputU8),
    SLA(DoubleInputU8),
    SWAP(DoubleInputU8),
    SRA(DoubleInputU8),
    SRL(DoubleInputU8),
}

/// Single Bit Operation command set
#[cfg_attr(test, derive(Debug))]
pub enum BitCmd {
    BIT(BitInput), // todo!()
    RES(BitInput), // todo!()
    SET(BitInput), // todo!()
}

// CPU Control command set
#[cfg_attr(test, derive(Debug))]
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

#[cfg_attr(test, derive(Debug))]
pub enum JmpCmd {
    // todo!("Figure out inputs & implement")
    JP(JPInput),
    JR(JmpCmdInput), 
    CALL(JmpCmdInput), // todo!()
    RET(JmpCmdInput), // todo!()
    RETI, // todo!()
    RST(), // todo!()
}