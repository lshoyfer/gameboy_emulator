mod register;
mod instruction;

// Brings in 
use instruction::*;
use register::*;

struct MemoryBus {
    memory: [u8; 0xFFFF] // 65535 bytes    
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
} 

// DIRECT INSTRUCTION EXECUTION impl-block
impl CPU {
    /// Executes a given CPU instruction
    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Load8Bit(command) => {
                match command {
                    LoadU8Cmd::LD => todo!("Implement loads"),
                    LoadU8Cmd::LDI => todo!("Implement loads"),
                    LoadU8Cmd::LDD => todo!("Implement loads"),
                }
            }

            Instruction::Load16Bit(command) => {
                match command {
                    LoadU16Cmd::LD => todo!("Implement loads"),
                    LoadU16Cmd::PUSH => todo!("Implement loads"),
                    LoadU16Cmd::POP => todo!("Implement loads"),
                }
            }

            Instruction::ArithmeticLogical8Bit(command) => {
                match command {
                    AritLogiU8Cmd::ADD(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.add(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.add(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.add(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.add(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.add(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.add(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.add(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::ADC(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.adc(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.adc(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.adc(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.adc(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.adc(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.adc(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.adc(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                           }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::SUB(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.sub(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.sub(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.sub(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.sub(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.sub(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.sub(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.sub(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::SBC(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.sbc(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.sbc(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.sbc(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.sbc(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.sbc(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.sbc(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.sbc(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::AND(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.and(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.and(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.and(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.and(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.and(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.and(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.and(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::XOR(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.xor(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.xor(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.xor(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.xor(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.xor(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.xor(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.xor(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::OR(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.or(self.registers.a),
                                    RegisterU8::B => self.registers.a = self.or(self.registers.b),
                                    RegisterU8::C => self.registers.a = self.or(self.registers.c),
                                    RegisterU8::D => self.registers.a = self.or(self.registers.d),
                                    RegisterU8::E => self.registers.a = self.or(self.registers.e),
                                    RegisterU8::H => self.registers.a = self.or(self.registers.h),
                                    RegisterU8::L => self.registers.a = self.or(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::CP(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.cp(self.registers.a),
                                    RegisterU8::B => self.cp(self.registers.b),
                                    RegisterU8::C => self.cp(self.registers.c),
                                    RegisterU8::D => self.cp(self.registers.d),
                                    RegisterU8::E => self.cp(self.registers.e),
                                    RegisterU8::H => self.cp(self.registers.h),
                                    RegisterU8::L => self.cp(self.registers.l),
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => todo!("immediate value // clock cycles // addressing & accessing system"),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::INC(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => { 
                                        self.registers.a = self.registers.a.wrapping_add(1);
                                        self.inc_flags(self.registers.a);
                                    }
                                    RegisterU8::B => { 
                                        self.registers.b = self.registers.b.wrapping_add(1);
                                        self.inc_flags(self.registers.b);
                                    }
                                    RegisterU8::C => { 
                                        self.registers.c = self.registers.c.wrapping_add(1);
                                        self.inc_flags(self.registers.c);
                                    }
                                    RegisterU8::D => { 
                                        self.registers.d = self.registers.d.wrapping_add(1);
                                        self.inc_flags(self.registers.d);
                                    }
                                    RegisterU8::E => { 
                                        self.registers.e = self.registers.e.wrapping_add(1);
                                        self.inc_flags(self.registers.e);
                                    }
                                    RegisterU8::H => { 
                                        self.registers.h = self.registers.h.wrapping_add(1);
                                        self.inc_flags(self.registers.h);
                                    }
                                    RegisterU8::L => { 
                                        self.registers.l = self.registers.l.wrapping_add(1);
                                        self.inc_flags(self.registers.l);
                                    }
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => unreachable!("Immediate (n) variant for INC instruction does not exist in the CPU spec."),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::DEC(input) => {
                        match input {
                            CompoundInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => { 
                                        self.registers.a = self.registers.a.wrapping_sub(1);
                                        self.dec_flags(self.registers.a);
                                    }
                                    RegisterU8::B => { 
                                        self.registers.b = self.registers.b.wrapping_sub(1);
                                        self.dec_flags(self.registers.b);
                                    }
                                    RegisterU8::C => { 
                                        self.registers.c = self.registers.c.wrapping_sub(1);
                                        self.dec_flags(self.registers.c);
                                    }
                                    RegisterU8::D => { 
                                        self.registers.d = self.registers.d.wrapping_sub(1);
                                        self.dec_flags(self.registers.d);
                                    }
                                    RegisterU8::E => { 
                                        self.registers.e = self.registers.e.wrapping_sub(1);
                                        self.dec_flags(self.registers.e);
                                    }
                                    RegisterU8::H => { 
                                        self.registers.h = self.registers.h.wrapping_sub(1);
                                        self.dec_flags(self.registers.h);
                                    }
                                    RegisterU8::L => { 
                                        self.registers.l = self.registers.l.wrapping_sub(1);
                                        self.dec_flags(self.registers.l);
                                    }
                                }
                                self.pc.wrapping_add(1)
                            }
                            CompoundInputU8::Immediate => unreachable!("Immediate (n) variant for DEC instruction does not exist in the CPU spec."),
                            CompoundInputU8::Address => todo!("addressing HL")
                        }
                    }
                    AritLogiU8Cmd::DAA => todo!("Implement"),
                    AritLogiU8Cmd::CPL => {
                        self.registers.a = !self.registers.a;
                        // zero flag is not affected
                        self.registers.f.subtract = true;
                        self.registers.f.half_carry = true;
                        self.pc.wrapping_add(1)
                    }
                }
            }

            Instruction::ArithmeticLogical16Bit(command) => {
                match command {
                    AritLogiU16Cmd::ADDHL(InputU16(target)) => {
                        match target {
                            RegisterU16::BC => {
                                let sum = self.addhl(self.registers.get_bc());
                                self.registers.set_hl(sum);
                            },
                            RegisterU16::DE => {
                                let sum = self.addhl(self.registers.get_de());
                                self.registers.set_hl(sum);
                            }
                            RegisterU16::HL => {
                                let sum = self.addhl(self.registers.get_hl());
                                self.registers.set_hl(sum);
                            }
                            RegisterU16::SP => todo!("Stack Pointer Implementation")
                        }
                    }
                    AritLogiU16Cmd::INC(InputU16(target)) => todo!("Implement"),
                    AritLogiU16Cmd::DEC(InputU16(target)) => todo!("Implement"),
                    AritLogiU16Cmd::ADDSP(InputI8(number)) => todo!("Stack Pointer Implementation"),
                    AritLogiU16Cmd::LD(InputI8(number)) => todo!("Implement"),
                }
                // all AritLogiU16Cmd variants are 1-byte wide commands so the PC-increment can go here as an umbrella/dedupe
                self.pc.wrapping_add(1) 
            }

            Instruction::RotateShift(command) => {
                match command {
                    RSCmd::RLCA => {
                        self.registers.a = self.rlc(self.registers.a);
                        self.pc.wrapping_add(1) 
                    }
                    RSCmd::RLA => { 
                        self.registers.a = self.rl(self.registers.a);
                        self.pc.wrapping_add(1) 
                    }
                    RSCmd::RRCA => {
                        self.registers.a = self.rrc(self.registers.a);
                        self.pc.wrapping_add(1) 
                    }
                    RSCmd::RRA => {
                        self.registers.a = self.rr(self.registers.a);
                        self.pc.wrapping_add(1) 
                    }
                    RSCmd::RLC(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.rlc(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.rlc(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.rlc(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.rlc(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.rlc(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.rlc(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.rlc(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::RL(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.rl(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.rl(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.rl(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.rl(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.rl(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.rl(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.rl(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::RRC(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.rrc(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.rrc(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.rrc(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.rrc(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.rrc(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.rrc(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.rrc(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::RR(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.rr(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.rr(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.rr(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.rr(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.rr(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.rr(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.rr(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::SLA(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.sla(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.sla(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.sla(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.sla(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.sla(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.sla(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.sla(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::SWAP(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.swap(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.swap(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.swap(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.swap(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.swap(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.swap(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.swap(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::SRA(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.sra(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.sra(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.sra(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.sra(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.sra(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.sra(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.sra(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                    RSCmd::SRL(input) => {
                        match input {
                            DoubleInputU8::Register(target) => {
                                match target {
                                    RegisterU8::A => self.registers.a = self.srl(self.registers.a),
                                    RegisterU8::B => self.registers.b = self.srl(self.registers.b),
                                    RegisterU8::C => self.registers.c = self.srl(self.registers.c),
                                    RegisterU8::D => self.registers.d = self.srl(self.registers.d),
                                    RegisterU8::E => self.registers.e = self.srl(self.registers.e),
                                    RegisterU8::H => self.registers.h = self.srl(self.registers.h),
                                    RegisterU8::L => self.registers.l = self.srl(self.registers.l),
                                }
                            }
                            DoubleInputU8::Address => todo!("addressing HL")
                        }
                        self.pc.wrapping_add(2) // all variants of DoubleInputU8 for RSCmds are prefixed
                    }
                }
            }
            
            Instruction::SingleBit(command) => {
                match command {
                    BitCmd::BIT(bit, InputU8(target)) => {
                        match target {
                            RegisterU8::A => self.bit(bit, self.registers.a),
                            RegisterU8::B => self.bit(bit, self.registers.b),
                            RegisterU8::C => self.bit(bit, self.registers.c),
                            RegisterU8::D => self.bit(bit, self.registers.d),
                            RegisterU8::E => self.bit(bit, self.registers.e),
                            RegisterU8::H => self.bit(bit, self.registers.h),
                            RegisterU8::L => self.bit(bit, self.registers.l),
                        }
                    }
                    BitCmd::RES(bit, InputU8(target)) => {
                        match target {
                            RegisterU8::A => self.registers.a = self.res(bit, self.registers.a),
                            RegisterU8::B => self.registers.b = self.res(bit, self.registers.b),
                            RegisterU8::C => self.registers.c = self.res(bit, self.registers.c),
                            RegisterU8::D => self.registers.d = self.res(bit, self.registers.d),
                            RegisterU8::E => self.registers.e = self.res(bit, self.registers.e),
                            RegisterU8::H => self.registers.h = self.res(bit, self.registers.h),
                            RegisterU8::L => self.registers.l = self.res(bit, self.registers.l),
                        }
                    }
                    BitCmd::SET(bit, InputU8(target)) => {
                        match target {
                            RegisterU8::A => self.registers.a = self.set(bit, self.registers.a),
                            RegisterU8::B => self.registers.b = self.set(bit, self.registers.b),
                            RegisterU8::C => self.registers.c = self.set(bit, self.registers.c),
                            RegisterU8::D => self.registers.d = self.set(bit, self.registers.d),
                            RegisterU8::E => self.registers.e = self.set(bit, self.registers.e),
                            RegisterU8::H => self.registers.h = self.set(bit, self.registers.h),
                            RegisterU8::L => self.registers.l = self.set(bit, self.registers.l),
                        }
                    }
                }
                self.pc.wrapping_add(2) // all BitCmd variants are prefixed so the PC-increment can go here as an umbrella/dedupe
            }

            Instruction::Control(command) => {
                match command {
                    CtrCmd::CCF => {
                        // zero flag not affected
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = !self.registers.f.carry;
                        self.pc.wrapping_add(1)
                    }
                    CtrCmd::SCF => {
                        // zero flag not affected
                        self.registers.f.subtract = false;
                        self.registers.f.half_carry = false;
                        self.registers.f.carry = true;
                        self.pc.wrapping_add(1)
                    }
                    CtrCmd::NOP => todo!("Implement"),
                    CtrCmd::HALT => todo!("Implement"),
                    CtrCmd::STOP => todo!("Implement"),
                    CtrCmd::DI => todo!("Implement"),
                    CtrCmd::EI => todo!("Implement"),
                }
            }
            
            Instruction::Jump(command) => {
                match command {
                    JmpCmd::JP() => todo!("Design & Implement"),
                    JmpCmd::JR() => todo!("Design & Implement"),
                    JmpCmd::CALL() => todo!("Design & Implement"),
                    JmpCmd::RET() => todo!("Design & Implement"),
                    JmpCmd::RETI => todo!("Implement"),
                    JmpCmd::RST() => todo!("Design & Implement"),
                }
            }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (sum, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = did_overflow;

        sum
    }

    fn addhl(&mut self, value: u16) -> u16 {
        let (sum, did_overflow) = self.registers.get_hl().overflowing_add(value);
        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        /* the LR-CPU has a 4bit ALU, so I believe adding from least to most significant would
           mean the half_carry reflects the upper byte, checking @ bit 11 & 12. */
        self.registers.f.half_carry = ((self.registers.get_hl() & 0x0FFF) + (value & 0x0FFF)) & 0x1000 == 0x1000;
        self.registers.f.carry = did_overflow;

        sum
    }

    fn adc(&mut self, value: u8) -> u8 {
        // I believe the CPU only cares about the end result of all internally compound ops like this,
        // so if that is true, the following logic/implementation should be correct
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (value, value_did_overflow) = value.overflowing_add(carry);
        let (sum, did_overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) & 0x10 == 0x10;
        self.registers.f.carry = value_did_overflow || did_overflow;

        sum
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (diff, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.zero = diff == 0;
        self.registers.f.subtract = true;
        // "Set if no borrow from bit 4."
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        // "Set if no borrow."
        self.registers.f.carry = !did_overflow;

        diff
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let carry = if self.registers.f.carry { 1 } else { 0 };
        let (value, value_did_overflow) = value.overflowing_add(carry);
        let (diff, did_overflow) = self.registers.a.overflowing_sub(value);
        
        self.registers.f.zero = diff == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        self.registers.f.carry = !(value_did_overflow || did_overflow);
        
        diff
    }

    fn and(&mut self, value: u8) -> u8 {
        let res = self.registers.a & value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;

        res
    }

    fn or(&mut self, value: u8) -> u8 {
        let res = self.registers.a | value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn xor(&mut self, value: u8) -> u8 {
        let res = self.registers.a ^ value; 
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn cp(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtract = true;
        // "Set if no borrow from bit 4."
        self.registers.f.half_carry = (self.registers.a & 0xF) >= (value & 0xF);
        // "Set FOR no borrow."
        self.registers.f.carry = self.registers.a < value;
    }

    // sets the proper flags after an inc operation
    fn inc_flags(&mut self, value: u8) {
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (value & 0xF) == 0; 
        // carry flag is """not affected""" apparently ¯\_(ツ)_/¯ 
    }

    // sets the proper flags after a dec operation
    fn dec_flags(&mut self, value: u8) {
        self.registers.f.zero = value == 0;
        self.registers.f.subtract = true;
        self.registers.f.half_carry = (value.wrapping_add(1) & 0xF) > 0;
        // carry flag is not affected
    }

    fn bit(&mut self, bit: u8, value: u8) {
        self.registers.f.zero = (value & (0b1 << bit)) == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = true;
        // carry flag is not affected
    }

    fn res(&mut self, bit: u8, value: u8) -> u8 {
        if (value & (0b1 << bit)) == (0b1 << bit) {
            value - (0b1 << bit)
        } else {
            value
        }
        // all flags are not affected
    }

    fn set(&mut self, bit: u8, value: u8) -> u8 {
        if (value & (0b1 << bit)) != (0b1 << bit) {
            value + (0b1 << bit)
        } else {
            value
        }
        // all flags are not affected
    }

    fn srl(&mut self, value: u8) -> u8 {
        let res = value >> 1;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;

        res
    }

    // through the carry
    fn rr(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | ((self.registers.f.carry as u8) << 7);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
        
        res
    }

    // through the carry
    fn rl(&mut self, value: u8) -> u8 {
        let res = (value << 1) | (self.registers.f.carry as u8);

        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;
    
        res
    }

    // circular
    fn rrc(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | ((value & 0b1) << 7);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
    
        res
    }

    // circular
    fn rlc(&mut self, value: u8) -> u8 {
        let res = (value << 1) | (value & (0b1 << 7)) >> 7;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;
    
        res
    }

    fn sra(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | (value & (0b1 << 7));
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 1;
    
        res
    }

    fn sla(&mut self, value: u8) -> u8 {
        let res = value << 1;
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & (0b1 << 7)) == 1;

        res
    }

    fn swap(&mut self, value: u8) -> u8 {
        let res = ((value & 0xF0) >> 4) | ((value & 0x0F) << 4);
        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }
}

// MEMORY MANIPULATION / CPU-LOOP / ENCODING INSTRUCTIONS TO BE EXECUTED impl-block
impl CPU {
    fn step(&mut self) {
        // todo!("cover all instruction reading styles")
        let mut instruction_byte = self.bus.read_byte(self.pc); 
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
    
        let next_pc = match Instruction::from_byte(instruction_byte, prefixed) {
            Some(instruction) => self.execute(instruction),
            None => {
                panic!(
                    "Unknown instruction received: 0x{}{:X}", 
                    if prefixed { "CB" } else { "" },
                    instruction_byte
                )
            }
        };
        
        self.pc = next_pc;
    }
}