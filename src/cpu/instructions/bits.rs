use bus::Bus;
use cpu::{AddressingMode, Cpu, FLAG_Z, FLAG_C, FLAG_H, FLAG_N};

// RLCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rlca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let adj = val << 1;
    let carry = (val >> 7) & 0b00000001;

    cpu.regs.set_a(adj | carry);

    cpu.regs.set_f(
        (if adj == 0 { FLAG_Z } else { 0 }) // Z
        | carry << 4);                      // C
}

// RLA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rla(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let adj = (val << 1) | ((cpu.regs.f() & FLAG_C) >> 4);

    cpu.regs.set_a(adj);

    cpu.regs.set_f(
        (if adj == 0 { FLAG_Z } else { 0 }) // Z
        | (val & 0b10000000) >> 3);         // C
}

// RRCA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rrca(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let adj = val >> 1;
    let carry = val & 1;

    let flags = 
        (if adj == 0 { FLAG_Z } else { 0 }) // Z
        | carry << 4;                       // C

    cpu.regs.set_a(adj | (carry << 7));
    cpu.regs.set_f(flags);
}

// RRA
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn rra(cpu: &mut Cpu) {
    let val = cpu.regs.a();
    let adj = (val >> 1) | ((cpu.regs.f() & FLAG_C) << 3);

    let flags = 
        (if adj == 0 { FLAG_Z } else { 0 }) // Z
        | adj << 4;                         // C

    cpu.regs.set_a(adj);
    cpu.regs.set_f(flags);
}

// RLC
// Affects flags: Z, N, H, C
pub fn rlc(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let lb = val & 0b10000000;
    let shifted = val << 1;
    let flags = 
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | lb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
        
}

// RRC
// Affects flags: Z, N, H, C
pub fn rrc(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let rb = val & 1;
    let shifted = val >> 1;
    let flags = 
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | rb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
}

// RL
// Affects flags: Z, N, H, C
pub fn rl(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let lb = val & 0b10000000;
    let shifted = val << 1 | ((cpu.regs.f() & FLAG_C) >> 3);
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }
        | lb >> 3;

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);       
}

// RR
// Affects flags: Z, N, H, C
pub fn rr(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let rb = val & 1;
    let shifted = val >> 1 | ((cpu.regs.f() & FLAG_C) << 3);
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | rb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
}

// SLA
// Affects flags: Z, N, H, C
pub fn sla(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let lb = val & 0b10000000;
    let shifted = val << 1 & !1;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | lb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
}

// SRA
// Affects flags: Z, N, H, C
pub fn sra(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let rb = val & 1;
    let shifted = val >> 1;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | rb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
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
}

// SRL
// Affects flags: Z, N, H, C
pub fn srl(cpu: &mut Cpu, bus: &mut Bus, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let rb = val & 1;
    let shifted = val >> 1 & 0b01111111;
    let flags =
        if shifted == 0 { FLAG_Z } else { 0 }   // Z
        | rb >> 3;                              // C

    reg.write(cpu, bus, shifted);
    cpu.regs.set_f(flags);
}

// BIT
// Affects flags: Z, N, H
pub fn bit(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    let test = 1 << bit;
    let flags = 
        cpu.regs.f()
        | if val & test == test { FLAG_Z } else { 0 }   // Z
        & !FLAG_N                                       // N
        | FLAG_H;                                       // C

    cpu.regs.set_f(flags);
}

// RES
pub fn res(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val & !(1 << bit));
}

// SET
pub fn set(cpu: &mut Cpu, bus: &mut Bus, bit: u8, reg: &AddressingMode<u8>) {
    let val = reg.read(cpu, bus);
    reg.write(cpu, bus, val | (1 << bit));
}