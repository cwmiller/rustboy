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
    Z,
    C,
    Nz,
    Nc
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Condition::Z => write!(f, "Z"),
            Condition::C => write!(f, "C"),
            Condition::Nz => write!(f, "NZ"),
            Condition::Nc => write!(f, "NC")
        }
    }
}

fn condition_table(idx: u8) -> Condition {
    match idx {
        0 => Condition::Nz,
        1 => Condition::Z,
        2 => Condition::Nc,
        3 => Condition::C,
        _ => unreachable!()
    }
}

fn register_addr_table(idx: u8) -> Box<AddressingMode<u8>> {
    match idx {
        0 => Box::new(RegisterAddressing(Register::B)),
        1 => Box::new(RegisterAddressing(Register::C)),
        2 => Box::new(RegisterAddressing(Register::D)),
        3 => Box::new(RegisterAddressing(Register::E)),
        4 => Box::new(RegisterAddressing(Register::H)),
        5 => Box::new(RegisterAddressing(Register::L)),
        6 => Box::new(RegisterIndirectAddressing(Register::HL)),
        7 => Box::new(RegisterAddressing(Register::A)),
        _ => unreachable!()
    }
}

fn register_pair_table(idx: u8) -> Register {
    match idx {
        0 => Register::BC,
        1 => Register::DE,
        2 => Register::HL,
        3 => Register::SP,
        _ => unreachable!()
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
        let opcode = self.next_byte(&bus);

        println!("{:#X}: {:#X}", pc, opcode);

        self.decode_and_execute(bus, opcode);

        0
    }

    fn next_byte(&mut self, bus: &Bus) -> u8 {
        let pc = self.regs.pc();
        let byte = bus.read(pc);
        self.regs.set_pc(pc.wrapping_add(1));

        byte
    }

    fn next_word(&mut self, bus: &Bus) -> u16 {
        let lb = self.next_byte(bus);
        let hb = self.next_byte(bus);

        LittleEndian::read_u16(&[lb, hb])
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
            Condition::C => (self.regs.f() & FLAG_C) == FLAG_C,
            Condition::Z => (self.regs.f() & FLAG_Z) == FLAG_Z,
            Condition::Nc => (self.regs.f() & FLAG_C) != FLAG_C,
            Condition::Nz => (self.regs.f() & FLAG_Z) != FLAG_Z
        }
    }

    fn decode_and_execute(&mut self, bus: &mut Bus, opcode: u8) {
        let x = (0b11000000 & opcode) >> 6;
        let y = (0b00111000 & opcode) >> 3;
        let z = 0b00000111 & opcode;
        let p = y >> 1;
        let q = (0b00001000 & opcode) >> 3;

        match x {
            0 => match z {
                0 => self.decode_x_0_z_0(bus, y),
                1 => self.decode_x_0_z_1(bus, q, p),
                2 => self.decode_x_0_z_2(bus, q, p),
                3 => self.decode_x_0_z_3(bus, q, p),
                4 => {
                    // INC r
                    let dest = register_addr_table(y);
                    inst::inc_8(self, bus, dest.as_ref());
                },
                5 => {
                    // DEC r
                    let dest = register_addr_table(y);
                    inst::dec_8(self, bus, dest.as_ref());
                },
                6 => {
                    // LD r, n
                    let dest = register_addr_table(y);
                    let src = ImmediateAddressing(self.next_byte(bus));
                    inst::ld(self, bus, dest.as_ref(), &src);
                },
                7 => self.decode_x_0_z_7(bus, y),
                _ => unreachable!()
            },
            1 => {
                if y == 6 {
                    // HALT
                    inst::halt();
                } else {
                    // LD r, r
                    let dest = register_addr_table(y);
                    let src = register_addr_table(z);
                    inst::ld(self, bus, dest.as_ref(), src.as_ref());
                }
            },
            2 => self.decode_x_2(bus, y, z),
            3 => self.decode_x_3(bus, y, z, q, p),
            _ => unreachable!()
        };
    }

    fn decode_x_0_z_0(&mut self, bus: &mut Bus, y: u8) {
        match y {
            0 => inst::nop(), // NOP
            1 => {
                // LD (nn),SP
                let dest = ExtendedAddressing(self.next_word(bus));
                let src = RegisterAddressing(Register::SP);
                inst::ld(self, bus, &dest, &src);
            },
            2 => { 
                // STOP 0
                self.next_byte(bus); 
                inst::stop();
            },
            3 => { 
                // JR i8
                let offset = ImmediateAddressing(self.next_byte(bus));
                inst::jr(self, bus, &offset); 
            },
            4...7 => { 
                // JR cc, i8
                let condition = condition_table(y - 4);
                if self.condition_met(condition) {
                    let offset = ImmediateAddressing(self.next_byte(bus));
                    inst::jr(self, bus, &offset); 
                }
            },
            _ => unreachable!()
        }
    }

    fn decode_x_0_z_1(&mut self, bus: &mut Bus, q: u8, p: u8) {
        match q {
            0 => {
                // LD rr, nn
                let register = register_pair_table(p);
                let dest = RegisterAddressing(register);
                let src = ImmediateAddressing(self.next_word(bus));
                inst::ld(self, bus, &dest, &src);
            },
            1 => {
                // ADD HL, rr
                let register = register_pair_table(p);
                let dest = RegisterAddressing(Register::HL);
                let src = RegisterAddressing(register);
                inst::add_16(self, bus, &dest, &src);
            },
            _ => unreachable!()
        }
    }

    fn decode_x_0_z_2(&mut self, bus: &mut Bus, q: u8, p: u8) {
        match q {
            0 => match p {
                0 => {
                    // LD (BC), A
                    let dest = RegisterIndirectAddressing(Register::BC);
                    let src = RegisterAddressing(Register::A);
                    inst::ld(self, bus, &dest, &src);
                },
                1 => {
                    // LD (DE), A
                    let dest = RegisterIndirectAddressing(Register::DE);
                    let src = RegisterAddressing(Register::A);
                    inst::ld(self, bus, &dest, &src);
                },
                2 => {
                    // LD (nn), HL
                    let dest = ExtendedAddressing(self.next_word(bus));
                    let src = RegisterAddressing(Register::HL);
                    inst::ld(self, bus, &dest, &src);
                },
                3 => {
                    // LD (nn), A
                    let dest = ExtendedAddressing(self.next_word(bus));
                    let src = RegisterAddressing(Register::A);
                    inst::ld(self, bus, &dest, &src);
                },
                _ => unreachable!()
            },
            1 => match p {
                0 => {
                    // LD A, (BC)
                    let dest = RegisterAddressing(Register::A);
                    let src = RegisterIndirectAddressing(Register::BC);
                    inst::ld(self, bus, &dest, &src);
                },
                1 => {
                    // LD A, (DE)
                    let dest = RegisterAddressing(Register::A);
                    let src = RegisterIndirectAddressing(Register::DE);
                    inst::ld(self, bus, &dest, &src);
                },
                2 => {
                    // LD HL, (nn)
                    let dest = RegisterAddressing(Register::HL);
                    let src = ExtendedAddressing(self.next_word(bus));
                    inst::ld(self, bus, &dest, &src);
                },
                3 => {
                    // LD A, (nn)
                    let dest = RegisterAddressing(Register::A);
                    let src = ExtendedAddressing(self.next_word(bus));
                    inst::ld(self, bus, &dest, &src);
                },
                _ => unreachable!()
            },
            _ => unreachable!()
        }
    }

    fn decode_x_0_z_3(&mut self, bus: &mut Bus, q: u8, p: u8) {
        let register = register_pair_table(p);
        let dest = RegisterAddressing(register);

        match q {
            0 => inst::inc_16(self, bus, &dest),    // INC rr
            1 => inst::dec_16(self, bus, &dest),    // DEC rr
            _ => unreachable!()
        }
    }

    fn decode_x_0_z_7(&mut self, bus: &mut Bus, y: u8) {
        match y {
            0 => inst::rlca(self),  // RLCA
            1 => inst::rrca(self),  // RRCA
            2 => inst::rla(self),   // RLA
            3 => inst::rra(self),   // RRA
            4 => inst::daa(self),   // DAA
            5 => inst::cpl(self),   // CPL
            6 => inst::scf(self),   // SCF
            7 => inst::ccf(self),   // CCF
            _ => unreachable!()
        }
    }

    fn decode_x_2(&mut self, bus: &mut Bus, y: u8, z: u8) {
        let src = register_addr_table(z);

        match y {
            0 => inst::add_8(self, bus, src.as_ref()),  // ADD A, r
            1 => inst::adc(self, bus, src.as_ref()),    // ADC A, r
            2 => inst::sub(self, bus, src.as_ref()),    // SUB r
            3 => inst::sbc(self, bus, src.as_ref()),    // SBC A, r
            4 => inst::and(self, bus, src.as_ref()),    // AND
            5 => inst::xor(self, bus, src.as_ref()),    // XOR
            6 => inst::or(self, bus, src.as_ref()),     // OR
            7 => inst::cp(self, bus, src.as_ref()),     // CP
            _ => unreachable!()
        }
    }

    fn decode_x_3(&mut self, bus: &mut Bus, y: u8, z: u8, q: u8, p: u8) {
        match z {
            0 => {
                let condition = condition_table(y);
                if self.condition_met(condition) {
                    inst::ret(self, bus);
                }
            },
            _ => unreachable!()
        }
    }
}