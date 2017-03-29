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
        4 => Register::BC,
        5 => Register::DE,
        6 => Register::HL,
        7 => Register::AF,
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

        println!("{:#X} -> {:#X},{:#X},{:#X},{:#X},{:#X},", opcode, x, y, z, p, q);

        match x {
            // X=0
            0 => match z {
                // X=0, Z=0
                0 => {
                    match y {
                        // X=0, Z=0, Y=0
                        // NOP
                        0 => inst::nop(), 
                        1 => {
                            // X=0, Z=0, Y=1
                            // LD (nn),SP
                            let dest = ExtendedAddressing(self.next_word(bus));
                            let src = RegisterAddressing(Register::SP);
                            inst::ld(self, bus, &dest, &src);
                        },
                        2 => { 
                            // X=0, Z=0, Y=2
                            // STOP 0
                            self.next_byte(bus); 
                            inst::stop();
                        },
                        3 => { 
                            // X=0, Z=0, Y=3
                            // JR i8
                            let offset = ImmediateAddressing(self.next_byte(bus));
                            inst::jr(self, bus, &offset); 
                        },
                        4...7 => { 
                            // X=0, Z=0, Y=4-7
                            // JR cc, i8
                            let condition = condition_table(y - 4);
                            if self.condition_met(condition) {
                                let offset = ImmediateAddressing(self.next_byte(bus));
                                inst::jr(self, bus, &offset); 
                            }
                        },
                        _ => unreachable!()
                    }
                },
                // X=0, Z=1
                1 => {
                    match q {
                        0 => {
                            // X=0, Z=1, Q=0
                            // LD rr, nn
                            let register = register_pair_table(p);
                            let dest = RegisterAddressing(register);
                            let src = ImmediateAddressing(self.next_word(bus));
                            inst::ld(self, bus, &dest, &src);
                        },
                        1 => {
                            // X=0, Z=1, Q=1
                            // ADD HL, rr
                            let register = register_pair_table(p);
                            let dest = RegisterAddressing(Register::HL);
                            let src = RegisterAddressing(register);
                            inst::add_16(self, bus, &dest, &src);
                        },
                        _ => unreachable!()
                    }
                },
                // X=0, Z=2
                2 => {
                    match q {
                        // X=0, Z=2, Q=0
                        0 => match p {
                            0 => {
                                // X=0, Z=2, Q=0, P=0
                                // LD (BC), A
                                let dest = RegisterIndirectAddressing(Register::BC);
                                let src = RegisterAddressing(Register::A);
                                inst::ld(self, bus, &dest, &src);
                            },
                            1 => {
                                // X=0, Z=2, Q=0, P=1
                                // LD (DE), A
                                let dest = RegisterIndirectAddressing(Register::DE);
                                let src = RegisterAddressing(Register::A);
                                inst::ld(self, bus, &dest, &src);
                            },
                            2 => {
                                // X=0, Z=2, Q=0, P=2
                                // LD (nn), HL
                                let dest = ExtendedAddressing(self.next_word(bus));
                                let src = RegisterAddressing(Register::HL);
                                inst::ld(self, bus, &dest, &src);
                            },
                            3 => {
                                // X=0, Z=2, Q=0, P=3
                                // LD (nn), A
                                let dest = ExtendedAddressing(self.next_word(bus));
                                let src = RegisterAddressing(Register::A);
                                inst::ld(self, bus, &dest, &src);
                            },
                            _ => unreachable!()
                        },
                        // X=0, Z=2, Q=1
                        1 => match p {
                            0 => {
                                // X=0, Z=2, Q=1, P=0
                                // LD A, (BC)
                                let dest = RegisterAddressing(Register::A);
                                let src = RegisterIndirectAddressing(Register::BC);
                                inst::ld(self, bus, &dest, &src);
                            },
                            1 => {
                                // X=0, Z=2, Q=1, P=1
                                // LD A, (DE)
                                let dest = RegisterAddressing(Register::A);
                                let src = RegisterIndirectAddressing(Register::DE);
                                inst::ld(self, bus, &dest, &src);
                            },
                            2 => {
                                // X=0, Z=2, Q=1, P=2
                                // LD HL, (nn)
                                let dest = RegisterAddressing(Register::HL);
                                let src = ExtendedAddressing(self.next_word(bus));
                                inst::ld(self, bus, &dest, &src);
                            },
                            3 => {
                                // X=0, Z=2, Q=1, P=3
                                // LD A, (nn)
                                let dest = RegisterAddressing(Register::A);
                                let src = ExtendedAddressing(self.next_word(bus));
                                inst::ld(self, bus, &dest, &src);
                            },
                            _ => unreachable!()
                        },
                        _ => unreachable!()
                    }
                },
                // X=0, Z=3
                3 => {
                    let register = register_pair_table(p);
                    let dest = RegisterAddressing(register);

                    match q {
                        // X=0, Z=3, Q=0
                        // INC rr
                        0 => inst::inc_16(self, bus, &dest),    
                        // X=0, Z=3, Q=1
                        // DEC rr
                        1 => inst::dec_16(self, bus, &dest),
                        _ => unreachable!()
                    }
                },
                // X=0, Z=4
                // INC r
                4 => {
                    let dest = register_addr_table(y);
                    inst::inc_8(self, bus, dest.as_ref());
                },
                // X=0, Z=5
                // DEC r
                5 => {
                    let dest = register_addr_table(y);
                    inst::dec_8(self, bus, dest.as_ref());
                },
                // X=0, Z=6
                // LD r, n
                6 => {
                    let dest = register_addr_table(y);
                    let src = ImmediateAddressing(self.next_byte(bus));
                    inst::ld(self, bus, dest.as_ref(), &src);
                },
                // X=0, Z=7
                7 => {
                    match y {
                        // X=0, Z=7, Y=0
                        // RLCA
                        0 => inst::rlca(self),  
                        // X=0, Z=7, Y=1
                        // RRCA
                        1 => inst::rrca(self),
                        // X=0, Z=7, Y=2
                        // RLA
                        2 => inst::rla(self),
                        // X=0, Z=7, Y=3
                        // RRA
                        3 => inst::rra(self),
                        // X=0, Z=7, Y=4
                        // DAA
                        4 => inst::daa(self),
                        // X=0, Z=7, Y=5
                        // CPL
                        5 => inst::cpl(self),
                        // X=0, Z=7, Y=6
                        // SCF
                        6 => inst::scf(self),
                        // X=0, Z=7, Y=7
                        // CCF
                        7 => inst::ccf(self),
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            },
            // X=1
            1 => {
                if y == 6 {
                    // X=1, Y=6
                    // HALT
                    inst::halt();
                } else {
                    // X=1, Y=1..5,7
                    // LD r, r
                    let dest = register_addr_table(y);
                    let src = register_addr_table(z);
                    inst::ld(self, bus, dest.as_ref(), src.as_ref());
                }
            },
            // X=2
            2 => {
                let src = register_addr_table(z);
                self.decode_alu(bus, y, src.as_ref());
            },
            // X=3
            3 => {
                match z {
                    // X=3, Z=0
                    // RET
                    0 => {
                        let condition = condition_table(y);
                        if self.condition_met(condition) {
                            inst::ret(self, bus);
                        }
                    },
                    // X=3, Z=1
                    1 => {
                        match q {
                            // X=3, Z=1, Q=0
                            // POP rr
                            0 => {
                                let register = register_pair_table(p + 4);
                                let dest = RegisterAddressing(register);
                                inst::pop(self, bus, &dest);
                            },
                            // X=3, Z=1, Q=1
                            1 => {
                                match p {
                                    // X=3, Z=1, Q=1, P=0
                                    // RET
                                    0 => inst::ret(self, bus),
                                    // X=3, Z=1, Q=1, P=1
                                    // RETI
                                    1 => inst::reti(self, bus),
                                    // X=3, Z=1, Q=1, P=2
                                    // JP (HL)
                                    2 => {
                                        // Although this instruction is written as JP (HL), it's not
                                        // an indirect addressing mode.
                                        let src = RegisterAddressing(Register::HL);
                                        inst::jp(self, bus, &src);
                                    },
                                    // X=3, Z=1, Q=1, P=3
                                    // LD SP, HL
                                    3 => {
                                        let dest = RegisterAddressing(Register::SP);
                                        let src = RegisterAddressing(Register::HL);
                                        inst::ld::<u16>(self, bus, &dest, &src);
                                    },
                                    _ => unreachable!()
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    // X=3, Z=2
                    // JP cc, nn
                    2 => {
                        let condition = condition_table(y);
                        if self.condition_met(condition) {
                            let src = ImmediateAddressing(self.next_word(bus));
                            inst::jp(self, bus, &src);
                        }
                    },
                    // X=3, Z=3
                    3 => {
                        match y {
                            // X=3, Z=3, Y=0
                            // JP nn
                            0 => {
                                let src = ImmediateAddressing(self.next_word(bus));
                                inst::jp(self, bus, &src);
                            },
                            // X=3, Z=3, Y=1
                            // PREFIX CB
                            1 => inst::prefix(self),
                            // X=3, Z=3, Y=2-5
                            // Exception
                            2...5 => unimplemented!(),
                            // X=3, Z=3, Y=6
                            // DI
                            6 => inst::di(self),
                            // X=3, Z=3, Y=7
                            // EI
                            7 => inst::ei(self),
                            _ => unreachable!()
                        }
                    },
                    // X=3, Z=4
                    // CALL cc, nn
                    4 => {
                        let condition = condition_table(y);
                        if self.condition_met(condition) {
                            let src = ImmediateAddressing(self.next_word(bus));
                            inst::call(self, bus, &src);
                        }
                    },
                    // X=3, Z=5
                    5 => {
                        match q {
                            // X=3, Z=5, Q=0
                            // PUSH rr
                            0 => {
                                let register = register_pair_table(p + 4);
                                let src = RegisterAddressing(register);
                                inst::push(self, bus, &src);
                            },
                            // X=3, Z=5, Q=1
                            1 => {
                                match p {
                                    // X=3, Z=5, Q=1, P=0
                                    // CALL nn
                                    0 => {
                                        let src = ImmediateAddressing(self.next_word(bus));
                                        inst::call(self, bus, &src);
                                    },
                                    // X=3, Z=5, Q=1, P=1-3
                                    // Exception
                                    1...3 => unimplemented!(),
                                    _ => unreachable!()
                                }
                            },
                            _ =>unreachable!()
                        }
                    },
                    // X=3, Z=6
                    6 => {
                        let src = ImmediateAddressing(self.next_byte(bus));
                        self.decode_alu(bus, y, &src);
                    },
                    // X=3, Z=7
                    // RST
                    7 => inst::rst(self, bus, y * 8),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        };
    }

    fn decode_alu(&mut self, bus: &mut Bus, y: u8, src: &AddressingMode<u8>) {
        match y {         
            0 => inst::add_8(self, bus, src), // ADD A, r           
            1 => inst::adc(self, bus, src), // ADC A, r   
            2 => inst::sub(self, bus, src), // SUB r    
            3 => inst::sbc(self, bus, src), // SBC A, r
            4 => inst::and(self, bus, src), // AND 
            5 => inst::xor(self, bus, src), // XOR 
            6 => inst::or(self, bus, src), // OR
            7 => inst::cp(self, bus, src), // CP 
            _ => unreachable!()
        }
    }
}