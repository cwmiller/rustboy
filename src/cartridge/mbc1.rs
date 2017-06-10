use enum_primitive::FromPrimitive;
use super::Mapper;

struct BankSelection(u8);

impl BankSelection {
    fn set_mode(&mut self, mode: BankMode) {
        self.0 = (self.0 & 0b0111_1111) | ((mode as u8) << 7)
    }

    fn mode(&self) -> BankMode {
        BankMode::from_u8((self.0 & 0b1000_0000) >> 7).unwrap()
    }

    fn set_upper(&mut self, val: u8) {
        self.0 = (self.0 & 0b1001_1111) | ((val << 5) & 0b0110_0000);
    }

    fn upper(&self) -> u8 {
        (self.0 & 0b0110_0000) >> 5
    }

    fn set_lower(&mut self, val: u8) {
        self.0 = (self.0 & 0b1110_0000) | (val & 0b0001_1111);
    }

    fn lower(&self) -> u8 {
        self.0 & 0b0001_1111
    }

    fn rom_bank(&self) -> usize {
        let bank = if self.mode() == BankMode::Rom {
            (self.upper() << 5) | self.lower()
        } else {
            self.lower()
        };

        (match bank {
            0 | 0x20 | 0x40 | 0x60 => bank + 1,
            _ => bank
        }) as usize
    }

    fn ram_bank(&self) -> usize {
        if self.mode() == BankMode::Ram {
            self.upper() as usize
        } else {
            0
        }
    }
}

enum_from_primitive! {
#[derive(PartialEq)]
enum BankMode {
    Rom = 0,
    Ram = 1
}
}

pub struct Mbc1 {
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    ram_data: Vec<u8>,
    bank_selection: BankSelection
}

impl Mbc1 {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Mbc1 {
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            ram_enabled: false,
            ram_data: vec![0; (ram_banks * 0x2000)],
            bank_selection: BankSelection(0)
        }
    }

    fn rom_index(&self, addr: u16) -> usize {
        let bank = self.bank_selection.rom_bank() & (self.rom_banks - 1);
        ((addr as usize) - 0x4000) + (bank * 0x4000)
    }

    fn ram_index(&self, addr: u16) -> usize {
        let bank = self.bank_selection.ram_bank() & (self.ram_banks - 1);

        ((addr as usize) - 0xA000) + (bank * 0x2000)
    }
}

impl Mapper for Mbc1 {
    fn read(&self, rom: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => {
                // ROM bank 0
                rom[addr as usize]
            },
            0x4000...0x7FFF => {
                // Switchable ROM bank
                rom[self.rom_index(addr)]
            },
            0xA000...0xBFFF => {
                // Switchable RAM bank
                if self.ram_enabled {
                    self.ram_data[self.ram_index(addr)]
                } else {
                    0xFF
                }
            },
            _ => panic!("Address {:#X} not handled by MBC1")
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x1FFF => {
                // Writing to this space toggles RAM
                // Any value with A in the lower bits enables it
                self.ram_enabled = (val & 0x0A) == 0x0A;
            },
            0x2000...0x3FFF => {
                // Writing to this space switches ROM bank lower bits
                self.bank_selection.set_lower(val);
            },
            0x4000...0x5FFF => {
                // Writing to this space switches the ROM bank upper bits/RAM bank
                self.bank_selection.set_upper(val);
            },
            0x6000...0x7FFF => {
                // Writing to this space toggles RAM/ROM bank mode. 1 = RAM mode
                self.bank_selection.set_mode(BankMode::from_u8(val & 1).unwrap_or_else(|| panic!("Unknown bank mode: {:#X}", val)));
            },
            0xA000...0xBFFF => {
                // Write to RAM Bank
                if self.ram_enabled {
                    let index = self.ram_index(addr);
                    self.ram_data[index] = val;
                }
            },
             _ => panic!("Address {:#X} not handled by MBC1")
        }
    }
}