use bus::Bus;
use super::super::{AddressingMode, Condition, Cpu};

// JR
#[inline(always)]
pub fn jr(cpu: &mut Cpu, bus: &mut Bus, cond: Condition, src: &AddressingMode<u8>) {
    if cpu.condition_met(cond) {
        let pc = cpu.regs.pc();
        let offset = src.read(cpu, bus) as i8;

        if offset > 0 {
            cpu.regs.set_pc(pc.wrapping_add(offset as u16));
        } else {
            cpu.regs.set_pc(pc.wrapping_sub(offset.abs() as u16));
        }
    }
}

// JP
#[inline(always)]
pub fn jp(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u16>) {
    let addr = src.read(cpu, bus);
    cpu.regs.set_pc(addr);
}

// CALL
#[inline(always)]
pub fn call(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u16>) {
    let addr = cpu.next_word(bus);
    let pc = cpu.regs.pc();
    
    cpu.push_stack(bus, pc);
    cpu.regs.set_pc(addr);
}

// RET
#[inline(always)]
pub fn ret(cpu: &mut Cpu, bus: &mut Bus, cond: Condition) {
    if cpu.condition_met(cond) {
        let addr = cpu.pop_stack(bus);

        cpu.regs.set_pc(addr);
    }
}

// RETI
#[inline(always)]
pub fn reti(cpu: &mut Cpu, bus: &mut Bus) {
    let addr = cpu.pop_stack(bus);
    cpu.regs.set_pc(addr);
    cpu.ime = true;
}

// RST
#[inline(always)]
pub fn rst(cpu: &mut Cpu, bus: &mut Bus, addr: u8) {
    let pc = cpu.regs.pc();
    cpu.push_stack(bus, pc);
    cpu.regs.set_pc(addr as u16);
}