use bus::Bus;
use cpu::{AddressingMode, Cpu, FLAG_Z, FLAG_C, FLAG_H, FLAG_N};

// RLCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rlca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = (val >> 7) & 1;
    let shifted = val << 1 | carry;
    let flags = carry << 4;

    cpu.regs.set_a(shifted);
    cpu.regs.set_f(flags);
}

// RLA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rla(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = (val >> 7) & 1;
    let shifted = (val << 1) | ((cpu.regs.f() & FLAG_C) >> 4);
    let flags = carry << 4;

    cpu.regs.set_a(shifted);
    cpu.regs.set_f(flags);
}

// RRCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rrca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let carry = val & 1;
    let shifted = carry << 7 | val >> 1;
    let flags = carry << 4;

    cpu.regs.set_a(shifted);
    cpu.regs.set_f(flags);
}

// RRA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rra(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let shifted = (val >> 1) | ((cpu.regs.f() & FLAG_C) << 3);
    let flags = (val & 1) << 4;

    cpu.regs.set_a(shifted);
    cpu.regs.set_f(flags);
}

// RLC
// Affects flags: Z, N, H, C
pub fn rlc(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = (val >> 7) & 1;
    let shifted = val << 1 | carry;
    let flags = 
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | if carry == 1 { FLAG_C } else { 0 };  // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// RRC
// Affects flags: Z, N, H, C
pub fn rrc(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = carry << 7 | val >> 1;
    let flags = 
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | carry << 4;                               // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// RL
// Affects flags: Z, N, H, C
pub fn rl(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 0b10000000;
    let shifted = val << 1 | ((cpu.regs.f() & FLAG_C) >> 4);
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }
        | carry >> 3;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// RR
// Affects flags: Z, N, H, C
pub fn rr(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = val >> 1 | ((cpu.regs.f() & FLAG_C) << 3);
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | carry << 4;                            // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// SLA
// Affects flags: Z, N, H, C
pub fn sla(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 0b10000000;
    let shifted = val << 1 & !1;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | carry >> 3;                            // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// SRA
// Affects flags: Z, N, H, C
pub fn sra(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let carry = val & 1;
    let shifted = val & 0b10000000 | val >> 1;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | carry << 4;                            // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// SWAP
// Affects flags: Z, N, H, C
pub fn swap(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let low = val & 0x0F;
    let high = val & 0xF0;
    let swapped = (low << 4) | (high >> 4);
    let flags =
        if swapped == 0 { FLAG_Z } else { 0 };  // Z

    reg.write(cpu, bus, swapped);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// SRL
// Affects flags: Z, N, H, C
pub fn srl(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let rb = val & 1;
    let shifted = val >> 1 & 0b01111111;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | rb << 4;                               // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// BIT
// Affects flags: Z, N, H
pub fn bit(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let test = 1 << bit;
    let flags =
        (if val & test == 0 { FLAG_Z } else { 0 })  // Z
        & !FLAG_N                                    // N
        | FLAG_H                                     // H
        | cpu.regs.f() & FLAG_C;                     // C

    cpu.regs.set_f(flags);
    cpu.prefixed = false;
}

// RES
pub fn res(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val & !(1 << bit));
    cpu.prefixed = false;
}

// SET
pub fn set(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val | (1 << bit));
    cpu.prefixed = false;
}