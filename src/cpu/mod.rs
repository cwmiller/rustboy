mod addressing;
mod instructions;
mod registers;

use self::addressing::*;
use bus::{Bus, Addressable};
use byteorder::{ByteOrder, LittleEndian};
use self::instructions as inst;
use self::registers::*;
use std::fmt;

pub enum Condition {
    None,
    Z,
    C,
    Nz,
    Nc
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Condition::None => write!(f, ""),
            Condition::Z => write!(f, "Z"),
            Condition::C => write!(f, "C"),
            Condition::Nz => write!(f, "NZ"),
            Condition::Nc => write!(f, "NC")
        }
    }
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
        let pc = self.regs.pc();
        let opcode = bus.read(pc);
        let mut length = 1;
        let decoded_instruction;

        {
            let mut imm8 = || { 
                let byte = bus.read(pc.wrapping_add(length));
                length = length + 1; 
                byte
            };

            decoded_instruction = inst::decode(opcode, self.prefixed, &mut imm8);
        }

        self.regs.set_pc(pc + length);

        if decoded_instruction.is_some() {
            let instruction = decoded_instruction.unwrap();

            println!("{:#X}: {}", pc, instruction);
            inst::execute(self, bus, instruction);
        }

        0
    }

    fn pop_stack(&mut self, bus: &Bus) -> u16 {
        let addr = self.regs.sp();
        let pc = self.regs.pc();
        let word = &[bus.read(addr), bus.read(addr + 1)];
        self.regs.set_pc(pc + 2);

        LittleEndian::read_u16(word)
    }

    fn push_stack(&mut self, bus: &mut Bus, val: u16) {
        let addr = self.regs.sp();
        
        bus.write(addr - 2, (val & 0x00FF) as u8);
        bus.write(addr - 1, ((val >> 8) & 0x00FF) as u8);

        self.regs.set_sp(addr - 2);
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