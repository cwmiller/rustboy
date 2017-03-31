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
use super::{Condition, Cpu};
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
    AddSp(Box<AddressingMode<u8>>),
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
            Ld8(ref dest, ref src) => write!(f, "LD {}, {}", dest, src),
            Ld16(ref dest, ref src) => write!(f, "LD {}, {}", dest, src),
            Stop => write!(f, "STOP 0"),
            Jr(ref cond, ref addr) => write!(f, "JR {}, {}", cond, addr),
            Add16(ref dest, ref src) => write!(f, "ADD {}, {}", dest, src),
            Inc8(ref reg) => write!(f, "INC {}", reg),
            Inc16(ref reg) => write!(f, "INC {}", reg),
            Dec8(ref reg) => write!(f, "DEC {}", reg),
            Dec16(ref reg) => write!(f, "DEC {}", reg),
            Rlca => write!(f, "RLCA"),
            Rrca => write!(f, "RRCA"),
            Rla => write!(f, "RLA"),
            Rra => write!(f, "RRA"),
            Daa => write!(f, "DAA"),
            Cpl => write!(f, "CPL"),
            Scf => write!(f, "SCF"),
            Ccf => write!(f, "CCF"),
            Add8(ref reg) => write!(f, "ADD A, {}", reg),
            Adc(ref reg) => write!(f, "ADC A, {}", reg),
            Sub(ref reg) => write!(f, "SUB {}", reg), 
            Sbc(ref reg) => write!(f, "SBC {}", reg),
            And(ref reg) => write!(f, "AND {}", reg),
            Xor(ref reg) => write!(f, "XOR {}", reg),
            Or(ref reg) => write!(f, "OR {}", reg),
            Cp(ref reg) => write!(f, "CP {}", reg),
            Halt => write!(f, "HALT"),
            Ret(ref cond) => write!(f, "RET {}", cond),
            AddSp(ref reg) => write!(f, "ADD SP, {}", reg),
            Ldh(ref dest, ref src) => write!(f, "LDH {}, {}", dest, src),
            Ldhl(ref offset) => write!(f, "LDHL {}", offset),
            Pop(ref reg) => write!(f, "POP {}", reg),
            Reti => write!(f, "RETI"),
            Jp(ref cond, ref addr) => write!(f, "JP {}, {}", cond, addr),
            Di => write!(f, "DI"),
            Ei => write!(f, "EI"),
            Prefix => write!(f, "PREFIX CB"),
            Call(ref cond, ref addr) => write!(f, "CALL {}, {}", cond, addr),
            Push(ref reg) => write!(f, "PUSH {}", reg),
            Rst(idx) => write!(f, "RST {}", idx),
            Rlc(ref reg) => write!(f, "RLC {}", reg),
            Rrc(ref reg) => write!(f, "RRC {}", reg),
            Rl(ref reg) => write!(f, "RL {}", reg),
            Rr(ref reg) => write!(f, "RR {}", reg),
            Sla(ref reg) => write!(f, "SLA {}", reg),
            Sra(ref reg) => write!(f, "SRA {}", reg),
            Swap(ref reg) => write!(f, "SWAP {}", reg),
            Srl(ref reg) => write!(f, "SRL {}", reg),
            Bit(ref bit, ref reg) => write!(f, "BIT {}, {}", bit, reg),
            Res(ref bit, ref reg) => write!(f, "RES {}, {}", bit, reg),
            Set(ref bit, ref reg) => write!(f, "SET {}, {}", bit, reg)
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
                (3, 5, 0, _, _) => Some(AddSp(imm8(bus, &mut pc))),
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

pub fn execute(cpu: &mut Cpu, bus: &mut Bus, instruction: Instruction) {
    use self::Instruction::*;
    match instruction {
        Nop => nop(),
        Ld8(dest, src) => ld(cpu, bus, dest.as_ref(), src.as_ref()),
        Ld16(dest, src) => ld(cpu, bus, dest.as_ref(), src.as_ref()),
        Stop => stop(),
        Jr(cond, addr) => jr(cpu, bus, cond, addr.as_ref()),
        Add16(dest, src) => add_16(cpu, bus, dest.as_ref(), src.as_ref()),
        Inc8(reg) => inc_8(cpu, bus, reg.as_ref()),
        Inc16(reg) => inc_16(cpu, bus, reg.as_ref()),
        Dec8(reg) => dec_8(cpu, bus, reg.as_ref()),
        Dec16(reg) => dec_16(cpu, bus, reg.as_ref()),
        Rlca => rlca(cpu),
        Rrca => rrca(cpu),
        Rla => rla(cpu),
        Rra => rra(cpu),
        Daa => daa(cpu),
        Cpl => cpl(cpu),
        Scf => scf(cpu),
        Ccf => ccf(cpu),
        Add8(reg) => add_8(cpu, bus, reg.as_ref()),       
        Adc(reg) => adc(cpu, bus, reg.as_ref()),
        Sub(reg) => sub(cpu, bus, reg.as_ref()),  
        Sbc(reg) => sbc(cpu, bus, reg.as_ref()),
        And(reg) => and(cpu, bus, reg.as_ref()),
        Xor(reg) => xor(cpu, bus, reg.as_ref()),
        Or(reg) => or(cpu, bus, reg.as_ref()),
        Cp(reg) => cp(cpu, bus, reg.as_ref()),
        Halt => halt(),
        Ret(cond) => ret(cpu, bus, cond),
        AddSp(reg) => add_sp(cpu, bus, reg.as_ref()),
        Ldh(dest, src) => ld(cpu, bus, dest.as_ref(), src.as_ref()),
        _ => unimplemented!()
    }
}