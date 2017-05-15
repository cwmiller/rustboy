use bus::Bus;
use super::super::{AddressingMode, Cpu};

// ADD (8bit)
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn add_8(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let inc = src.read(cpu, bus);
    let sum = val.wrapping_add(inc);

    cpu.regs.set_a(sum);
    cpu.regs.set_carry(sum < val);
    cpu.regs.set_halfcarry(((val & 0xF) + (inc & 0xF)) & 0x10 == 0x10);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(sum == 0);
}

// ADD (16bit)
// Affects flags: N, H, C
#[inline(always)]
pub fn add_16(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u16>, src: &AddressingMode<u16>) {
    let src_val = src.read(cpu, bus);
    let dest_val = dest.read(cpu, bus);

    dest.write(cpu, bus, dest_val.wrapping_add(src_val));

    cpu.regs.set_carry(dest_val.wrapping_add(src_val) < src_val);
    cpu.regs.set_halfcarry((((dest_val & 0xFFF) + (src_val & 0xFFF)) & 0x1000) == 0x1000);
    cpu.regs.set_subtract(false);
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

    cpu.regs.set_carry(((sp & 0xFF) + (unsigned & 0xFF)) & 0x100 == 0x100);
    cpu.regs.set_halfcarry(((sp & 0xF) + (unsigned & 0xF)) & 0x10 == 0x10);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(false);
}

// ADC
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn adc(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a() as u32;
    let inc = src.read(cpu, bus) as u32;
    let carry = cpu.regs.carry() as u32;
    let sum = val.wrapping_add(inc).wrapping_add(carry);

    cpu.regs.set_a((sum & 0xFF) as u8);
    cpu.regs.set_carry(sum & 0x100 == 0x100);
    cpu.regs.set_halfcarry(((val & 0xF) + (inc & 0xF) + carry) & 0x10 == 0x10);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero((sum & 0xFF) == 0);
}

// SUB
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn sub(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let dec = src.read(cpu, bus);
    let diff = val.wrapping_sub(dec);

    cpu.regs.set_a(diff);
    cpu.regs.set_carry(val < dec);
    cpu.regs.set_halfcarry((val & 0xF) < (dec & 0xF));
    cpu.regs.set_subtract(true);
    cpu.regs.set_zero(diff == 0);
}

// SBC
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn sbc(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a() as u32;
    let dec = src.read(cpu, bus) as u32;
    let carry = cpu.regs.carry() as u32;

    let diff = val.wrapping_sub(dec).wrapping_sub(carry);

    cpu.regs.set_a(diff as u8);
    cpu.regs.set_carry(diff & 0x100 == 0x100);
    cpu.regs.set_halfcarry((val & 0xF) < (dec & 0xF) + carry);
    cpu.regs.set_subtract(true);
    cpu.regs.set_zero(diff & 0xFF == 0);
}

// AND
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn and(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing & val;

    cpu.regs.set_a(res);
    cpu.regs.set_carry(false);
    cpu.regs.set_halfcarry(true);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(res == 0);
}

// XOR
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn xor(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing ^ val;

    cpu.regs.set_a(res);
    cpu.regs.set_carry(false);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(res == 0);
}

// OR
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn or(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let existing = cpu.regs.a();
    let val = src.read(cpu, bus);
    let res = existing | val;

    cpu.regs.set_a(res);
    cpu.regs.set_carry(false);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(res == 0);
}

// CP
// Flags affected: Z, N, H, C
#[inline(always)]
pub fn cp(cpu: &mut Cpu, bus: &mut Bus, src: &AddressingMode<u8>) {
    let val = cpu.regs.a();
    let dec = src.read(cpu, bus);
    let diff = val.wrapping_sub(dec);

    cpu.regs.set_carry(val < dec);
    cpu.regs.set_halfcarry((val & 0xF) < (dec & 0xF));
    cpu.regs.set_subtract(true);
    cpu.regs.set_zero(diff == 0);
}

// INC (8bit)
// Affects flags: Z, N, H
#[inline(always)]
pub fn inc_8(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>) {
    let val = dest.read(cpu, bus);
    let increased = val.wrapping_add(1);

    dest.write(cpu, bus, increased);
    cpu.regs.set_halfcarry((((val & 0xF) + 1) & 0x10) == 0x10);
    cpu.regs.set_subtract(false);
    cpu.regs.set_zero(increased == 0);
}

// DEC (8bit)
// Affects flags: Z, N, H
#[inline(always)]
pub fn dec_8(cpu: &mut Cpu, bus: &mut Bus, dest: &AddressingMode<u8>) {
    let val = dest.read(cpu, bus);
    let decreased = val.wrapping_sub(1);

    dest.write(cpu, bus, decreased);
    cpu.regs.set_halfcarry(val & 0xF == 0);
    cpu.regs.set_subtract(true);
    cpu.regs.set_zero(decreased == 0);
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

    if !cpu.regs.subtract() {
        if cpu.regs.halfcarry() || (a & 0x0F) > 9 {
            a = a.wrapping_add(0x06);
        }

        if cpu.regs.carry() || a > 0x9F {
            a = a.wrapping_add(0x60);
        }
    } else {
        if cpu.regs.halfcarry() {
            a = a.wrapping_sub(6) & 0xFF;
        }

        if cpu.regs.carry() {
            a = a.wrapping_sub(0x60);
        }
    }

    cpu.regs.set_a((a & 0xFF) as u8);
    cpu.regs.set_carry((a & 0x100) == 0x100);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_zero((a & 0xFF) == 0);

}

// SCF
// Flags affected: N, H, C
#[inline(always)]
pub fn scf(cpu: &mut Cpu) {
    cpu.regs.set_carry(true);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
}

// CCF
// Flags affected: N, H, C
#[inline(always)]
pub fn ccf(cpu: &mut Cpu) {
    let carry = cpu.regs.carry();

    cpu.regs.set_carry(!carry);
    cpu.regs.set_halfcarry(false);
    cpu.regs.set_subtract(false);
}

// CPL
// Flags affected: N, H
#[inline(always)]
pub fn cpl(cpu: &mut Cpu) {
    let a = !cpu.regs.a();

    cpu.regs.set_a(a);
    cpu.regs.set_halfcarry(true);
    cpu.regs.set_subtract(true);
}