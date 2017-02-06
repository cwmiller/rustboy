use std::fmt;

use super::bus::{Bus, Addressable};
use super::byteorder::{BigEndian, ByteOrder, LittleEndian};

#[derive(Debug)]
enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10
}

#[derive(Debug)]
enum Condition {
    None,
    C,
    Z,
    NC,
    NZ
}

#[derive(Debug)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    FLAGS,
    H,
    L
}

#[derive(Debug)]
enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

#[derive(Debug)]
enum Register {
    Byte(Register8),
    Word(Register16)
}

enum Operand8 {
    Immediate(u8),
    Register(Register8),
    Offset(u8),
    RegisterOffset(Register8)
}

impl fmt::Debug for Operand8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand8::Immediate(ref imm) => write!(f, "{:#X}", imm),
            Operand8::Register(ref reg) => write!(f, "{:?}", reg),
            Operand8::Offset(ref offset) => write!(f, "({:#X})", offset),
            Operand8::RegisterOffset(ref reg) => write!(f, "({:?})", reg)
        }
    }
}

enum Operand16 {
    Immediate(u16),
    ImmediatePointer(u16),
    Register(Register16),
    RegisterPointer(Register16)
}

impl fmt::Debug for Operand16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand16::Immediate(ref imm) => write!(f, "{:#X}", imm),
            Operand16::ImmediatePointer(ref imm) => write!(f, "({:#X})", imm),
            Operand16::Register(ref reg) => write!(f, "{:?}", reg),
            Operand16::RegisterPointer(ref reg) => write!(f, "({:?})", reg)
        }
    }
}


enum Operand {
    Byte(Operand8),
    Word(Operand16)
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Byte(ref oper) => write!(f, "{:?}", oper),
            Operand::Word(ref oper) => write!(f, "{:?}", oper)
        }
    }
}

//#[derive(Debug)]
enum Instruction {
    Nop,
    Ld { dest: Operand, src: Operand },
    Inc { dest: Operand },
    Dec { dest: Operand },
    Rlca,
    Add { dest: Operand, src: Operand },
    Rrca,
    Stop,
    Rla,
    Jr { offset: i8, condition: Condition },
    Rra,
    Ldi { dest: Operand, src: Operand },
    Daa,
    Cpl,
    Ldd { dest: Operand, src: Operand },
    Scf,
    Ccf,
    Halt,
    Adc { dest: Operand, src: Operand },
    Sub { dest: Operand, src: Operand },
    Sbc { dest: Operand, src: Operand },
    Xor { dest: Operand },
    Or { dest: Operand },
    Cp { dest: Operand },
    Ret { condition: Condition },
    Pop { dest: Operand },
    Jp { address: Operand, condition: Condition },
    Call { address: Operand, condition: Condition },
    Push { src: Operand },
    Rst { address: u8 },
    Prefix,
    Reti,
    Ldh { dest: Operand, src: Operand },
    Di,
    Ldhl { offset: i8 },
    Ei
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Nop                                => write!(f, "NOP"),
            Instruction::Ld { ref dest ,ref src }           => write!(f, "LD {:?}, {:?}", dest, src),
            Instruction::Inc { ref dest }                   => write!(f, "INC {:?}", dest),
            Instruction::Dec { ref dest }                   => write!(f, "DEC {:?}", dest),
            Instruction::Rlca                               => write!(f, "RLCA"),
            Instruction::Add { ref dest, ref src }          => write!(f, "ADD {:?}, {:?}", dest, src),
            Instruction::Rrca                               => write!(f, "RRCA"),
            Instruction::Stop                               => write!(f, "STOP"),
            Instruction::Rla                                => write!(f, "RLA"),
            Instruction::Jr { ref offset, ref condition }   => write!(f, "JR {:?}, {:?}", condition, offset),
            Instruction::Rra                                => write!(f, "RRA"),
            Instruction::Ldi { ref dest, ref src }          => write!(f, "LDI {:?}, {:?}", dest, src),
            Instruction::Daa                                => write!(f, "DAA"),
            Instruction::Cpl                                => write!(f, "CPL"),
            Instruction::Ldd { ref dest, ref src }          => write!(f, "LDD {:?}, {:?}", dest, src),
            Instruction::Scf                                => write!(f, "SCF"),
            Instruction::Ccf                                => write!(f, "CCF"),
            Instruction::Halt                               => write!(f, "HALT"),
            Instruction::Adc { ref dest, ref src }          => write!(f, "ADC {:?}, {:?}", dest, src),
            Instruction::Sub { ref dest, ref src }          => write!(f, "SUB {:?}, {:?}", dest, src),
            Instruction::Sbc { ref dest, ref src }          => write!(f, "SBC {:?}, {:?}", dest, src),
            Instruction::Xor { ref dest }                   => write!(f, "XOR {:?}", dest),
            Instruction::Or { ref dest }                    => write!(f, "OR {:?}", dest),
            Instruction::Cp { ref dest }                    => write!(f, "CP {:?}", dest),
            Instruction::Ret { ref condition }              => write!(f, "RET {:?}", condition),
            Instruction::Pop { ref dest }                   => write!(f, "POP {:?}", dest),
            Instruction::Jp { ref address, ref condition }  => write!(f, "JP {:?}, {:?}", condition, address),
            Instruction::Call { ref address, ref condition }=> write!(f, "CALL {:?}, {:?}", condition, address),
            Instruction::Push { ref src }                   => write!(f, "PUSH {:?}", src),
            Instruction::Rst { ref address }                => write!(f, "RST {:?}", address),
            Instruction::Prefix                             => write!(f, "PREFIX CB"),
            Instruction::Reti                               => write!(f, "RETI"),
            Instruction::Ldh { ref dest, ref src }          => write!(f, "LDH {:?}, {:?}", dest, src),
            Instruction::Di                                 => write!(f, "DI"),
            Instruction::Ldhl { ref offset }                => write!(f, "LDHL SP, {:?}", offset),
            Instruction::Ei                                 => write!(f, "EI")
        }
    }
}


#[derive(Default)]
struct CpuRegs {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    flags: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16
}

impl CpuRegs {
    pub fn is_flag_set(&self, flag: Flag) -> bool {
        let mask = flag as u8;
        (mask & self.flags) == mask
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        self.flags = self.flags | flag as u8
    }

    pub fn write_u8(&mut self, reg: Register8, val: u8) {
        match reg {
            Register8::A => self.a = val,
            Register8::B => self.b = val,
            Register8::C => self.c = val,
            Register8::D => self.d = val,
            Register8::E => self.e = val,
            Register8::FLAGS => self.flags = val,
            Register8::H => self.h = val,
            Register8::L => self.l = val
        }
    }

    pub fn write_u16(&mut self, reg: Register16, val: u16) {
        match reg {
            Register16::AF => {
                self.a = (val >> 8) as u8;
                self.flags = (val & 0x0F) as u8;
            },
            Register16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0x0F) as u8;
            },
            Register16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0x0F) as u8;
            },
            Register16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0x0F) as u8;
            },
            Register16::SP => self.sp = val,
            Register16::PC => self.pc = val
        }
    }
}

pub struct Cpu {
    regs: CpuRegs,
    halted: bool,
    bus: Bus
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            regs: CpuRegs::default(),
            bus: bus,
            halted: false
        }
    }

    pub fn power_on(&mut self) {
        self.regs.a = 0x01;
        self.regs.b = 0x13;
        self.regs.c = 0x00;
        self.regs.d = 0xD8;
        self.regs.e = 0x00;
        self.regs.flags = 0xB0;
        self.regs.h = 0x4D;
        self.regs.l = 0x01;
        self.regs.sp = 0xFFFE;
        self.regs.pc = 0x100;

        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let pc = self.regs.pc;

        let opcode = self.step_byte();
        let instruction = self.decode(opcode);

        println!("{:#X} {:?}", pc, instruction);

        self.execute(instruction);
    }

    pub fn step_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc + 1;

        byte
    }

    pub fn step_word(&mut self) -> u16 {
        let word = &[self.bus.read(self.regs.pc), self.bus.read(self.regs.pc+1)];
        self.regs.pc = self.regs.pc + 2;

        LittleEndian::read_u16(word)
    }

    fn decode(&mut self, opcode: u8) -> Instruction {
        let op_reg16 = |reg: Register16| Operand::Word(Operand16::Register(reg));
        let op_imm16 = |word: u16| Operand::Word(Operand16::Immediate(word));


        match opcode {
            0x00 => Instruction::Nop,
            0x01 => Instruction::Ld { dest: op_reg16(Register16::BC), src: op_imm16(self.step_word()) },

            0xC3 => Instruction::Jp { address: op_imm16(self.step_word()), condition: Condition::None },
            _ => panic!("Unknown opcode {:#X}", opcode)
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Nop => self.inst_nop(),
            _ => panic!("Unhandled instruction: {:?}", instruction)
        }
    }

    fn condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::None => true,
            Condition::C => self.regs.is_flag_set(Flag::C),
            Condition::Z => self.regs.is_flag_set(Flag::Z),
            Condition::NC => !self.regs.is_flag_set(Flag::C),
            Condition::NZ => !self.regs.is_flag_set(Flag::Z)
        }
    }

    fn inst_nop(&self) {
        // Do nothing
    }

    fn inst_jp(&mut self, addr: u16, condition: Condition) {
        if self.condition_met(condition) {
            self.regs.pc = addr;
        }
    }
}