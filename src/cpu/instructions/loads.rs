use bus::Bus;
use cpu::{AddressingMode, Cpu};

// LD
#[inline(always)]
pub fn ld<T>(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<T>, src: &AddressingMode<T>) {
    let val = src.read(cpu, bus);
    dest.write(cpu, bus, val);
}