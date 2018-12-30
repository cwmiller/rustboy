use cpu::{Condition, Cpu};
use cpu::addressing::*;
use cpu::instructions::*;
use bus::Bus;
use cartridge::Cartridge;

// JR tests

#[test]
fn jr_addr() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(0xF);

    cpu.regs.set_pc(0xFF0);

    jr(&mut cpu, &mut bus, Condition::None, &addr);

    assert_eq!(cpu.regs.pc(), 0xFFF);
}

#[test]
fn jr_wrapping() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(10);

    cpu.regs.set_pc(0xFFFF);

    jr(&mut cpu, &mut bus, Condition::None, &addr);

    assert_eq!(cpu.regs.pc(), 9);
}

// JP tests

#[test]
fn jp_addr() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(0xFFFF);

    cpu.regs.set_pc(0);

    jp(&mut cpu, &mut bus, Condition::None, &addr);

    assert_eq!(cpu.regs.pc(), 0xFFFF);
}

// CALL tests

#[test]
fn call_addr() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(0xFFFF);

    cpu.regs.set_pc(0xFF);

    call(&mut cpu, &mut bus, Condition::None, &addr);

    assert_eq!(cpu.regs.pc(), 0xFFFF);
    assert_eq!(cpu.pop_stack(&bus), 0xFF);
}

// RET tests

#[test]
fn ret_after_call() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(0xFFFF);

    cpu.regs.set_pc(0xFF);

    call(&mut cpu, &mut bus, Condition::None, &addr);
    ret(&mut cpu, &mut bus, Condition::None);

    assert_eq!(cpu.regs.pc(), 0xFF);
}

// RETI tests

#[test]
fn reti_after_call() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);
    let addr = ImmediateAddressing(0xFFFF);

    cpu.regs.set_pc(0xFF);

    call(&mut cpu, &mut bus, Condition::None, &addr);
    reti(&mut cpu, &mut bus);

    assert_eq!(cpu.regs.pc(), 0xFF);
    assert_eq!(cpu.ime, true);
}

// RST tests

#[test]
fn rst_10h() {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::from_vec(vec![0; 65536]);
    let mut bus = Bus::new(&mut cartridge);

    cpu.regs.set_pc(0xFF);

    rst(&mut cpu, &mut bus, 2);

    assert_eq!(cpu.regs.pc(), 0x10);
}