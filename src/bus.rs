use cartridge::Cartridge;
use video::{StepResult, Video};

const CARTRIDGE_ROM_START: u16 = 0;
const CARTRIDGE_ROM_END: u16 = 0x7FFF;

const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

const SWITCHABLE_RAM_START: u16 = 0xA000;
const SWITCHABLE_RAM_END: u16 = 0xBFFF;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xDFFF;
const WORK_RAM_SIZE: usize = (WORK_RAM_END as usize) - (WORK_RAM_START as usize)+1;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;

const IO_VIDEO_START: u16 = 0xFF40;
const IO_VIDEO_END: u16 = 0xFF4B;

pub const IO_IF_ADDR: u16 = 0xFF0F;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;
const HIGH_RAM_SIZE: usize = (HIGH_RAM_END as usize) - (HIGH_RAM_START as usize)+1;

pub const IO_IE_ADDR: u16 = 0xFFFF;

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

struct WorkRam {
    data: Vec<u8>
}

impl WorkRam {
    fn new() -> Self {
        WorkRam {
            data: vec![0; WORK_RAM_SIZE]
        }
    }
}

#[inline(always)]
fn work_ram_addr(addr: u16) -> u16 {
    if addr >= ECHO_RAM_START { addr - ECHO_RAM_START } else { addr }
}

impl Addressable for WorkRam {
    fn read(&self, addr: u16) -> u8 {
        self.data[(work_ram_addr(addr) - WORK_RAM_START) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[(work_ram_addr(addr) - WORK_RAM_START) as usize] = val;
    }
}

struct HighRam {
    data: Vec<u8>
}

impl HighRam {
    fn new() -> Self {
        HighRam {
            data: vec![0; HIGH_RAM_SIZE]
        }
    }
}

impl Addressable for HighRam {
    fn read(&self, addr: u16) -> u8 {
        self.data[(addr - HIGH_RAM_START) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[(addr - HIGH_RAM_START) as usize] = val;
    }
}

pub struct Bus {
    cartridge_rom: Cartridge,
    io_ie: u8,
    io_if: u8,
    high_ram: HighRam,
    pub video: Video,
    work_ram: WorkRam,
}

impl Bus {
    pub fn new(cart: Cartridge) -> Self {
        Bus {
            cartridge_rom: cart,
            io_ie: 0,
            io_if: 0,
            high_ram: HighRam::new(),
            video: Video::default(),
            work_ram: WorkRam::new()
        }
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            CARTRIDGE_ROM_START...CARTRIDGE_ROM_END => self.cartridge_rom.read(addr),
            VIDEO_RAM_START...VIDEO_RAM_END => { println!("Video RAM read unimplemented ({:#X})", addr); 0 },
            SWITCHABLE_RAM_START...SWITCHABLE_RAM_END => { println!("Switchable RAM read unimplemented ({:#X})", addr); 0 },
            WORK_RAM_START...WORK_RAM_END 
            | ECHO_RAM_START...ECHO_RAM_END => self.work_ram.read(addr),
            OAM_START...OAM_END => { println!("OAM read unimplemented ({:#X})", addr); 0 },
            IO_VIDEO_START...IO_VIDEO_END => self.video.read(addr),
            IO_IF_ADDR => self.io_if,
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.read(addr),
            IO_IE_ADDR => self.io_ie,
            _ => { println!("Unimplemented read ({:#X})", addr); 0 }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            CARTRIDGE_ROM_START...CARTRIDGE_ROM_END => self.cartridge_rom.write(addr, val),
            VIDEO_RAM_START...VIDEO_RAM_END => println!("Video RAM write unimplemented ({:#X} -> {:#X})", val, addr),
            SWITCHABLE_RAM_START...SWITCHABLE_RAM_END => println!("Switchable RAM write unimplemented ({:#X} -> {:#X})", val, addr),
            WORK_RAM_START...WORK_RAM_END 
            | ECHO_RAM_START...ECHO_RAM_END => self.work_ram.write(addr, val),
            OAM_START...OAM_END => println!("OAM write unimplemented ({:#X} -> {:#X})", val, addr),
            IO_VIDEO_START...IO_VIDEO_END => self.video.write(addr, val),
            IO_IF_ADDR => self.io_if = val,
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.write(addr, val),
            IO_IE_ADDR => self.io_ie = val,
            _ => println!("Unimplemented write ({:#X} -> {:#X})", val, addr)
        }
    }
}