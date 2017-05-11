use bus::Bus;
use super::super::{AddressingMode, Cpu, FLAG_Z, FLAG_C, FLAG_H, FLAG_N};

// ADD (8bit)
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn add_8(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let inc = src.read(cpu, bus);
    let sum = val.wrapping_add(inc);
    let flags = 
        if sum == 0 { FLAG_Z } else { 0 }           // Z
        | (((val & 0xF) + (inc & 0xF)) & 0x10) << 1  // H
        | if sum < val { FLAG_C } else { 0 };       // C

    cpu.regs.set_a(sum);
    cpu.regs.set_f(flags);     
}

// ADD (16bit)
// Affects flags: N, H, C
#[inline(always)]
pub fn add_16(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u16>, src: &AddressingMode<u16>) {
    let src_val = src.read(cpu, bus);
    let dest_val = dest.read(cpu, bus);

    dest.write(cpu, bus, dest_val.wrapping_add(src_val));

    let flags = 
        (cpu.regs.f() & FLAG_Z)                                                 // Z
        | if (((dest_val & 0xFFF) + (src_val & 0xFFF)) & 0x1000) == 0x1000      // H
            { FLAG_H }
            else { 0 }
        | if dest_val.wrapping_add(src_val) < src_val                           // C
            { FLAG_C }
            else { 0 };

    cpu.regs.set_f(flags);
}

// ADD SP, r8
// Affects flags: Z, N, H, C
#[inline(always)]
pub fn add_sp(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let current = cpu.regs.sp();
    let val = src.read(cpu, bus) as i8;
    let unsigned = val as u16;

    if val > 0 {
        cpu.regs.set_sp(current.wrapping_add(unsigned));
    } else {
        cpu.regs.set_sp(current.wrapping_sub(val.abs() as u16));
    }

    let flags =
        if ((current & 0xFFF) + (unsigned & 0xFFF)) & 0x1000 == 0x1000  // H
            { FLAG_H }
            else { 0 }
        | if current.wrapping_add(unsigned) < current                   // C
            { FLAG_C } 
            else { 0 };

    cpu.regs.set_f(flags);  
}

// ADC
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn adc(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let inc = src.read(cpu, bus);
    let mut sum = val.wrapping_add(inc);

    if (cpu.regs.f() & FLAG_C) == FLAG_C {
        sum = sum.wrapping_add(1);
    }

    let flags = 
        if sum == 0 { FLAG_Z } else { 0 }           // Z
        | (((val & 0xF) + (inc & 0xF)) & 0x10) << 1  // H
        | if sum < val { FLAG_C } else { 0 };       // C

    cpu.regs.set_a(sum);
    cpu.regs.set_f(flags);       
}

// SUB
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn sub(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let dec = src.read(cpu, bus);
    let diff = val.wrapping_sub(dec);
    let flags = 
        if diff == 0 { FLAG_Z } else { 0 }                      // Z
        | FLAG_N                                                 // N
        | if (val & 0xF) < (dec & 0xF) { FLAG_H } else { 0 }    // H
        | if val < dec { FLAG_C } else { 0 };                   // C

    cpu.regs.set_a(diff);
    cpu.regs.set_f(flags);
}

// SBC
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn sbc(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let dec = src.read(cpu, bus);
    let mut diff = val.wrapping_sub(dec);

    if (cpu.regs.f() & FLAG_C) == FLAG_C {
        diff = diff.wrapping_sub(1);
    }

    let flags =
        if diff == 0 { FLAG_Z } else { 0 }                      // Z
        | FLAG_N                                                 // N
        | if (val & 0xF) < (dec & 0xF) { FLAG_H } else { 0 }    // H
        | if val < dec { FLAG_C } else { 0 };                   // C

    cpu.regs.set_a(diff);
    cpu.regs.set_f(flags);
}

// AND
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn and(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing & val;
    let flags =
        if res == 0 { FLAG_Z } else { 0 }   // Z
        | FLAG_H;                            // H

    cpu.regs.set_a(res);
    cpu.regs.set_f(flags);
}

// XOR
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn xor(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing ^ val;

    cpu.regs.set_a(res);
    cpu.regs.set_f(if res == 0 { FLAG_Z } else { 0 });  // Z
}

// OR
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn or(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing | val;

    cpu.regs.set_a(res);
    cpu.regs.set_f(if res == 0 { FLAG_Z } else { 0 });   // Z
}

// CP
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn cp(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let dec = src.read(cpu, bus);
    let diff = val.wrapping_sub(dec);

    let flags =
        if diff == 0 { FLAG_Z } else { 0 }                      // Z
        | FLAG_N                                                 // N
        | if (val & 0xF) < (dec & 0xF) { FLAG_H } else { 0 }    // H
        | if val < dec { FLAG_C } else { 0 };                   // C

    cpu.regs.set_f(flags);
}

// INC (8bit)
// Affects flags: Z, N, H
#[inline(always)]
pub fn inc_8(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>) {
    let val = dest.read(cpu, bus);
    let increased = val.wrapping_add(1);

    dest.write(cpu, bus, increased);

    let flags = 
        (if increased == 0 { FLAG_Z } else { 0 })   // Z
        | (((val & 0xF) + 1) & 0x10) << 1            // H
        | cpu.regs.f() & FLAG_C;                     // C

    cpu.regs.set_f(flags);
}

// DEC (8bit)
// Affects flags: Z, N, H
#[inline(always)]
pub fn dec_8(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>) {
    let val = dest.read(cpu, bus);
    let decreased = val.wrapping_sub(1);

    dest.write(cpu, bus, decreased);

    let flags =
        (if decreased == 0 { FLAG_Z } else { 0 })   // Z
        | FLAG_N                                     // N
        | if val & 0xF == 0 { FLAG_H } else { 0 }   // H
        | cpu.regs.f() & FLAG_C;                     // C

    cpu.regs.set_f(flags);
}

// INC (16bit)
#[inline(always)]
pub fn inc_16(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u16>) {
    let val = dest.read(cpu, bus);
    dest.write(cpu, bus, val.wrapping_add(1));
}

// DEC (16bit)
#[inline(always)]
pub fn dec_16(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u16>) {
    let val = dest.read(cpu, bus);
    dest.write(cpu, bus, val.wrapping_sub(1));
}

// DAA
// Flags affected: Z, H, C
// Based off of http://forums.nesdev.com/viewtopic.php?t=9088
#[inline(always)]
pub fn daa(cpu: &mut Cpu) {
    let mut a = cpu.regs.a() as u16;

    if cpu.regs.has_flag(FLAG_N) {
        if cpu.regs.has_flag(FLAG_H) {
            a = (a - 6) & 0xFF;
        }

        if cpu.regs.has_flag(FLAG_C) {
            a = a - 0x60;
        }
    } else {
        if cpu.regs.has_flag(FLAG_H) || (a & 0x0F) > 9 {
            a = a + 0x06;
        }

        if cpu.regs.has_flag(FLAG_C) || a > 0x9F {
            a = a + 0x60;
        }
    }

    let flags = 
        if (a & 0xFF) == 0 { FLAG_Z } else { 0 }            // Z
        | cpu.regs.f() & FLAG_N                              // N
        | if (a & 0x100) == 0x100 { FLAG_C } else { 0 };    // C

    a = a & 0xFF;

    cpu.regs.set_f(flags);
    cpu.regs.set_a(a as u8);
}

// SCF
// Flags affected: N, H, C
#[inline(always)]
pub fn scf(cpu: &mut Cpu) {
    let flags = (cpu.regs.f() & FLAG_Z) | FLAG_C;
    cpu.regs.set_f(flags);
}

// CCF
// Flags affected: N, H, C
#[inline(always)]
pub fn ccf(cpu: &mut Cpu) {
    let flags = (cpu.regs.f() & FLAG_Z) | ((cpu.regs.f() ^ FLAG_C) & FLAG_C);
    cpu.regs.set_f(flags);
}

// CPL
// Flags affected: N, H
#[inline(always)]
pub fn cpl(cpu: &mut Cpu) {
    let a = !cpu.regs.a();
    let flags = cpu.regs.f() | FLAG_N | FLAG_H;

    cpu.regs.set_a(a);
    cpu.regs.set_f(flags);
}