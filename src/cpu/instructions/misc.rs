use super::super::Cpu;

 // NOP
#[inline(always)]
pub fn nop() {
    // Ahh, doing nothing feels so good!
}

// STOP 0
#[inline(always)]
pub fn stop(cpu: &mut Cpu) {
    cpu.halted = true;
}

// HALT
#[inline(always)]
pub fn halt(cpu: &mut Cpu) {
    cpu.halted = true;
}

// PREFIX CB
#[inline(always)]
pub fn prefix(cpu: &mut Cpu) {
    cpu.prefixed = true;
}

// DI
pub fn di(cpu: &mut Cpu) {
    cpu.ime = false;
}

// EI
pub fn ei(cpu: &mut Cpu) {
    cpu.ime = true;
}