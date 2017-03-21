use bus::Bus;
use bus::Addressable;
use byteorder::{ByteOrder, LittleEndian};

pub fn disassemble(bus: &Bus, pc: &mut u16, prefixed: &mut bool) -> (Vec<u8>, String) {
    let mut data = Vec::new();
    let opcode = imm_byte(bus, pc, &mut data);

    let instruction = if !*prefixed {
        match opcode {
            0x00 => "NOP".to_string(),
            0x01 => format!("LD BC, {:#X}", imm_word(bus, pc, &mut data)),
            0x02 => "LD (BC), A".to_string(),
            0x03 => "INC BC".to_string(),
            0x04 => "INC B".to_string(),
            0x05 => "DEC B".to_string(),
            0x06 => format!("LD B, {:#X}", imm_byte(bus, pc, &mut data)),
            0x07 => "RLCA".to_string(),
            0x08 => format!("LD ({:#X}), SP", imm_word(bus, pc, &mut data)),
            0x09 => "ADD HL, BC".to_string(),
            0x0A => "LD A (BC)".to_string(),
            0x0B => "DEC BC".to_string(),
            0x0C => "INC C".to_string(),
            0x0D => "DEC C".to_string(),
            0x0E => format!("LD C, {:#X}", imm_byte(bus, pc, &mut data)),
            0x0F => "RRCA".to_string(),

            0x10 => { imm_byte(bus, pc, &mut data); "STOP 0".to_string() },
            0x11 => format!("LD DE, {:#X}", imm_word(bus, pc, &mut data)),
            0x12 => "LD (DE), A".to_string(),
            0x13 => "INC DE".to_string(),
            0x14 => "INC D".to_string(),
            0x15 => "DEC D".to_string(),
            0x16 => format!("LD D, {:#X}", imm_byte(bus, pc, &mut data)),
            0x17 => "RLA".to_string(),
            0x18 => format!("JR {:#X}", imm_byte(bus, pc, &mut data)),
            0x19 => "ADD HL, DE".to_string(),
            0x1A => "LD A (DE)".to_string(),
            0x1B => "DEC DE".to_string(),
            0x1C => "INC E".to_string(),
            0x1D => "DEC E".to_string(),
            0x1E => format!("LD E, {:#X}", imm_byte(bus, pc, &mut data)),
            0x1F => "RRA".to_string(),

            0x20 => format!("JR NZ, {:#X}", imm_byte(bus, pc, &mut data)),
            0x21 => format!("LD HL, {:#X}", imm_word(bus, pc, &mut data)),
            0x22 => "LD (HL+),A".to_string(),
            0x23 => "INC HL".to_string(),
            0x24 => "INC H".to_string(),
            0x25 => "DEC H".to_string(),
            0x26 => format!("LD H, {:#X}", imm_byte(bus, pc, &mut data)),
            0x27 => "DAA".to_string(),
            0x28 => format!("JR Z, {:#X}", imm_byte(bus, pc, &mut data)),
            0x29 => "ADD HL, HL".to_string(),
            0x2A => "LD A (HL+)".to_string(),
            0x2B => "DEC HL".to_string(),
            0x2C => "INC L".to_string(),
            0x2D => "DEC L".to_string(),
            0x2E => format!("LD L, {:#X}", imm_byte(bus, pc, &mut data)),
            0x2F => "CPL".to_string(),

            0x30 => format!("JR NC, {:#X}", imm_byte(bus, pc, &mut data)),
            0x31 => format!("LD SP, {:#X}", imm_word(bus, pc, &mut data)),
            0x32 => "LD (HL-),A".to_string(),
            0x33 => "INC SP".to_string(),
            0x34 => "INC (HL)".to_string(),
            0x35 => "DEC (HL)".to_string(),
            0x36 => format!("LD (HL), {:#X}", imm_byte(bus, pc, &mut data)),
            0x37 => "SCF".to_string(),
            0x38 => format!("JR C, {:#X}", imm_byte(bus, pc, &mut data)),
            0x39 => "ADD HL, SP".to_string(),
            0x3A => "LD A (HL-)".to_string(),
            0x3B => "DEC SP".to_string(),
            0x3C => "INC A".to_string(),
            0x3D => "DEC A".to_string(),
            0x3E => format!("LD A, {:#X}", imm_byte(bus, pc, &mut data)),
            0x3F => "CCF".to_string(),

            0x40...0x47 => format!("LD B, {}", reg_for_opcode(opcode)),
            0x48...0x4F => format!("LD C, {}", reg_for_opcode(opcode)),

            0x50...0x57 => format!("LD D, {}", reg_for_opcode(opcode)),
            0x58...0x5F => format!("LD E, {}", reg_for_opcode(opcode)),

            0x60...0x67 => format!("LD H, {}", reg_for_opcode(opcode)),
            0x68...0x6F => format!("LD L, {}", reg_for_opcode(opcode)),

            0x70...0x75 => format!("LD (HL), {}", reg_for_opcode(opcode)),
            0x76 => "HALT".to_string(),
            0x77...0x7F => format!("LD A, {}", reg_for_opcode(opcode)),

            0x80...0x87 => format!("ADD A, {}", reg_for_opcode(opcode)),
            0x88...0x8F => format!("ADC A, {}", reg_for_opcode(opcode)),

            0x90...0x97 => format!("SUB A, {}", reg_for_opcode(opcode)),
            0x98...0x9F => format!("SBC A, {}", reg_for_opcode(opcode)),

            0xA0...0xA7 => format!("AND A, {}", reg_for_opcode(opcode)),
            0xA8...0xAF => format!("XOR A, {}", reg_for_opcode(opcode)),
            
            0xB0...0xB7 => format!("OR A, {}", reg_for_opcode(opcode)),
            0xB8...0xBF => format!("XP A, {}", reg_for_opcode(opcode)),

            0xC0 => "RET NZ".to_string(),
            0xC1 => "POP BC".to_string(),
            0xC2 => format!("JP NZ, {:#X}", imm_word(bus, pc, &mut data)),
            0xC3 => format!("JP {:#X}", imm_word(bus, pc, &mut data)),
            0xC4 => format!("CALL NZ, {:#X}", imm_word(bus, pc, &mut data)),
            0xC5 => "PUSH BC".to_string(),
            0xC6 => format!("ADD A, {:#X}", imm_byte(bus, pc, &mut data)),
            0xC7 => "RST 0x0".to_string(),
            0xC8 => "RET Z".to_string(),
            0xC9 => "RET".to_string(),
            0xCA => format!("JP Z, {:#X}", imm_word(bus, pc, &mut data)),
            0xCB => { *prefixed = true; "PREFIX CB".to_string() },
            0xCC => format!("CALL Z, {:#X}", imm_word(bus, pc, &mut data)),
            0xCD => format!("CALL {:#X}", imm_word(bus, pc, &mut data)),
            0xCE => format!("ADC A, {:#X}", imm_byte(bus, pc, &mut data)),
            0xCF => "RST 0x8".to_string(),

            0xD0 => "RET NC".to_string(),
            0xD1 => "POP DE".to_string(),
            0xD2 => format!("JP NC, {:#X}", imm_word(bus, pc, &mut data)),
            0xD3 => "".to_string(),
            0xD4 => format!("CALL NC, {:#X}", imm_word(bus, pc, &mut data)),
            0xD5 => "PUSH DE".to_string(),
            0xD6 => format!("SUB {:#X}", imm_byte(bus, pc, &mut data)),
            0xD7 => "RST 0x10".to_string(),
            0xD8 => "RET C".to_string(),
            0xD9 => "RETI".to_string(),
            0xDA => format!("JP C, {:#X}", imm_word(bus, pc, &mut data)),
            0xDB => "".to_string(),
            0xDC => format!("CALL C, {:#X}", imm_word(bus, pc, &mut data)),
            0xDD => "".to_string(),
            0xDE => format!("SBC A, {:#X}", imm_byte(bus, pc, &mut data)),
            0xDF => "RST 0x18".to_string(),

            0xE0 => format!("LDH ({:#X}), A", imm_byte(bus, pc, &mut data)),
            0xE1 => "POP HL".to_string(),
            0xE2 => "LD (C), A".to_string(),
            0xE3 => "".to_string(),
            0xE4 => "".to_string(),
            0xE5 => "PUSH HL".to_string(),
            0xE6 => format!("AND {:#X}", imm_byte(bus, pc, &mut data)),
            0xE7 => "RST 0x20".to_string(),
            0xE8 => format!("ADD SP, {:#X}", imm_byte(bus, pc, &mut data)),
            0xE9 => "JP (HL)".to_string(),
            0xEA => format!("LD ({:#X}), A", imm_word(bus, pc, &mut data)),
            0xEB => "EI".to_string(),
            0xEC => "".to_string(),
            0xED => "".to_string(),
            0xEE => format!("XOR {:#X}", imm_byte(bus, pc, &mut data)),
            0xEF => "RST 0x28".to_string(),

            0xF0 => format!("LDH A, ({:#X})", imm_byte(bus, pc, &mut data)),
            0xF1 => "POP AF".to_string(),
            0xF2 => "LD A, (C)".to_string(),
            0xF3 => "DI".to_string(),
            0xF4 => "".to_string(),
            0xF5 => "PUSH AF".to_string(),
            0xF6 => format!("OR {:#X}", imm_byte(bus, pc, &mut data)),
            0xF7 => "RST 0x30".to_string(),
            0xF8 => format!("LD HL, SP+{:#X}", imm_byte(bus, pc, &mut data)),
            0xF9 => "LD SP, HL".to_string(),
            0xFA => format!("LD A, ({:#X})", imm_word(bus, pc, &mut data)),
            0xFB => "EI".to_string(),
            0xFC => "".to_string(),
            0xFD => "".to_string(),
            0xFE => format!("CP {:#X}", imm_byte(bus, pc, &mut data)),
            0xFF => "RST 0x38".to_string(),

            _ => "".to_string().to_string()
        }
    } else {
        *prefixed = false;
        match opcode {
            0x00...0x07 => format!("RLC {}", reg_for_opcode(opcode)),
            0x00...0x0F => format!("RRC {}", reg_for_opcode(opcode)),

            0x10...0x17 => format!("RL {}", reg_for_opcode(opcode)),
            0x10...0x1F => format!("RR {}", reg_for_opcode(opcode)),

            0x20...0x27 => format!("SLA {}", reg_for_opcode(opcode)),
            0x20...0x2F => format!("SLR {}", reg_for_opcode(opcode)),

            0x30...0x37 => format!("SWAP {}", reg_for_opcode(opcode)),
            0x30...0x3F => format!("SRL {}", reg_for_opcode(opcode)),

            0x40...0x47 => format!("BIT 0, {}", reg_for_opcode(opcode)),
            0x40...0x4F => format!("BIT 1, {}", reg_for_opcode(opcode)),

            0x50...0x57 => format!("BIT 2, {}", reg_for_opcode(opcode)),
            0x50...0x5F => format!("BIT 3, {}", reg_for_opcode(opcode)),

            0x60...0x67 => format!("BIT 4, {}", reg_for_opcode(opcode)),
            0x60...0x6F => format!("BIT 5, {}", reg_for_opcode(opcode)),

            0x70...0x77 => format!("BIT 6, {}", reg_for_opcode(opcode)),
            0x70...0x7F => format!("BIT 7, {}", reg_for_opcode(opcode)),

            0x80...0x87 => format!("RES 0, {}", reg_for_opcode(opcode)),
            0x80...0x8F => format!("RES 1, {}", reg_for_opcode(opcode)),

            0x90...0x97 => format!("RES 2, {}", reg_for_opcode(opcode)),
            0x90...0x9F => format!("RES 3, {}", reg_for_opcode(opcode)),

            0xA0...0xA7 => format!("RES 4, {}", reg_for_opcode(opcode)),
            0xA0...0xAF => format!("RES 5, {}", reg_for_opcode(opcode)),

            0xB0...0xB7 => format!("RES 6, {}", reg_for_opcode(opcode)),
            0xB0...0xBF => format!("RES 7, {}", reg_for_opcode(opcode)),

            0xC0...0xC7 => format!("SET 0, {}", reg_for_opcode(opcode)),
            0xC0...0xCF => format!("SET 1, {}", reg_for_opcode(opcode)),

            0xD0...0xD7 => format!("SET 2, {}", reg_for_opcode(opcode)),
            0xD0...0xDF => format!("SET 3, {}", reg_for_opcode(opcode)),

            0xE0...0xE7 => format!("SET 4, {}", reg_for_opcode(opcode)),
            0xE0...0xEF => format!("SET 5, {}", reg_for_opcode(opcode)),

            0xF0...0xF7 => format!("SET 6, {}", reg_for_opcode(opcode)),
            0xF0...0xFF => format!("SET 7, {}", reg_for_opcode(opcode)),

            _ => "".to_string()
        }
    };

    (data, instruction)
}

fn imm_byte(bus: &Bus, pc: &mut u16, data: &mut Vec<u8>) -> u8 {
    let byte = bus.read(*pc);
    data.push(byte);
    *pc = *pc + 1;

    byte
}

fn imm_word(bus: &Bus, pc: &mut u16, data: &mut Vec<u8>) -> u16 {
    let lb = bus.read(*pc);
    let hb = bus.read(*pc + 1);
    data.push(lb);
    data.push(hb);
    *pc = *pc + 2;

    LittleEndian::read_u16(&[lb, hb])
}

fn reg_for_opcode(opcode: u8) -> &'static str {
    let offset = opcode & 0x0F;

    match offset {
        0x0 | 0x8 => "B",
        0x1 | 0x9 => "C",
        0x2 | 0xA => "D",
        0x3 | 0xB => "E",
        0x4 | 0xC => "H",
        0x5 | 0xD => "L",
        0x6 | 0xE => "(HL)",
        0x7 | 0xF => "A",
        _ => "Unknown"
    }
}