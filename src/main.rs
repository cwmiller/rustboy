#[macro_use] 
extern crate bitflags;
extern crate byteorder;
#[macro_use] 
extern crate enum_primitive;
extern crate fnv;
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
use joypad::Button;
use lcd::{BUFFER_WIDTH, BUFFER_HEIGHT};
use minifb::{Key, Scale, WindowOptions, Window};
use std::env;
use std::path::Path;
use std::process;
use std::time::Instant;

fn main() {
    if env::args().len() < 2 {
        let bin_arg = env::args().nth(0).unwrap();
        let bin_path = Path::new(bin_arg.as_str());

        println!("Usage: {} [ROM]", bin_path.file_name().unwrap().to_str().unwrap());
        process::exit(1);
    }

    let rom_arg = env::args().nth(1).unwrap();
    let rom_path = Path::new(rom_arg.as_str());

    if !rom_path.is_file()  {
        println!("File {} does not exist.", rom_arg);
        process::exit(1);
    }

    let cart = Cartridge::new(rom_arg.as_str());

    println!("Loaded {}", rom_path.file_name().unwrap().to_str().unwrap());
    println!("{:?}", cart);

    start_emu(cart);
}

fn start_emu(cart: Cartridge) {
    let mut framebuffer = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
    let mut window = Window::new("Rustboy",
                                 BUFFER_WIDTH,
                                 BUFFER_HEIGHT,
                                 WindowOptions {
                                     scale: Scale::X2,
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

    let mut break_pressed = false;
    let mut start_time = Instant::now();
    let mut frames = 0;
    let mut keys = Vec::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if debugger.should_break(&cpu) || break_pressed {
            break_pressed = false;
            debugger.brk(&mut cpu, &mut bus);
        }

        let cycles = cpu.step(&mut bus);
        let timer_result = bus.timer.step(cycles);
        let lcd_result = bus.lcd.step(cycles, &mut framebuffer);
        let joypad_result = bus.joypad.step(&keys);

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

            pressed_keys(&mut keys, &window);

            if window.is_key_down(Key::Pause) {
                break_pressed = true;
            }
        }
    }
}

#[inline(always)]
fn pressed_keys<'a>(keys: &'a mut Vec<Button>, window: &Window) -> &'a Vec<Button> {
    keys.clear();

    if window.is_key_down(Key::Enter) {
        keys.push(Button::Start);
    }

    if window.is_key_down(Key::RightShift) || window.is_key_down(Key::LeftShift) {
        keys.push(Button::Select);
    }

    if window.is_key_down(Key::Up) {
        keys.push(Button::Up);
    }

    if window.is_key_down(Key::Right) {
        keys.push(Button::Right);
    }

    if window.is_key_down(Key::Down) {
        keys.push(Button::Down);
    }

    if window.is_key_down(Key::Left) {
        keys.push(Button::Left);
    }

    if window.is_key_down(Key::Z) {
        keys.push(Button::B);
    }

    if window.is_key_down(Key::X) {
        keys.push(Button::A);
    }

    keys
}