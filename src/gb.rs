use super::bus::Bus;
use super::cpu::Cpu;

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu
}

impl Gameboy {
    pub fn new(rom: Vec<u8>) -> Self {
        let bus = Bus::new(rom);
        let cpu = Cpu::new();

        Gameboy {
            bus: bus,
            cpu: cpu
        }
    }

    pub fn power_on(&mut self) {
        self.cpu.power_on();

        loop {
            self.cpu.step(&mut self.bus);
        }
    }
}