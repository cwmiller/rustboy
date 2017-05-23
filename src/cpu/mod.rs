mod addressing;
mod instructions;
mod registers;

use self::addressing::*;
use bus::{Addressable, Bus, IO_IE_ADDR, IO_IF_ADDR};
use byteorder::{ByteOrder, LittleEndian};
use enum_primitive::FromPrimitive;
use self::instructions as inst;
use self::registers::*;
use std::fmt;

pub use self::instructions::{decode, Instruction};

#[derive(PartialEq)]
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
            Condition::Z    => write!(f, "Z"),
            Condition::C    => write!(f, "C"),
            Condition::Nz   => write!(f, "NZ"),
            Condition::Nc   => write!(f, "NC")
        }
    }
}

enum_from_primitive! {
#[derive(Copy, Clone)]
pub enum Interrupt {
    Keypad = 0b00010000,
    Serial = 0b00001000,
    Timer  = 0b00000100,
    Stat   = 0b00000010,
    VBlank = 0b00000001
}
}

fn interrupt_start_address(interrupt: Interrupt) -> u16 {
    match interrupt {
        Interrupt::Keypad => 0x60,
        Interrupt::Serial => 0x58,
        Interrupt::Timer  => 0x50,
        Interrupt::Stat   => 0x48,
        Interrupt::VBlank => 0x40
    }
}

static CYCLES: &'static [usize] = &[
    4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4,
    4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    12, 12, 8, 8, 12, 12, 12, 4, 12, 8, 8, 8, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    8, 8, 8, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4,
    20, 12, 16, 16, 24, 16, 8, 16, 20, 16, 16, 4, 24, 24, 8, 16,
    20, 12, 16, 0, 24, 16, 8, 16, 20, 16, 16, 0, 24, 0, 8, 16,
    12, 12, 8, 0, 0, 16, 8, 16, 16, 4, 16, 0, 0, 0, 8, 16,
    12, 12, 8, 4, 0, 16, 8, 16, 12, 8, 16, 4, 0, 0, 8, 16
];

static PREFIXED_CYCLES: &'static [usize] = &[
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8
];

pub struct Cpu {
    pub regs: Registers,
    ime: bool,
    halted: bool,
    prefixed: bool
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: Registers::default(),
            halted: false,
            ime: true,
            prefixed: false
        }
    }

    pub fn reset(&mut self) {
        self.regs.set_af(0x01B0);
        self.regs.set_bc(0x0013);
        self.regs.set_de(0x00D8);
        self.regs.set_hl(0x014D);
        self.regs.set_sp(0xFFFE);
        self.regs.set_pc(0x100);
        self.ime = true;
        self.prefixed = false;
    }

    pub fn step(&mut self, bus: &mut Bus) -> usize {
        let interrupt = self.pending_interrupt(bus);

        // If an interrupt is pending and interrupts are enabled, jump to interrupt
        if interrupt.is_some() && self.ime {
            self.handle_interrupt(bus, interrupt.unwrap());

            // It takes 20 cycles to dispatch interrupt, 24 if HALTed.
            if self.halted {
                // Clear HALT.
                self.halted = false;
                24
            } else {
                20
            }
        // An an interrupt is pending, interrupts are disabled, and the CPU is halted, then unhalt the CPU
        } else if interrupt.is_some() & !self.ime && self.halted {
            self.halted = true;
            4
        // Else execute the next instruction
        } else {
            let pc = self.regs.pc();
            let opcode = bus.read(pc);
            let mut length = 1;

            let cycles =
                if self.prefixed {
                    PREFIXED_CYCLES[opcode as usize & 0x0F]
                } else {
                    CYCLES[opcode as usize]
                };

            let decoded_instruction = {
                let mut next = || {
                    let byte = bus.read(pc.wrapping_add(length));
                    length = length + 1;
                    byte
                };

                inst::decode(opcode, self.prefixed, &mut next)
            };

            self.regs.set_pc(pc + length);

            if let Some(instruction) = decoded_instruction {
                //println!("{:#X}: {}", pc, instruction);
                inst::execute(self, bus, instruction);
            }

            cycles
        }
    }

    pub fn interrupt(&mut self, bus: &mut Bus, interrupt: Interrupt) {
        let flags = bus.read(IO_IF_ADDR);
        let interrupt_flag = interrupt as u8;

        self.halted = false;
        bus.write(IO_IF_ADDR, flags | interrupt_flag);
    }

    fn pop_stack(&mut self, bus: &Bus) -> u16 {
        let addr = self.regs.sp();
        let word = &[bus.read(addr), bus.read(addr.wrapping_add(1))];
        self.regs.set_sp(addr.wrapping_add(2));

        LittleEndian::read_u16(word)
    }

    fn push_stack(&mut self, bus: &mut Bus, val: u16) {
        let addr = self.regs.sp();
        
        bus.write(addr.wrapping_sub(2), (val & 0x00FF) as u8);
        bus.write(addr.wrapping_sub(1), ((val >> 8) & 0x00FF) as u8);

        self.regs.set_sp(addr.wrapping_sub(2));
    }

    fn condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::None => true,
            Condition::C    => (self.regs.f() & FLAG_C) == FLAG_C,
            Condition::Z    => (self.regs.f() & FLAG_Z) == FLAG_Z,
            Condition::Nc   => (self.regs.f() & FLAG_C) != FLAG_C,
            Condition::Nz   => (self.regs.f() & FLAG_Z) != FLAG_Z
        }
    }

    fn pending_interrupt(&self, bus: &Bus) -> Option<Interrupt> {
        let mut result = None;

        let interrupts = bus.read(IO_IF_ADDR) & bus.read(IO_IE_ADDR);
        if interrupts > 0 {
            let mut flag = Interrupt::Keypad as u8;
            while flag > 0 {
                if interrupts & flag == flag {
                    let interrupt = Interrupt::from_u8(flag).unwrap();
                    result = Some(interrupt);
                    break;
                }

                flag = flag >> 1;
            }
        }

        result
    }

    fn handle_interrupt(&mut self, bus: &mut Bus, interrupt: Interrupt) {
        let addr = interrupt_start_address(interrupt);

        println!("Interrupted! Setting PC to {:#X}", addr);

        // Push current PC to stack and set PC to interrupt address.
        let pc = self.regs.pc();
        self.push_stack(bus, pc);
        self.regs.set_pc(addr);

        // Disable interrupts
        self.ime = false;

        // Clear IF flag
        let if_register = bus.read(IO_IF_ADDR);
        bus.write(IO_IF_ADDR, if_register ^ (interrupt as u8));
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Ints: {}\tPrefixed: {}",
            if self.ime { "Enabled" } else { "Disabled" },
            if self.prefixed { "Yes" } else { "No" },
        )?;

        writeln!(f, "Registers:")?;
        write!(f, "{:?}", self.regs)
    }
}