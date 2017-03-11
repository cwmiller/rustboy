use super::bus::Bus;
use super::cpu::Lr35902;

pub struct Gameboy {
    pub cpu: Lr35902
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Self {
        let cpu = Lr35902::new(Bus::new(rom));

        Gameboy {
            cpu: cpu
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();
    }

    pub fn step(&mut self, count: usize) {
        for _ in 0..count {
            self.cpu.step();
        }
    }
}