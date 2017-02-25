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

    /*
     * 8-bit math instructions
     */

     // INC r
    #[inline(always)]
    fn inc_r(&mut self, reg: Register8) {
        let val = self.regs.read_u8(&reg);
        self.regs.write_u8(&reg, val.wrapping_add(1));

        // TODO: flags
    }

    #[inline(always)]
    fn dec_r(&mut self, reg: Register8) {
        let val = self.regs.read_u8(&reg);
        self.regs.write_u8(&reg, val.wrapping_sub(1));

        // TODO: flags
    }

    /*
     * 8-bit bit instructions
     */

    // RLCA
    fn rlca(&mut self) {
        self.regs.a = self.regs.a << 1;

        // TODO: flags
    }

    // RRCA
    fn rrca(&mut self) {
        self.regs.a = self.regs.a >> 1;

        // TODO: flags
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
    #[inline(always)]
    fn add_rr_rr(&mut self, dest: Register16, src: Register16) {
        let src_val = self.regs.read_u16(&src);
        let dest_val = self.regs.read_u16(&dest);

        self.regs.write_u16(&dest, dest_val.wrapping_add(src_val));

        // TODO: flags
    }

    /*
     * Misc instructions
     */

     // NOP
    #[inline(always)]
    fn nop(&self) {
        // Ahh, doing nothing feels so good!
    }
}