pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    FLAGS,
    H,
    L
}

pub enum Register16 {
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
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub flags: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16
}

impl Registers {
    pub fn read_u8(&self, reg: &Register8) -> u8 {
        use self::Register8::*;

        match *reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            FLAGS => self.flags,
            H => self.h,
            L => self.l
        }
    }

    pub fn write_u8(&mut self, reg: &Register8, val: u8) {
        use self::Register8::*;

        match *reg {
            A => self.a = val,
            B => self.b = val,
            C => self.c = val,
            D => self.d = val,
            E => self.e = val,
            FLAGS => self.flags = val,
            H => self.h = val,
            L => self.l = val
        }
    }

    pub fn read_u16(&self, reg: &Register16) -> u16 {
        use self::Register16::*;

        match *reg {
            AF => ((self.a as u16) << 8) | (self.flags as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
            SP => self.sp,
            PC => self.pc
        }
    }

    pub fn write_u16(&mut self, reg: &Register16, val: u16) {
        use self::Register16::*;

        match *reg {
            AF => {
                self.a = (val >> 8) as u8;
                self.flags = (val & 0x0F) as u8;
            },
            BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0x0F) as u8;
            },
            DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0x0F) as u8;
            },
            HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0x0F) as u8;
            },
            SP => self.sp = val,
            PC => self.pc = val
        }
    }
}
