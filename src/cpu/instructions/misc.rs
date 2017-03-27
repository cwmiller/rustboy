use cpu::Cpu;

 // NOP
#[inline(always)]
pub fn nop() {
    // Ahh, doing nothing feels so good!
}

// STOP 0
// TODO: halt cpu/lcd
#[inline(always)]
pub fn stop() {
    // TODO
}

// HALT
#[inline(always)]
pub fn halt() {
    // TODO
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