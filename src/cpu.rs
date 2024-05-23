//! The GENERAL implementation structure in the CPU is:
//! 
//! The private util functions for operations change the f-registers appropriately
//! and calculate whatever business logic is generally needed in a reusable format.
//! Then, [`CPU::execute`] uses the utils where appropriate and updates whatever main data
//! specific to that operation it had to update (usually the register the operation must put
//! the result in) and then returns the appropriate next PCAddr. A notable exception
//! to the PCAddr calculation & return placement is in the Jump Command utilities where
//! the utils themselves return the proper PCAddr and [`CPU::execute`] immediately returns
//! that value due to the Jump commands' main "business logic" basically being manipulating 
//! the PC.
//! 
//! So in sum: 
//! Utils calculate the business logic of an operation which includes setting appropriate 
//! f-registers, [`CPU::execute`] uses the result of a util function appropriately and 
//! returns the next PCAddr value for [`CPU::step`] to work with.
//! 
//! NOTE: not all operations have a utility function but are rather inlined directly. I may
//! change this by making every operation have a utility function, and marking the util
//! as inline where appropriate, but I am still debating this.

mod register;
mod instruction;

// Brings in 
use instruction::*;
use register::*;

struct MemoryBus {
    memory: [u8; 0xFFFF] // 65535 bytes    
}

impl MemoryBus {
    #[inline]
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    #[inline]
    fn borrow_byte(&self, address: u16) -> &u8 {
        &self.memory[address as usize]
    }
    #[inline]
    fn borrow_byte_mut(&mut self, address: u16) -> &mut u8 {
        &mut self.memory[address as usize]
    }
}

/// 2-byte unsigned value representing the PC's value
type PCAddr = u16;
struct CPU {
    registers: Registers,
    /// Program Counter
    pc: PCAddr,
    bus: MemoryBus
} 

// DIRECT INSTRUCTION EXECUTION impl-block
impl CPU {
    /// Executes a given CPU instruction
    fn execute(&mut self, instruction: Instruction) -> PCAddr {
        match instruction {
            Instruction::Load8Bit(command) => {
                /* NOTE ON POINTER ARITHMETIC/IMPLEMENTATION OF LOADS
                    I refuse to enumerate all registers with a match on all of these arms but I
                    may in the future (I'll need to measure performance/memory etc)
                    but for now the implementation is done with pointer arithmetic on the Registers
                    struct to access the field containing the proper register dynamically
                    in an efficient manner. The RegisterU8 enum encodes in its variants'
                    numerical values the offset of where the respective register is relative
                    to the struct's starting address and as such is what is used in the arithmetic
                    as seen in the ptr.add(r/r1/r2 as usize) type operations.
                */
                match command {
                    LoadU8Cmd::LD(input) => {
                        match input {
                            LDInputU8::RR(r1, r2) => {
                                let p_registers = &mut self.registers as *mut Registers as *mut u8;

                                // SAFETY: Relies on invariants outlined in comments throughout the [`register`]
                                // module that guarantee these fields are next to each other in memory and that the
                                // u8 value of the variants of the RegisterU8 enum correspond to a proper ptr offset
                                // for their respective fields.
                                unsafe { *p_registers.add(r1 as usize) = *p_registers.add(r2 as usize); }
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::RI(r) => {
                                let p_registers = &mut self.registers as *mut Registers as *mut u8;

                                // SAFETY: see LDInputU8::RR execution (first arm in this match statement) comments
                                unsafe { *p_registers.add(r as usize) = self.read_immediate_u8(); }
                                self.pc.wrapping_add(2)
                            }
                            LDInputU8::RHL(r) => {
                                let p_registers = &mut self.registers as *mut Registers as *mut u8;

                                // SAFETY: see LDInputU8::RR execution (first arm in this match statement) comments
                                unsafe { *p_registers.add(r as usize) = self.bus.read_byte(self.registers.get_hl()); }
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::HLR(r) => {
                                let p_registers = &mut self.registers as *mut Registers as *mut u8;

                                // SAFETY: see LDInputU8::RR execution (first arm in this match statement) comments
                                unsafe { *self.bus.borrow_byte_mut(self.registers.get_hl()) = *p_registers.add(r as usize); }
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::HLI => {
                                *self.bus.borrow_byte_mut(self.registers.get_hl()) = self.read_immediate_u8();
                                self.pc.wrapping_add(2)
                            }
                            LDInputU8::ABC => {
                                self.registers.a = self.bus.read_byte(self.registers.get_bc());
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::ADE => {
                                self.registers.a = self.bus.read_byte(self.registers.get_de());
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::AII => {
                                self.registers.a = self.bus.read_byte(self.read_immediate_u16());
                                self.pc.wrapping_add(3)
                            }
                            LDInputU8::BCA => {
                                *self.bus.borrow_byte_mut(self.registers.get_bc()) = self.registers.a;
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::DEA => {
                                *self.bus.borrow_byte_mut(self.registers.get_de()) = self.registers.a;
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::IIA => {
                                *self.bus.borrow_byte_mut(self.read_immediate_u16()) = self.registers.a;
                                self.pc.wrapping_add(3)
                            }
                            // todo!("io-ports aren't yet implemented/designed/considered")
                            LDInputU8::ReadIoN => {
                                self.registers.a = self.bus.read_byte(
                                    0xFF00 + (self.read_immediate_u8() as u16)
                                );
                                self.pc.wrapping_add(2)
                            },
                            LDInputU8::WriteIoN => {
                                *self.bus.borrow_byte_mut(
                                    0xFF00 + (self.read_immediate_u8() as u16)
                                ) = self.registers.a;
                                self.pc.wrapping_add(2)
                            }
                            LDInputU8::ReadIoC => {
                                self.registers.a = self.bus.read_byte(
                                    0xFF00 + (self.registers.c as u16)
                                );
                                self.pc.wrapping_add(1)
                            }
                            LDInputU8::WriteIoC => {
                                *self.bus.borrow_byte_mut(
                                    0xFF00 + (self.registers.c as u16)
                                ) = self.registers.a;
                                self.pc.wrapping_add(1)
                            }
                        }
                    }
                    LoadU8Cmd::LDI(input) => {
                        match input {
                            LDIInputU8::HLA => {
                                *self.bus.borrow_byte_mut(self.registers.get_hl()) = self.registers.a;
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.pc.wrapping_add(1)
                            }
                            LDIInputU8::AHL => {
                                self.registers.a = self.bus.read_byte(self.registers.get_hl());
                                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
                                self.pc.wrapping_add(1)
                            }
                        }
                    }
                    LoadU8Cmd::LDD(input) => {
                        match input {
                            LDDInputU8::HLA => {
                                *self.bus.borrow_byte_mut(self.registers.get_hl()) = self.registers.a;
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.pc.wrapping_add(1)
                            }
                            LDDInputU8::AHL => {
                                self.registers.a = self.bus.read_byte(self.registers.get_hl());
                                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
                                self.pc.wrapping_add(1)
                            }
                        }
                    }
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
            
            // NOTE: Jump Commands return the next PC from the util functions / each arm
            // and some mutate the SP as part of their "business logic"
            Instruction::Jump(command) => {
                match command {
                    JmpCmd::JP(input) => {
                        match input {
                            JPInput::Direct => self.jump(true),
                            JPInput::HL => self.registers.get_hl(),
                            JPInput::Conditional(cond) => {
                                let should_jump = self.check_cond(cond);
                                self.jump(should_jump)
                            }
                        }
                    }
                    JmpCmd::JR(input) => {
                        match input {
                            JmpCmdInput::Direct => self.jump_relative(true),
                            JmpCmdInput::Conditional(cond) => {
                                let should_jump = self.check_cond(cond);
                                self.jump_relative(should_jump)
                            }
                        }
                    }
                    JmpCmd::CALL(input) => todo!("Design & Implement"),
                    JmpCmd::RET(input) => todo!("Design & Implement"),
                    JmpCmd::RETI => todo!("Implement"),
                    JmpCmd::RST() => todo!("Design & Implement"),
                }
            }
        }
    }

/* ArithmeticLogical8Bit Utils */
    fn add(&mut self, value: u8) -> u8 {
        let (sum, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = sum == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (value & 0xF)) & 0x10 == 0x10;
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

/* ArithmeticLogical16Bit Utils */
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

/* SingleBit Utils */ 
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

/* RotateShift Utils */
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

/* Jump Utils */
    fn check_cond(&self, cond: JmpCmdCondition) -> bool {
        match cond {
            JmpCmdCondition::NZ => !self.registers.f.zero,
            JmpCmdCondition::Z => self.registers.f.zero,
            JmpCmdCondition::NC => !self.registers.f.carry,
            JmpCmdCondition::C => !self.registers.f.carry,
        }
    }
    
    // CPU is Little-Endian
    fn jump(&mut self, should_jump: bool) -> PCAddr {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // JP nn is 3-bytes wide (OPCODE | ADDR_LEAST_SIG_BYTE | ADDR_MOST_SIG_BYTE)
            self.pc.wrapping_add(3)
        }
    }

    fn jump_relative(&mut self, should_jump: bool) -> PCAddr {
        if should_jump {
            // offset is signed
            let offset = self.bus.read_byte(self.pc + 1) as i8; 
            // todo!("verify if i should wrap this or if this will never actually wrap in proper JR calls")
            if offset >= 0 {
                self.pc.wrapping_add(offset as u16)
            } else {
                self.pc.wrapping_sub((offset as i16).abs() as u16)
            }
        } else {
            // JR dd is 2-bytes wide (OPCODE | RELATIVE_BYTE)
            self.pc.wrapping_add(2)
        }
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

    /// Reads the byte immediately after the opcode in memory.
    #[inline]
    fn read_immediate_u8(&self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(2))
    }

    /// Reads the next two bytes immediately after the opcode in memory as a u16.
    /// Used for CPU commands that are 3-bytes wide.
    #[inline]
    fn read_immediate_u16(&self) -> u16 {
        (self.read_immediate_u8() as u16) // LS-byte first
        | ((self.bus.read_byte(self.pc.wrapping_add(3)) as u16) << 8) // MS-byte last
    }

}

