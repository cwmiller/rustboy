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
mod joypad;
mod lcd;
mod serial;
mod sound;
mod timer;

use bus::Bus;
use cartridge::Cartridge;
use cpu::{Cpu, Interrupt};
use debugger::Debugger;
use lcd::{BUFFER_WIDTH, BUFFER_HEIGHT};
use minifb::{Key, Scale, WindowOptions, Window};
use std::env;
use std::process;
use std::time::Instant;

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
    let mut framebuffer = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
    let mut window = Window::new("Rustboy",
                                 BUFFER_WIDTH,
                                 BUFFER_HEIGHT,
                                 WindowOptions {
                                     scale: Scale::X1,
                                     ..WindowOptions::default()
                                 }).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    for i in framebuffer.iter_mut() {
        *i = 0xFFFFFF;
    }
    window.update_with_buffer(&framebuffer);

    let mut cpu = Cpu::new();
    let mut bus = Bus::new(cart);
    let mut debugger = Debugger::new();

    cpu.reset();

    // Used to debounce pause key
    let mut break_pressed = false;
    let mut start_time = Instant::now();
    let mut frames = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if debugger.should_break(&cpu) || (window.is_key_down(Key::Pause) && !break_pressed) {
            break_pressed = true;
            debugger.brk(&mut cpu, &mut bus);
        }

        if !window.is_key_down(Key::Pause) {
            break_pressed = false;
        }

        let cycles = cpu.step(&mut bus);
        let timer_result = bus.timer.step(cycles);
        let lcd_result = bus.lcd.step(cycles, &mut framebuffer);

        // window.get_keys() causes major performance issues
        // let keys = window.get_keys();
        let keys = pressed_keys(&window);
        let joypad_result = bus.joypad.step(keys);

        if timer_result.interrupt {
            cpu.interrupt(&mut bus, Interrupt::Timer);
        }

        if lcd_result.int_stat {
            cpu.interrupt(&mut bus, Interrupt::Stat);
        }

        if joypad_result.interrupt {
            cpu.interrupt(&mut bus, Interrupt::Joypad);
        }

        if lcd_result.int_vblank {
            cpu.interrupt(&mut bus, Interrupt::VBlank);
            window.update_with_buffer(&framebuffer);
            frames = frames + 1;

            let elapsed = start_time.elapsed();
            if elapsed.as_secs() > 0 {
                window.set_title(format!("Rustboy ({} FPS)", frames).as_str());
                start_time = Instant::now();
                frames = 0;
            }
        }
    }
}

fn pressed_keys(window: &Window) -> Vec<Key> {
    let mut keys = Vec::new();

    if window.is_key_down(Key::Enter) {
        keys.push(Key::Enter);
    }

    if window.is_key_down(Key::RightShift) || window.is_key_down(Key::LeftShift) {
        keys.push(Key::RightShift);
    }

    if window.is_key_down(Key::Up) {
        keys.push(Key::Up);
    }

    if window.is_key_down(Key::Right) {
        keys.push(Key::Right);
    }

    if window.is_key_down(Key::Down) {
        keys.push(Key::Down);
    }

    if window.is_key_down(Key::Left) {
        keys.push(Key::Left);
    }

    if window.is_key_down(Key::Z) {
        keys.push(Key::Z);
    }

    if window.is_key_down(Key::X) {
        keys.push(Key::X);
    }

    keys
}