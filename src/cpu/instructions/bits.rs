use bus::Bus;
use cpu::{AddressingMode, Cpu};

// RLCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rlca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = (val >> 7) & 1;
    let shifted = val << 1 | carry;

    cpu.regs.set_a(shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// RLA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rla(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = (val >> 7) & 1;
    let shifted = (val << 1) | (cpu.regs.carry() as u8);

    cpu.regs.set_a(shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// RRCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rrca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = val & 1;
    let shifted = carry << 7 | val >> 1;

    cpu.regs.set_a(shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// RRA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rra(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let shifted = (val >> 1) | (cpu.regs.carry() as u8) << 7;

    cpu.regs.set_a(shifted);
    cpu.regs.set_carry((val & 1) == 1);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// RLC
// Affects flags: Z, N, H, C
pub fn rlc(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = (val >> 7) & 1;
    let shifted = val << 1 | carry;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// RRC
// Affects flags: Z, N, H, C
pub fn rrc(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = carry << 7 | val >> 1;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// RL
// Affects flags: Z, N, H, C
pub fn rl(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 0b10000000;
    let shifted = val << 1 | (cpu.regs.carry() as u8);

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// RR
// Affects flags: Z, N, H, C
pub fn rr(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = val >> 1 | (cpu.regs.carry() as u8) << 7;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// SLA
// Affects flags: Z, N, H, C
pub fn sla(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 0b10000000;
    let shifted = val << 1 & !1;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// SRA
// Affects flags: Z, N, H, C
pub fn sra(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = val & 0b10000000 | val >> 1;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// SWAP
// Affects flags: Z, N, H, C
pub fn swap(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let low = val & 0x0F;
    let high = val & 0xF0;
    let swapped = (low << 4) | (high >> 4);

    reg.write(cpu, bus, swapped);
    cpu.regs.set_carry(false);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(swapped == 0);
}

// SRL
// Affects flags: Z, N, H, C
pub fn srl(cpu: &mut Cpu, bus: &mut Bus, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = val >> 1 & 0b01111111;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_carry(carry > 0);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(shifted == 0);
}

// BIT
// Affects flags: Z, N, H
pub fn bit(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let test = 1 << bit;

    cpu.regs.set_halfcarry(true);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(val & test == 0);
}

// RES
pub fn res(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val & !(1 << bit));
}

// SET
pub fn set(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &dyn AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val | (1 << bit));
}