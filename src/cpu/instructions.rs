use std::{convert::TryFrom,fmt};

#[derive(Debug, PartialEq, Eq)]
enum AluOp {
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
}

impl fmt::Display for AluOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AluOp::*;

        match self {
            ADD => write!(f, "ADD A,"),
            ADC => write!(f, "ADC A,"),
            SUB => write!(f, "SUB"),
            SBC => write!(f, "SBC A,"),
            AND => write!(f, "AND"),
            XOR => write!(f, "XOR"),
            OR => write!(f, "OR"),
            CP => write!(f, "CP"),
        }
    }
}

impl TryFrom<u8> for AluOp {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use AluOp::*;

        match value {
            0 => Ok(ADD),
            1 => Ok(ADC),
            2 => Ok(SUB),
            3 => Ok(SBC),
            4 => Ok(AND),
            5 => Ok(XOR),
            6 => Ok(OR),
            7 => Ok(CP),
            _ => Err("invalid alu op"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    NonZero,
    Zero,
    NonCarry,
    Carry,
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Condition::*;

        match self {
            NonZero => write!(f, "NZ"),
            Zero => write!(f, "Z"),
            NonCarry => write!(f, "NC"),
            Carry => write!(f, "C"),
        }
    }
}

impl TryFrom<u8> for Condition {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Condition::*;

        match value {
            0 => Ok(NonZero),
            1 => Ok(Zero),
            2 => Ok(NonCarry),
            3 => Ok(Carry),
            _ => Err("invalid condition"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Register::*;

        match self {
            B => write!(f, "B"),
            C => write!(f, "C"),
            D => write!(f, "D"),
            E => write!(f, "E"),
            H => write!(f, "H"),
            L => write!(f, "L"),
            HL => write!(f, "(HL)"),
            A => write!(f, "A"),
        }
    }
}

impl TryFrom<u8> for Register {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Register::*;

        match value {
            0 => Ok(B),
            1 => Ok(C),
            2 => Ok(D),
            3 => Ok(E),
            4 => Ok(H),
            5 => Ok(L),
            6 => Ok(HL),
            7 => Ok(A),
            _ => Err("invalid register"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegisterPairSP {
    BC,
    DE,
    HL,
    SP,
}

impl fmt::Display for RegisterPairSP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RegisterPairSP::*;

        match self {
            BC => write!(f, "BC"),
            DE => write!(f, "DE"),
            HL => write!(f, "HL"),
            SP => write!(f, "SP"),
        }
    }
}

impl TryFrom<u8> for RegisterPairSP {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use RegisterPairSP::*;

        match value {
            0 => Ok(BC),
            1 => Ok(DE),
            2 => Ok(HL),
            3 => Ok(SP),
            _ => Err("invalid register pair"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RegisterPairAF {
    BC,
    DE,
    HL,
    AF,
}

impl fmt::Display for RegisterPairAF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RegisterPairAF::*;

        match self {
            BC => write!(f, "BC"),
            DE => write!(f, "DE"),
            HL => write!(f, "HL"),
            AF => write!(f, "AF"),
        }
    }
}

impl TryFrom<u8> for RegisterPairAF {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use RegisterPairAF::*;

        match value {
            0 => Ok(BC),
            1 => Ok(DE),
            2 => Ok(HL),
            3 => Ok(AF),
            _ => Err("invalid register pair"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rotation {
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Rotation::*;

        match self {
            RLC => write!(f, "RLC"),
            RRC => write!(f, "RRC"),
            RL => write!(f, "RL"),
            RR => write!(f, "RR"),
            SLA => write!(f, "SLA"),
            SRA => write!(f, "SRA"),
            SWAP => write!(f, "SWAP"),
            SRL => write!(f, "SRL"),
        }
    }
}

impl TryFrom<u8> for Rotation {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Rotation::*;

        match value {
            0 => Ok(RLC),
            1 => Ok(RRC),
            2 => Ok(RL),
            3 => Ok(RR),
            4 => Ok(SLA),
            5 => Ok(SRA),
            6 => Ok(SWAP),
            7 => Ok(SRL),
            _ => Err("invalid rotation"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AluArg {
    Register(Register),
    ImmediateU8,
}

impl fmt::Display for AluArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AluArg::*;

        match self {
            Register(register) => write!(f, "{}", register),
            ImmediateU8 => write!(f, "n"),
        }
    }
}

impl From<Register> for AluArg {
    fn from(value: Register) -> Self {
        AluArg::Register(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddArg {
    Register(Register),
    RegisterPairSP(RegisterPairSP),
    SPd,
    ImmediateU8,
}

impl fmt::Display for AddArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AddArg::*;

        match self {
            ImmediateU8 => write!(f, "A,n"),
            Register(register) => write!(f, "A,{}", register),
            RegisterPairSP(pair) => write!(f, "HL,{}", pair),
            SPd => write!(f, "SP,d"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IncDecArg {
    Register(Register),
    RegisterPairSP(RegisterPairSP),
}

impl fmt::Display for IncDecArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IncDecArg::*;

        match self {
            Register(register) => write!(f, "{}", register),
            RegisterPairSP(pair) => write!(f, "{}", pair),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Indirect {
    BC,
    DE,
    HLPlus,
    HLMinus,
    NN,
    FFPlusN,
    FFPlusC,
}

impl fmt::Display for Indirect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Indirect::*;

        match self {
            BC => write!(f, "(BC)"),
            DE => write!(f, "(DE)"),
            HLPlus => write!(f, "(HL+)"),
            HLMinus => write!(f, "(HL-)"),
            NN => write!(f, "(nn)"),
            FFPlusN => write!(f, "(0xFF00 + n)"),
            FFPlusC => write!(f, "(0xFF00 + C)"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadArgs {
    RegisterToRegister(Register, Register),
    RegisterFromImmediateU8(Register),
    RegisterPairFromImmediateU16(RegisterPairSP),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    SPHL,
    HLSPd,
    NNSP,
}

impl fmt::Display for LoadArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LoadArgs::*;

        match self {
            AFromIndirect(indirect) => write!(f, "A,{}", indirect),
            IndirectFromA(indirect) => write!(f, "{},A", indirect),
            SPHL => write!(f, "SP,HL"),
            RegisterFromImmediateU8(register) => write!(f, "{},n", register),
            RegisterPairFromImmediateU16(pair) => write!(f, "{},nn", pair),
            RegisterToRegister(to, from) => write!(f, "{},{}", to, from),
            HLSPd => write!(f, "HL,SP + d"),
            NNSP => write!(f, "(nn),SP"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    ADC(AluArg),
    ADD(AddArg),
    AND(AluArg),
    CALL(Option<Condition>),
    CCF,
    CP(AluArg),
    CPL,
    DAA,
    DEC(IncDecArg),
    DI,
    EI,
    HALT,
    INC(IncDecArg),
    JPHL,
    JP(Option<Condition>),
    JR(Option<Condition>),
    LD(LoadArgs),
    NOP,
    OR(AluArg),
    POP(RegisterPairAF),
    PUSH(RegisterPairAF),
    RET(Option<Condition>),
    RETI,
    RLA,
    RLCA,
    RRA,
    RRCA,
    RST(u16),
    SBC(AluArg),
    SCF,
    STOP,
    SUB(AluArg),
    XOR(AluArg),

    // CB Prefix
    BIT(u8, Register),
    RES(u8, Register),
    RL(Register),
    RLC(Register),
    RR(Register),
    RRC(Register),
    SET(u8, Register),
    SLA(Register),
    SRA(Register),
    SRL(Register),
    SWAP(Register),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;

        match self {
            ADC(arg) => write!(f, "ADC A,{}", arg),
            ADD(args) => write!(f, "ADD {}", args),
            AND(arg) => write!(f, "AND {}", arg),
            CALL(None) => write!(f, "CALL nn"),
            CALL(Some(condition)) => write!(f, "CALL {},nn", condition),
            CCF => write!(f, "CCF"),
            CP(arg) => write!(f, "CP {}", arg),
            CPL => write!(f, "CPL"),
            DAA => write!(f, "DAA"),
            DEC(arg) => write!(f, "DEC {}", arg),
            DI => write!(f, "DI"),
            EI => write!(f, "EI"),
            HALT => write!(f, "HALT"),
            INC(arg) => write!(f, "INC {}", arg),
            JP(None) => write!(f, "JP nn"),
            JP(Some(condition)) => write!(f, "JP {},nn", condition),
            JPHL => write!(f, "JP HL"),
            JR(None) => write!(f, "JR d"),
            JR(Some(condition)) => write!(f, "JR {},d", condition),
            LD(args) => write!(f, "LD {}", args),
            NOP => write!(f, "NOP"),
            OR(arg) => write!(f, "OR {}", arg),
            POP(pair) => write!(f, "POP {}", pair),
            PUSH(pair) => write!(f, "PUSH {}", pair),
            RET(None) => write!(f, "RET"),
            RET(Some(condition)) => write!(f, "RET {}", condition),
            RETI => write!(f, "RETI"),
            RLA => write!(f, "RLA"),
            RLCA => write!(f, "RLCA"),
            RRA => write!(f, "RRA"),
            RRCA => write!(f, "RRCA"),
            RST(arg) => write!(f, "RST {:02x}", arg),
            SBC(arg) => write!(f, "SBC A,{}", arg),
            SCF => write!(f, "SCF"),
            STOP => write!(f, "STOP"),
            SUB(arg) => write!(f, "SUB {}", arg),
            XOR(arg) => write!(f, "XOR {}", arg),

            BIT(bit, register) => write!(f, "BIT {},{}", bit, register),
            RES(bit, register) => write!(f, "RES {},{}", bit, register),
            RL(register) => write!(f, "RL {}", register),
            RLC(register) => write!(f, "RLC {}", register),
            RR(register) => write!(f, "RR {}", register),
            RRC(register) => write!(f, "RRC {}", register),
            SET(bit, register) => write!(f, "SET {},{}", bit, register),
            SLA(register) => write!(f, "SLA {}", register),
            SRA(register) => write!(f, "SRA {}", register),
            SRL(register) => write!(f, "SRL {}", register),
            SWAP(register) => write!(f, "SWAP {}", register),
        }
    }
}

impl Instruction {
    pub fn try_decode(value: u8) -> Result<Self, &'static str> {
        // https://gb-archive.github.io/salvage/decoding_gbz80_opcodes/Decoding%20Gamboy%20Z80%20Opcodes.html
        let x = (value & 0b1100_0000) >> 6;
        let y = (value & 0b0011_1000) >> 3;
        let z = value & 0b0000_0111;
        let p = (value & 0b0011_0000) >> 4;
        let q = ((value & 0b0000_1000) != 0) as u8;

        match x {
            // FOR x=0
            0 => match z {
                0 => match y {
                    0 => Ok(Instruction::NOP),
                    1 => Ok(Instruction::LD(LoadArgs::NNSP)),
                    2 => Ok(Instruction::STOP),
                    3 => Ok(Instruction::JR(None)),
                    4..=7 => {
                        Condition::try_from(y - 4).map(|condition| Instruction::JR(Some(condition)))
                    }
                    _ => unreachable!(),
                },

                1 => match q {
                    0 => RegisterPairSP::try_from(p)
                        .map(|pair| Instruction::LD(LoadArgs::RegisterPairFromImmediateU16(pair))),
                    1 => RegisterPairSP::try_from(p).map(|pair| {
                        Instruction::ADD(AddArg::RegisterPairSP(RegisterPairSP::from(pair)))
                    }),
                    _ => unreachable!(),
                },

                2 => match q {
                    0 => match p {
                        0 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::BC))),
                        1 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::DE))),
                        2 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::HLPlus))),
                        3 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::HLMinus))),
                        _ => unreachable!(),
                    },
                    1 => match p {
                        0 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::BC))),
                        1 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::DE))),
                        2 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::HLPlus))),
                        3 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::HLMinus))),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },

                3 => match q {
                    0 => RegisterPairSP::try_from(p)
                        .map(|pair| Instruction::INC(IncDecArg::RegisterPairSP(pair))),
                    1 => RegisterPairSP::try_from(p)
                        .map(|pair| Instruction::DEC(IncDecArg::RegisterPairSP(pair))),
                    _ => unreachable!(),
                },
                4 => Register::try_from(y)
                    .map(|register| Instruction::INC(IncDecArg::Register(register))),
                5 => Register::try_from(y)
                    .map(|register| Instruction::DEC(IncDecArg::Register(register))),
                6 => Register::try_from(y)
                    .map(|register| Instruction::LD(LoadArgs::RegisterFromImmediateU8(register))),
                7 => match y {
                    0 => Ok(Instruction::RLCA),
                    1 => Ok(Instruction::RRCA),
                    2 => Ok(Instruction::RLA),
                    3 => Ok(Instruction::RRA),
                    4 => Ok(Instruction::DAA),
                    5 => Ok(Instruction::CPL),
                    6 => Ok(Instruction::SCF),
                    7 => Ok(Instruction::CCF),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            // FOR x=1
            1 => match z {
                6 if y == 6 => Ok(Instruction::HALT),
                _ => Register::try_from(y).and_then(|to| {
                    Register::try_from(z)
                        .map(|from| Instruction::LD(LoadArgs::RegisterToRegister(to, from)))
                }),
            },
            // FOR x=2
            2 => AluOp::try_from(y).and_then(|op| {
                Register::try_from(z).map(|register| match op {
                    AluOp::ADD => Instruction::ADD(AddArg::Register(register)),
                    AluOp::ADC => Instruction::ADC(AluArg::Register(register)),
                    AluOp::SUB => Instruction::SUB(AluArg::Register(register)),
                    AluOp::SBC => Instruction::SBC(AluArg::Register(register)),
                    AluOp::AND => Instruction::AND(AluArg::Register(register)),
                    AluOp::XOR => Instruction::XOR(AluArg::Register(register)),
                    AluOp::OR => Instruction::OR(AluArg::Register(register)),
                    AluOp::CP => Instruction::CP(AluArg::Register(register)),
                })
            }),
            // FOR x=3
            3 => {
                match z {
                    0 => match y {
                        0..=3 => Condition::try_from(y)
                            .map(|condition| Instruction::RET(Some(condition))),
                        4 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::FFPlusN))),
                        5 => Ok(Instruction::ADD(AddArg::SPd)),
                        6 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::FFPlusN))),
                        7 => Ok(Instruction::LD(LoadArgs::HLSPd)),
                        _ => unreachable!(),
                    },
                    1 => match q {
                        0 => RegisterPairAF::try_from(p).map(|pair| Instruction::POP(pair)),
                        1 => match p {
                            0 => Ok(Instruction::RET(None)),
                            1 => Ok(Instruction::RETI),
                            2 => Ok(Instruction::JPHL),
                            3 => Ok(Instruction::LD(LoadArgs::SPHL)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    2 => match y {
                        0..=3 => {
                            Condition::try_from(y).map(|condition| Instruction::JP(Some(condition)))
                        }
                        4 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::FFPlusC))),
                        5 => Ok(Instruction::LD(LoadArgs::IndirectFromA(Indirect::NN))),
                        6 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::FFPlusC))),
                        7 => Ok(Instruction::LD(LoadArgs::AFromIndirect(Indirect::NN))),
                        _ => unreachable!(),
                    },
                    3 => match y {
                        0 => Ok(Instruction::JP(None)),
                        1 => Err("CB"),
                        2..=5 => Err(""),
                        6 => Ok(Instruction::DI),
                        7 => Ok(Instruction::EI),
                        _ => unreachable!(),
                    },
                    4 => match y {
                        0..=3 => Condition::try_from(y)
                            .map(|condition| Instruction::CALL(Some(condition))),
                        4..=7 => Err(""),
                        _ => unreachable!(),
                    },
                    5 => match q {
                        0 => RegisterPairAF::try_from(p).map(|pair| Instruction::PUSH(pair)),
                        1 => match p {
                            0 => Ok(Instruction::CALL(None)),
                            1..=3 => Err(""),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    6 => AluOp::try_from(y).map(|op| match op {
                        AluOp::ADD => Instruction::ADD(AddArg::ImmediateU8),
                        AluOp::ADC => Instruction::ADC(AluArg::ImmediateU8),
                        AluOp::SUB => Instruction::SUB(AluArg::ImmediateU8),
                        AluOp::SBC => Instruction::SBC(AluArg::ImmediateU8),
                        AluOp::AND => Instruction::AND(AluArg::ImmediateU8),
                        AluOp::XOR => Instruction::XOR(AluArg::ImmediateU8),
                        AluOp::OR => Instruction::OR(AluArg::ImmediateU8),
                        AluOp::CP => Instruction::CP(AluArg::ImmediateU8),
                    }),
                    7 => Ok(Instruction::RST(u16::from(y * 8))),
                    _ => unreachable!(),
                }
            }
            _ => Err(""),
        }
    }

    pub fn try_decode_prefixed(value: u8) -> Result<Self, &'static str> {
        let x = (value & 0b1100_0000) >> 6;
        let y = (value & 0b0011_1000) >> 3;
        let z = value & 0b0000_0111;

        match x {
            // FOR x=0
            0 => Rotation::try_from(y).and_then(|rotation| {
                Register::try_from(z).map(|register| match rotation {
                    Rotation::RLC => Instruction::RLC(register),
                    Rotation::RRC => Instruction::RRC(register),
                    Rotation::RL => Instruction::RL(register),
                    Rotation::RR => Instruction::RR(register),
                    Rotation::SLA => Instruction::SLA(register),
                    Rotation::SRA => Instruction::SRA(register),
                    Rotation::SWAP => Instruction::SWAP(register),
                    Rotation::SRL => Instruction::SRL(register),
                })
            }),
            // FOR x=1
            1 => Register::try_from(z).map(|register| Instruction::BIT(y, register)),
            // FOR x=2
            2 => Register::try_from(z).map(|register| Instruction::RES(y, register)),
            // FOR x=3
            3 => Register::try_from(z).map(|register| Instruction::SET(y, register)),
            _ => Err(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    #[test]
    fn test_try_decode() {
        let result: Vec<_> = (0..u8::MAX).map(Instruction::try_decode).collect();

        assert_eq!(
            vec![
                Ok(NOP),
                Ok(LD(LoadArgs::RegisterPairFromImmediateU16(
                    RegisterPairSP::BC
                ))),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::BC))),
                Ok(INC(IncDecArg::RegisterPairSP(RegisterPairSP::BC))),
                Ok(INC(IncDecArg::Register(Register::B))),
                Ok(DEC(IncDecArg::Register(Register::B))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::B))),
                Ok(RLCA),
                Ok(LD(LoadArgs::NNSP)),
                Ok(ADD(AddArg::RegisterPairSP(RegisterPairSP::BC))),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::BC))),
                Ok(DEC(IncDecArg::RegisterPairSP(RegisterPairSP::BC))),
                Ok(INC(IncDecArg::Register(Register::C))),
                Ok(DEC(IncDecArg::Register(Register::C))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::C))),
                Ok(RRCA),
                Ok(STOP),
                Ok(LD(LoadArgs::RegisterPairFromImmediateU16(
                    RegisterPairSP::DE
                ))),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::DE))),
                Ok(INC(IncDecArg::RegisterPairSP(RegisterPairSP::DE))),
                Ok(INC(IncDecArg::Register(Register::D))),
                Ok(DEC(IncDecArg::Register(Register::D))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::D))),
                Ok(RLA),
                Ok(JR(None)),
                Ok(ADD(AddArg::RegisterPairSP(RegisterPairSP::DE))),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::DE))),
                Ok(DEC(IncDecArg::RegisterPairSP(RegisterPairSP::DE))),
                Ok(INC(IncDecArg::Register(Register::E))),
                Ok(DEC(IncDecArg::Register(Register::E))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::E))),
                Ok(RRA),
                Ok(JR(Some(Condition::NonZero))),
                Ok(LD(LoadArgs::RegisterPairFromImmediateU16(
                    RegisterPairSP::HL
                ))),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::HLPlus))),
                Ok(INC(IncDecArg::RegisterPairSP(RegisterPairSP::HL))),
                Ok(INC(IncDecArg::Register(Register::H))),
                Ok(DEC(IncDecArg::Register(Register::H))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::H))),
                Ok(DAA),
                Ok(JR(Some(Condition::Zero))),
                Ok(ADD(AddArg::RegisterPairSP(RegisterPairSP::HL))),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::HLPlus))),
                Ok(DEC(IncDecArg::RegisterPairSP(RegisterPairSP::HL))),
                Ok(INC(IncDecArg::Register(Register::L))),
                Ok(DEC(IncDecArg::Register(Register::L))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::L))),
                Ok(CPL),
                Ok(JR(Some(Condition::NonCarry))),
                Ok(LD(LoadArgs::RegisterPairFromImmediateU16(
                    RegisterPairSP::SP
                ))),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::HLMinus))),
                Ok(INC(IncDecArg::RegisterPairSP(RegisterPairSP::SP))),
                Ok(INC(IncDecArg::Register(Register::HL))),
                Ok(DEC(IncDecArg::Register(Register::HL))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::HL))),
                Ok(SCF),
                Ok(JR(Some(Condition::Carry))),
                Ok(ADD(AddArg::RegisterPairSP(RegisterPairSP::SP))),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::HLMinus))),
                Ok(DEC(IncDecArg::RegisterPairSP(RegisterPairSP::SP))),
                Ok(INC(IncDecArg::Register(Register::A))),
                Ok(DEC(IncDecArg::Register(Register::A))),
                Ok(LD(LoadArgs::RegisterFromImmediateU8(Register::A))),
                Ok(CCF),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::B, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::C, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::D, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::E, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::H, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::L, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::L))),
                Ok(HALT),
                Ok(LD(LoadArgs::RegisterToRegister(Register::HL, Register::A))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::B))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::C))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::D))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::E))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::H))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::L))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::HL))),
                Ok(LD(LoadArgs::RegisterToRegister(Register::A, Register::A))),
                Ok(ADD(AddArg::Register(Register::B))),
                Ok(ADD(AddArg::Register(Register::C))),
                Ok(ADD(AddArg::Register(Register::D))),
                Ok(ADD(AddArg::Register(Register::E))),
                Ok(ADD(AddArg::Register(Register::H))),
                Ok(ADD(AddArg::Register(Register::L))),
                Ok(ADD(AddArg::Register(Register::HL))),
                Ok(ADD(AddArg::Register(Register::A))),
                Ok(ADC(AluArg::Register(Register::B))),
                Ok(ADC(AluArg::Register(Register::C))),
                Ok(ADC(AluArg::Register(Register::D))),
                Ok(ADC(AluArg::Register(Register::E))),
                Ok(ADC(AluArg::Register(Register::H))),
                Ok(ADC(AluArg::Register(Register::L))),
                Ok(ADC(AluArg::Register(Register::HL))),
                Ok(ADC(AluArg::Register(Register::A))),
                Ok(SUB(AluArg::Register(Register::B))),
                Ok(SUB(AluArg::Register(Register::C))),
                Ok(SUB(AluArg::Register(Register::D))),
                Ok(SUB(AluArg::Register(Register::E))),
                Ok(SUB(AluArg::Register(Register::H))),
                Ok(SUB(AluArg::Register(Register::L))),
                Ok(SUB(AluArg::Register(Register::HL))),
                Ok(SUB(AluArg::Register(Register::A))),
                Ok(SBC(AluArg::Register(Register::B))),
                Ok(SBC(AluArg::Register(Register::C))),
                Ok(SBC(AluArg::Register(Register::D))),
                Ok(SBC(AluArg::Register(Register::E))),
                Ok(SBC(AluArg::Register(Register::H))),
                Ok(SBC(AluArg::Register(Register::L))),
                Ok(SBC(AluArg::Register(Register::HL))),
                Ok(SBC(AluArg::Register(Register::A))),
                Ok(AND(AluArg::Register(Register::B))),
                Ok(AND(AluArg::Register(Register::C))),
                Ok(AND(AluArg::Register(Register::D))),
                Ok(AND(AluArg::Register(Register::E))),
                Ok(AND(AluArg::Register(Register::H))),
                Ok(AND(AluArg::Register(Register::L))),
                Ok(AND(AluArg::Register(Register::HL))),
                Ok(AND(AluArg::Register(Register::A))),
                Ok(XOR(AluArg::Register(Register::B))),
                Ok(XOR(AluArg::Register(Register::C))),
                Ok(XOR(AluArg::Register(Register::D))),
                Ok(XOR(AluArg::Register(Register::E))),
                Ok(XOR(AluArg::Register(Register::H))),
                Ok(XOR(AluArg::Register(Register::L))),
                Ok(XOR(AluArg::Register(Register::HL))),
                Ok(XOR(AluArg::Register(Register::A))),
                Ok(OR(AluArg::Register(Register::B))),
                Ok(OR(AluArg::Register(Register::C))),
                Ok(OR(AluArg::Register(Register::D))),
                Ok(OR(AluArg::Register(Register::E))),
                Ok(OR(AluArg::Register(Register::H))),
                Ok(OR(AluArg::Register(Register::L))),
                Ok(OR(AluArg::Register(Register::HL))),
                Ok(OR(AluArg::Register(Register::A))),
                Ok(CP(AluArg::Register(Register::B))),
                Ok(CP(AluArg::Register(Register::C))),
                Ok(CP(AluArg::Register(Register::D))),
                Ok(CP(AluArg::Register(Register::E))),
                Ok(CP(AluArg::Register(Register::H))),
                Ok(CP(AluArg::Register(Register::L))),
                Ok(CP(AluArg::Register(Register::HL))),
                Ok(CP(AluArg::Register(Register::A))),
                Ok(RET(Some(Condition::NonZero))),
                Ok(POP(RegisterPairAF::BC)),
                Ok(JP(Some(Condition::NonZero))),
                Ok(JP(None)),
                Ok(CALL(Some(Condition::NonZero))),
                Ok(PUSH(RegisterPairAF::BC)),
                Ok(ADD(AddArg::ImmediateU8)),
                Ok(RST(0)),
                Ok(RET(Some(Condition::Zero))),
                Ok(RET(None)),
                Ok(JP(Some(Condition::Zero))),
                Err("CB"),
                Ok(CALL(Some(Condition::Zero))),
                Ok(CALL(None)),
                Ok(ADC(AluArg::ImmediateU8)),
                Ok(RST(8)),
                Ok(RET(Some(Condition::NonCarry))),
                Ok(POP(RegisterPairAF::DE)),
                Ok(JP(Some(Condition::NonCarry))),
                Err(""),
                Ok(CALL(Some(Condition::NonCarry))),
                Ok(PUSH(RegisterPairAF::DE)),
                Ok(SUB(AluArg::ImmediateU8)),
                Ok(RST(16)),
                Ok(RET(Some(Condition::Carry))),
                Ok(RETI),
                Ok(JP(Some(Condition::Carry))),
                Err(""),
                Ok(CALL(Some(Condition::Carry))),
                Err(""),
                Ok(SBC(AluArg::ImmediateU8)),
                Ok(RST(24)),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::FFPlusN))),
                Ok(POP(RegisterPairAF::HL)),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::FFPlusC))),
                Err(""),
                Err(""),
                Ok(PUSH(RegisterPairAF::HL)),
                Ok(AND(AluArg::ImmediateU8)),
                Ok(RST(32)),
                Ok(ADD(AddArg::SPd)),
                Ok(JPHL),
                Ok(LD(LoadArgs::IndirectFromA(Indirect::NN))),
                Err(""),
                Err(""),
                Err(""),
                Ok(XOR(AluArg::ImmediateU8)),
                Ok(RST(40)),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::FFPlusN))),
                Ok(POP(RegisterPairAF::AF)),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::FFPlusC))),
                Ok(DI),
                Err(""),
                Ok(PUSH(RegisterPairAF::AF)),
                Ok(OR(AluArg::ImmediateU8)),
                Ok(RST(48)),
                Ok(LD(LoadArgs::HLSPd)),
                Ok(LD(LoadArgs::SPHL)),
                Ok(LD(LoadArgs::AFromIndirect(Indirect::NN))),
                Ok(EI),
                Err(""),
                Err(""),
                Ok(CP(AluArg::ImmediateU8))
            ],
            result
        );
    }

    #[test]
    fn test_try_decode_prefixed() {
        let result: Vec<_> = (0..u8::MAX).map(Instruction::try_decode_prefixed).collect();

        assert_eq!(
            vec![
                Ok(RLC(Register::B)),
                Ok(RLC(Register::C)),
                Ok(RLC(Register::D)),
                Ok(RLC(Register::E)),
                Ok(RLC(Register::H)),
                Ok(RLC(Register::L)),
                Ok(RLC(Register::HL)),
                Ok(RLC(Register::A)),
                Ok(RRC(Register::B)),
                Ok(RRC(Register::C)),
                Ok(RRC(Register::D)),
                Ok(RRC(Register::E)),
                Ok(RRC(Register::H)),
                Ok(RRC(Register::L)),
                Ok(RRC(Register::HL)),
                Ok(RRC(Register::A)),
                Ok(RL(Register::B)),
                Ok(RL(Register::C)),
                Ok(RL(Register::D)),
                Ok(RL(Register::E)),
                Ok(RL(Register::H)),
                Ok(RL(Register::L)),
                Ok(RL(Register::HL)),
                Ok(RL(Register::A)),
                Ok(RR(Register::B)),
                Ok(RR(Register::C)),
                Ok(RR(Register::D)),
                Ok(RR(Register::E)),
                Ok(RR(Register::H)),
                Ok(RR(Register::L)),
                Ok(RR(Register::HL)),
                Ok(RR(Register::A)),
                Ok(SLA(Register::B)),
                Ok(SLA(Register::C)),
                Ok(SLA(Register::D)),
                Ok(SLA(Register::E)),
                Ok(SLA(Register::H)),
                Ok(SLA(Register::L)),
                Ok(SLA(Register::HL)),
                Ok(SLA(Register::A)),
                Ok(SRA(Register::B)),
                Ok(SRA(Register::C)),
                Ok(SRA(Register::D)),
                Ok(SRA(Register::E)),
                Ok(SRA(Register::H)),
                Ok(SRA(Register::L)),
                Ok(SRA(Register::HL)),
                Ok(SRA(Register::A)),
                Ok(SWAP(Register::B)),
                Ok(SWAP(Register::C)),
                Ok(SWAP(Register::D)),
                Ok(SWAP(Register::E)),
                Ok(SWAP(Register::H)),
                Ok(SWAP(Register::L)),
                Ok(SWAP(Register::HL)),
                Ok(SWAP(Register::A)),
                Ok(SRL(Register::B)),
                Ok(SRL(Register::C)),
                Ok(SRL(Register::D)),
                Ok(SRL(Register::E)),
                Ok(SRL(Register::H)),
                Ok(SRL(Register::L)),
                Ok(SRL(Register::HL)),
                Ok(SRL(Register::A)),
                Ok(BIT(0, Register::B)),
                Ok(BIT(0, Register::C)),
                Ok(BIT(0, Register::D)),
                Ok(BIT(0, Register::E)),
                Ok(BIT(0, Register::H)),
                Ok(BIT(0, Register::L)),
                Ok(BIT(0, Register::HL)),
                Ok(BIT(0, Register::A)),
                Ok(BIT(1, Register::B)),
                Ok(BIT(1, Register::C)),
                Ok(BIT(1, Register::D)),
                Ok(BIT(1, Register::E)),
                Ok(BIT(1, Register::H)),
                Ok(BIT(1, Register::L)),
                Ok(BIT(1, Register::HL)),
                Ok(BIT(1, Register::A)),
                Ok(BIT(2, Register::B)),
                Ok(BIT(2, Register::C)),
                Ok(BIT(2, Register::D)),
                Ok(BIT(2, Register::E)),
                Ok(BIT(2, Register::H)),
                Ok(BIT(2, Register::L)),
                Ok(BIT(2, Register::HL)),
                Ok(BIT(2, Register::A)),
                Ok(BIT(3, Register::B)),
                Ok(BIT(3, Register::C)),
                Ok(BIT(3, Register::D)),
                Ok(BIT(3, Register::E)),
                Ok(BIT(3, Register::H)),
                Ok(BIT(3, Register::L)),
                Ok(BIT(3, Register::HL)),
                Ok(BIT(3, Register::A)),
                Ok(BIT(4, Register::B)),
                Ok(BIT(4, Register::C)),
                Ok(BIT(4, Register::D)),
                Ok(BIT(4, Register::E)),
                Ok(BIT(4, Register::H)),
                Ok(BIT(4, Register::L)),
                Ok(BIT(4, Register::HL)),
                Ok(BIT(4, Register::A)),
                Ok(BIT(5, Register::B)),
                Ok(BIT(5, Register::C)),
                Ok(BIT(5, Register::D)),
                Ok(BIT(5, Register::E)),
                Ok(BIT(5, Register::H)),
                Ok(BIT(5, Register::L)),
                Ok(BIT(5, Register::HL)),
                Ok(BIT(5, Register::A)),
                Ok(BIT(6, Register::B)),
                Ok(BIT(6, Register::C)),
                Ok(BIT(6, Register::D)),
                Ok(BIT(6, Register::E)),
                Ok(BIT(6, Register::H)),
                Ok(BIT(6, Register::L)),
                Ok(BIT(6, Register::HL)),
                Ok(BIT(6, Register::A)),
                Ok(BIT(7, Register::B)),
                Ok(BIT(7, Register::C)),
                Ok(BIT(7, Register::D)),
                Ok(BIT(7, Register::E)),
                Ok(BIT(7, Register::H)),
                Ok(BIT(7, Register::L)),
                Ok(BIT(7, Register::HL)),
                Ok(BIT(7, Register::A)),
                Ok(RES(0, Register::B)),
                Ok(RES(0, Register::C)),
                Ok(RES(0, Register::D)),
                Ok(RES(0, Register::E)),
                Ok(RES(0, Register::H)),
                Ok(RES(0, Register::L)),
                Ok(RES(0, Register::HL)),
                Ok(RES(0, Register::A)),
                Ok(RES(1, Register::B)),
                Ok(RES(1, Register::C)),
                Ok(RES(1, Register::D)),
                Ok(RES(1, Register::E)),
                Ok(RES(1, Register::H)),
                Ok(RES(1, Register::L)),
                Ok(RES(1, Register::HL)),
                Ok(RES(1, Register::A)),
                Ok(RES(2, Register::B)),
                Ok(RES(2, Register::C)),
                Ok(RES(2, Register::D)),
                Ok(RES(2, Register::E)),
                Ok(RES(2, Register::H)),
                Ok(RES(2, Register::L)),
                Ok(RES(2, Register::HL)),
                Ok(RES(2, Register::A)),
                Ok(RES(3, Register::B)),
                Ok(RES(3, Register::C)),
                Ok(RES(3, Register::D)),
                Ok(RES(3, Register::E)),
                Ok(RES(3, Register::H)),
                Ok(RES(3, Register::L)),
                Ok(RES(3, Register::HL)),
                Ok(RES(3, Register::A)),
                Ok(RES(4, Register::B)),
                Ok(RES(4, Register::C)),
                Ok(RES(4, Register::D)),
                Ok(RES(4, Register::E)),
                Ok(RES(4, Register::H)),
                Ok(RES(4, Register::L)),
                Ok(RES(4, Register::HL)),
                Ok(RES(4, Register::A)),
                Ok(RES(5, Register::B)),
                Ok(RES(5, Register::C)),
                Ok(RES(5, Register::D)),
                Ok(RES(5, Register::E)),
                Ok(RES(5, Register::H)),
                Ok(RES(5, Register::L)),
                Ok(RES(5, Register::HL)),
                Ok(RES(5, Register::A)),
                Ok(RES(6, Register::B)),
                Ok(RES(6, Register::C)),
                Ok(RES(6, Register::D)),
                Ok(RES(6, Register::E)),
                Ok(RES(6, Register::H)),
                Ok(RES(6, Register::L)),
                Ok(RES(6, Register::HL)),
                Ok(RES(6, Register::A)),
                Ok(RES(7, Register::B)),
                Ok(RES(7, Register::C)),
                Ok(RES(7, Register::D)),
                Ok(RES(7, Register::E)),
                Ok(RES(7, Register::H)),
                Ok(RES(7, Register::L)),
                Ok(RES(7, Register::HL)),
                Ok(RES(7, Register::A)),
                Ok(SET(0, Register::B)),
                Ok(SET(0, Register::C)),
                Ok(SET(0, Register::D)),
                Ok(SET(0, Register::E)),
                Ok(SET(0, Register::H)),
                Ok(SET(0, Register::L)),
                Ok(SET(0, Register::HL)),
                Ok(SET(0, Register::A)),
                Ok(SET(1, Register::B)),
                Ok(SET(1, Register::C)),
                Ok(SET(1, Register::D)),
                Ok(SET(1, Register::E)),
                Ok(SET(1, Register::H)),
                Ok(SET(1, Register::L)),
                Ok(SET(1, Register::HL)),
                Ok(SET(1, Register::A)),
                Ok(SET(2, Register::B)),
                Ok(SET(2, Register::C)),
                Ok(SET(2, Register::D)),
                Ok(SET(2, Register::E)),
                Ok(SET(2, Register::H)),
                Ok(SET(2, Register::L)),
                Ok(SET(2, Register::HL)),
                Ok(SET(2, Register::A)),
                Ok(SET(3, Register::B)),
                Ok(SET(3, Register::C)),
                Ok(SET(3, Register::D)),
                Ok(SET(3, Register::E)),
                Ok(SET(3, Register::H)),
                Ok(SET(3, Register::L)),
                Ok(SET(3, Register::HL)),
                Ok(SET(3, Register::A)),
                Ok(SET(4, Register::B)),
                Ok(SET(4, Register::C)),
                Ok(SET(4, Register::D)),
                Ok(SET(4, Register::E)),
                Ok(SET(4, Register::H)),
                Ok(SET(4, Register::L)),
                Ok(SET(4, Register::HL)),
                Ok(SET(4, Register::A)),
                Ok(SET(5, Register::B)),
                Ok(SET(5, Register::C)),
                Ok(SET(5, Register::D)),
                Ok(SET(5, Register::E)),
                Ok(SET(5, Register::H)),
                Ok(SET(5, Register::L)),
                Ok(SET(5, Register::HL)),
                Ok(SET(5, Register::A)),
                Ok(SET(6, Register::B)),
                Ok(SET(6, Register::C)),
                Ok(SET(6, Register::D)),
                Ok(SET(6, Register::E)),
                Ok(SET(6, Register::H)),
                Ok(SET(6, Register::L)),
                Ok(SET(6, Register::HL)),
                Ok(SET(6, Register::A)),
                Ok(SET(7, Register::B)),
                Ok(SET(7, Register::C)),
                Ok(SET(7, Register::D)),
                Ok(SET(7, Register::E)),
                Ok(SET(7, Register::H)),
                Ok(SET(7, Register::L)),
                Ok(SET(7, Register::HL))
            ],
            result
        );
    }
}
