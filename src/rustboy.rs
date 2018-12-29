use cartridge::Cartridge;

use bus::Bus;
use cpu::{Cpu, Interrupt};
use joypad::Button;
use lcd::{SCREEN_WIDTH, SCREEN_HEIGHT};
use minifb::{Key, Scale, WindowOptions, Window};
use std::time::Instant;

pub struct Rustboy<'a> {
    bus: Bus<'a>,
    cpu: Cpu,
    screen_buffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    window: Window
}

impl<'a> Rustboy<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> Self {
        Self {
            bus: Bus::new(cartridge),
            cpu: Cpu::new(),
            screen_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            window: create_window(Scale::X4)
        }
    }

    pub fn run(&mut self) {
        // Clear the window
        for i in self.screen_buffer.iter_mut() {
            *i = 0xFFFFFF;
        }

        self.window.update_with_buffer(&self.screen_buffer).unwrap();

        // Set the CPU to initial values
        self.cpu.reset();

        // FPS counter variables
        let mut fps_counter_time = Instant::now();
        let mut ftp_counter_frames = 0;

        // A vector containing the keys pressed during a step
        let mut keys = Vec::new();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let cycles = self.cpu.step(&mut self.bus);
            let timer_result = self.bus.timer.step(cycles);
            let lcd_result = self.bus.lcd.step(cycles, &mut self.screen_buffer);
            let joypad_result = self.bus.joypad.step(&keys);

            if timer_result.interrupt {
                self.cpu.interrupt(&mut self.bus, Interrupt::Timer);
            }

            if lcd_result.int_stat {
                self.cpu.interrupt(&mut self.bus, Interrupt::Stat);
            }

            if joypad_result.interrupt {
                self.cpu.interrupt(&mut self.bus, Interrupt::Joypad);
            }

            if lcd_result.int_vblank {
                self.cpu.interrupt(&mut self.bus, Interrupt::VBlank);
                self.window.update_with_buffer(&self.screen_buffer).unwrap();
                ftp_counter_frames += 1;

                let elapsed = fps_counter_time.elapsed();

                if elapsed.as_secs() > 0 {
                    self.window.set_title(format!("Rustboy ({} FPS)", ftp_counter_frames).as_str());
                    fps_counter_time = Instant::now();
                    ftp_counter_frames = 0;
                }

                get_key_presses(&mut keys, &self.window);
            }
        }
    }
}

fn create_window(scale: Scale) -> Window {
    Window::new(
        "Rustboy",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: scale,
            ..WindowOptions::default()
        }).unwrap_or_else(|e| {
            panic!("{}", e);
        })
}

#[inline(always)]
fn get_key_presses<'a>(keys: &'a mut Vec<Button>, window: &Window) {
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
}