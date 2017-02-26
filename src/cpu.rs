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
    bus: Bus
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            regs: Registers::default(),
            bus: bus
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

            _ => panic!("Unknown opcode: {:#X}", opcode)
        }
    }

    fn condition_met(&self, cond: Condition) -> bool {
        match cond {
            Condition::None => true,
            Condition::C => self.regs.is_flag_set(Flag::C),
            Condition::Z => self.regs.is_flag_set(Flag::Z),
            Condition::Nc => !self.regs.is_flag_set(Flag::C),
            Condition::Nz => !self.regs.is_flag_set(Flag::Z)
        }
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
            (if increased == 0 { 1 << 7 } else { 0 })   // Z
            | (((val & 0xF) + 1) & 0b00010000) << 1     // H
            | self.regs.flags & (Flag::C as u8)         // C

    }

    // DEC r
    // Affects flags: Z, N, H
    #[inline(always)]
    fn dec_r(&mut self, reg: Register8) {
        let val = self.regs.read_u8(&reg);
        let decreased = val.wrapping_sub(1);

        self.regs.write_u8(&reg, decreased);

        self.regs.flags =
            (if decreased == 0 { 1 << 7 } else { 0 })                   // Z
            | Flag::N as u8                                             // N
            | if (((val & 0xF0) - 1) & 0b00001000) == 0b00001000        // H
                { 1 << 6 }
                else { 0 }
            | self.regs.flags & (Flag::C as u8)                         // C
    }

    // CPL
    // Flags affected: N, H
    #[inline(always)]
    fn cpl(&mut self) {
        self.regs.a = !self.regs.a;

        self.regs.flags =
            self.regs.flags
            | (Flag::N as u8)
            | (Flag::H as u8)
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
            (if adj == 0 { 1 << 7 } else { 0 }) // Z
            | carry << 4          // C
    }

    // RLA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rla(&mut self) {
        let val = self.regs.a;
        let adj = (val << 1) | ((self.regs.flags & Flag::C as u8) >> 4);

        self.regs.a = adj;

        self.regs.flags =
            (if adj == 0 { 1 << 7 } else { 0 }) // Z
            | (val & 0b10000000) >> 3           // C
    }

    // RRCA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rrca(&mut self) {
        let val = self.regs.a;
        let adj = val >> 1;
        let carry = val & 0b00000001;

        self.regs.a = adj | (carry << 7);

        self.regs.flags =
            (if adj == 0 { 1 << 7 } else { 0 }) // Z
            | carry << 4                        // C
    }

    // RRA
    // Affects flags: Z, N, H, C
    #[inline(always)]
    fn rra(&mut self) {
        let val = self.regs.a;
        let adj = (val >> 1) | ((self.regs.flags & Flag::C as u8) << 3);

        self.regs.a = adj;

        self.regs.flags =
            (if adj == 0 { 1 << 7 } else { 0 }) // Z
            | adj << 4                          // C
    }

    // DAA
    // Flags affected: Z, H, C
    #[inline(always)]
    fn daa (&mut self) {
        // TODO
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
            (self.regs.flags & Flag::Z as u8) // Z
            | (((dest_high & 0xF) + (src_high & 0xF)) & 0b00010000) << 1    // H
            | if dest_val.wrapping_add(src_val) < src_val                   // C
                { Flag::C as u8 }
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
}