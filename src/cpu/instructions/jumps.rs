use bus::Bus;
use cpu::{AddressingMode, Condition, Cpu};

// JR
#[inline(always)]
pub fn jr(cpu: &mut Cpu, bus: &mut Bus, mode: &AddressingMode<u8>) {
    let pc = cpu.regs.pc();
    let offset = mode.read(cpu, bus) as i8;

    if offset > 0 {
        cpu.regs.set_pc(pc.wrapping_add(offset as u16));
    } else {
        cpu.regs.set_pc(pc.wrapping_sub(offset.abs() as u16));
    }
}

// RET
#[inline(always)]
pub fn ret(cpu: &mut Cpu, bus: &mut Bus) {
    let addr = cpu.pop_stack(bus);

    cpu.regs.set_pc(addr);
}