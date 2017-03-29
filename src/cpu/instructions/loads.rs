use bus::Bus;
use super::super::{AddressingMode, Cpu};

// LD
#[inline(always)]
pub fn ld<T>(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<T>, src: &AddressingMode<T>) {
    let val = src.read(cpu, bus);
    dest.write(cpu, bus, val);
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