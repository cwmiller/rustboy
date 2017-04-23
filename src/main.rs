#[macro_use] 
extern crate bitflags;
extern crate byteorder;
#[macro_use] 
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate minifb;
extern crate regex;

mod bus;
mod cartridge;
mod debugger;
mod cpu;
mod lcd;

use bus::Bus;
use cartridge::Cartridge;
use cpu::{Cpu, Interrupt};
use debugger::Debugger;
use lcd::{LCD_WIDTH, LCD_HEIGHT};
use minifb::{Key, WindowOptions, Window};
use std::env;
use std::process;

fn main() {
    if env::args().len() < 2 {
        println!("Usage: {} [ROM]", env::args().nth(0).unwrap());
        process::exit(1);
    } 

    let rom_path = env::args().nth(1).unwrap();
    let cart = Cartridge::new(rom_path);
    println!("Loaded {:?}", cart);

    start_emu(cart);
}

fn start_emu(cart: Cartridge) {
    let mut framebuffer: Vec<u32> = vec![0; LCD_WIDTH * LCD_HEIGHT];
    let mut window = Window::new("Rustboy",
                                 LCD_WIDTH,
                                 LCD_HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    for i in framebuffer.iter_mut() {
        *i = 0;
    }
    window.update_with_buffer(&framebuffer);

    let mut cpu = Cpu::new();
    let mut bus = Bus::new(cart);
    let mut debugger = Debugger::new();

    cpu.reset();

    // Used to debounce pause key
    let mut break_pressed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if debugger.should_break(&cpu) || (window.is_key_down(Key::Pause) && !break_pressed) {
            break_pressed = true;
            debugger.brk(&mut cpu, &mut bus);
        }

        if !window.is_key_down(Key::Pause) {
            break_pressed = false;
        }

        let cycles = cpu.step(&mut bus);
        let result = bus.lcd.step(cycles);

        if result.int_vblank {
            cpu.interrupt(&mut bus, Interrupt::VBlank);
            window.update_with_buffer(&framebuffer);
        }

        if result.int_stat {
            cpu.interrupt(&mut bus, Interrupt::Stat);
        }
    }
}