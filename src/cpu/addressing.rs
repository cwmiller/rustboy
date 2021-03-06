use super::Cpu;
use bus::{Addressable, Bus};
use super::registers::Register;
use std::fmt;

pub trait AddressingMode<T> : fmt::Display {
    fn read(&self, cpu: &Cpu, bus: &Bus) -> T;

    fn write(&self, cpu: &mut Cpu, bus: &mut Bus, val: T);
}

pub struct ImmediateAddressing<T>(pub T);

impl<T> AddressingMode<T> for ImmediateAddressing<T> where T : Copy + fmt::UpperHex {
    fn read(&self, _: &Cpu, _: &Bus) -> T {
        self.0
    }

    fn write(&self, _: &mut Cpu, _: &mut Bus, _: T) {
        panic!("Write not supported for immediate addressing.");
    }
}

impl<T> fmt::Display for ImmediateAddressing<T> where T : fmt::UpperHex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}

pub struct IndirectAddressing<T>(pub T);

impl AddressingMode<u8> for IndirectAddressing<u8> {
    fn read(&self, _: &Cpu, bus: &Bus) -> u8 {
        bus.read(0xFF00 + self.0 as u16)
    }

    fn write(&self, _: &mut Cpu, bus: &mut Bus, val: u8) {
        bus.write(0xFF00 + self.0 as u16, val);
    }
}

impl AddressingMode<u8> for IndirectAddressing<u16> {
    fn read(&self, _: &Cpu, bus: &Bus) -> u8 {
        bus.read(self.0)
    }

    fn write(&self, _: &mut Cpu, bus: &mut Bus, val: u8) {
        bus.write(self.0, val);
    }
}

impl AddressingMode<u16> for IndirectAddressing<u16> {
    fn read(&self, _: &Cpu, bus: &Bus) -> u16 {
        let low = bus.read(self.0) as u16;
        let high = bus.read(self.0 + 1) as u16;

        (high << 8) | low
    }

    fn write(&self, _: &mut Cpu, bus: &mut Bus, val: u16) {
        let low = (val & 0xFF) as u8;
        let high = ((val & 0xFF00) >> 8) as u8;

        bus.write(self.0, low);
        bus.write(self.0 + 1, high);
    }
}

impl<T> fmt::Display for IndirectAddressing<T> where T : fmt::UpperHex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:#X})", self.0)
    }
}

pub struct RegisterAddressing(pub Register);

impl AddressingMode<u8> for RegisterAddressing {
    fn read(&self, cpu: &Cpu, _: &Bus) -> u8 {
        match self.0 {
            Register::A => cpu.regs.a(),
            Register::B => cpu.regs.b(),
            Register::C => cpu.regs.c(),
            Register::D => cpu.regs.d(),
            Register::E => cpu.regs.e(),
            Register::F => cpu.regs.f(),
            Register::H => cpu.regs.h(),
            Register::L => cpu.regs.l(),
            Register::AF
            | Register::BC
            | Register::DE
            | Register::HL
            | Register::PC
            | Register::SP => panic!("Attempted to read 8bit value from 16bit register")
        }
    }

    fn write(&self, cpu: &mut Cpu, _: &mut Bus, val: u8) {
        match self.0 {
            Register::A => cpu.regs.set_a(val),
            Register::B => cpu.regs.set_b(val),
            Register::C => cpu.regs.set_c(val),
            Register::D => cpu.regs.set_d(val),
            Register::E => cpu.regs.set_e(val),
            Register::F => cpu.regs.set_f(val),
            Register::H => cpu.regs.set_h(val),
            Register::L => cpu.regs.set_l(val),
            Register::AF
            | Register::BC
            | Register::DE
            | Register::HL
            | Register::PC
            | Register::SP => panic!("Attempted to write 8bit value to 16bit register")
        }
    }
}

impl AddressingMode<u16> for RegisterAddressing {
    fn read(&self, cpu: &Cpu, _: &Bus) -> u16 {
        match self.0 {
            Register::A
            | Register::B
            | Register::C
            | Register::D
            | Register::E
            | Register::F
            | Register::H
            | Register::L => panic!("Attempted to read 16bit value from 8bit register"),
            Register::AF => cpu.regs.af(),
            Register::BC => cpu.regs.bc(),
            Register::DE => cpu.regs.de(),
            Register::HL => cpu.regs.hl(),
            Register::PC => cpu.regs.pc(),
            Register::SP => cpu.regs.sp()
        }
    }

    fn write(&self, cpu: &mut Cpu, _: &mut Bus, val: u16) {
        match self.0 {
            Register::A
            | Register::B
            | Register::C
            | Register::D
            | Register::E
            | Register::F
            | Register::H
            | Register::L => panic!("Attempted to write 16bit value to 8bit register"),
            Register::AF => cpu.regs.set_af(val),
            Register::BC => cpu.regs.set_bc(val),
            Register::DE => cpu.regs.set_de(val),
            Register::HL => cpu.regs.set_hl(val),
            Register::PC => cpu.regs.set_pc(val),
            Register::SP => cpu.regs.set_sp(val)
        }
    }
}

impl fmt::Display for RegisterAddressing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct RegisterIndirectAddressing(pub Register);

impl AddressingMode<u8> for RegisterIndirectAddressing {
    fn read(&self, cpu: &Cpu, bus: &Bus) -> u8 {
        let addr = register_indirect_addr(cpu, &self.0);
        bus.read(addr)
    }

    fn write(&self, cpu: &mut Cpu, bus: &mut Bus, val: u8) {
        let addr = register_indirect_addr(cpu, &self.0);
        bus.write(addr, val)
    }
}

impl fmt::Display for RegisterIndirectAddressing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

fn register_indirect_addr(cpu: &Cpu, reg: &Register) -> u16 {
    match *reg {
        Register::A => 0xFF00 | (cpu.regs.a() as u16),
        Register::B => 0xFF00 | (cpu.regs.b() as u16),
        Register::C => 0xFF00 | (cpu.regs.c() as u16),
        Register::D => 0xFF00 | (cpu.regs.d() as u16),
        Register::E => 0xFF00 | (cpu.regs.e() as u16),
        Register::F => 0xFF00 | (cpu.regs.f() as u16),
        Register::H => 0xFF00 | (cpu.regs.h() as u16),
        Register::L => 0xFF00 | (cpu.regs.l() as u16),
        Register::AF => cpu.regs.af(),
        Register::BC => cpu.regs.bc(),
        Register::DE => cpu.regs.de(),
        Register::HL => cpu.regs.hl(),
        Register::PC => cpu.regs.pc(),
        Register::SP => cpu.regs.sp()
    }
}
