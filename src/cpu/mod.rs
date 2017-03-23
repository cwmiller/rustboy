mod addressing;
mod registers;

use self::addressing::*;
use bus::{Bus, Addressable};
use self::registers::*;

enum Condition {
    None,
    Z,
    C,
    Nz,
    Nc
}

pub struct Cpu {
    regs: Registers,
    ime: bool,
    prefixed: bool
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers::default(),
            ime: false,
            prefixed: false
        }
    }

    pub fn power_on(&mut self) {
        self.regs.set_af(0x01B0);
        self.regs.set_bc(0x1300);
        self.regs.set_de(0xD800);
        self.regs.set_hl(0x4D01);
        self.regs.set_sp(0xFFFE);
        self.regs.set_pc(0x100);
        self.ime = true;
        self.prefixed = false;
    }

    pub fn step(&mut self, bus: &mut Bus) -> usize {
        let opcode = self.imm_byte(&bus);

        0
    }

    fn imm_byte(&mut self, bus: &Bus) -> u8 {
        let pc = self.regs.pc();
        let byte = bus.read(pc);
        self.regs.set_pc(pc.wrapping_add(1));

        byte
    }

    fn condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::None => true,
            Condition::C => (self.regs.f() & FLAG_C) == FLAG_C,
            Condition::Z => (self.regs.f() & FLAG_Z) == FLAG_Z,
            Condition::Nc => (self.regs.f() & FLAG_C) != FLAG_C,
            Condition::Nz => (self.regs.f() & FLAG_Z) != FLAG_Z
        }
    }
}