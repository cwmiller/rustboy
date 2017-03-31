mod bits;
mod jumps;
mod loads;
mod math;
mod misc;

pub use self::bits::*;
pub use self::jumps::*;
pub use self::loads::*;
pub use self::math::*;
pub use self::misc::*;

use super::addressing::*;
use bus::{Addressable, Bus};
use super::Condition;
use byteorder::{ByteOrder, LittleEndian};
use std::fmt;
use super::registers::Register;

pub enum Instruction {
    Nop,
    Ld8(Box<AddressingMode<u8>>, Box<AddressingMode<u8>>),
    Ld16(Box<AddressingMode<u16>>, Box<AddressingMode<u16>>),
    Stop,
    Jr(Condition, Box<AddressingMode<u8>>),
    Add16(Box<AddressingMode<u16>>,Box<AddressingMode<u16>>),
    Inc8(Box<AddressingMode<u8>>),
    Inc16(Box<AddressingMode<u16>>),
    Dec8(Box<AddressingMode<u8>>),
    Dec16(Box<AddressingMode<u16>>),
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Add8(Box<AddressingMode<u8>>),       
    Adc(Box<AddressingMode<u8>>),
    Sub(Box<AddressingMode<u8>>),  
    Sbc(Box<AddressingMode<u8>>),
    And(Box<AddressingMode<u8>>),
    Xor(Box<AddressingMode<u8>>),
    Or(Box<AddressingMode<u8>>),
    Cp(Box<AddressingMode<u8>>),
    Halt,
    Ret(Condition),
    Addsp(Box<AddressingMode<u8>>),
    Ldh(Box<AddressingMode<u8>>,Box<AddressingMode<u8>>),
    Ldhl(Box<AddressingMode<u8>>),
    Pop(Box<AddressingMode<u16>>),
    Reti,
    Jp(Condition, Box<AddressingMode<u16>>),
    Di,
    Ei,
    Prefix,
    Call(Condition, Box<AddressingMode<u16>>),
    Push(Box<AddressingMode<u16>>),
    Rst(u8),
    Rlc(Box<AddressingMode<u8>>),
    Rrc(Box<AddressingMode<u8>>),
    Rl(Box<AddressingMode<u8>>),
    Rr(Box<AddressingMode<u8>>),
    Sla(Box<AddressingMode<u8>>),
    Sra(Box<AddressingMode<u8>>),
    Swap(Box<AddressingMode<u8>>),
    Srl(Box<AddressingMode<u8>>),
    Bit(u8, Box<AddressingMode<u8>>),
    Res(u8, Box<AddressingMode<u8>>),
    Set(u8, Box<AddressingMode<u8>>)
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop => write!(f, "NOP"),
            Ld8(_, _) => write!(f, "LD8"),
            Ld16(_, _) => write!(f, "LD16"),
            Stop => write!(f, "STOP"),
            Jr(_, _) => write!(f, "JR"),
            Add16(_, _) => write!(f, "AND16"),
            Inc8(_) => write!(f, "INC8"),
            Inc16(_) => write!(f, "INC16"),
            Dec8(_) => write!(f, "DEC8"),
            Dec16(_) => write!(f, "DEC16"),
            Rlca => write!(f, "RLCA"),
            Rrca => write!(f, "RRCA"),
            Rla => write!(f, "RLA"),
            Rra => write!(f, "RRA"),
            Daa => write!(f, "DAA"),
            Cpl => write!(f, "CPL"),
            Scf => write!(f, "SCF"),
            Ccf => write!(f, "CCF"),
            Add8(_) => write!(f, "ADD8"),
            Adc(_) => write!(f, "ADC"),
            Sub(_) => write!(f, "SUB"), 
            Sbc(_) => write!(f, "SBC"),
            And(_) => write!(f, "AND"),
            Xor(_) => write!(f, "XOR"),
            Or(_) => write!(f, "OR"),
            Cp(_) => write!(f, "CP"),
            Halt => write!(f, "HALT"),
            Ret(_) => write!(f, "RET"),
            Addsp(_) => write!(f, "ADD SP"),
            Ldh(_, _) => write!(f, "LDH"),
            Ldhl(_) => write!(f, "LDHL"),
            Pop(_) => write!(f, "POP"),
            Reti => write!(f, "RETI"),
            Jp(_, _) => write!(f, "JP"),
            Di => write!(f, "DI"),
            Ei => write!(f, "EI"),
            Prefix => write!(f, "PREFIX CB"),
            Call(_, _) => write!(f, "CALL"),
            Push(_) => write!(f, "PUSH"),
            Rst(idx) => write!(f, "RST {}", idx),
            Rlc(_) => write!(f, "RLC"),
            Rrc(_) => write!(f, "RRC"),
            Rl(_) => write!(f, "RL"),
            Rr(_) => write!(f, "RR"),
            Sla(_) => write!(f, "SLA"),
            Sra(_) => write!(f, "SRA"),
            Swap(_) => write!(f, "SWAP"),
            Srl(_) => write!(f, "SRL"),
            Bit(_, _) => write!(f, "BIT"),
            Res(_, _) => write!(f, "RES"),
            Set(_, _) => write!(f, "SET")
        }
    }
}

fn step_byte(bus: &Bus, mut pc: &mut u16) -> u8 {
    let byte = bus.read(*pc);
    *pc = *pc + 1;
    byte
}

fn step_word(bus: &Bus, mut pc: &mut u16) -> u16 {
    let lb = step_byte(bus, &mut pc);
    let hb = step_byte(bus, &mut pc);

    LittleEndian::read_u16(&[lb, hb])
}

fn cond_table(idx: u8) -> Condition {
    match idx {
        0 => Condition::Nz,
        1 => Condition::Z,
        2 => Condition::Nc,
        3 => Condition::C,
        _ => unreachable!()
    }
}

fn reg(reg: Register) -> Box<RegisterAddressing> {
    Box::new(RegisterAddressing(reg))
}

fn regind(reg: Register) -> Box<RegisterIndirectAddressing> {
    Box::new(RegisterIndirectAddressing(reg))
}

fn imm8(bus: &Bus, mut pc: &mut u16) -> Box<ImmediateAddressing<u8>> {
    Box::new(ImmediateAddressing(step_byte(bus, &mut pc)))
}

fn imm16(bus: &Bus, mut pc: &mut u16) -> Box<ImmediateAddressing<u16>> {
    Box::new(ImmediateAddressing(step_word(bus, &mut pc)))
}

fn rel(bus: &Bus, mut pc: &mut u16) -> Box<RelativeAddressing> {
    Box::new(RelativeAddressing(step_byte(bus, &mut pc) as i8))
}

fn ind8(bus: &Bus, mut pc: &mut u16) -> Box<IndirectAddressing<u8>> {
    Box::new(IndirectAddressing(step_byte(bus, &mut pc)))
}

fn ind16(bus: &Bus, mut pc: &mut u16) -> Box<IndirectAddressing<u16>> {
    Box::new(IndirectAddressing(step_word(bus, &mut pc)))
}

fn ext(bus: &Bus, mut pc: &mut u16) -> Box<ExtendedAddressing> {
    Box::new(ExtendedAddressing(step_word(bus, pc)))
}

fn reg_addr_table(idx: u8) -> Box<AddressingMode<u8>> {
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

fn reg_pair_table(idx: u8) -> Register {
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

pub fn decode(bus: &Bus, mut pc: u16, prefixed: bool) -> (u8, Option<Instruction>, u16) {
    use self::Register::*;
    use self::Instruction::*;

    let start_pc = pc;
    let opcode = step_byte(bus, &mut pc);

    let x = (0b11000000 & opcode) >> 6;
    let y = (0b00111000 & opcode) >> 3;
    let z = 0b00000111 & opcode;
    let p = y >> 1;
    let q = (0b00001000 & opcode) >> 3;

    let instruction = 
        if !prefixed {
            match (x, y, z, q, p) {
                // X=0, Z=0
                (0, 0, 0, _, _) => Some(Nop),
                (0, 1, 0, _, _) => Some(Ld8(ext(bus, &mut pc), reg(SP))),
                (0, 2, 0, _, _) => { step_word(bus, &mut pc); Some(Stop) },
                (0, 3, 0, _, _) => Some(Jr(Condition::None, imm8(bus, &mut pc))),
                (0, 4...7, 0, _, _) => Some(Jr(cond_table(y-4), imm8(bus, &mut pc))),
                // X=0, Z=1
                (0, _, 1, 0, _) => Some(Ld16(reg(reg_pair_table(p)), imm16(bus, &mut pc))),
                (0, _, 1, 1, _) => Some(Add16(reg(HL), reg(reg_pair_table(p)))),
                // X=0, Z=2
                (0, _, 2, 0, 0) => Some(Ld8(regind(BC), reg(A))),
                (0, _, 2, 0, 1) => Some(Ld8(regind(DE), reg(A))),
                (0, _, 2, 0, 2) => Some(Ld8(ext(bus, &mut pc), reg(HL))),
                (0, _, 2, 0, 3) => Some(Ld8(ext(bus, &mut pc), reg(A))),
                (0, _, 2, 1, 0) => Some(Ld8(reg(A), regind(BC))),
                (0, _, 2, 1, 1) => Some(Ld8(reg(A), regind(DE))),
                (0, _, 2, 1, 2) => Some(Ld8(reg(HL), ext(bus, &mut pc))),
                (0, _, 2, 1, 3) => Some(Ld8(reg(A), ext(bus, &mut pc))),
                // X=0, Z=3
                (0, _, 3, 0, _) => Some(Inc16(reg(reg_pair_table(p)))),
                (0, _, 3, 1, _) => Some(Dec16(reg(reg_pair_table(p)))),
                // X=0, Z=4
                (0, _, 4, _, _) => Some(Inc8(reg_addr_table(p))),
                // X=0, Z=5
                (0, _, 5, _, _) => Some(Dec8(reg_addr_table(p))),
                // X=0, Z=6
                (0, _, 6, _, _) => Some(Ld8(reg_addr_table(y), imm8(bus, &mut pc))),
                // X=0, Z=7
                (0, 0, 7, _, _) => Some(Rlca),
                (0, 1, 7, _, _) => Some(Rrca),
                (0, 2, 7, _, _) => Some(Rla),
                (0, 3, 7, _, _) => Some(Rra),
                (0, 4, 7, _, _) => Some(Daa),
                (0, 5, 7, _, _) => Some(Cpl),
                (0, 6, 7, _, _) => Some(Scf),
                (0, 7, 7, _, _) => Some(Ccf),
                // X=1
                (1, _, _, _, _) if !(z == 6 && x == 6) => Some(Ld8(reg_addr_table(y), reg_addr_table(z))),
                (1, _, _, _, _) if z == 6 && x == 6 => Some(Halt),
                // X=2
                (2, _, _, _, _) => decode_alu(y, reg_addr_table(z)),
                // X=3, Z=0
                (3, 0...3, 0, _, _) => Some(Ret(cond_table(y))),
                (3, 4, 0, _, _) => Some(Ldh(ind8(bus, &mut pc), reg(A))),
                (3, 5, 0, _, _) => Some(Addsp(imm8(bus, &mut pc))),
                (3, 6, 0, _, _) => Some(Ldh(reg(A), ind8(bus, &mut pc))),
                (3, 7, 0, _, _) => Some(Ldhl(imm8(bus, &mut pc))),
                // X=3, Z=1
                (3, _, 1, 0, _) => Some(Pop(reg(reg_pair_table(p + 4)))),
                (3, _, 1, 1, 0) => Some(Ret(Condition::None)),
                (3, _, 1, 1, 1) => Some(Reti),
                (3, _, 1, 1, 2) => Some(Jp(Condition::None, reg(HL))),
                (3, _, 1, 1, 3) => Some(Ld16(reg(SP), reg(HL))),
                // X=3, Z=2
                (3, 0...3, 2, _, _) => Some(Jp(cond_table(y), imm16(bus, &mut pc))),
                (3, 4, 2, _, _) => Some(Ld8(regind(C), reg(A))),
                (3, 5, 2, _, _) => Some(Ld8(ind16(bus, &mut pc), reg(A))),
                (3, 6, 2, _, _) => Some(Ld8(reg(A), regind(C))),
                (3, 7, 2, _, _) => Some(Ld8(reg(A), ind16(bus, &mut pc))),
                // X=3, Z=3
                (3, 0, 3, _, _) => Some(Jp(Condition::None, imm16(bus, &mut pc))),
                (3, 1, 3, _, _) => Some(Prefix),
                (3, 6, 3, _, _) => Some(Di),
                (3, 7, 3, _, _) => Some(Ei),
                // X=3, Z=4
                (3, 0...3, 4, _, _) => Some(Call(cond_table(y), imm16(bus, &mut pc))),
                // X=3, Z=5
                (3, 7, 5, 0, _) => Some(Push(reg(reg_pair_table(p + 4)))),
                (3, 7, 5, 1, 0) => Some(Call(Condition::None, imm16(bus, &mut pc))),
                // X=3, Z=6
                (3, _, 6, _, _) => decode_alu(y, reg_addr_table(z)),
                // X=3, Z=7
                (3, _, 7, _, _) => Some(Rst(y)),
                _ => None
            }
        } else {
            let register = reg_addr_table(z);
            match x {
                0 => {
                    match y {
                        0 => Some(Rlc(register)),
                        1 => Some(Rrc(register)),
                        2 => Some(Rl(register)),
                        3 => Some(Rr(register)),
                        4 => Some(Sla(register)),
                        5 => Some(Sra(register)),
                        6 => Some(Swap(register)),
                        7 => Some(Srl(register)),
                        _ => None
                    }
                },
                1 => Some(Bit(y, register)),
                2 => Some(Res(y, register)),
                3 => Some(Set(y, register)),
                _ => None
            }
        };

    (opcode, instruction, pc - start_pc)
}

fn decode_alu(y: u8, src: Box<AddressingMode<u8>>) -> Option<Instruction> {
    use self::Instruction::*;

    match y {         
        0 => Some(Add8(src)),
        1 => Some(Adc(src)),
        2 => Some(Sub(src)),
        3 => Some(Sbc(src)),
        4 => Some(And(src)),
        5 => Some(Xor(src)),
        6 => Some(Or(src)),
        7 => Some(Cp(src)),
        _ => None
    }
}