use bus::Bus;
use super::super::{AddressingMode, Cpu};

// LD
#[inline(always)]
pub fn ld<T>(cpu: &mut Cpu, bus: &mut Bus, dest: &dyn AddressingMode<T>, src: &dyn AddressingMode<T>) {
    let val = src.read(cpu, bus);
    dest.write(cpu, bus, val);
}

// LDHL SP, r8
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn ldhl(cpu: &mut Cpu, bus: &mut Bus, src: &dyn AddressingMode<u8>) {
    let sp = cpu.regs.sp();
    let unsigned = src.read(cpu, bus) as u16;
    let signed = src.read(cpu, bus) as i8;

    if signed < 0 {
        cpu.regs.set_hl(sp.wrapping_sub(signed.abs() as u16));
    } else {
        cpu.regs.set_hl(sp.wrapping_add(signed.abs() as u16));
    }

    cpu.regs.set_carry(((sp & 0xFF) + (unsigned & 0xFF)) & 0x100 == 0x100);
    cpu.regs.set_halfcarry(((sp & 0xF) + (unsigned & 0xF)) & 0x10 == 0x10);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// LDD
#[inline(always)]
pub fn ldd(cpu: &mut Cpu, bus: &mut Bus, dest: &dyn AddressingMode<u8>, src: &dyn AddressingMode<u8>) {
    let val = src.read(cpu, bus);
    let hl = cpu.regs.hl();

    dest.write(cpu, bus, val);
    cpu.regs.set_hl(hl.wrapping_sub(1));
}

// LDI
#[inline(always)]
pub fn ldi(cpu: &mut Cpu, bus: &mut Bus, dest: &dyn AddressingMode<u8>, src: &dyn AddressingMode<u8>) {
    let val = src.read(cpu, bus);
    let hl = cpu.regs.hl();

    dest.write(cpu, bus, val);
    cpu.regs.set_hl(hl.wrapping_add(1));
}

// PUSH
#[inline(always)]
pub fn push(cpu: &mut Cpu, bus: &mut Bus, src: &dyn AddressingMode<u16>) {
    let val = src.read(cpu, bus);
    cpu.push_stack(bus, val);
}

// POP
#[inline(always)]
pub fn pop(cpu: &mut Cpu, bus: &mut Bus, dest: &dyn AddressingMode<u16>) {
    let val = cpu.pop_stack(bus);
    dest.write(cpu, bus, val);
}