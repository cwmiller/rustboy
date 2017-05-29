use cartridge::Cartridge;
use joypad::Joypad;
use lcd::{Lcd, ADDR_DMA};
use serial::Serial;
use sound::Sound;
use timer::Timer;

const CARTRIDGE_ROM_START: u16 = 0;
const CARTRIDGE_ROM_END: u16 = 0x7FFF;

pub const VIDEO_RAM_START: u16 = 0x8000;
pub const VIDEO_RAM_END: u16 = 0x9FFF;

const SWITCHABLE_RAM_START: u16 = 0xA000;
const SWITCHABLE_RAM_END: u16 = 0xBFFF;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xDFFF;
const WORK_RAM_SIZE: usize = (WORK_RAM_END as usize) - (WORK_RAM_START as usize)+1;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;

const UNUSED_START: u16 = 0xFEA0;
const UNUSED_END: u16 = 0xFEFF;

const IO_JOYPAD: u16 = 0xFF00;

const IO_SERIAL_START: u16 = 0xFF01;
const IO_SERIAL_END: u16 = 0xFF02;

const IO_TIMER_START: u16 = 0xFF04;
const IO_TIMER_END: u16 = 0xFF07;

const IO_VIDEO_START: u16 = 0xFF40;
const IO_VIDEO_END: u16 = 0xFF4B;

pub const IO_IF_ADDR: u16 = 0xFF0F;

const IO_SOUND_START: u16 = 0xFF10;
const IO_SOUND_END: u16 = 0xFF3F;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;
const HIGH_RAM_SIZE: usize = (HIGH_RAM_END as usize) - (HIGH_RAM_START as usize)+1;

pub const IO_IE_ADDR: u16 = 0xFFFF;

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

struct Ram {
    start_addr: u16,
    data: Vec<u8>
}

impl Ram {
    fn new(start_addr: u16, size: usize) -> Self {
        Ram {
            start_addr: start_addr,
            data: vec![0; size]
        }
    }
}

impl Addressable for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.data[(addr - self.start_addr) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[(addr - self.start_addr) as usize] = val;
    }
}

pub struct Bus {
    cartridge: Cartridge,
    io_ie: u8,
    io_if: u8,
    high_ram: Ram,
    pub joypad: Joypad,
    pub lcd: Lcd,
    serial: Serial,
    sound: Sound,
    pub timer: Timer,
    work_ram: Ram,
}

impl Bus {
    pub fn new(cart: Cartridge) -> Self {
        Bus {
            cartridge: cart,
            io_ie: 0,
            io_if: 0,
            high_ram: Ram::new(HIGH_RAM_START, HIGH_RAM_SIZE),
            joypad: Joypad::new(),
            lcd: Lcd::new(),
            serial: Serial::default(),
            sound: Sound::default(),
            timer: Timer::new(),
            work_ram: Ram::new(WORK_RAM_START, WORK_RAM_SIZE)
        }
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            // 0x0000 - 0x7FFF Cartridge ROM
            CARTRIDGE_ROM_START...CARTRIDGE_ROM_END => self.cartridge.read(addr),
            // 0x8000 - 0x9FFF Video ROM
            VIDEO_RAM_START...VIDEO_RAM_END => self.lcd.read(addr),
            // 0xA000 - 0xBFFF RAM bank
            SWITCHABLE_RAM_START...SWITCHABLE_RAM_END => self.cartridge.read(addr),
            // 0xC000 - 0xDFFF Work RAM
            WORK_RAM_START...WORK_RAM_END => self.work_ram.read(addr),
            // 0xE000 - 0xFDFF Echo of Work RAM
            ECHO_RAM_START...ECHO_RAM_END => self.work_ram.read(addr - 0x2000),
            // 0xFE00 - 0xFE9F Sprite OAM
            OAM_START...OAM_END => self.lcd.read(addr),
            // 0xFEA0 - 0xFEFF UNUSED
            UNUSED_START...UNUSED_END => 0,
            // 0xFF00 Joypad
            IO_JOYPAD => self.joypad.read(addr),
            // 0xFF01 - 0xFF02 Serial IO ports
            IO_SERIAL_START...IO_SERIAL_END => self.serial.read(addr),
            // 0xFF04 - 0xFF07 Timer IO ports
            IO_TIMER_START...IO_TIMER_END => self.timer.read(addr),
            // 0xFF40 - 0xFE9F Video IO ports
            IO_VIDEO_START...IO_VIDEO_END => self.lcd.read(addr),
            // 0xFF0F IF IO port
            IO_IF_ADDR => self.io_if | 0b11100000,
            // 0xFF10 - 0xFF3F Sound IO ports
            IO_SOUND_START...IO_SOUND_END => self.sound.read(addr),
            // 0xFF80 - 0xFFFE High RAM
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.read(addr),
            // 0xFFFF IE IO port
            IO_IE_ADDR => self.io_ie,
            _ => { println!("Unimplemented read ({:#X})", addr); 0xFF }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // 0x0000 - 0x7FFF Cartridge ROM
            CARTRIDGE_ROM_START...CARTRIDGE_ROM_END => self.cartridge.write(addr, val),
            // 0x8000 - 0x9FFF Video ROM
            VIDEO_RAM_START...VIDEO_RAM_END => self.lcd.write(addr, val),
            // 0xA000 - 0xBFFF RAM bank
            SWITCHABLE_RAM_START...SWITCHABLE_RAM_END => self.cartridge.write(addr, val),
            // 0xC000 - 0xDFFF Work RAM
            WORK_RAM_START...WORK_RAM_END => self.work_ram.write(addr, val),
            // 0xE000 - 0xFDFF Echo of Work RAM
            ECHO_RAM_START...ECHO_RAM_END => self.work_ram.write(addr - 0x2000, val),
            // 0xFE00 - 0xFE9F Sprite OAM
            OAM_START...OAM_END => self.lcd.write(addr, val),
            // 0xFEA0 - 0xFEFF UNUSED
            UNUSED_START...UNUSED_END => { },
            // 0xFF00 Joypad
            IO_JOYPAD => self.joypad.write(addr, val),
            // 0xFF01 - 0xFF02 Serial IO ports
            IO_SERIAL_START...IO_SERIAL_END => self.serial.write(addr, val),
            // 0xFF04 - 0xFF07 Timer IO ports
            IO_TIMER_START...IO_TIMER_END => self.timer.write(addr, val),
            // 0xFF40 - 0xFE9F Video IO ports
            IO_VIDEO_START...IO_VIDEO_END => {
                if addr == ADDR_DMA {
                    for lsb in 0..0xA0 {
                        let src_val = self.read(((val as u16) << 8) | lsb);
                        self.write(0xFE00 | lsb, src_val);
                    }
                    self.lcd.write(addr, val);
                } else {
                    self.lcd.write(addr, val);
                }
            },
            // 0xFF0F IF IO port
            IO_IF_ADDR => self.io_if = val & 0b11111,
            // 0xFF10 - 0xFF3F Sound IO ports
            IO_SOUND_START...IO_SOUND_END => self.sound.write(addr, val),
            // 0xFF80 - 0xFFFE High RAM
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.write(addr, val),
            // 0xFFFF IE IO port
            IO_IE_ADDR => self.io_ie = val,
            _ => println!("Unimplemented write ({:#X} -> {:#X})", val, addr)
        }
    }
}
