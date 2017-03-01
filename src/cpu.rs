use super::bus::{Bus, Addressable};
use super::byteorder::{ByteOrder, LittleEndian};
use super::registers::*;

enum Condition {
    None,
    Z,
    C,
    Nz,
    Nc
}

pub struct Cpu {
    regs: Registers,
    bus: Bus,
    ime: bool
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            regs: Registers::default(),
            bus: bus,
            ime: false
        }
    }

    pub fn power_on(&mut self) {
        self.regs.a = 0x01;
        self.regs.b = 0x13;
        self.regs.c = 0x00;
        self.regs.d = 0xD8;
        self.regs.e = 0x00;
        self.regs.flags = 0xB0;
        self.regs.h = 0x4D;
        self.regs.l = 0x01;
        self.regs.sp = 0xFFFE;
        self.regs.pc = 0x100;
        self.ime = true;
    }

    pub fn step(&mut self) -> usize {
        let opcode = self.inc_imm_byte();
        self.execute(opcode)
    }

    fn inc_imm_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc + 1;

        byte
    }

    fn inc_imm_word(&mut self) -> u16 {
        let word = &[self.bus.read(self.regs.pc), self.bus.read(self.regs.pc+1)];
        self.regs.pc = self.regs.pc + 2;

        LittleEndian::read_u16(word)
    }

    fn execute(&mut self, opcode: u8) -> usize {
        use super::registers::Register8::*;
        use super::registers::Register16::*;
        
        match opcode {
            0x00 => { self.nop(); 4 },
            0x01 => { self.ld_rr_d16(BC); 12 },
            0x02 => { self.ld_indrr_a(BC); 8 },
            0x03 => { self.inc_rr(BC); 8 },
            0x04 => { self.inc_r(B); 4 },
            0x05 => { self.dec_r(B); 4 },
            0x06 => { self.ld_r_d8(B); 8 },
            0x07 => { self.rlca(); 4 },
            0x08 => { self.ld_a16_sp(); 20 },
            0x09 => { self.add_rr_rr(HL, BC); 8 },
            0x0A => { self.ld_a_indrr(BC); 8 },
            0x0B => { self.dec_rr(BC); 8 },
            0x0C => { self.inc_r(C); 4 },
            0x0D => { self.dec_r(C); 4 },
            0x0E => { self.ld_r_d8(C); 8 },
            0x0F => { self.rrca(); 4 },

            0x10 => { self.stop(); 4 },
            0x11 => { self.ld_rr_d16(DE); 12 },
            0x12 => { self.ld_indrr_a(DE); 8 },
            0x13 => { self.inc_rr(DE); 8 },
            0x14 => { self.inc_r(D); 4 },
            0x15 => { self.dec_r(D); 4 },
            0x16 => { self.ld_r_d8(D); 8 },
            0x17 => { self.rla(); 4 },
            0x18 => { self.jr(Condition::None); 12 },
            0x19 => { self.add_rr_rr(HL, DE); 8 },
            0x1A => { self.ld_a_indrr(DE); 8 },
            0x1B => { self.dec_rr(DE); 8 },
            0x1C => { self.inc_r(E); 4 },
            0x1D => { self.dec_r(E); 4 },
            0x1E => { self.ld_r_d8(E); 8 },
            0x1F => { self.rra(); 4 },

            0x20 => self.jr(Condition::Nz),
            0x21 => { self.ld_rr_d16(HL); 12 },
            0x22 => { self.ldi_hl_a(); 8 },
            0x23 => { self.inc_rr(HL); 8 },
            0x24 => { self.inc_r(H); 4 },
            0x25 => { self.dec_r(H); 4 },
            0x26 => { self.ld_r_d8(H); 8 },
            0x27 => { self.daa(); 4 },
            0x28 => { self.jr(Condition::Z) },
            0x29 => { self.add_rr_rr(HL, HL); 8 },
            0x2A => { self.ldi_a_hl(); 8 },
            0x2B => { self.dec_rr(HL); 8 },
            0x2C => { self.inc_r(L); 4 },
            0x2D => { self.dec_r(L); 4 },
            0x2E => { self.ld_r_d8(L); 8 },
            0x2F => { self.cpl(); 4 },

            0x30 => self.jr(Condition::Nc),
            0x31 => { self.ld_rr_d16(SP); 12 },
            0x32 => { self.ldd_hl_a(); 8 },
            0x33 => { self.inc_rr(SP); 8 },
            0x34 => { self.inc_hl(); 12 },
            0x35 => { self.dec_hl(); 12 },
            0x36 => { self.ld_hl_d8(); 12 },
            0x37 => { self.scf(); 4 },
            0x38 => { self.jr(Condition::C) },
            0x39 => { self.add_rr_rr(HL, SP); 8 },
            0x3A => { self.ldd_a_hl(); 8 },
            0x3B => { self.dec_rr(SP); 8 },
            0x3C => { self.inc_r(A); 4 },
            0x3D => { self.dec_r(A); 4 },
            0x3E => { self.ld_r_d8(A); 8 },
            0x3F => { self.ccf(); 4 },

            0x40 => { self.ld_r_r(B, B); 4 },
            0x41 => { self.ld_r_r(B, C); 4 },
            0x42 => { self.ld_r_r(B, D); 4 },
            0x43 => { self.ld_r_r(B, E); 4 },
            0x44 => { self.ld_r_r(B, H); 4 },
            0x45 => { self.ld_r_r(B, L); 4 },
            0x46 => { self.ld_r_hl(B); 4 },
            0x47 => { self.ld_r_r(B, A); 4 },
            0x48 => { self.ld_r_r(C, B); 4 },
            0x49 => { self.ld_r_r(C, C); 4 },
            0x4A => { self.ld_r_r(C, D); 4 },
            0x4B => { self.ld_r_r(C, E); 4 },
            0x4C => { self.ld_r_r(C, H); 4 },
            0x4D => { self.ld_r_r(C, L); 4 },
            0x4E => { self.ld_r_hl(C); 4 },
            0x4F => { self.ld_r_r(C, A); 4 },

            0x50 => { self.ld_r_r(D, B); 4 },
            0x51 => { self.ld_r_r(D, C); 4 },
            0x52 => { self.ld_r_r(D, D); 4 },
            0x53 => { self.ld_r_r(D, E); 4 },
            0x54 => { self.ld_r_r(D, H); 4 },
            0x55 => { self.ld_r_r(D, L); 4 },
            0x56 => { self.ld_r_hl(D); 4 },
            0x57 => { self.ld_r_r(D, A); 4 },
            0x58 => { self.ld_r_r(E, B); 4 },
            0x59 => { self.ld_r_r(E, C); 4 },
            0x5A => { self.ld_r_r(E, D); 4 },
            0x5B => { self.ld_r_r(E, E); 4 },
            0x5C => { self.ld_r_r(E, H); 4 },
            0x5D => { self.ld_r_r(E, L); 4 },
            0x5E => { self.ld_r_hl(E); 4 },
            0x5F => { self.ld_r_r(E, A); 4 },

            0x60 => { self.ld_r_r(H, B); 4 },
            0x61 => { self.ld_r_r(H, C); 4 },
            0x62 => { self.ld_r_r(H, D); 4 },
            0x63 => { self.ld_r_r(H, E); 4 },
            0x64 => { self.ld_r_r(H, H); 4 },
            0x65 => { self.ld_r_r(H, L); 4 },
            0x66 => { self.ld_r_hl(H); 4 },
            0x67 => { self.ld_r_r(H, A); 4 },
            0x68 => { self.ld_r_r(L, B); 4 },
            0x69 => { self.ld_r_r(L, C); 4 },
            0x6A => { self.ld_r_r(L, D); 4 },
            0x6B => { self.ld_r_r(L, E); 4 },
            0x6C => { self.ld_r_r(L, H); 4 },
            0x6D => { self.ld_r_r(L, L); 4 },
            0x6E => { self.ld_r_hl(L); 4 },
            0x6F => { self.ld_r_r(L, A); 4 },

            0x70 => { self.ld_hl_r(B); 4 },
            0x71 => { self.ld_hl_r(C); 4 },
            0x72 => { self.ld_hl_r(D); 4 },
            0x73 => { self.ld_hl_r(E); 4 },
            0x74 => { self.ld_hl_r(H); 4 },
            0x75 => { self.ld_hl_r(L); 4 },
            0x76 => { self.halt(); 4 },
            0x77 => { self.ld_hl_r(A); 4 },
            0x78 => { self.ld_r_r(A, B); 4 },
            0x79 => { self.ld_r_r(A, C); 4 },
            0x7A => { self.ld_r_r(A, D); 4 },
            0x7B => { self.ld_r_r(A, E); 4 },
            0x7C => { self.ld_r_r(A, H); 4 },
            0x7D => { self.ld_r_r(A, L); 4 },
            0x7E => { self.ld_r_hl(A); 4 },
            0x7F => { self.ld_r_r(A, A); 4 },

            0x80 => { self.add_r(B); 4 },
            0x81 => { self.add_r(C); 4 },
            0x82 => { self.add_r(D); 4 },
            0x83 => { self.add_r(E); 4 },
            0x84 => { self.add_r(H); 4 },
            0x85 => { self.add_r(L); 4 },
            0x86 => { self.add_hl(); 8 },
            0x87 => { self.add_r(A); 4 },
            0x88 => { self.adc_r(B); 4},
            0x89 => { self.adc_r(C); 4 },
            0x8A => { self.adc_r(D); 4 },
            0x8B => { self.adc_r(E); 4 },
            0x8C => { self.adc_r(H); 4 },
            0x8D => { self.adc_r(L); 4 },
            0x8E => { self.adc_hl(); 8 },
            0x8F => { self.adc_r(A); 4 },

            0x90 => { self.sub_r(B); 4 },
            0x91 => { self.sub_r(C); 4 },
            0x92 => { self.sub_r(D); 4 },
            0x93 => { self.sub_r(E); 4 },
            0x94 => { self.sub_r(H); 4 },
            0x95 => { self.sub_r(L); 4 },
            0x96 => { self.sub_hl(); 8 },
            0x97 => { self.sub_r(A); 4 },
            0x98 => { self.sbc_r(B); 4},
            0x99 => { self.sbc_r(C); 4 },
            0x9A => { self.sbc_r(D); 4 },
            0x9B => { self.sbc_r(E); 4 },
            0x9C => { self.sbc_r(H); 4 },
            0x9D => { self.sbc_r(L); 4 },
            0x9E => { self.sbc_hl(); 8 },
            0x9F => { self.sbc_r(A); 4 },

            0xA0 => { self.and_r(B); 4 },
            0xA1 => { self.and_r(C); 4 },
            0xA2 => { self.and_r(D); 4 },
            0xA3 => { self.and_r(E); 4 },
            0xA4 => { self.and_r(H); 4 },
            0xA5 => { self.and_r(L); 4 },
            0xA6 => { self.and_hl(); 8 },
            0xA7 => { self.and_r(A); 4 },
            0xA8 => { self.xor_r(B); 4},
            0xA9 => { self.xor_r(C); 4 },
            0xAA => { self.xor_r(D); 4 },
            0xAB => { self.xor_r(E); 4 },
            0xAC => { self.xor_r(H); 4 },
            0xAD => { self.xor_r(L); 4 },
            0xAE => { self.xor_hl(); 8 },
            0xAF => { self.xor_r(A); 4 },

            0xB0 => { self.or_r(B); 4 },
            0xB1 => { self.or_r(C); 4 },
            0xB2 => { self.or_r(D); 4 },
            0xB3 => { self.or_r(E); 4 },
            0xB4 => { self.or_r(H); 4 },
            0xB5 => { self.or_r(L); 4 },
            0xB6 => { self.or_hl(); 8 },
            0xB7 => { self.or_r(A); 4 },
            0xB8 => { self.cp_r(B); 4},
            0xB9 => { self.cp_r(C); 4 },
            0xBA => { self.cp_r(D); 4 },
            0xBB => { self.cp_r(E); 4 },
            0xBC => { self.cp_r(H); 4 },
            0xBD => { self.cp_r(L); 4 },
            0xBE => { self.cp_hl(); 8 },
            0xBF => { self.cp_r(A); 4 },

            0xC0 => self.ret(Condition::Nz),
            0xC1 => { self.pop(BC); 12 },
            0xC2 => self.jp_a16(Condition::Nz),
            0xC3 => self.jp_a16(Condition::None),
            0xC4 => self.call(Condition::Nz),
            0xC5 => { self.push(BC); 16 },
            0xC6 => { self.add_d8(); 8 },
            0xC7 => { self.rst(0x00); 16 },
            0xC8 => self.ret(Condition::Z),
            0xC9 => self.ret(Condition::None),
            0xCA => self.jp_a16(Condition::Z),
            0xCB => { self.prefix(); 4 },
            0xCC => self.call(Condition::Z),
            0xCD => self.call(Condition::None),
            0xCE => { self.adc_d8(); 8 },
            0xCF => { self.rst(0x08); 16 },

            0xD0 => self.ret(Condition::Nc),
            0xD1 => { self.pop(DE); 12 },
            0xD2 => self.jp_a16(Condition::Nc),
            0xD3 => unimplemented!(),
            0xD4 => self.call(Condition::Nc),
            0xD5 => { self.push(DE); 16 },
            0xD6 => { self.sub_d8(); 8 },
            0xD7 => { self.rst(0x10); 16 },
            0xD8 => self.ret(Condition::C),
            0xD9 => { self.reti(); 16 },
            0xDA => self.jp_a16(Condition::C),
            0xDB => unimplemented!(),
            0xDC => self.call(Condition::C),
            0xDD => unimplemented!(),
            0xDE => { self.sbc_d8(); 8 },
            0xDF => { self.rst(0x18); 16 },

            0xE0 => { self.ldh_a8_a(); 12 },
            0xE1 => { self.pop(HL); 12 },
            0xE2 => { self.ld_c_a(); 8 },
            0xE3 => unimplemented!(),
            0xE4 => unimplemented!(),
            0xE5 => { self.push(HL); 16 },
            0xE6 => { self.and_d8(); 8 },
            0xE7 => { self.rst(0x20); 16 },
            0xE8 => { self.add_sp_r8(); 16 },
            0xE9 => { self.jp_hl(); 4 },
            0xEA => { self.ld_a16_a(); 16 },
            0xEB => unimplemented!(),
            0xEC => unimplemented!(),
            0xED => unimplemented!(),
            0xEE => { self.xor_d8(); 8 },
            0xEF => { self.rst(0x28); 16 },

            0xF0 => { self.ldh_a_a8(); 12 },
            0xF1 => { self.pop(AF); 12 },
            0xF2 => { self.ld_a_c(); 8 },
            0xF3 => { self.di(); 4 },
            0xF4 => unimplemented!(),
            0xF5 => { self.push(AF); 16 },
            0xF6 => { self.or_d8(); 8 },
            0xF7 => { self.rst(0x30); 16 },
            0xF8 => { self.ldhl_sp_r8(); 12 },
            0xF9 => { self.ld_sp_hl(); 8 },
            0xFA => { self.ld_a_a16(); 16 },
            0xFB => { self.ei(); 4 },
            0xFC => unimplemented!(),
            0xFD => unimplemented!(),
            0xFE => { self.cp_d8(); 8 },
            0xFF => { self.rst(0x38); 16 },

            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }

    fn condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::None => true,
            Condition::C => (self.regs.flags & FLAG_C) == FLAG_C,
            Condition::Z => (self.regs.flags & FLAG_Z) == FLAG_Z,
            Condition::Nc => (self.regs.flags & FLAG_C) != FLAG_C,
            Condition::Nz => (self.regs.flags & FLAG_Z) != FLAG_Z
        }
    }

    fn push_stack(&mut self, val: u16) {
        let addr = self.regs.read_u16(&Register16::SP);
        
        self.bus.write(addr - 2, (val & 0x00FF) as u8);
        self.bus.write(addr - 1, ((val >> 8) & 0x00FF) as u8);

        self.regs.write_u16(&Register16::SP, addr - 2);
    }

    fn pop_stack(&mut self) -> u16 {
        let addr = self.regs.read_u16(&Register16::SP);
        let word = &[self.bus.read(addr), self.bus.read(addr + 1)];
        self.regs.pc = self.regs.pc + 2;

        LittleEndian::read_u16(word)
    }

    /* 
     * 8-bit load instructions
     */

    // LD r, d8
    #[inline(always)]
    fn ld_r_d8(&mut self, dest: Register8) {
        let val = self.inc_imm_byte();

        self.regs.write_u8(&dest, val);
    }

    // LD (HL), d8
    #[inline(always)]
    fn ld_hl_d8(&mut self) {
        let val = self.inc_imm_byte();
        let addr = self.regs.read_u16(&Register16::HL);

        self.bus.write(addr, val);
    }

    // LD r, r
    #[inline(always)]
    fn ld_r_r(&mut self, dest: Register8, src: Register8) {
        let val = self.regs.read_u8(&src);

        self.regs.write_u8(&dest, val);
    }

    // LD (rr), A
    #[inline(always)]
    fn ld_indrr_a(&mut self, dest: Register16) {
        let addr = self.regs.read_u16(&dest);
        let val = self.regs.a;

        self.bus.write(addr, val);
    }

    // LD A, (rr)
    #[inline(always)]
    fn ld_a_indrr(&mut self, src: Register16) {
        let addr = self.regs.read_u16(&src);
        let val = self.bus.read(addr);

        self.regs.a = val;
    }

    // LD (HL), r
    #[inline(always)]
    fn ld_hl_r(&mut self, src: Register8) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.regs.read_u8(&src);

        self.bus.write(addr, val);
    }

    // LD r, (HL)
    #[inline(always)]
    fn ld_r_hl(&mut self, dest: Register8) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);

        self.regs.write_u8(&dest, val);
    }

    // LD (HL+), A
    #[inline(always)]
    fn ldi_hl_a(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);

        self.bus.write(addr, self.regs.a);
        self.regs.write_u16(&Register16::HL, addr.wrapping_add(1));
    }

    // LD A, (HL+)
    #[inline(always)]
    fn ldi_a_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);

        self.regs.a = self.bus.read(addr);
        self.regs.write_u16(&Register16::HL, addr.wrapping_add(1));
    }

    // LD (HL-), A
    #[inline(always)]
    fn ldd_hl_a(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);

        self.bus.write(addr, self.regs.a);
        self.regs.write_u16(&Register16::HL, addr.wrapping_sub(1));
    }

    // LD A, (HL-)
    #[inline(always)]
    fn ldd_a_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);

        self.regs.a = self.bus.read(addr);
        self.regs.write_u16(&Register16::HL, addr.wrapping_sub(1));
    }

    // LDH (a8), A
    #[inline(always)]
    fn ldh_a8_a(&mut self) {
        let addr = (self.inc_imm_byte() as u16) + 0xFF00;
        let val = self.regs.a;

        self.bus.write(addr, val);
    }

    // LDH A, (a8)
    #[inline(always)]
    fn ldh_a_a8(&mut self) {
        let addr = (self.inc_imm_byte() as u16) + 0xFF00;

        self.regs.a = self.bus.read(addr);
    }

    // LD (C), A
    #[inline(always)]
    fn ld_c_a(&mut self) {
        let addr = (self.regs.c as u16) + 0xFF00;
        let val = self.regs.a;

        self.bus.write(addr, val);
    }

    // LD A, (C)
    #[inline(always)]
    fn ld_a_c(&mut self) {
        let addr = (self.regs.c as u16) + 0xFF00;

        self.regs.a = self.bus.read(addr);
    }

    /*
     * 8-bit math instructions
     */

    // INC r
    // Affects flags: Z, N, H
    #[inline(always)]
    fn inc_r(&mut self, reg: Register8) {
        let val = self.regs.read_u8(&reg);
        let increased = val.wrapping_add(1);

        self.regs.write_u8(&reg, increased);

        self.regs.flags =
            (if increased == 0 { FLAG_Z } else { 0 })   // Z
            | (((val & 0xF) + 1) & 0x10) << 1           // H
            | self.regs.flags & FLAG_C                  // C
    }

    // INC (HL)
    // Affects flags: Z, N, H
    #[inline(always)]
    fn inc_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);
        let increased = val.wrapping_add(1);

        self.bus.write(addr, increased);

        self.regs.flags =
            (if increased == 0 { FLAG_Z } else { 0 })   // Z
            | (((val & 0xF) + 1) & 0x10) << 1           // H
            | self.regs.flags & FLAG_C                  // C
    }

    // DEC r
    // Affects flags: Z, N, H
    #[inline(always)]
    fn dec_r(&mut self, reg: Register8) {
        let val = self.regs.read_u8(&reg);
        let decreased = val.wrapping_sub(1);

        self.regs.write_u8(&reg, decreased);

        self.regs.flags =
            (if decreased == 0 { FLAG_Z } else { 0 })   // Z
            | FLAG_N                                    // N
            | if val > 0 { FLAG_H } else { 0 }          // H
            | self.regs.flags & FLAG_C                  // C
    }

    // DEC (HL)
    // Affects flags: Z, N, H
    #[inline(always)]
    fn dec_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);
        let decreased = val.wrapping_sub(1);

        self.bus.write(addr, decreased);

        self.regs.flags =
            (if decreased == 0 { FLAG_Z } else { 0 })   // Z
            | FLAG_N                                    // N
            | if val > 0 { FLAG_H } else { 0 }          // H
            | self.regs.flags & FLAG_C                  // C
    }

    // CPL
    // Flags affected: N, H
    #[inline(always)]
    fn cpl(&mut self) {
        self.regs.a = !self.regs.a;
        self.regs.flags = self.regs.flags | FLAG_N | FLAG_H;
    }


    /*
     * 8-bit bit instructions
     */

    // RLCA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rlca(&mut self) {
        let val = self.regs.a;
        let adj = val << 1;
        let carry = (val >> 7) & 0b00000001;

        self.regs.a = adj | carry;

        self.regs.flags =
            (if adj == 0 { FLAG_Z } else { 0 }) // Z
            | carry << 4                        // C
    }

    // RLA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rla(&mut self) {
        let val = self.regs.a;
        let adj = (val << 1) | ((self.regs.flags & FLAG_C) >> 4);

        self.regs.a = adj;

        self.regs.flags =
            (if adj == 0 { FLAG_Z } else { 0 }) // Z
            | (val & 0b10000000) >> 3           // C
    }

    // RRCA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rrca(&mut self) {
        let val = self.regs.a;
        let adj = val >> 1;
        let carry = val & 1;

        self.regs.a = adj | (carry << 7);

        self.regs.flags =
            (if adj == 0 { FLAG_Z } else { 0 }) // Z
            | carry << 4                        // C
    }

    // RRA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rra(&mut self) {
        let val = self.regs.a;
        let adj = (val >> 1) | ((self.regs.flags & FLAG_C) << 3);

        self.regs.a = adj;

        self.regs.flags =
            (if adj == 0 { FLAG_Z } else { 0 }) // Z
            | adj << 4                          // C
    }

    // DAA
    // Flags affected: Z, H, C
    #[inline(always)]
    fn daa(&mut self) {
        println!("DAA unimplemented");
    }

    // SCF
    // Flags affected: N, H, C
    #[inline(always)]
    fn scf(&mut self) {
        self.regs.flags = (self.regs.flags & FLAG_Z) | FLAG_C;
    }

    // CCF
    // Flags affected: N, H, C
    #[inline(always)]
    fn ccf(&mut self) {
        self.regs.flags = (self.regs.flags & FLAG_Z) ^ FLAG_C
    }

    // ADD A, r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn add_r(&mut self, src: Register8) {
        let val = self.regs.a;
        let inc = self.regs.read_u8(&src);
        
        self.add(val, inc);
    }

    // ADD A, (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn add_hl(&mut self) {
        let val = self.regs.a;
        let addr = self.regs.read_u16(&Register16::HL);
        let inc = self.bus.read(addr);

        self.add(val, inc);
    }

    // ADD A, d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn add_d8(&mut self) {
        let val = self.regs.a;
        let inc = self.inc_imm_byte();
        
        self.add(val, inc);
    }

    #[inline(always)]
    fn add(&mut self, val: u8, inc: u8) {
        let sum = val.wrapping_add(inc);

        self.regs.a = sum;

        self.regs.flags =
            if sum == 0 { FLAG_Z } else { 0 }           // Z
            | (((val & 0xF) + (inc & 0xF)) & 0x10) << 1 // H
            | if sum < val { FLAG_C } else { 0 }        // C
    }

    // ADD SP, r8
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn add_sp_r8(&mut self) {
        let current = self.regs.sp;
        let immediate = self.inc_imm_byte() as u16;
        let offset = immediate as i16;

        if offset > 0 {
            self.regs.sp = current.wrapping_add(offset as u16);
        } else {
            self.regs.sp = current.wrapping_sub(offset.abs() as u16);
        }

        // I don't think this is right
        self.regs.flags =
            ((((current & 0xF) + (immediate  & 0xF)) & 0x10) as u8) << 1    // H
            | if current.wrapping_add(immediate) < current                  // C
                { FLAG_C } 
                else { 0 }
    }

    // ADC A, r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn adc_r(&mut self, src: Register8) {
        let val = self.regs.a;
        let inc = self.regs.read_u8(&src);
        
        self.adc(val, inc);
    }

    // ADC A, (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn adc_hl(&mut self) {
        let val = self.regs.a;
        let addr = self.regs.read_u16(&Register16::HL);
        let inc = self.bus.read(addr);

        self.adc(val, inc);
    }

    // ADC A, d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn adc_d8(&mut self) {
        let val = self.regs.a;
        let inc = self.inc_imm_byte();
        
        self.adc(val, inc);
    }

    #[inline(always)]
    fn adc(&mut self, val: u8, inc: u8) {
        let mut sum = val.wrapping_add(inc);

        if (self.regs.flags & FLAG_C) == FLAG_C {
            sum = sum.wrapping_add(1);
        }

        self.regs.a = sum;

        self.regs.flags =
            if sum == 0 { FLAG_Z } else { 0 }           // Z
            | (((val & 0xF) + (inc & 0xF)) & 0x10) << 1 // H
            | if sum < val { FLAG_C } else { 0 }        // C
    }

    // SUB r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sub_r(&mut self, src: Register8) {
        let val = self.regs.a;
        let dec = self.regs.read_u8(&src);
        
        self.sub(val, dec);
    }

    // SUB (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sub_hl(&mut self) {
        let val = self.regs.a;
        let addr = self.regs.read_u16(&Register16::HL);
        let dec = self.bus.read(addr);

        self.sub(val, dec);
    }

    // SUB d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sub_d8(&mut self) {
        let val = self.regs.a;
        let dec = self.inc_imm_byte();
        
        self.sub(val, dec);
    }

    #[inline(always)]
    fn sub(&mut self, val: u8, dec: u8) {
        let diff = val.wrapping_sub(dec);

        self.regs.a = diff;

        self.regs.flags =
            if diff == 0 { FLAG_Z} else { 0 }                       // Z
            | FLAG_N                                                // N
            | if (val & 0xF0) > (dec & 0xF0) { FLAG_H } else { 0 }  // H
            | if diff > val { FLAG_C } else { 0 }                   // C
    }

    // SBC A, r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sbc_r(&mut self, src: Register8) {
        let val = self.regs.a;
        let dec = self.regs.read_u8(&src);
        
        self.sbc(val, dec);
    }

    // SBC A, (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sbc_hl(&mut self) {
        let val = self.regs.a;
        let addr = self.regs.read_u16(&Register16::HL);
        let dec = self.bus.read(addr);

        self.sbc(val, dec);
    }

    // SBC A, d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn sbc_d8(&mut self) {
        let val = self.regs.a;
        let dec = self.inc_imm_byte();
        
        self.sbc(val, dec);
    }

    #[inline(always)]
    fn sbc(&mut self, val: u8, dec: u8) {
        let mut diff = val.wrapping_sub(dec);

        if (self.regs.flags & FLAG_C) == FLAG_C {
            diff = diff.wrapping_sub(1);
        }

        self.regs.a = diff;

        self.regs.flags =
            if diff == 0 { FLAG_Z } else { 0 }                      // Z
            | FLAG_N                                                // N
            | if (val & 0xF0) > (dec & 0xF0) { FLAG_H } else { 0 }  // H
            | if diff > val { FLAG_C } else { 0 }                   // C
    }

    // AND r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn and_r(&mut self, src: Register8) {
        let val = self.regs.read_u8(&src);

        self.and(val);
    }

    // AND (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn and_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);

        self.and(val);
    }

    // AND d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn and_d8(&mut self) {
        let val = self.inc_imm_byte();

        self.and(val);
    }

    #[inline(always)]
    fn and(&mut self, val: u8) {
        let existing = self.regs.a;
        let res = existing & val;

        self.regs.a = res;

        self.regs.flags =
            if res == 0 { FLAG_Z } else { 0 }   // Z
            | FLAG_H                            // H
    }

    // XOR r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn xor_r(&mut self, src: Register8) {
        let val = self.regs.read_u8(&src);

        self.xor(val);
    }

    // XOR (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn xor_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);

        self.xor(val);
    }

    // XOR d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn xor_d8(&mut self) {
        let val = self.inc_imm_byte();

        self.xor(val);
    }

    #[inline(always)]
    fn xor(&mut self, val: u8) {
        let existing = self.regs.a;
        let res = existing ^ val;

        self.regs.a = res;

        self.regs.flags =
            if res == 0 { FLAG_Z } else { 0 }   // Z
    }

    // OR r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn or_r(&mut self, src: Register8) {
        let val = self.regs.read_u8(&src);

        self.or(val);
    }

    // OR (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn or_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);

        self.or(val);
    }

    // OR d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn or_d8(&mut self) {
        let val = self.inc_imm_byte();

        self.or(val);
    }

    #[inline(always)]
    fn or(&mut self, val: u8) {
        let existing = self.regs.a;
        let res = existing | val;

        self.regs.a = res;

        self.regs.flags =
            if res == 0 { FLAG_Z } else { 0 }   // Z
    }

    // CP r
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn cp_r(&mut self, src: Register8) {
        let val = self.regs.read_u8(&src);

        self.cp(val);
    }

    // CP (HL)
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn cp_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        let val = self.bus.read(addr);

        self.cp(val);
    }

    // CP d8
    // Flags affected: Z, N, H, C
    #[inline(always)]
    fn cp_d8(&mut self) {
        let val = self.inc_imm_byte();

        self.cp(val);
    }

    #[inline(always)]
    fn cp(&mut self, val: u8) {
        let existing = self.regs.a;

        self.regs.flags =
            if existing == val { FLAG_Z } else { 0 }                    // Z
            | FLAG_N                                                    // N
            | if (existing & 0xF0) > (val & 0xF0) { FLAG_H } else { 0 } // H
            | if existing < val { FLAG_C } else { 0 }                   // C
    }

    /*
     * 16-bit load instructions
     */

     // LD rr, d16
    #[inline(always)]
    fn ld_rr_d16(&mut self, dest: Register16) {
        let val = self.inc_imm_word();
        self.regs.write_u16(&dest, val);
    }

    // LD (a16), SP
    #[inline(always)]
    fn ld_a16_sp(&mut self) {
        let addr = self.inc_imm_word();
        self.bus.write(addr, (self.regs.sp & 0x00FF) as u8);
        self.bus.write(addr + 1, (self.regs.sp >> 8) as u8);
    }

    // LD (a16), A
    #[inline(always)]
    fn ld_a16_a(&mut self) {
        let addr = self.inc_imm_word();
        self.bus.write(addr, self.regs.a);
    }

    // LD A, (a16)
    #[inline(always)]
    fn ld_a_a16(&mut self) {
        let addr = self.inc_imm_word();
        let val = self.bus.read(addr);

        self.regs.a = val;
    }

    // LDHL SP, r8
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn ldhl_sp_r8(&mut self) {
        let sp = self.regs.sp;
        let immediate = self.inc_imm_byte() as u16;
        let offset = immediate as i16;

        if offset > 0 {
            self.regs.write_u16(&Register16::HL, sp.wrapping_add(offset as u16));
        } else {
            self.regs.write_u16(&Register16::HL, sp.wrapping_sub(offset.abs() as u16));
        }

        // I don't think this is right
        self.regs.flags =
            ((((sp & 0xF) + (immediate  & 0xF)) & 0x10) as u8) << 1 // H
            | if sp.wrapping_add(immediate) < sp                    // C
                { FLAG_C } 
                else { 0 }
    }

    // LD SP, HL
    #[inline(always)]
    fn ld_sp_hl(&mut self) {
        let hl = self.regs.read_u16(&Register16::HL);

        self.regs.write_u16(&Register16::SP, hl);
    }

    // POP rr
    #[inline(always)]
    fn pop(&mut self, dest: Register16) {
        let val = self.pop_stack();

        self.regs.write_u16(&dest, val);
    }

    // PUSH rr
    #[inline(always)]
    fn push(&mut self, src: Register16) {
        let val = self.regs.read_u16(&src);
        
        self.push_stack(val);
    }

    /*
     * 16-bit math instructions
     */

    // INC rr
    #[inline(always)]
    fn inc_rr(&mut self, reg: Register16) {
        let val = self.regs.read_u16(&reg);
        self.regs.write_u16(&reg, val.wrapping_add(1));
    }

    // DEC rr
    #[inline(always)]
    fn dec_rr(&mut self, reg: Register16) {
        let val = self.regs.read_u16(&reg);
        self.regs.write_u16(&reg, val.wrapping_sub(1));
    }

    // ADD rr, rr
    // Affects flags: N, H, C
    #[inline(always)]
    fn add_rr_rr(&mut self, dest: Register16, src: Register16) {
        let src_val = self.regs.read_u16(&src);
        let dest_val = self.regs.read_u16(&dest);

        self.regs.write_u16(&dest, dest_val.wrapping_add(src_val));

        let src_high = ((src_val & 0xF0) >> 8) as u8;
        let dest_high = ((dest_val & 0xF0) >> 8) as u8;

        self.regs.flags = 
            (self.regs.flags & FLAG_Z)                              // Z
            | (((dest_high & 0xF) + (src_high & 0xF)) & 0x10) << 1  // H
            | if dest_val.wrapping_add(src_val) < src_val           // C
                { FLAG_C }
                else { 0 };
    }

    // JR c, r8
    #[inline(always)]
    fn jr(&mut self, condition: Condition) -> usize {
        let mut cycles = 8;

        if self.condition_met(condition) {
            cycles = 12;
            let pc = self.regs.pc;
            let offset = self.inc_imm_byte() as i16;

            if offset > 0 {
                self.regs.pc = pc.wrapping_add(offset as u16);
            } else {
                self.regs.pc = pc.wrapping_sub(offset.abs() as u16);
            }
        }

        cycles
    }

    // RET
    #[inline(always)]
    fn ret(&mut self, condition: Condition) -> usize {
        let mut cycles = 8;

        if self.condition_met(condition) {
            cycles = 20;
            let addr = self.pop_stack();
            self.regs.write_u16(&Register16::PC, addr);
        }

        cycles
    }

    // RETI
    #[inline(always)]
    fn reti(&mut self) {
        let addr = self.pop_stack();
        self.regs.write_u16(&Register16::PC, addr);
        self.ime = true;
    }

    // JP c, a16
    #[inline(always)]
    fn jp_a16(&mut self, condition: Condition) -> usize {
        let mut cycles = 12;

        if self.condition_met(condition) {
            cycles = 16;
            let addr = self.inc_imm_word();
            self.regs.write_u16(&Register16::PC, addr);
        }

        cycles
    }

    // JP (HL)
    #[inline(always)]
    fn jp_hl(&mut self) {
        let addr = self.regs.read_u16(&Register16::HL);
        self.regs.write_u16(&Register16::PC, addr);
    }

    // CALL c, a16
    #[inline(always)]
    fn call(&mut self, condition: Condition) -> usize {
        let mut cycles = 12;

        if self.condition_met(condition) {
            cycles = 24;
            let addr = self.inc_imm_word();
            let pc = self.regs.pc;
           
            self.push_stack(pc);
            self.regs.write_u16(&Register16::PC, addr);
        }

        cycles
    }

    // RST u8
    #[inline(always)]
    fn rst(&mut self, addr: u8) {
        let pc = self.regs.pc;
        self.push_stack(pc);
        self.regs.write_u16(&Register16::PC, addr as u16);
    }


    /*
     * Misc instructions
     */

     // NOP
    #[inline(always)]
    fn nop(&self) {
        // Ahh, doing nothing feels so good!
    }

    // STOP 0
    // TODO: halt cpu/lcd
    #[inline(always)]
    fn stop(&mut self) {
        // This instruction is 2 bytes.
        self.inc_imm_byte();
    }

    // HALT
    #[inline(always)]
    fn halt(&self) {
        println!("HALT unimplemented");
    }

    // PREFIX
    #[inline(always)]
    fn prefix(&self) {
        println!("PREFIX unimplemented");
    }

    // DI
    fn di(&mut self) {
        self.ime = false;
    }

    // ER
    fn ei(&mut self) {
        self.ime = true;
    }
}