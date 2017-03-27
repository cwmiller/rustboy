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