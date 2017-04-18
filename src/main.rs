extern crate byteorder;
#[macro_use] 
extern crate enum_primitive;
extern crate minifb;

mod bus;
mod cartridge;
mod cpu;
mod lcd;

use bus::Bus;
use cartridge::Cartridge;
use cpu::{Cpu, Interrupt};
use lcd::{CYCLES_PER_FRAME, LCD_WIDTH, LCD_HEIGHT, StepResult as LcdStepResult};
use minifb::{Key, WindowOptions, Window};
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut file = File::open(&file_name).unwrap();
    let mut rom_data: Vec<u8> = Vec::new();
    file.read_to_end(&mut rom_data).unwrap();

    let cart = Cartridge::new(rom_data);

    println!("Loaded {}", cart);
    run(cart);
}

fn run(cart: Cartridge) {
    let mut framebuffer: Vec<u32> = vec![0; LCD_WIDTH * LCD_HEIGHT];
    let mut window = Window::new("Rustboy",
                                 LCD_WIDTH,
                                 LCD_HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    for i in framebuffer.iter_mut() {
        *i = 0; // write something more funny here!
    }

    let mut cpu = Cpu::new();
    let mut bus = Bus::new(cart);
    let mut cycles = 0;

    cpu.reset();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cycles = cycles + cpu.step(&mut bus);

        let result = bus.lcd.step(cycles);
        if result == LcdStepResult::InterruptVBlank {
            cpu.interrupt(&mut bus, Interrupt::VBlank);
        }

        // Reset on frame
        if cycles >= CYCLES_PER_FRAME {
            cycles = 0;
            window.update_with_buffer(&framebuffer);
        }
    }
}