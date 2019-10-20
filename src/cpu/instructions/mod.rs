mod bits;
mod jumps;
mod loads;
mod math;
mod misc;

#[cfg(test)]
mod tests;

pub use self::bits::*;
pub use self::jumps::*;
pub use self::loads::*;
pub use self::math::*;
pub use self::misc::*;

use super::{Condition, Cpu};
use super::addressing::*;
use bus::Bus;
use std::fmt;
use super::registers::Register;

pub enum Instruction {
    Nop,
    Ld8(Box<dyn AddressingMode<u8>>, Box<dyn AddressingMode<u8>>),
    Ld16(Box<dyn AddressingMode<u16>>, Box<dyn AddressingMode<u16>>),
    Stop,
    Jr(Condition, Box<dyn AddressingMode<u8>>),
    Add16(Box<dyn AddressingMode<u16>>,Box<dyn AddressingMode<u16>>),
    Inc8(Box<dyn AddressingMode<u8>>),
    Inc16(Box<dyn AddressingMode<u16>>),
    Dec8(Box<dyn AddressingMode<u8>>),
    Dec16(Box<dyn AddressingMode<u16>>),
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Ldd(Box<dyn AddressingMode<u8>>, Box<dyn AddressingMode<u8>>),
    Ldi(Box<dyn AddressingMode<u8>>, Box<dyn AddressingMode<u8>>),
    Add8(Box<dyn AddressingMode<u8>>),       
    Adc(Box<dyn AddressingMode<u8>>),
    Sub(Box<dyn AddressingMode<u8>>),  
    Sbc(Box<dyn AddressingMode<u8>>),
    And(Box<dyn AddressingMode<u8>>),
    Xor(Box<dyn AddressingMode<u8>>),
    Or(Box<dyn AddressingMode<u8>>),
    Cp(Box<dyn AddressingMode<u8>>),
    Halt,
    Ret(Condition),
    AddSp(Box<dyn AddressingMode<u8>>),
    Ldh(Box<dyn AddressingMode<u8>>,Box<dyn AddressingMode<u8>>),
    Ldhl(Box<dyn AddressingMode<u8>>),
    Pop(Box<dyn AddressingMode<u16>>),
    Reti,
    Jp(Condition, Box<dyn AddressingMode<u16>>),
    Di,
    Ei,
    Call(Condition, Box<dyn AddressingMode<u16>>),
    Push(Box<dyn AddressingMode<u16>>),
    Rst(u8),
    Rlc(Box<dyn AddressingMode<u8>>),
    Rrc(Box<dyn AddressingMode<u8>>),
    Rl(Box<dyn AddressingMode<u8>>),
    Rr(Box<dyn AddressingMode<u8>>),
    Sla(Box<dyn AddressingMode<u8>>),
    Sra(Box<dyn AddressingMode<u8>>),
    Swap(Box<dyn AddressingMode<u8>>),
    Srl(Box<dyn AddressingMode<u8>>),
    Bit(u8, Box<dyn AddressingMode<u8>>),
    Res(u8, Box<dyn AddressingMode<u8>>),
    Set(u8, Box<dyn AddressingMode<u8>>)
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Nop => write!(f, "NOP"),
            Ld8(ref dest, ref src) => write!(f, "LD {}, {}", dest, src),
            Ld16(ref dest, ref src) => write!(f, "LD {}, {}", dest, src),
            Stop => write!(f, "STOP 0"),
            Jr(ref cond, ref addr) => {
                if *cond == Condition::None {
                    write!(f, "JR {}", addr)
                } else {
                    write!(f, "JR {}, {}", cond, addr)
                }
            },
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
            Ldd(ref dest, ref src) => write!(f, "LDD {}, {}", dest, src),
            Ldi(ref dest, ref src) => write!(f, "LDI {}, {}", dest, src),
            Add8(ref reg) => write!(f, "ADD A, {}", reg),
            Adc(ref reg) => write!(f, "ADC A, {}", reg),
            Sub(ref reg) => write!(f, "SUB {}", reg), 
            Sbc(ref reg) => write!(f, "SBC {}", reg),
            And(ref reg) => write!(f, "AND {}", reg),
            Xor(ref reg) => write!(f, "XOR {}", reg),
            Or(ref reg) => write!(f, "OR {}", reg),
            Cp(ref reg) => write!(f, "CP {}", reg),
            Halt => write!(f, "HALT"),
            Ret(ref cond) => {
                if *cond == Condition::None {
                    write!(f, "RET")
                } else {
                    write!(f, "RET {}", cond)
                }
            },
            AddSp(ref reg) => write!(f, "ADD SP, {}", reg),
            Ldh(ref dest, ref src) => write!(f, "LDH {}, {}", dest, src),
            Ldhl(ref offset) => write!(f, "LDHL {}", offset),
            Pop(ref reg) => write!(f, "POP {}", reg),
            Reti => write!(f, "RETI"),
            Jp(ref cond, ref addr) => {
                if *cond == Condition::None {
                    write!(f, "JP {}", addr)
                } else {
                    write!(f, "JP {}, {}", cond, addr)
                }
            },
            Di => write!(f, "DI"),
            Ei => write!(f, "EI"),
            Call(ref cond, ref addr) => {
                if *cond == Condition::None {
                    write!(f, "CALL {}", addr)
                } else {
                    write!(f, "CALL {}, {}", cond, addr)
                }
            },
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

fn cond_table(idx: u8) -> Condition {
    match idx {
        0 => Condition::Nz,
        1 => Condition::Z,
        2 => Condition::Nc,
        3 => Condition::C,
        _ => unreachable!()
    }
}

fn reg_addr(reg: Register) -> Box<RegisterAddressing> {
    Box::new(RegisterAddressing(reg))
}

fn regind_addr(reg: Register) -> Box<RegisterIndirectAddressing> {
    Box::new(RegisterIndirectAddressing(reg))
}

fn imm8_addr(byte: u8) -> Box<ImmediateAddressing<u8>> {
    Box::new(ImmediateAddressing(byte))
}

fn imm16_addr(word: u16) -> Box<ImmediateAddressing<u16>> {
    Box::new(ImmediateAddressing(word))
}

fn ind8_addr(byte: u8) -> Box<IndirectAddressing<u8>> {
    Box::new(IndirectAddressing(byte))
}

fn ind16_addr(word: u16) -> Box<IndirectAddressing<u16>> {
    Box::new(IndirectAddressing(word))
}

fn reg_addr_table(idx: u8) -> Box<dyn AddressingMode<u8>> {
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

pub fn decode(cpu: &mut Cpu, bus: &Bus, opcode: u8, prefixed: bool) -> Instruction {
    use self::Register::*;
    use self::Instruction::*;

    let x = (0b1100_0000 & opcode) >> 6;
    let y = (0b0011_1000 & opcode) >> 3;
    let z = 0b0000_0111 & opcode;
    let p = y >> 1;
    let q = (0b0000_1000 & opcode) >> 3;

    let instruction = 
        if !prefixed {
            match (x, y, z, q, p) {
                // X=0, Z=0
                (0, 0, 0, _, _) => Nop,
                (0, 1, 0, _, _) => Ld16(ind16_addr(cpu.step_next_word(bus)), reg_addr(SP)),
                (0, 2, 0, _, _) => { cpu.step_next_word(bus); Stop },
                (0, 3, 0, _, _) => Jr(Condition::None, imm8_addr(cpu.step_next_byte(bus))),
                (0, 4..=7, 0, _, _) => Jr(cond_table(y-4), imm8_addr(cpu.step_next_byte(bus))),
                // X=0, Z=1
                (0, _, 1, 0, _) => Ld16(reg_addr(reg_pair_table(p)), imm16_addr(cpu.step_next_word(bus))),
                (0, _, 1, 1, _) => Add16(reg_addr(HL), reg_addr(reg_pair_table(p))),
                // X=0, Z=2
                (0, _, 2, 0, 0) => Ld8(regind_addr(BC), reg_addr(A)),
                (0, _, 2, 0, 1) => Ld8(regind_addr(DE), reg_addr(A)),
                (0, _, 2, 0, 2) => Ldi(regind_addr(HL), reg_addr(A)),
                (0, _, 2, 0, 3) => Ldd(regind_addr(HL), reg_addr(A)),
                (0, _, 2, 1, 0) => Ld8(reg_addr(A), regind_addr(BC)),
                (0, _, 2, 1, 1) => Ld8(reg_addr(A), regind_addr(DE)),
                (0, _, 2, 1, 2) => Ldi(reg_addr(A), regind_addr(HL)),
                (0, _, 2, 1, 3) => Ldd(reg_addr(A), regind_addr(HL)),
                // X=0, Z=3
                (0, _, 3, 0, _) => Inc16(reg_addr(reg_pair_table(p))),
                (0, _, 3, 1, _) => Dec16(reg_addr(reg_pair_table(p))),
                // X=0, Z=4
                (0, _, 4, _, _) => Inc8(reg_addr_table(y)),
                // X=0, Z=5
                (0, _, 5, _, _) => Dec8(reg_addr_table(y)),
                // X=0, Z=6
                (0, _, 6, _, _) => Ld8(reg_addr_table(y), imm8_addr(cpu.step_next_byte(bus))),
                // X=0, Z=7
                (0, 0, 7, _, _) => Rlca,
                (0, 1, 7, _, _) => Rrca,
                (0, 2, 7, _, _) => Rla,
                (0, 3, 7, _, _) => Rra,
                (0, 4, 7, _, _) => Daa,
                (0, 5, 7, _, _) => Cpl,
                (0, 6, 7, _, _) => Scf,
                (0, 7, 7, _, _) => Ccf,
                // X=1
                (1, _, _, _, _) if !(z == 6 && y == 6) => Ld8(reg_addr_table(y), reg_addr_table(z)),
                (1, _, _, _, _) if z == 6 && y == 6 => Halt,
                // X=2
                (2, _, _, _, _) => decode_alu(y, reg_addr_table(z)),
                // X=3, Z=0
                (3, 0..=3, 0, _, _) => Ret(cond_table(y)),
                (3, 4, 0, _, _) => Ldh(ind8_addr(cpu.step_next_byte(bus)), reg_addr(A)),
                (3, 5, 0, _, _) => AddSp(imm8_addr(cpu.step_next_byte(bus))),
                (3, 6, 0, _, _) => Ldh(reg_addr(A), ind8_addr(cpu.step_next_byte(bus))),
                (3, 7, 0, _, _) => Ldhl(imm8_addr(cpu.step_next_byte(bus))),
                // X=3, Z=1
                (3, _, 1, 0, _) => Pop(reg_addr(reg_pair_table(p + 4))),
                (3, _, 1, 1, 0) => Ret(Condition::None),
                (3, _, 1, 1, 1) => Reti,
                (3, _, 1, 1, 2) => Jp(Condition::None, reg_addr(HL)),
                (3, _, 1, 1, 3) => Ld16(reg_addr(SP), reg_addr(HL)),
                // X=3, Z=2
                (3, 0..=3, 2, _, _) => Jp(cond_table(y), imm16_addr(cpu.step_next_word(bus))),
                (3, 4, 2, _, _) => Ld8(regind_addr(C), reg_addr(A)),
                (3, 5, 2, _, _) => Ld8(ind16_addr(cpu.step_next_word(bus)), reg_addr(A)),
                (3, 6, 2, _, _) => Ld8(reg_addr(A), regind_addr(C)),
                (3, 7, 2, _, _) => Ld8(reg_addr(A), ind16_addr(cpu.step_next_word(bus))),
                // X=3, Z=3
                (3, 0, 3, _, _) => Jp(Condition::None, imm16_addr(cpu.step_next_word(bus))),
                (3, 6, 3, _, _) => Di,
                (3, 7, 3, _, _) => Ei,
                // X=3, Z=4
                (3, 0..=3, 4, _, _) => Call(cond_table(y), imm16_addr(cpu.step_next_word(bus))),
                // X=3, Z=5
                (3, _, 5, 0, _) => Push(reg_addr(reg_pair_table(p + 4))),
                (3, _, 5, 1, 0) => Call(Condition::None, imm16_addr(cpu.step_next_word(bus))),
                // X=3, Z=6
                (3, _, 6, _, _) => decode_alu(y, imm8_addr(cpu.step_next_byte(bus))),
                // X=3, Z=7
                (3, _, 7, _, _) => Rst(y),
                _ => unreachable!()
            }
        } else {
            let register = reg_addr_table(z);
            match x {
                0 => {
                    match y {
                        0 => Rlc(register),
                        1 => Rrc(register),
                        2 => Rl(register),
                        3 => Rr(register),
                        4 => Sla(register),
                        5 => Sra(register),
                        6 => Swap(register),
                        7 => Srl(register),
                        _ => unreachable!()
                    }
                },
                1 => Bit(y, register),
                2 => Res(y, register),
                3 => Set(y, register),
                _ => unreachable!()
            }
        };

    instruction
}

fn decode_alu(y: u8, src: Box<dyn AddressingMode<u8>>) -> Instruction {
    use self::Instruction::*;

    match y {         
        0 => Add8(src),
        1 => Adc(src),
        2 => Sub(src),
        3 => Sbc(src),
        4 => And(src),
        5 => Xor(src),
        6 => Or(src),
        7 => Cp(src),
        _ => unreachable!()
    }
}

pub fn execute(cpu: &mut Cpu, bus: &mut Bus, instruction: &Instruction) -> bool {
    use self::Instruction::*;
    match instruction {
        Nop                 => { nop(); true },
        Ld8(dest, src)      => { ld(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Ld16(dest, src)     => { ld(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Stop                => { stop(cpu); true },
        Jr(cond, addr)      => { jr(cpu, bus, *cond, addr.as_ref()) },
        Add16(dest, src)    => { add_16(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Inc8(reg)           => { inc_8(cpu, bus, reg.as_ref()); true },
        Inc16(reg)          => { inc_16(cpu, bus, reg.as_ref()); true },
        Dec8(reg)           => { dec_8(cpu, bus, reg.as_ref()); true },
        Dec16(reg)          => { dec_16(cpu, bus, reg.as_ref()); true },
        Rlca                => { rlca(cpu); true },
        Rrca                => { rrca(cpu); true },
        Rla                 => { rla(cpu); true },
        Rra                 => { rra(cpu); true },
        Daa                 => { daa(cpu); true },
        Cpl                 => { cpl(cpu); true },
        Scf                 => { scf(cpu); true },
        Ccf                 => { ccf(cpu); true },
        Ldd(dest, src)      => { ldd(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Ldi(dest, src)      => { ldi(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Add8(reg)           => { add_8(cpu, bus, reg.as_ref()); true },
        Adc(reg)            => { adc(cpu, bus, reg.as_ref()); true },
        Sub(reg)            => { sub(cpu, bus, reg.as_ref()); true },
        Sbc(reg)            => { sbc(cpu, bus, reg.as_ref()); true },
        And(reg)            => { and(cpu, bus, reg.as_ref()); true },
        Xor(reg)            => { xor(cpu, bus, reg.as_ref()); true },
        Or(reg)             => { or(cpu, bus, reg.as_ref()); true },
        Cp(reg)             => { cp(cpu, bus, reg.as_ref()); true },
        Halt                => { halt(cpu); true },
        Ret(cond)           => { ret(cpu, bus, *cond) },
        AddSp(reg)          => { add_sp(cpu, bus, reg.as_ref()); true },
        Ldh(dest, src)      => { ld(cpu, bus, dest.as_ref(), src.as_ref()); true },
        Ldhl(offset)        => { ldhl(cpu, bus, offset.as_ref()); true },
        Pop(reg)            => { pop(cpu, bus, reg.as_ref()); true },
        Reti                => { reti(cpu, bus); true },
        Jp(cond, addr)      => { jp(cpu, bus, *cond, addr.as_ref()) },
        Di                  => { di(cpu); true },
        Ei                  => { ei(cpu); true },
        Call(cond, addr)    => { call(cpu, bus, *cond, addr.as_ref()) },
        Push(reg)           => { push(cpu, bus, reg.as_ref()); true },
        Rst(index)          => { rst(cpu, bus, *index); true },
        Rlc(reg)            => { rlc(cpu, bus, reg.as_ref()); true },
        Rrc(reg)            => { rrc(cpu, bus, reg.as_ref()); true },
        Rl(reg)             => { rl(cpu, bus, reg.as_ref()); true },
        Rr(reg)             => { rr(cpu, bus, reg.as_ref()); true },
        Sla(reg)            => { sla(cpu, bus, reg.as_ref()); true },
        Sra(reg)            => { sra(cpu, bus, reg.as_ref()); true },
        Swap(reg)           => { swap(cpu, bus, reg.as_ref()); true },
        Srl(reg)            => { srl(cpu, bus, reg.as_ref()); true },
        Bit(b, reg)         => { bit(cpu, bus, *b, reg.as_ref()); true },
        Res(bit, reg)       => { res(cpu, bus, *bit, reg.as_ref()); true },
        Set(bit, reg)       => { set(cpu, bus, *bit, reg.as_ref()); true }
    }
}