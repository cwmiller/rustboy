use cpu::Cpu;
use cpu::addressing::*;
use cpu::instructions::*;
use cpu::instructions as instr;
use bus::Bus;
use cartridge::Cartridge;

// ADD 8-bit tests

#[test]
fn add8_no_flags() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0xDE);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0x11);

    add_8(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.a(), 0xEF);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn add8_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0xF0);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0xF0);

    add_8(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.a(), 0xE0);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn add8_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0x0F);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0x0F);

    add_8(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.a(), 0x1E);
    assert_eq!(cpu.regs.f(), 0b0010 << 4);
}

#[test]
fn add8_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0xFF);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0x01);

    add_8(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1011 << 4);
}

// ADD 16-bit tests

#[test]
fn add16_no_flags() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let dest = RegisterAddressing(Register::HL);
    let inc = ImmediateAddressing(0x01);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_hl(0x11);

    add_16(&mut cpu, &mut bus, &dest, &inc);

    assert_eq!(cpu.regs.hl(), 0x12);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

#[test]
fn add16_carry_flag() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let dest = RegisterAddressing(Register::HL);
    let inc = ImmediateAddressing(0xEEEE);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_hl(0x2000);

    add_16(&mut cpu, &mut bus, &dest, &inc);

    assert_eq!(cpu.regs.hl(),  0x0EEE);
    assert_eq!(cpu.regs.f(), 0b1001 << 4);
}

#[test]
fn add16_half_carry_flag() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let dest = RegisterAddressing(Register::HL);
    let inc = ImmediateAddressing(0xBBB);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_hl(0xBBB);

    add_16(&mut cpu, &mut bus, &dest, &inc);

    assert_eq!(cpu.regs.hl(), 0x1776);
    assert_eq!(cpu.regs.f(), 0b1010 << 4);
}

// ADD SP tests

#[test]
fn add_sp_carry_flag() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0x10);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_sp(0x00F0);

    add_sp(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.sp(), 0x100);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn add_sp_half_carry_flag() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let inc = ImmediateAddressing(0xF);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_sp(0xF01);

    add_sp(&mut cpu, &mut bus, &inc);

    assert_eq!(cpu.regs.sp(), 0xF10);
    assert_eq!(cpu.regs.f(), 0b010 << 4);
}

// ADC tests

#[test]
fn adc_no_carry_no_flags() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x12);

    cpu.regs.set_a(0x34);
    cpu.regs.set_f(0);

    adc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x46);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn adc_no_carry_does_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xFE);

    cpu.regs.set_a(0x10);
    cpu.regs.set_f(0);

    adc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x0E);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn adc_with_carry_does_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xFE);

    cpu.regs.set_a(0x10);
    cpu.regs.set_f(0b0001 << 4);

    adc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x0F);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn adc_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x0F);

    cpu.regs.set_a(0x01);
    cpu.regs.set_f(0);

    adc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x10);
    assert_eq!(cpu.regs.f(), 0b0010 << 4);
}

#[test]
fn adc_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0);

    cpu.regs.set_a(0xFF);
    cpu.regs.set_f(0b0001 << 4);

    adc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1011 << 4);
}

// SUB tests

#[test]
fn sub_doesnt_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x05);

    cpu.regs.set_a(0xFF);
    cpu.regs.set_f(0b1111 << 4);

    sub(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xFA);
    assert_eq!(cpu.regs.f(), 0b0100 << 4);
}

#[test]
fn sub_does_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xB0);

    cpu.regs.set_a(0xAA);
    cpu.regs.set_f(0b1111 << 4);

    sub(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xFA);
    assert_eq!(cpu.regs.f(), 0b0101 << 4);
}

#[test]
fn sub_half_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x0F);

    cpu.regs.set_a(0xAA);
    cpu.regs.set_f(0b1111 << 4);

    sub(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x9B);
    assert_eq!(cpu.regs.f(), 0b0110 << 4);
}

// SBC tests

#[test]
fn sbc_no_carry_no_flags() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x12);

    cpu.regs.set_a(0x34);
    cpu.regs.set_f(0);

    sbc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x22);
    assert_eq!(cpu.regs.f(), 0b0100 << 4);
}

#[test]
fn sbc_no_carry_does_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x20);

    cpu.regs.set_a(0x10);
    cpu.regs.set_f(0);

    sbc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xF0);
    assert_eq!(cpu.regs.f(), 0b0101 << 4);
}

#[test]
fn sbc_with_carry_does_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x20);

    cpu.regs.set_a(0x10);
    cpu.regs.set_f(0b0001 << 4);

    sbc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xEF);
    assert_eq!(cpu.regs.f(), 0b0111 << 4);
}

#[test]
fn sbc_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x03);

    cpu.regs.set_a(0x31);
    cpu.regs.set_f(0);

    sbc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x2E);
    assert_eq!(cpu.regs.f(), 0b0110 << 4);
}

#[test]
fn sbc_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(1);

    cpu.regs.set_a(2);
    cpu.regs.set_f(0b0001 << 4);

    sbc(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1100 << 4);
}

// AND tests

#[test]
fn and_not_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xBE);

    cpu.regs.set_a(0xEF);
    cpu.regs.set_f(0b1111 << 4);

    and(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xAE);
    assert_eq!(cpu.regs.f(), 0b0010 << 4);
}

#[test]
fn and_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xF0);

    cpu.regs.set_a(0x0F);
    cpu.regs.set_f(0b1111 << 4);

    and(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1010 << 4);
}

// XOR tests

#[test]
fn xor_not_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xBE);

    cpu.regs.set_a(0xEF);
    cpu.regs.set_f(0b1111 << 4);

    xor(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x51);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn xor_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x42);

    cpu.regs.set_a(0x42);
    cpu.regs.set_f(0b1111 << 4);

    xor(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// OR tests

#[test]
fn or_not_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x45);

    cpu.regs.set_a(0x10);
    cpu.regs.set_f(0b1111 << 4);

    or(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0x55);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn or_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0);

    cpu.regs.set_a(0);
    cpu.regs.set_f(0b1111 << 4);

    or(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// CP tests

#[test]
fn cp_doesnt_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x05);

    cpu.regs.set_a(0xFF);
    cpu.regs.set_f(0b1111 << 4);

    cp(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xFF);
    assert_eq!(cpu.regs.f(), 0b0100 << 4);
}

#[test]
fn cp_does_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0xB0);

    cpu.regs.set_a(0xAA);
    cpu.regs.set_f(0b1111 << 4);

    cp(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xAA);
    assert_eq!(cpu.regs.f(), 0b0101 << 4);
}

#[test]
fn cp_half_borrow() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let n = ImmediateAddressing(0x0F);

    cpu.regs.set_a(0xAA);
    cpu.regs.set_f(0b1111 << 4);

    cp(&mut cpu, &mut bus, &n);

    assert_eq!(cpu.regs.a(), 0xAA);
    assert_eq!(cpu.regs.f(), 0b0110 << 4);
}

// INC 8-bit tests

#[test]
fn inc8_no_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0xDE);
    cpu.regs.set_f(0b1111 << 4);

    inc_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0xDF);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn inc8_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0xDF);
    cpu.regs.set_f(0b1111 << 4);

    inc_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0xE0);
    assert_eq!(cpu.regs.f(), 0b0011 << 4);
}

#[test]
fn inc8_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0xFF);
    cpu.regs.set_f(0b1111 << 4);

    inc_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1011 << 4);
}

// DEC 8-bit tests

#[test]
fn dec8_no_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0xDE);
    cpu.regs.set_f(0b1111 << 4);

    dec_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0xDD);
    assert_eq!(cpu.regs.f(), 0b0101 << 4);
}

#[test]
fn dec8_half_carry() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0xD0);
    cpu.regs.set_f(0b1111 << 4);

    dec_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0xCF);
    assert_eq!(cpu.regs.f(), 0b0111 << 4);
}

#[test]
fn dec8_zero() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let a = RegisterAddressing(Register::A);

    cpu.regs.set_a(0x01);
    cpu.regs.set_f(0b1111 << 4);

    dec_8(&mut cpu, &mut bus, &a);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1101 << 4);
}

// INC 16-bit tests

#[test]
fn inc16() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let bc = RegisterAddressing(Register::BC);

    cpu.regs.set_bc(0xBEEF);
    cpu.regs.set_f(0b1111 << 4);

    inc_16(&mut cpu, &mut bus, &bc);

    assert_eq!(cpu.regs.bc(), 0xBEF0);
    assert_eq!(cpu.regs.f(), 0b1111 << 4);
}

// DEC 16-bit tests

#[test]
fn dec16() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let bc = RegisterAddressing(Register::BC);

    cpu.regs.set_bc(0xBEEF);
    cpu.regs.set_f(0b1111 << 4);

    dec_16(&mut cpu, &mut bus, &bc);

    assert_eq!(cpu.regs.bc(), 0xBEEE);
    assert_eq!(cpu.regs.f(), 0b1111 << 4);
}

// DAA tests

#[test]
fn daa() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let b = RegisterAddressing(Register::B);

    cpu.regs.set_a(0x49);
    cpu.regs.set_b(0x01);

    add_8(&mut cpu, &mut bus, &b);
    instr::daa(&mut cpu);
    assert_eq!(cpu.regs.a(), 0x50);

    cpu.regs.set_a(0x49);
    cpu.regs.set_b(0x11);

    sub(&mut cpu, &mut bus, &b);
    instr::daa(&mut cpu);
    assert_eq!(cpu.regs.a(), 0x38);
}

// SCF tests

#[test]
fn scf() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    instr::scf(&mut cpu);

    assert_eq!(cpu.regs.f(), 0b1001 << 4);
}

// CCF tests

#[test]
fn ccf_with_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    ccf(&mut cpu);

    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

#[test]
fn ccf_without_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1110 << 4);
    ccf(&mut cpu);

    assert_eq!(cpu.regs.f(), 0b1001 << 4);
}

// CPL tests

#[test]
fn cpl() {
    let mut cpu = Cpu::new();
    cpu.regs.set_a(0b10101010);
    instr::cpl(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b01010101);
}