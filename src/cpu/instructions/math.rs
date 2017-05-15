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
    let sp = cpu.regs.sp();
    let signed = src.read(cpu, bus) as i8;
    let unsigned = src.read(cpu, bus) as u16;

    if signed < 0 {
        cpu.regs.set_sp(sp.wrapping_sub(signed.abs() as u16));
    } else {
        cpu.regs.set_sp(sp.wrapping_add(signed.abs() as u16));
    }

    let flags =
        if ((sp & 0xF) + (unsigned & 0xF)) & 0x10 == 0x10           // H
            { FLAG_H }
            else { 0 }
        | if ((sp & 0xFF) + (unsigned & 0xFF)) & 0x100 == 0x100     // C
            { FLAG_C } 
            else { 0 };

    cpu.regs.set_f(flags);
}

// ADC
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn adc(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a() as u32;
    let inc = src.read(cpu, bus) as u32;
    let carry = cpu.regs.has_flag(FLAG_C) as u32;

    let sum = val.wrapping_add(inc).wrapping_add(carry);

    let flags =
        if sum & 0xFF == 0 { FLAG_Z } else { 0 }           // Z
        | if ((val & 0xF) + (inc & 0xF) + carry) & 0x10 == 0x10     // H
            { FLAG_H }
            else { 0 }
        | if sum & 0x100 == 0x100 // C
            { FLAG_C }
            else { 0 };

    cpu.regs.set_a((sum & 0xFF) as u8);
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
    let val = cpu.regs.a() as u32;
    let dec = src.read(cpu, bus) as u32;
    let carry = cpu.regs.has_flag(FLAG_C) as u32;

    let diff = val.wrapping_sub(dec).wrapping_sub(carry);

    let flags =
        FLAG_N
        | if diff & 0xFF == 0 { FLAG_Z } else { 0 }           // Z
        | if (val & 0xF) < (dec & 0xF) + carry      // H
            { FLAG_H }
            else { 0 }
        | if diff & 0x100 == 0x100 // C
            { FLAG_C }
            else { 0 };

    cpu.regs.set_a(diff as u8);
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
// Based off blargg's code http://forums.nesdev.com/viewtopic.php?p=41926#p41926
#[inline(always)]
pub fn daa(cpu: &mut Cpu) {
    let mut a = cpu.regs.a() as u16;

    if !cpu.regs.has_flag(FLAG_N) {
        if cpu.regs.has_flag(FLAG_H) || (a & 0x0F) > 9 {
            a = a.wrapping_add(0x06);
        }

        if cpu.regs.has_flag(FLAG_C) || a > 0x9F {
            a = a.wrapping_add(0x60);
        }
    } else {
        if cpu.regs.has_flag(FLAG_H) {
            a = a.wrapping_sub(6) & 0xFF;
        }

        if cpu.regs.has_flag(FLAG_C) {
            a = a.wrapping_sub(0x60);
        }
    }

    let flags =
        if (a & 0xFF) == 0 { FLAG_Z } else { 0 }            // Z
        | cpu.regs.f() & FLAG_N                              // N
        | if (a & 0x100) == 0x100 { FLAG_C } else { 0 };    // C

    cpu.regs.set_a(a as u8);
    cpu.regs.set_f(flags);

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