mod alu;
mod flags;
mod instructions;
mod interrupt;
mod io;
mod registers;

use alu::AluOp;
use instructions::{AddArg, Condition, IncDecArg, Instruction, LoadArgs, Register};
use io::{In16, In8, Out16, Out8};
use registers::Registers;

pub use flags::Flags;
pub use interrupt::Interrupt;

pub trait Bus {
    fn read_m_cycle(&mut self, addr: u16) -> u8;
    fn tick_m_cycle(&mut self);
    fn write_m_cycle(&mut self, addr: u16, value: u8);

    fn pop_interrupt(&mut self) -> Option<Interrupt>;
    fn should_handle_interrupt(&self) -> bool;
}

pub struct CPU<B: Bus> {
    bus: B,
    registers: Registers,
    halt: bool,
    ime: bool,
}

impl<B: Bus> CPU<B> {
    pub fn new(bus: B) -> Self {
        CPU {
            bus,
            registers: Registers::default(),
            halt: false,
            ime: false,
        }
    }

    pub fn halt(&self) -> bool {
        self.halt
    }

    pub fn ime(&self) -> bool {
        self.ime
    }

    pub fn bus(&self) -> &B {
        &self.bus
    }

    pub fn registers(&self) -> &Registers {
        &self.registers
    }

    fn fetch_next(&mut self) -> u8 {
        let next = self.bus.read_m_cycle(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(1));

        next
    }

    fn fetch_next16(&mut self) -> u16 {
        let low = self.fetch_next();
        let high = self.fetch_next();

        u16::from_le_bytes([low, high])
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let opcode = self.fetch_next();

        if opcode == 0xCB {
            let opcode = self.fetch_next();
            Instruction::try_decode_prefixed(opcode).unwrap()
        } else {
            Instruction::try_decode(opcode).unwrap()
        }
    }

    fn execute(&mut self, instr: Instruction) {
        use Instruction::*;

        match instr {
            ADC(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::ADC, self.registers.a(), y, Register::A)
            }
            ADD(arg) => match arg {
                AddArg::ImmediateU8 => {
                    let y = self.read_byte(In8::N);
                    self.alu_op(AluOp::ADD, self.registers.a(), y, Register::A)
                }
                AddArg::Register(register) => {
                    let y = self.read_byte(register);
                    self.alu_op(AluOp::ADD, self.registers.a(), y, Register::A)
                } //self.add(target),
                AddArg::RegisterPairSP(pair) => {
                    let flags_mask = !Flags::Zero;
                    let flags = self.registers.f();

                    let [l, h] = self.read_word(In16::HL).to_le_bytes();
                    let [low, high] = self.read_word(pair).to_le_bytes();

                    // On real hardware flags get set between these but the effect is not
                    // observable so I've not emulated it
                    let (low, flags) = alu::execute(AluOp::ADD, l, low, flags);
                    let (high, flags) = alu::execute(AluOp::ADC, h, high, flags);

                    self.write_word(u16::from_le_bytes([low, high]), Out16::HL);

                    let flags = (flags & flags_mask) | (self.registers.f() & !flags_mask);
                    self.registers.set_f(flags);

                    self.bus.tick_m_cycle();
                }
                AddArg::SPd => {
                    let d = self.fetch_next();
                    let d = i16::from(d as i8) as u16;
                    let [low, high] = d.to_le_bytes();

                    let [p, s] = self.read_word(In16::SP).to_le_bytes();

                    let flags = Flags::empty();

                    // I'm not 100% sure if discarding the flags from the ADC is correct, but it
                    // passes all the tests
                    let (low, flags) = alu::execute(AluOp::ADD, p, low, flags);
                    let (high, _) = alu::execute(AluOp::ADC, s, high, flags);

                    self.write_word(u16::from_le_bytes([low, high]), Out16::SP);

                    let flags = flags & !(Flags::Zero | Flags::Subtract);
                    self.registers.set_f(flags);

                    self.bus.tick_m_cycle();
                    self.bus.tick_m_cycle();
                }
            },
            AND(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::AND, self.registers.a(), y, Register::A)
            }
            BIT(bit, target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::BIT, x, bit, None)
            }
            CALL(condition) => {
                let addr = self.fetch_next16();

                if self.check_condition(condition) {
                    self.push(self.registers.pc());
                    self.registers.set_pc(addr);
                }
            }
            CCF => self.alu_op(AluOp::CCF, self.registers.a(), 0, Register::A), //self.ccf(),
            CP(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::CP, self.registers.a(), y, Register::A)
            }
            CPL => self.alu_op(AluOp::CPL, self.registers.a(), 0, Register::A), //self.cpl(),
            DAA => self.alu_op(AluOp::DAA, self.registers.a(), 0, Register::A), //self.daa(),
            DEC(arg) => match arg {
                IncDecArg::Register(register) => {
                    let value = self.read_byte(register);

                    // Half Carry is set if the lower nibble of the value is equal to 0xF.
                    // If the nibble is equal to 0xF (0b1111) that means incrementing the value
                    // by 1 would cause a carry from the lower nibble to the upper nibble.
                    let half_carry = value.trailing_zeros() >= 4;

                    let result = value.wrapping_sub(1);
                    self.write_byte(result, register);

                    let mut flags = self.registers.f();
                    flags.set(Flags::Zero, result == 0);
                    flags.insert(Flags::Subtract);
                    flags.set(Flags::HalfCarry, half_carry);

                    self.registers.set_f(flags);
                }
                IncDecArg::RegisterPairSP(pair) => {
                    let value = self.read_word(pair);

                    let value = value.wrapping_sub(1);
                    self.write_word(value, pair);

                    self.bus.tick_m_cycle();
                }
            },
            DI => {
                self.ime = false;
            }
            EI => {
                self.ime = true;
            }
            INC(arg) => match arg {
                IncDecArg::Register(register) => {
                    let value = self.read_byte(register);

                    // Half Carry is set if the lower nibble of the value is equal to 0xF.
                    // If the nibble is equal to 0xF (0b1111) that means incrementing the value
                    // by 1 would cause a carry from the lower nibble to the upper nibble.
                    let half_carry = (value & 0xF) == 0xF;

                    let result = value.wrapping_add(1);
                    self.write_byte(result, register);

                    let mut flags = self.registers.f();
                    flags.set(Flags::Zero, result == 0);
                    flags.remove(Flags::Subtract);
                    flags.set(Flags::HalfCarry, half_carry);

                    self.registers.set_f(flags);
                }
                IncDecArg::RegisterPairSP(pair) => {
                    let value = self.read_word(pair);

                    let value = value.wrapping_add(1);
                    self.write_word(value, pair);

                    self.bus.tick_m_cycle();
                }
            },
            LD(args) => match args {
                LoadArgs::RegisterToRegister(to, from) => self.ld_byte(to, from),
                LoadArgs::RegisterFromImmediateU8(to) => self.ld_byte(to, In8::N),
                LoadArgs::RegisterPairFromImmediateU16(to) => self.ld_word(to, In16::NN),
                LoadArgs::AFromIndirect(from) => self.ld_byte(Out8::A, from),
                LoadArgs::IndirectFromA(to) => self.ld_byte(to, In8::A),
                LoadArgs::NNSP => self.ld_word(Out16::NN, In16::SP),
                LoadArgs::SPHL => {
                    let value = self.read_word(In16::HL);
                    self.write_word(value, Out16::SP);

                    self.bus.tick_m_cycle();
                }
                LoadArgs::HLSPd => {
                    let d = self.fetch_next();
                    let d = i16::from(d as i8) as u16;

                    let [low, high] = d.to_le_bytes();
                    let [p, s] = self.read_word(In16::SP).to_le_bytes();

                    let flags = Flags::empty();

                    // I'm not 100% sure if discarding the flags from the ADC is correct, but it
                    // passes all the tests
                    let (low, flags) = alu::execute(AluOp::ADD, p, low, flags);
                    let (high, _) = alu::execute(AluOp::ADC, s, high, flags);

                    self.write_word(u16::from_le_bytes([low, high]), Out16::HL);

                    let flags = flags & !(Flags::Zero | Flags::Subtract);
                    self.registers.set_f(flags);

                    self.bus.tick_m_cycle();
                }
            },
            HALT => {
                self.halt = true;
            }
            JP(condition) => {
                let addr = self.fetch_next16();

                if self.check_condition(condition) {
                    self.registers.set_pc(addr);
                    self.bus.tick_m_cycle();
                }
            }
            JPHL => self.registers.set_pc(self.registers.hl()),
            JR(condition) => {
                let d = self.fetch_next();
                let d = i16::from(d as i8) as u16;

                if self.check_condition(condition) {
                    let addr = self.registers.pc().wrapping_add(d);
                    self.registers.set_pc(addr);

                    self.bus.tick_m_cycle();
                }
            }
            NOP => {}
            OR(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::OR, self.registers.a(), y, Register::A)
            }
            POP(to) => {
                let value = self.pop();
                self.write_word(value, to);
            }
            PUSH(from) => {
                let value = self.read_word(from);
                self.push(value);
            }
            RES(bit, target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::RES, x, bit, target)
            }
            RET(condition) => {
                if condition.is_some() {
                    self.bus.tick_m_cycle();
                }

                if self.check_condition(condition) {
                    let popped = self.pop();
                    self.registers.set_pc(popped);

                    self.bus.tick_m_cycle();
                }
            }
            RETI => {
                let popped = self.pop();
                self.registers.set_pc(popped);

                self.bus.tick_m_cycle();

                self.ime = true;
            }
            RL(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::RL, x, 0, target)
            }
            RLA => {
                self.alu_op(AluOp::RL, self.registers.a(), 0, Register::A); //self.rla(),
                self.registers.set_f(self.registers.f() - Flags::Zero);
            }
            RLC(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::RLC, x, 0, target)
            }
            RLCA => {
                self.alu_op(AluOp::RLC, self.registers.a(), 0, Register::A); //self.rla(),
                self.registers.set_f(self.registers.f() - Flags::Zero);
            }
            RR(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::RR, x, 0, target)
            }
            RRA => {
                self.alu_op(AluOp::RR, self.registers.a(), 0, Register::A); //self.rla(),
                self.registers.set_f(self.registers.f() - Flags::Zero);
            }
            RRC(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::RRC, x, 0, target)
            }
            RRCA => {
                self.alu_op(AluOp::RRC, self.registers.a(), 0, Register::A); //self.rla(),
                self.registers.set_f(self.registers.f() - Flags::Zero);
            }
            RST(addr) => {
                self.push(self.registers.pc());
                self.registers.set_pc(addr);
            }
            SBC(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::SBC, self.registers.a(), y, Register::A)
            }
            SCF => self.alu_op(AluOp::SCF, self.registers.a(), 0, Register::A), //self.scf(),
            SET(bit, target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::SET, x, bit, target)
            }
            SLA(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::SLA, x, 0, target)
            }
            SRA(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::SRA, x, 0, target)
            }
            SRL(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::SRL, x, 0, target)
            }
            STOP => unimplemented!("STOP"),
            SWAP(target) => {
                let x = self.read_byte(target);
                self.alu_op(AluOp::SWAP, x, 0, target)
            }
            SUB(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::SUB, self.registers.a(), y, Register::A)
            }
            XOR(from) => {
                let y = self.read_byte(from);
                self.alu_op(AluOp::XOR, self.registers.a(), y, Register::A)
            }
        }
    }

    fn handle_interrupts(&mut self) {
        if !self.ime && !self.halt {
            return;
        }

        if !self.bus.should_handle_interrupt() {
            return;
        }

        self.halt = false;

        if !self.ime {
            return;
        }

        self.ime = false;

        self.bus.tick_m_cycle();
        self.bus.tick_m_cycle();

        let pc = self.registers.pc().to_le_bytes();

        self.registers.set_sp(self.registers.sp().wrapping_sub(1));
        self.bus.write_m_cycle(self.registers.sp(), pc[1]);

        self.registers.set_sp(self.registers.sp().wrapping_sub(1));
        self.bus.write_m_cycle(self.registers.sp(), pc[0]);

        let interrupt = self.bus.pop_interrupt().unwrap();
        self.registers.set_pc(interrupt.to_vector());
    }

    pub fn step(&mut self) {
        if !self.halt {
            let instr = self.fetch_and_decode();
            self.execute(instr);
        } else {
            self.bus.tick_m_cycle();
        }

        self.handle_interrupts();
    }
}

impl<B: Bus> CPU<B> {
    fn alu_op(&mut self, op: AluOp, x: u8, y: u8, out: impl Into<Option<Register>>) {
        let flags = self.registers.f();

        let (result, flags) = alu::execute(op, x, y, flags);
        self.registers.set_f(flags);

        if let Some(out) = out.into() {
            self.write_byte(result, out);
        }
    }

    fn check_condition(&self, condition: Option<Condition>) -> bool {
        match condition {
            None => true,
            Some(Condition::Carry) => self.registers.f().contains(Flags::Carry),
            Some(Condition::NonCarry) => !self.registers.f().contains(Flags::Carry),
            Some(Condition::Zero) => self.registers.f().contains(Flags::Zero),
            Some(Condition::NonZero) => !self.registers.f().contains(Flags::Zero),
        }
    }

    fn ld_byte<O, I>(&mut self, to: O, from: I)
    where
        O: Into<Out8>,
        I: Into<In8>,
    {
        let value = self.read_byte(from);
        self.write_byte(value, to);
    }

    fn ld_word<O, I>(&mut self, to: O, from: I)
    where
        O: Into<Out16>,
        I: Into<In16>,
    {
        let value = self.read_word(from);
        self.write_word(value, to);
    }

    fn pop(&mut self) -> u16 {
        let sp = self.registers.sp();

        let low = self.bus.read_m_cycle(sp);
        let sp = sp.wrapping_add(1);

        let high = self.bus.read_m_cycle(sp);
        let sp = sp.wrapping_add(1);

        self.registers.set_sp(sp);

        u16::from_le_bytes([low, high])
    }

    fn push(&mut self, value: u16) {
        let sp = self.registers.sp();
        self.bus.tick_m_cycle();

        let [low, high] = value.to_le_bytes();

        let sp = sp.wrapping_sub(1);
        self.bus.write_m_cycle(sp, high);

        let sp = sp.wrapping_sub(1);
        self.bus.write_m_cycle(sp, low);

        self.registers.set_sp(sp);
    }

    fn read_byte<T>(&mut self, from: T) -> u8
    where
        T: Into<In8>,
    {
        match from.into() {
            In8::A => self.registers.a(),
            In8::B => self.registers.b(),
            In8::C => self.registers.c(),
            In8::D => self.registers.d(),
            In8::E => self.registers.e(),
            In8::H => self.registers.h(),
            In8::L => self.registers.l(),
            In8::N => self.fetch_next(),
            In8::BC => self.bus.read_m_cycle(self.registers.bc()),
            In8::DE => self.bus.read_m_cycle(self.registers.de()),
            In8::HL => self.bus.read_m_cycle(self.registers.hl()),
            In8::HLMinus => {
                let hl = self.registers.hl();
                self.registers.set_hl(hl.wrapping_sub(1));

                self.bus.read_m_cycle(hl)
            }
            In8::HLPlus => {
                let hl = self.registers.hl();
                self.registers.set_hl(hl.wrapping_add(1));

                self.bus.read_m_cycle(hl)
            }
            In8::NN => {
                let nn = self.fetch_next16();
                self.bus.read_m_cycle(nn)
            }
            In8::CHigh => {
                let addr = 0xFF00 + u16::from(self.registers.c());
                self.bus.read_m_cycle(addr)
            }
            In8::NHigh => {
                let n = self.fetch_next();

                let addr = 0xFF00 + u16::from(n);
                self.bus.read_m_cycle(addr)
            }
        }
    }

    fn read_word<T>(&mut self, from: T) -> u16
    where
        T: Into<In16>,
    {
        match from.into() {
            In16::AF => self.registers.af(),
            In16::BC => self.registers.bc(),
            In16::DE => self.registers.de(),
            In16::HL => self.registers.hl(),
            In16::NN => self.fetch_next16(),
            In16::SP => self.registers.sp(),
        }
    }

    fn write_byte<T>(&mut self, value: u8, to: T)
    where
        T: Into<Out8>,
    {
        match to.into() {
            Out8::A => self.registers.set_a(value),
            Out8::B => self.registers.set_b(value),
            Out8::C => self.registers.set_c(value),
            Out8::D => self.registers.set_d(value),
            Out8::E => self.registers.set_e(value),
            Out8::H => self.registers.set_h(value),
            Out8::L => self.registers.set_l(value),
            Out8::BC => self.bus.write_m_cycle(self.registers.bc(), value),
            Out8::DE => self.bus.write_m_cycle(self.registers.de(), value),
            Out8::HL => self.bus.write_m_cycle(self.registers.hl(), value),
            Out8::HLMinus => {
                let hl = self.registers.hl();
                self.registers.set_hl(hl.wrapping_sub(1));

                self.bus.write_m_cycle(hl, value);
            }
            Out8::HLPlus => {
                let hl = self.registers.hl();
                self.registers.set_hl(hl.wrapping_add(1));

                self.bus.write_m_cycle(hl, value);
            }
            Out8::NN => {
                let nn = self.fetch_next16();
                self.bus.write_m_cycle(nn, value);
            }
            Out8::CHigh => {
                let addr = 0xFF00 + u16::from(self.registers.c());
                self.bus.write_m_cycle(addr, value);
            }
            Out8::NHigh => {
                let n = self.fetch_next();

                let addr = 0xFF00 + u16::from(n);
                self.bus.write_m_cycle(addr, value);
            }
        }
    }

    fn write_word<T>(&mut self, value: u16, to: T)
    where
        T: Into<Out16>,
    {
        match to.into() {
            Out16::AF => self.registers.set_af(value),
            Out16::BC => self.registers.set_bc(value),
            Out16::DE => self.registers.set_de(value),
            Out16::HL => self.registers.set_hl(value),
            Out16::NN => {
                let addr = self.fetch_next16();

                let [low, high] = value.to_le_bytes();
                self.bus.write_m_cycle(addr, low);
                self.bus.write_m_cycle(addr.wrapping_add(1), high);
            }
            Out16::SP => self.registers.set_sp(value),
        }
    }
}
