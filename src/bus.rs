const CARTRIDGE_ROM_START: u16 = 0;
const CARTRIDGE_ROM_END: u16 = 0x7FFF;
#[allow(dead_code)]
const CARTRIDGE_ROM_SIZE: usize = 32 * 1024;

const WORK_RAM_START: u16 = 0xC000;
const WORK_RAM_END: u16 = 0xDFFF;
const WORK_RAM_SIZE: usize = 8 * 1024;

pub trait Addressable {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

struct WorkRam {
    data: Vec<u8>
}

impl WorkRam {
    fn new() -> WorkRam {
        WorkRam {
            data: vec![0; WORK_RAM_SIZE]
        }
    }
}

impl Addressable for WorkRam {
    fn read(&self, addr: u16) -> u8 {
        self.data[(addr - WORK_RAM_START) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[(addr - WORK_RAM_START) as usize] = val;
    }
}

struct CartridgeRom {
    data: Vec<u8>
}

impl CartridgeRom {
    pub fn new(data: Vec<u8>) -> CartridgeRom {
        CartridgeRom {
            data: data
        }
    }
}

impl Addressable for CartridgeRom {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }


    fn write(&mut self, _: u16, _: u8) {
        panic!("Cannot write to ROM!")
    }
}

pub struct Bus {
    cartridge_rom: CartridgeRom,
    work_ram: WorkRam
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        Bus {
            cartridge_rom: CartridgeRom::new(rom),
            work_ram: WorkRam::new()
        }
    }
}

impl Addressable for Bus {
    fn read(&self, addr: u16) -> u8 {
        if addr >= CARTRIDGE_ROM_START && addr <= CARTRIDGE_ROM_END {
            self.cartridge_rom.read(addr)
        } else if addr >= WORK_RAM_START && addr <= WORK_RAM_END {
            self.work_ram.read(addr)
        } else {
            0
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        if addr >= CARTRIDGE_ROM_START && addr <= CARTRIDGE_ROM_END {
            self.cartridge_rom.write(addr, val)
        } else if addr >= WORK_RAM_START && addr <= WORK_RAM_END {
            self.work_ram.write(addr, val)
        }
    }
}