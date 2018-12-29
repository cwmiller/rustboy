use std::fmt;

#[derive(Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::F => write!(f, "F"),
            Register::H => write!(f, "H"),
            Register::L => write!(f, "L"),
            Register::AF => write!(f, "AF"),
            Register::BC => write!(f, "BC"),
            Register::DE => write!(f, "DE"),
            Register::HL => write!(f, "HL"),
            Register::SP => write!(f, "SP"),
            Register::PC => write!(f, "PC"),
        }
    }
}

pub const FLAG_Z: u8 = 0b1000_0000;
pub const FLAG_N: u8 = 0b0100_0000;
pub const FLAG_H: u8 = 0b0010_0000;
pub const FLAG_C: u8 = 0b0001_0000;

#[derive(Default)]
pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16
}

impl Registers {
    pub fn a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    pub fn set_a(&mut self, val: u8) {
        let af = (self.af & 0x00FF) | ((val as u16) << 8);
        self.set_af(af);
    }

    pub fn b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    pub fn set_b(&mut self, val: u8) {
        self.bc = (self.bc & 0x00FF) | ((val as u16) << 8);
    }

    pub fn c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }

    pub fn set_c(&mut self, val: u8) {
        self.bc = (self.bc & 0xFF00) | (val as u16) & 0xFF;
    }

    pub fn d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    pub fn set_d(&mut self, val: u8) {
        self.de = (self.de & 0x00FF) | ((val as u16) << 8);
    }

    pub fn e(&self) -> u8 {
        (self.de & 0x00FF) as u8
    }

    pub fn set_e(&mut self, val: u8) {
        self.de = self.de & 0xFF00 | (val as u16) & 0xFF;
    }

    pub fn f(&self) -> u8 {
        (self.af & 0x00FF) as u8
    }

    pub fn set_f(&mut self, val: u8) {
        let af = (self.af & 0xFF00) | (val as u16) & 0xFF;
        self.set_af(af);
    }

    pub fn h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    pub fn set_h(&mut self, val: u8) {
        self.hl = (self.hl & 0x00FF) | ((val as u16) << 8);
    }

    pub fn l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    pub fn set_l(&mut self, val: u8) {
        self.hl = (self.hl & 0xFF00) | (val as u16) & 0xFF;
    }

    pub fn af(&self) -> u16 {
        self.af
    }

    pub fn set_af(&mut self, val: u16) {
        // Do not allow values to be written to the lower nibble of the F register.
        self.af = val & 0xFFF0;
    }

    pub fn bc(&self) -> u16 {
        self.bc
    }

    pub fn set_bc(&mut self, val: u16) {
        self.bc = val;
    }

    pub fn de(&self) -> u16 {
        self.de
    }

    pub fn set_de(&mut self, val: u16) {
        self.de = val;
    }

    pub fn hl(&self) -> u16 {
        self.hl
    }

    pub fn set_hl(&mut self, val: u16) {
        self.hl = val;
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }
    
    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        (self.f() & flag) == flag
    }

    pub fn set_flag(&mut self, flag: u8, set: bool) {
        let flags = if set {
            self.f() | flag
        } else {
            self.f() & !flag
        };

        self.set_f(flags);
    }

    pub fn carry(&self) -> bool {
        self.has_flag(FLAG_C)
    }

    pub fn set_carry(&mut self, set: bool) {
        self.set_flag(FLAG_C, set);
    }

    pub fn halfcarry(&self) -> bool {
        self.has_flag(FLAG_H)
    }

    pub fn set_halfcarry(&mut self, set: bool) {
        self.set_flag(FLAG_H, set);
    }

    pub fn subtract(&self) -> bool {
        self.has_flag(FLAG_N)
    }

    pub fn set_subtract(&mut self, set: bool) {
        self.set_flag(FLAG_N, set);
    }

    pub fn zero(&self) -> bool {
        self.has_flag(FLAG_N)
    }

    pub fn set_zero(&mut self, set: bool) {
        self.set_flag(FLAG_Z, set);
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "A : {:#04X}\tF : {:#04X} ({}{}{}{})",
            self.a(), 
            self.f(),
            if self.zero() { "Z" } else { "" },
            if self.subtract() { "N" } else { "" },
            if self.halfcarry() { "H" } else { "" },
            if self.carry() { "C" } else { "" }
        )?;

        writeln!(f, "B : {:#04X}\tC : {:#04X}", self.b(), self.c())?;
        writeln!(f, "D : {:#04X}\tE : {:#04X}", self.d(), self.e())?;
        writeln!(f, "H : {:#04X}\tL : {:#04X}", self.h(), self.l())?;
        writeln!(f, "SP: {:#06X}\tPC: {:#06X}", self.sp(), self.pc())
    }
}