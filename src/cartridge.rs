use bus::Addressable;

pub struct Cartridge {
    data: Vec<u8>
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Cartridge {
        Cartridge {
            data: data
        }
    }
}

impl Addressable for Cartridge {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        //panic!("Cannot write to ROM!")
        // or can they?
    }
}