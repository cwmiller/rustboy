use super::Mapper;

pub struct Mbc1 {
    total_ram_banks: usize,
    ram_bank_size: usize,
    ram_enabled: bool,
    ram_data: Vec<u8>,

    // Bit 7: RAM/ROM select. 1 = RAM
    // Bit 6, 5: RAM bank or high bits of ROM bank
    // Bit 4-0: Lower ROM bank bits
    bank: u8
}

impl Mbc1 {
    pub fn new(ram_banks: usize, ram_bank_size: usize) -> Self {
        Mbc1 {
            total_ram_banks: ram_banks,
            ram_bank_size: ram_bank_size,
            ram_enabled: false,
            ram_data: vec![0; (ram_bank_size * ram_banks)],
            bank: 0
        }
    }

    fn rom_bank(&self) -> usize {
        let mut rom_bank = self.bank & 0b1111;

        // If in ROM mode, include the upper bits
        if 0b10000000 & self.bank == 0 {
            rom_bank = rom_bank | (self.bank & 0b01100000)
        }

        // Banks 0x00, 0x20, 0x40, and 0x60 aren't accessible and just access the next bank
        (match rom_bank {
            0x00 | 0x20 | 0x40 |0x60 => rom_bank + 1,
            _ => rom_bank
        }) as usize
    }

    fn ram_bank(&self) -> usize {
        if 0b1000000 & self.bank == 0b10000000 {
            ((self.bank & 0b01100000) >> 4) as usize
        } else {
            0
        }
    }

    fn rom_index(&self, addr: u16) -> usize {
        ((addr as usize) - 0x4000) + (self.rom_bank() * 0x4000)
    }

    fn ram_index(&self, addr: u16) -> usize {
        (addr as usize) + (self.ram_bank() * self.ram_bank_size) - 0xA000
    }
}

impl Mapper for Mbc1 {
    fn read(&self, data: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => {
                // ROM bank 0
                data[addr as usize]
            },
            0x4000...0x7FFF => {
                // Switchable ROM bank
                let index = self.rom_index(addr);
                data[index]
            },
            0xA000...0xBFFF => {
                // Switchable RAM bank
                if self.ram_enabled {
                    self.ram_data[self.ram_index(addr)]
                } else {
                    0
                }
            },
            _ => panic!("Address {:#X} not handled by MBC1")
        }
    }

    fn write(&mut self, data: &Vec<u8>, addr: u16, val: u8) {
        match addr {
            0x0000...0x1FFF => {
                // Writing to this space toggles RAM
                // Any value with A in the lower bits enables it
                self.ram_enabled = (val & 0x0A) == 0x0A;

            },
            0x2000...0x3FFFF => {
                // Writing to this space switches ROM bank lower bits
                self.bank = (self.bank & 0b11100000) | val
            },
            0x4000...0x5FFF => {
                // Writing to this space switches the ROM bank upper bits/RAM bank
                self.bank = (self.bank & 0b10011111) | val
            },
            0x6000...0x7FFF => {
                // Writing to this space toggles RAM/ROM bank mode. 1 = RAM mode
                if val == 1 {
                    self.bank = self.bank | 0b10000000
                } else {
                    self.bank = self.bank & !0b10000000
                }
            },
            0xA000...0xBFFF => {
                // RAM Bank
                if self.ram_enabled {
                    let index = self.ram_index(addr);
                    self.ram_data[index] = val;
                }
            },
             _ => panic!("Address {:#X} not handled by MBC1")
        }
    }
}