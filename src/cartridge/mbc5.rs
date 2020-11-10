use super::Mapper;
use log::warn;

// Bank selection uses 16 bits to store all bank state
// Upper 4 bits store current RAM bank
// Lower 9 bits store current ROM bank

struct BankSelection(u16);

impl BankSelection {
    pub fn new() -> Self {
        Self(1)
    }

    // ROM Bank uses 9 bits, this sets the top bit
    fn set_rom_upper(&mut self, val: u8) {
        self.0 = (self.0 & 0b1111_0000_1111_1111) | (((val as u16) << 8) & 0b1_0000_0000);
    }

    fn rom_upper(&self) -> u8 {
        ((self.0 & 0b01_0000_0000) >> 8) as u8
    }

    // ROM Bank uses 9 bits, this sets the bottom 8 bits
    fn set_rom_lower(&mut self, val: u8) {
        self.0 = (self.0 & 0b1111_0001_0000_0000) | ((val as u16) & 0x00FF);
    }

    fn rom_lower(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    fn set_ram(&mut self, val: u8) {
        self.0 = (self.0 & 0b1111_0001_1111_1111) | ((val & 0x0F) as u16) << 12;
    }

    fn rom_bank(&self) -> usize {
        let upper = (self.rom_upper() as usize) << 8;
        let lower = self.rom_lower() as usize;

        upper | lower
    }

    fn ram_bank(&self) -> usize {
        ((self.0 & 0xF000) >> 12) as usize
    }
}

pub struct Mbc5 {
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    ram_data: Vec<u8>,

    bank_selection: BankSelection
}

impl Mbc5 {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            ram_enabled: false,
            ram_data: vec![0; ram_banks * 0x2000],
            bank_selection: BankSelection::new()
        }
    }

    fn rom_index(&self, addr: u16) -> usize {
        let bank = if self.rom_banks == 0 {
            0
        } else {
            self.bank_selection.rom_bank() & (self.rom_banks - 1)
        };

        ((addr as usize) - 0x4000) + (bank * 0x4000)
    }

    fn ram_index(&self, addr: u16) -> usize {
        let bank = if self.ram_banks == 0 {
            0
        } else {
            self.bank_selection.ram_bank() & (self.ram_banks - 1)
        };
        
        ((addr as usize) - 0xA000) + (bank * 0x2000)
    }
}

impl Mapper for Mbc5 {
    fn read(&self, rom: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                // ROM bank 0
                rom[addr as usize]
            },
            0x4000..=0x7FFF => {
                // Switchable ROM bank
                rom[self.rom_index(addr)]
            },
            0xA000..=0xBFFF => {
                // Switchable RAM bank
                if self.ram_enabled {
                    self.ram_data[self.ram_index(addr)]
                } else {
                    0xFF
                }
            },
            _ => {
                warn!("Read from address {:#X} not handled by MBC5", addr);

                0xFF
            }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // Writing to this space toggles RAM
                // Any value with A in the lower bits enables it
                self.ram_enabled = (val & 0x0A) == 0x0A;
            },
            0x2000..=0x2FFF => {
                // Writing to this space switches ROM bank lower bits
                self.bank_selection.set_rom_lower(val);
            },
            0x3000..=0x3FFF  => {
                // Writing to this space switches ROM bank upper bits
                self.bank_selection.set_rom_upper(val);
            },
            0x4000..=0x5FFF => {
                // Writing to this space switches RAM bank
                self.bank_selection.set_ram(val);
            },

            0xA000..=0xBFFF => {
                // Write to RAM Bank
                if self.ram_enabled {
                    let index = self.ram_index(addr);
                    if index >= self.ram_data.len() {
                        warn!("Attempted to write to out-of-bounds MBC5 RAM index {}, addr {:X}.", index, addr);
                    } else {
                        self.ram_data[index] = val;
                    }
                }
            },
             //_ => panic!("Address {:#X} not handled by MBC5", addr)
             _ => warn!("Write to address {:#X} not handled by MBC5", addr)
        }
    }
}