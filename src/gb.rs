
//use super::bus;
//use super::cpu;

use super::bus::Bus;
use super::cpu::Cpu;

pub struct Gameboy {
    cpu: Cpu
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Gameboy {
        Gameboy {
            cpu: Cpu::new(Bus::new(rom))
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();
    }
}