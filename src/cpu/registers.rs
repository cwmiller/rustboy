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

pub const FLAG_Z: u8 = 0b10000000;
pub const FLAG_N: u8 = 0b01000000;
pub const FLAG_H: u8 = 0b00100000;
pub const FLAG_C: u8 = 0b00010000;

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
        self.af = self.af | ((val as u16) << 8);
    }

    pub fn b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    pub fn set_b(&mut self, val: u8) {
        self.bc = self.bc | ((val as u16) << 8);
    }

    pub fn c(&self) -> u8 {
        (self.bc & 0xFF) as u8
    }

    pub fn set_c(&mut self, val: u8) {
        self.bc = self.bc | (val as u16) & 0xFF;
    }

    pub fn d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    pub fn set_d(&mut self, val: u8) {
        self.de = self.de | ((val as u16) << 8);
    }

    pub fn e(&self) -> u8 {
        (self.de & 0xFF) as u8
    }

    pub fn set_e(&mut self, val: u8) {
        self.de = self.de | (val as u16) & 0xFF;
    }

    pub fn f(&self) -> u8 {
        (self.af & 0xFF) as u8
    }

    pub fn set_f(&mut self, val: u8) {
        self.af = self.af | (val as u16) & 0xFF;
    }

    pub fn h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    pub fn set_h(&mut self, val: u8) {
        self.hl = self.hl | ((val as u16) << 8);
    }

    pub fn l(&self) -> u8 {
        (self.hl & 0xFF) as u8
    }

    pub fn set_l(&mut self, val: u8) {
        self.hl = self.hl | (val as u16) & 0xFF;
    }

    pub fn af(&self) -> u16 {
        self.af
    }

    pub fn set_af(&mut self, val: u16) {
        self.af = val;
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
}
