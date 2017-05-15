use cpu::Cpu;
use cpu::addressing::*;
use cpu::instructions::*;
use bus::Bus;
use cartridge::Cartridge;

// RLCA tests

#[test]
fn rlca_doesnt_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b01111111);

    rlca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b11111110);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rlca_does_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b11111111);

    rlca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b11111111);
    assert_eq!(cpu.regs.f(), 1 << 4);
}

#[test]
fn rlca_zero_result() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0);

    rlca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b0000 << 4);
}

// RLA tests

#[test]
fn rla_with_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b10000000);

    rla(&mut cpu);

    assert_eq!(cpu.regs.a(), 1);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rla_without_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0b01000000);

    rla(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rla_zero_result() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0b10000000);

    rla(&mut cpu);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

// RRCA tests

#[test]
fn rrca_does_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(1);

    rrca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rrca_doesnt_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b10000000);

    rrca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b01000000);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rrca_zero_result() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0);

    rrca(&mut cpu);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b0000 << 4);
}

// RRA tests

#[test]
fn rra_with_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(1);

    rra(&mut cpu);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rra_without_carry() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0b00000010);

    rra(&mut cpu);

    assert_eq!(cpu.regs.a(), 1);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rra_zero_result() {
    let mut cpu = Cpu::new();
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(1);

    rra(&mut cpu);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

// RLC tests

#[test]
fn rlc_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b10000000);

    rlc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 1);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rlc_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00000001);

    rlc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b00000010);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rlc_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0);

    rlc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// RRC tests

#[test]
fn rrc_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00000001);

    rrc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rrc_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00000010);

    rrc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 1);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rrc_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0);

    rrc(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// RL tests

#[test]
fn rl_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b10000000);

    rl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 1);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rl_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00000001);

    rl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b00000011);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rl_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0);

    rl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// RR tests

#[test]
fn rr_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(1);

    rr(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn rr_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00000010);

    rr(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b10000001);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn rr_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0);

    rr(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// SLA tests

#[test]
fn sla_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b11000000);

    sla(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn sla_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b01000000);

    sla(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b10000000);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn sla_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0b1110 << 4);
    cpu.regs.set_a(0b10000000);

    sla(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1001 << 4);
}

// SRA tests

#[test]
fn sra_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0);
    cpu.regs.set_a(0b10000001);

    sra(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b11000000);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn sra_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(0b10000000);

    sra(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b11000000);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn sra_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(0);

    sra(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// SWAP tests

#[test]
fn swap_not_zero() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(0xF0);

    swap(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0x0F);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn swap_zero() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(0);

    swap(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1000 << 4);
}

// SRL tests

#[test]
fn srl_does_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);

    cpu.regs.set_f(0);
    cpu.regs.set_a(0b00000011);

    srl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b00000001);
    assert_eq!(cpu.regs.f(), 0b0001 << 4);
}

#[test]
fn srl_doesnt_carry() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(0b00000010);

    srl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0b00000001);
    assert_eq!(cpu.regs.f(), 0);
}

#[test]
fn srl_zero_result() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let mut val = RegisterAddressing(Register::A);
    cpu.regs.set_f(0);
    cpu.regs.set_a(1);

    srl(&mut cpu, &mut bus, &mut val);

    assert_eq!(cpu.regs.a(), 0);
    assert_eq!(cpu.regs.f(), 0b1001 << 4);
}

// BIT tests

#[test]
fn bit_zero() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b11101111);

    bit(&mut cpu, &mut bus, 4, &reg);

    assert_eq!(cpu.regs.f(), 0b1011 << 4);
}

#[test]
fn bit_not_zero() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_f(0b1111 << 4);
    cpu.regs.set_a(0b00010000);

    bit(&mut cpu, &mut bus, 4, &reg);

    assert_eq!(cpu.regs.f(), 0b0011 << 4);
}

// RES tests

#[test]
fn res_bit() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_a(255);

    res(&mut cpu, &mut bus, 4, &reg);

    assert_eq!(cpu.regs.a(), 0b11101111);
}

// SET tests

#[test]
fn set_bit() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(Cartridge::from_vec(vec![0; 65536]));
    let reg = RegisterAddressing(Register::A);

    cpu.regs.set_a(0);

    set(&mut cpu, &mut bus, 4, &reg);

    assert_eq!(cpu.regs.a(), 0b00010000);
}