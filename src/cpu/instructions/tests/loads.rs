use cpu::Cpu;
use cpu::addressing::*;
use cpu::instructions::*;
use bus::{Addressable, Bus};
use cartridge::Cartridge;

// LD tests

#[test]
fn ld_a_b() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let a = RegisterAddressing(Register::A);
    let b = RegisterAddressing(Register::B);

    cpu.regs.set_a(0x00);
    cpu.regs.set_b(0x9A);

    ld::<u8>(&mut cpu, &mut bus, &a, &b);

    assert_eq!(cpu.regs.a(), 0x9A);
}

// LDHL tests

#[test]
fn ldhl_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111);
    cpu.regs.set_sp(0xFFFF);
    cpu.regs.set_a(1);

    ldhl(&mut cpu, &mut bus, &reg);

    assert_eq!(cpu.regs.hl(), 0);
    assert_eq!(cpu.regs.f(), 0b0011 << 4);
}

#[test]
fn ldhl_half_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111);
    cpu.regs.set_sp(0xFF);
    cpu.regs.set_a(1);

    ldhl(&mut cpu, &mut bus, &reg);

    assert_eq!(cpu.regs.hl(), 0x0100);
    assert_eq!(cpu.regs.f(), 0b0010 << 4);
}

// LDD tests

#[test]
fn ldd_a_hl() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let a = RegisterAddressing(Register::A);
    let hl = RegisterIndirectAddressing(Register::HL);

    bus.write(0xC001, 0x5E);
    cpu.regs.set_hl(0xC001);
    cpu.regs.set_a(0x00);

    ldd(&mut cpu, &mut bus, &a, &hl);

    assert_eq!(cpu.regs.a(), 0x5E);
    assert_eq!(cpu.regs.hl(), 0xC000);
}

// LDI tests

#[test]
fn ldi_a_hl() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let a = RegisterAddressing(Register::A);
    let hl = RegisterIndirectAddressing(Register::HL);

    bus.write(0xC000, 0x5E);
    cpu.regs.set_hl(0xC000);
    cpu.regs.set_a(0x00);

    ldi(&mut cpu, &mut bus, &a, &hl);

    assert_eq!(cpu.regs.a(), 0x5E);
    assert_eq!(cpu.regs.hl(), 0xC001);
}

// PUSH tests

#[test]
fn push_to_stack() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let bc = RegisterAddressing(Register::BC);

    cpu.regs.set_sp(0xFFF0);
    cpu.regs.set_bc(0xBBCC);

    push(&mut cpu, &mut bus, &bc);

    assert_eq!(bus.read(0xFFEF), 0xBB);
    assert_eq!(bus.read(0xFFEE), 0xCC);
    assert_eq!(cpu.regs.sp(), 0xFFEE);
}

// POP tests

#[test]
fn pop_from_stack() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let bc = RegisterAddressing(Register::BC);

    cpu.push_stack(&mut bus, 0xBBCC);

    pop(&mut cpu, &mut bus, &bc);

    assert_eq!(cpu.regs.bc(), 0xBBCC);
}

