use bus::Bus;
use super::super::{AddressingMode, Condition, Cpu};

// JR
#[inline(always)]
pub fn jr(cpu: &mut Cpu, bus: &mut Bus, cond: Condition, src: &dyn AddressingMode<u8>) -> bool {
    if cpu.condition_met(cond) {
        let pc = cpu.regs.pc();
        let offset = src.read(cpu, bus) as i8;

        if offset > 0 {
            cpu.regs.set_pc(pc.wrapping_add(offset as u16));
        } else {
            cpu.regs.set_pc(pc.wrapping_sub(offset.abs() as u16));
        }
        true
    } else {
        false
    }
}

// JP
#[inline(always)]
pub fn jp(cpu: &mut Cpu, bus: &mut Bus, cond: Condition, src: &dyn AddressingMode<u16>) -> bool {
    if cpu.condition_met(cond) {
        let addr = src.read(cpu, bus);
        cpu.regs.set_pc(addr);
        true
    } else {
        false
    }
}

// CALL
#[inline(always)]
pub fn call(cpu: &mut Cpu, bus: &mut Bus, cond: Condition, src: &dyn AddressingMode<u16>) -> bool {
    if cpu.condition_met(cond) {
        let addr = src.read(cpu, bus);
        let pc = cpu.regs.pc();
        
        cpu.push_stack(bus, pc);
        cpu.regs.set_pc(addr);
        true
    } else {
        false
    }
}

// RET
#[inline(always)]
pub fn ret(cpu: &mut Cpu, bus: &mut Bus, cond: Condition) -> bool {
    if cpu.condition_met(cond) {
        let addr = cpu.pop_stack(bus);

        cpu.regs.set_pc(addr);
        true
    } else {
        false
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
pub fn rst(cpu: &mut Cpu, bus: &mut Bus, index: u8) {
    let pc = cpu.regs.pc();
    cpu.push_stack(bus, pc);
    cpu.regs.set_pc((index * 8) as u16);
}