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

#[derive(Copy, Clone, PartialEq)]
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

struct DecodedInstruction {
    opcode: u8,
    prefixed: bool,
    instruction: Instruction
}

enum_from_primitive! {
    #[derive(Copy, Clone)]
    pub enum Interrupt {
        Joypad = 0b0001_0000,
        Serial = 0b0000_1000,
        Timer  = 0b0000_0100,
        Stat   = 0b0000_0010,
        VBlank = 0b0000_0001
    }
}

fn interrupt_start_address(interrupt: Interrupt) -> u16 {
    match interrupt {
        Interrupt::Joypad => 0x60,
        Interrupt::Serial => 0x58,
        Interrupt::Timer  => 0x50,
        Interrupt::Stat   => 0x48,
        Interrupt::VBlank => 0x40
    }
}

// Number of clock cycles (not machine cycles) used by each instruction. 
// First number is cycles used if instruction is conditional and condition is met. Secodn number if cycles used if condition is not met.
static CYCLES: &'static [(usize, usize)] = &[
    (4, 4), (12, 12), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4), (20, 20), (8, 8), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (12, 12), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4), (12, 12), (8, 8), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4),
    (12, 8), (12, 12), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4), (12, 8), (8, 8), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4),
    (12, 8), (12, 12), (8, 8), (8, 8), (12, 12), (12, 12), (12, 12), (4, 4), (12, 8), (8, 8), (8, 8), (8, 8), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (4, 4), (8, 8), (4, 4),
    (20, 8), (12, 12), (16, 12), (16, 16), (24, 12), (16, 16), (8, 8), (16, 16), (20, 8), (16, 16), (16, 12), (4, 4), (24, 24), (24, 24), (8, 8), (16, 16),
    (20, 8), (12, 12), (16, 12), (0, 0), (24, 12), (16, 16), (8, 8), (16, 16), (20, 8), (16, 16), (16, 12), (0, 0), (24, 12), (0, 0), (8, 8), (16, 16),
    (12, 12), (12, 12), (8, 8), (0, 0), (0, 0), (16, 16), (8, 8), (16, 16), (16, 16), (4, 4), (16, 16), (0, 0), (0, 0), (0, 0), (8, 8), (16, 16),
    (12, 12), (12, 12), (8, 8), (4, 4), (0, 0), (16, 16), (8, 8), (16, 16), (12, 12), (8, 8), (16, 16), (4, 4), (0, 0), (0, 0), (8, 8), (16, 16)
];

static PREFIXED_CYCLES: &'static [(usize, usize)] = &[
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (12, 12), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (12, 12), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (12, 12), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (12, 12), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8),
    (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (8, 8), (16, 16), (8, 8)
];

const DISPATCH_CYCLES: usize = 20; 
const CLEAR_HALT_CYCLES: usize = 4;

pub struct Cpu {
    pub regs: Registers,
    ime: bool,
    halted: bool
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::default(),
            ime: true,
            halted: false
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
    }

    pub fn step(&mut self, bus: &mut Bus) -> usize {
        let mut used_cycles = 0;
        let interrupt = self.pending_interrupt(bus);

        // If an interrupt is pending and interrupts are enabled, jump to interrupt
        if interrupt.is_some() && self.ime {
            self.handle_interrupt(bus, interrupt.unwrap());
            
            // It takes 20 cycles to dispatch interrupt, 24 if HALTed.
            used_cycles += DISPATCH_CYCLES;

            if self.halted {
                // Clear HALT.
                self.halted = false;
                used_cycles += CLEAR_HALT_CYCLES;   
            }
        // An an interrupt is pending, interrupts are disabled, and the CPU is halted, then unhalt the CPU
        } else if interrupt.is_some() & !self.ime && self.halted {
            self.halted = false;
            used_cycles += CLEAR_HALT_CYCLES;
        // CPU is halted and there's no interrupt
        } else if self.halted {
            used_cycles += 4;
        // Else execute the next instruction
        } else {
            let decoded = self.decode_next_instruction(bus);
            let condition_met = inst::execute(self, bus, &decoded.instruction);

            let (cond_met_cycles, cond_not_met_cycles) = 
                if decoded.prefixed {
                    PREFIXED_CYCLES[decoded.opcode as usize]
                } else {
                    CYCLES[decoded.opcode as usize]
                };

            used_cycles = 
                if condition_met { 
                    cond_met_cycles
                } else { 
                    cond_not_met_cycles
                }
        };

        used_cycles
    }

    fn decode_next_instruction(&mut self, bus: &Bus) -> DecodedInstruction {
        let mut opcode = self.step_next_byte(bus);
        let mut prefixed = false;

        if opcode == 0xCB {
            prefixed = true;
            opcode = self.step_next_byte(bus);
        }

        DecodedInstruction {
            opcode: opcode,
            prefixed: prefixed,
            instruction: inst::decode(self, bus, opcode, prefixed)
        }        
    }

    pub fn step_next_byte(&mut self, bus: &Bus) -> u8 {
        let pc = self.regs.pc();
        let byte = bus.read(pc);
        self.regs.set_pc(pc.wrapping_add(1));

        byte
    }

    pub fn step_next_word(&mut self, bus: &Bus) -> u16 {
        let lb = self.step_next_byte(bus);
        let hb = self.step_next_byte(bus);

        LittleEndian::read_u16(&[lb, hb])
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
            let mut flag = Interrupt::VBlank as u8;
            while flag > 0 {
                if interrupts & flag == flag {
                    let interrupt = Interrupt::from_u8(flag).unwrap();
                    result = Some(interrupt);
                    break;
                }

                flag = flag << 1;
            }
        }

        result
    }

    fn handle_interrupt(&mut self, bus: &mut Bus, interrupt: Interrupt) {
        let addr = interrupt_start_address(interrupt);

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
        writeln!(f, "Interrupts: {}",
            if self.ime { "Enabled" } else { "Disabled" }
        )?;

        writeln!(f, "Registers:")?;
        write!(f, "{:?}", self.regs)
    }
}