mod mbc1;

use bus::Addressable;
use self::mbc1::Mbc1;
use std::fmt;
use std::fs::File;
use std::io::Read;

pub enum MapperType {
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mmm01,
    Huc1
}

impl fmt::Display for MapperType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MapperType::Mbc1 => write!(f, "MBC1"),
            MapperType::Mbc2 => write!(f, "MBC2"),
            MapperType::Mbc3 => write!(f, "MBC3"),
            MapperType::Mbc5 => write!(f, "MBC5"),
            MapperType::Mmm01 => write!(f, "MMM01"),
            MapperType::Huc1 => write!(f, "HuC-1")
        }
    }
}

pub struct Cartridge {
    rom: Vec<u8>,
    mapper: Option<Box<Mapper>>
}

impl Cartridge {
    pub fn new(path: &str) -> Self {
        let mut rom = File::open(path).unwrap();
        let mut rom_data: Vec<u8> = Vec::new();
        rom.read_to_end(&mut rom_data).unwrap();

        Self::from_vec(rom_data)
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        let mapper = if let Some(mapper_type) = mapper_type(data[0x147]) {
            Some(create_mapper(mapper_type, total_rom_banks(data[0x148]), total_ram_banks(data[0x149])))
        } else {
            None
        };

        Self {
            mapper: mapper,
            rom: data
        }
    }

    pub fn name(&self) -> String {
        String::from_utf8((&self.rom[0x0134..0x0143]).to_vec()).unwrap_or("UNKNOWN".to_string())
    }

    pub fn gbc(&self) -> bool {
        self.rom[0x0143] == 0x80
    }

    pub fn sgb(&self) -> bool {
        self.rom[0x0146] == 0x03
    }

    pub fn mapper_type(&self) -> Option<MapperType> {
        mapper_type(self.rom[0x147])
    }

    pub fn total_rom_banks(&self) -> usize {
        total_rom_banks(self.rom[0x148])
    }

    pub fn total_ram_banks(&self) -> usize {
        total_ram_banks(self.rom[0x149])
    }
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mapper = self.mapper_type();

        write!(f, "Name: {}\nMapper: {} ({:#X})\nROM Banks: {} ({:#X})\nRAM Banks: {} ({:#X})\nGBC: {}\nSGB: {}",
            self.name(),
            if mapper.is_some() { mapper.unwrap().to_string() } else { "None".to_string() },
            self.rom[0x147],
            self.total_rom_banks(),
            self.rom[0x148],
            self.rom[0x149],
            self.total_ram_banks(),
            if self.gbc() { "Yes" } else { "No" },
            if self.sgb() { "Yes" } else { "No" })
    }
}

impl Addressable for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        if self.mapper.is_some() {
            let mapper = self.mapper.as_ref().unwrap();
            mapper.read(&self.rom, addr)
        } else {
            self.rom[addr as usize]
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        if self.mapper.is_some() {
            let mapper = self.mapper.as_mut().unwrap();
            mapper.write(addr, val)
        }
    }
}

trait Mapper {
    fn read(&self, rom: &Vec<u8>, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

fn mapper_type(code: u8) -> Option<MapperType> {
    match code {
        0x01 => Some(MapperType::Mbc1),
        0x02 => Some(MapperType::Mbc1),
        0x03 => Some(MapperType::Mbc1),
        0x05 => Some(MapperType::Mbc2),
        0x06 => Some(MapperType::Mbc2),
        0x0B => Some(MapperType::Mmm01),
        0x0C => Some(MapperType::Mmm01),
        0x0D => Some(MapperType::Mmm01),
        0x0F => Some(MapperType::Mbc3),
        0x10 => Some(MapperType::Mbc3),
        0x11 => Some(MapperType::Mbc3),
        0x12 => Some(MapperType::Mbc3),
        0x13 => Some(MapperType::Mbc3),
        0x19 => Some(MapperType::Mbc5),
        0x1A => Some(MapperType::Mbc5),
        0x1B => Some(MapperType::Mbc5),
        0x1C => Some(MapperType::Mbc5),
        0x1D => Some(MapperType::Mbc5),
        0x1E => Some(MapperType::Mbc5),
        0xFF => Some(MapperType::Huc1),
        _ => None
    }
}

fn total_rom_banks(code: u8) -> usize {
    match code {
        0x00 => 0,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x08 => 512,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => panic!("Unknown ROM size: {:#X}", code)
    }
}

fn total_ram_banks(code: u8) -> usize {
    match code  {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,
        _ => panic!("Unknown RAM size: {:#X}", code)
    }
}

fn create_mapper(mapper_type: MapperType, rom_banks: usize, ram_banks: usize) -> Box<Mapper> {
    match mapper_type {
        MapperType::Mbc1 => Box::new(Mbc1::new(rom_banks, ram_banks)),
        _ => panic!("Mapper {} not implemented.", mapper_type)
    }
}