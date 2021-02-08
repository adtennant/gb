use super::instructions::{AluArg, Indirect, Register, RegisterPairAF, RegisterPairSP};

#[derive(Copy, Clone, Debug)]
pub enum In8 {
    A,       // A
    B,       // B
    C,       // C
    D,       // D
    E,       // E
    H,       // H
    L,       // L
    N,       // n
    BC,      // (BC)
    DE,      // (DE)
    HL,      // (HL)
    HLMinus, // (HL-)
    HLPlus,  // (HL+)
    NN,      // (nn)
    CHigh,   // (C)
    NHigh,   // ($FF00+n)
}

impl From<AluArg> for In8 {
    fn from(value: AluArg) -> In8 {
        match value {
            AluArg::Register(register) => In8::from(register),
            AluArg::ImmediateU8 => In8::N,
        }
    }
}

impl From<Indirect> for In8 {
    fn from(value: Indirect) -> In8 {
        match value {
            Indirect::BC => In8::BC,
            Indirect::DE => In8::DE,
            Indirect::FFPlusC => In8::CHigh,
            Indirect::FFPlusN => In8::NHigh,
            Indirect::HLMinus => In8::HLMinus,
            Indirect::HLPlus => In8::HLPlus,
            Indirect::NN => In8::NN,
        }
    }
}

impl From<Register> for In8 {
    fn from(value: Register) -> In8 {
        match value {
            Register::A => In8::A,
            Register::B => In8::B,
            Register::C => In8::C,
            Register::D => In8::D,
            Register::E => In8::E,
            Register::H => In8::H,
            Register::L => In8::L,
            Register::HL => In8::HL,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Out8 {
    A,       // A
    B,       // B
    C,       // C
    D,       // D
    E,       // E
    H,       // H
    L,       // L
    BC,      // (BC)
    DE,      // (DE)
    HL,      // (HL)
    HLMinus, // (HL-)
    HLPlus,  // (HL+)
    NN,      // (nn)
    CHigh,   // (C)
    NHigh,   // ($FF00+n)
}

impl From<Indirect> for Out8 {
    fn from(value: Indirect) -> Out8 {
        match value {
            Indirect::BC => Out8::BC,
            Indirect::DE => Out8::DE,
            Indirect::FFPlusC => Out8::CHigh,
            Indirect::FFPlusN => Out8::NHigh,
            Indirect::HLMinus => Out8::HLMinus,
            Indirect::HLPlus => Out8::HLPlus,
            Indirect::NN => Out8::NN,
        }
    }
}

impl From<Register> for Out8 {
    fn from(value: Register) -> Out8 {
        match value {
            Register::A => Out8::A,
            Register::B => Out8::B,
            Register::C => Out8::C,
            Register::D => Out8::D,
            Register::E => Out8::E,
            Register::H => Out8::H,
            Register::L => Out8::L,
            Register::HL => Out8::HL,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum In16 {
    AF, // AF
    BC, // BC
    DE, // DE
    HL, // HL
    NN, // nn
    SP, // SP
}

impl From<RegisterPairAF> for In16 {
    fn from(value: RegisterPairAF) -> In16 {
        match value {
            RegisterPairAF::AF => In16::AF,
            RegisterPairAF::BC => In16::BC,
            RegisterPairAF::DE => In16::DE,
            RegisterPairAF::HL => In16::HL,
        }
    }
}

impl From<RegisterPairSP> for In16 {
    fn from(value: RegisterPairSP) -> In16 {
        match value {
            RegisterPairSP::BC => In16::BC,
            RegisterPairSP::DE => In16::DE,
            RegisterPairSP::HL => In16::HL,
            RegisterPairSP::SP => In16::SP,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Out16 {
    AF, // AF
    BC, // BC
    DE, // DE
    HL, // HL
    NN, // (nn)
    SP, // SP
}

impl From<RegisterPairAF> for Out16 {
    fn from(value: RegisterPairAF) -> Out16 {
        match value {
            RegisterPairAF::AF => Out16::AF,
            RegisterPairAF::BC => Out16::BC,
            RegisterPairAF::DE => Out16::DE,
            RegisterPairAF::HL => Out16::HL,
        }
    }
}

impl From<RegisterPairSP> for Out16 {
    fn from(value: RegisterPairSP) -> Out16 {
        match value {
            RegisterPairSP::BC => Out16::BC,
            RegisterPairSP::DE => Out16::DE,
            RegisterPairSP::HL => Out16::HL,
            RegisterPairSP::SP => Out16::SP,
        }
    }
}
