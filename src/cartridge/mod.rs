mod mbc1;

use bus::Addressable;
use self::mbc1::Mbc1;
use std::fmt;

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
    data: Vec<u8>,
    mapper: Option<Box<Mapper>>
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        Cartridge {
            mapper: create_mapper(data[0x147]),
            data: data
        }
    }

    pub fn name(&self) -> String {
        String::from_utf8((&self.data[0x0134..0x0143]).to_vec()).unwrap_or("UNKNOWN".to_string())
    }

    pub fn gbc(&self) -> bool {
        self.data[0x0143] == 0x80
    }

    pub fn sgb(&self) -> bool {
        self.data[0x0146] == 0x03
    }

    pub fn mapper_type(&self) -> Option<MapperType> {
        match self.data[0x0147] {
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
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Addressable for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        if self.mapper.is_some() {
            let mapper = self.mapper.as_ref().unwrap();
            mapper.read(&self.data, addr)
        } else {
            self.data[addr as usize]
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        if self.mapper.is_some() {
            let mapper = self.mapper.as_mut().unwrap();
            mapper.write(&self.data, addr, val)
        } else {
            panic!("Cannot write to ROM!");
        }
    }
}

trait Mapper {
    fn read(&self, data: &Vec<u8>, addr: u16) -> u8;
    fn write(&mut self, data: &Vec<u8>, addr: u16, val: u8);
}

fn create_mapper(cartridgeType: u8) -> Option<Box<Mapper>> {
    // TODO
    Some(Box::new(Mbc1::new(0, 0)))
}