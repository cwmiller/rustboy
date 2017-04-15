extern crate byteorder;
#[macro_use] 
extern crate enum_primitive;

mod bus;
mod cartridge;
mod cpu;
mod video;

use std::env;
use std::fs::File;
use std::io::Read;

use bus::Bus;
use cartridge::Cartridge;
use cpu::{Cpu, Interrupt};
use video::{CYCLES_PER_FRAME, StepResult as VideoStepResult};

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut file = File::open(&file_name).unwrap();
    let mut rom_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut rom_data).unwrap();

    let cart = Cartridge::new(rom_data);
    run(cart);
}

fn run(cart: Cartridge) {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(cart);
    let mut cycles = 0;

    cpu.reset();

    loop {
        cycles = cycles + cpu.step(&mut bus);

        let result = bus.video.step(cycles);
        if result == VideoStepResult::InterruptVBlank {
            cpu.interrupt(&mut bus, Interrupt::VBlank);
        }

        // Reset on frame
        if cycles >= CYCLES_PER_FRAME {
            cycles = 0;
        }
    }
}