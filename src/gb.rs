use super::bus::Bus;
use super::cpu::Lr35902;

pub struct Gameboy {
    cpu: Lr35902
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Gameboy {
        Gameboy {
            cpu: Lr35902::new(Bus::new(rom))
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();

        loop {
            self.cpu.step();
        }
    }
}