use bus::Bus;
use super::super::{AddressingMode, Cpu, FLAG_C};

// LD
#[inline(always)]
pub fn ld<T>(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<T>, src: &AddressingMode<T>) {
    let val = src.read(cpu, bus);
    dest.write(cpu, bus, val);
}

// LDHL SP, r8
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn ldhl(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let sp = cpu.regs.sp();
    let immediate = src.read(cpu, bus) as u16;
    let offset = immediate as i16;

    if offset > 0 {
        cpu.regs.set_hl(sp.wrapping_add(immediate));
    } else {
        cpu.regs.set_hl(sp.wrapping_sub(offset.abs() as u16));
    }

    // TODO: I don't think this is right
    let flags =
        ((((sp & 0xF) + (immediate  & 0xF)) & 0x10) as u8) << 1 // H
        | if sp.wrapping_add(immediate) < sp                    // C
            { FLAG_C } 
            else { 0 };

    cpu.regs.set_f(flags);
}

// LDD
#[inline(always)]
pub fn ldd(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>, src: &AddressingMode<u8>) {
    let val = src.read(cpu, bus);
    let hl = cpu.regs.hl();

    dest.write(cpu, bus, val);
    cpu.regs.set_hl(hl.wrapping_sub(1));
}

// LDI
#[inline(always)]
pub fn ldi(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>, src: &AddressingMode<u8>) {
    let val = src.read(cpu, bus);
    let hl = cpu.regs.hl();

    dest.write(cpu, bus, val);
    cpu.regs.set_hl(hl.wrapping_add(1));
}

// PUSH
#[inline(always)]
pub fn push(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u16>) {
    let val = src.read(cpu, bus);
    cpu.push_stack(bus, val);
}

// POP
#[inline(always)]
pub fn pop(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u16>) {
    let val = cpu.pop_stack(bus);
    dest.write(cpu, bus, val);
}