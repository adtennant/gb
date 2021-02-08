use super::flags::Flags;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: Flags::Zero | Flags::HalfCarry | Flags::Carry,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

impl Registers {
    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn e(&self) -> u8 {
        self.e
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn f(&self) -> Flags {
        self.f
    }

    pub fn set_f(&mut self, value: Flags) {
        self.f = value;
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a(), self.f().bits()])
    }

    pub fn set_af(&mut self, value: u16) {
        let [a, f] = value.to_be_bytes();

        self.a = a;
        self.f = Flags::from_bits_truncate(f);
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn set_bc(&mut self, value: u16) {
        let [b, c] = value.to_be_bytes();

        self.b = b;
        self.c = c;
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn set_de(&mut self, value: u16) {
        let [d, e] = value.to_be_bytes();

        self.d = d;
        self.e = e;
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_hl(&mut self, value: u16) {
        let [h, l] = value.to_be_bytes();

        self.h = h;
        self.l = l;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_af() {
        let mut registers = Registers::default();
        registers.set_a(0x42);
        registers.set_f(Flags::Zero);

        assert_eq!(0x4280, registers.af());

        let mut registers = Registers::default();
        registers.set_af(0x4269);

        assert_eq!(0x42, registers.a());
        assert_eq!(Flags::Subtract | Flags::HalfCarry, registers.f());
    }

    #[test]
    fn test_bc() {
        let mut registers = Registers::default();
        registers.set_b(0x42);
        registers.set_c(0x69);

        assert_eq!(0x4269, registers.bc());

        let mut registers = Registers::default();
        registers.set_bc(0x4269);

        assert_eq!(0x42, registers.b());
        assert_eq!(0x69, registers.c());
    }

    #[test]
    fn test_de() {
        let mut registers = Registers::default();
        registers.set_d(0x42);
        registers.set_e(0x69);

        assert_eq!(0x4269, registers.de());

        let mut registers = Registers::default();
        registers.set_de(0x4269);

        assert_eq!(0x42, registers.d());
        assert_eq!(0x69, registers.e());
    }

    #[test]
    fn test_hl() {
        let mut registers = Registers::default();
        registers.set_h(0x42);
        registers.set_l(0x69);

        assert_eq!(0x4269, registers.hl());

        let mut registers = Registers::default();
        registers.set_hl(0x4269);

        assert_eq!(0x42, registers.h());
        assert_eq!(0x69, registers.l());
    }
}
