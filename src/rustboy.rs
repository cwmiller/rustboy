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

        self.window.update_with_buffer(&self.screen_buffer).expect("Unable to render window");

        // Set the CPU to initial values
        self.cpu.reset();

        // FPS counter variables
        let mut fps_counter_time = Instant::now();
        let mut fps_counter_frames = 0;

        let mut buttons = Button::empty();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Execute the next CPU instruction. The number of cycles used is returned.
            let cycles = self.cpu.step(&mut self.bus);

            // Step timer
            let timer_result = self.bus.timer.step(cycles);

            // Step LCD
            let lcd_result = self.bus.lcd.step(cycles, &mut self.screen_buffer);

            // Step joypad
            let joypad_result = self.bus.joypad.step(buttons);

            // The timer interrupts when the counter reaches its goal
            if timer_result.interrupt {
                self.cpu.interrupt(&mut self.bus, Interrupt::Timer);
            }

            // LCD can generate a STAT interrupt when modes change or when the cursor reaches a specific line
            if lcd_result.int_stat {
                self.cpu.interrupt(&mut self.bus, Interrupt::Stat);
            }

            // Joypad interrupts when a button is pressed
            if joypad_result.interrupt {
                self.cpu.interrupt(&mut self.bus, Interrupt::Joypad);
            }

            // LCD interrupts when VLANK is reached
            // We'll use this time to update the framebuffer and FPS counter. Also we'll get the current pressed buttons
            if lcd_result.int_vblank {
                self.cpu.interrupt(&mut self.bus, Interrupt::VBlank);
                self.window.update_with_buffer(&self.screen_buffer).unwrap();

                fps_counter_frames += 1;

                // If a second has passed, update the FPS counter
                let elapsed = fps_counter_time.elapsed();

                if elapsed.as_secs() > 0 {
                    self.window.set_title(format!("Rustboy ({} FPS)", fps_counter_frames).as_str());
                    fps_counter_time = Instant::now();
                    fps_counter_frames = 0;
                }

                self.set_button_presses(&mut buttons);
            }
        }
    }

    fn set_button_presses(&self, buttons: &mut Button) {
        *buttons = Button::empty();

        if self.window.is_key_down(Key::Enter) {
            *buttons |= Button::START;
        }

        if self.window.is_key_down(Key::RightShift) || self.window.is_key_down(Key::LeftShift) {
            *buttons |= Button::SELECT;
        }

        if self.window.is_key_down(Key::Up) {
            *buttons |= Button::UP;
        }

        if self.window.is_key_down(Key::Right) {
            *buttons |= Button::RIGHT;
        }

        if self.window.is_key_down(Key::Down) {
            *buttons |= Button::DOWN;
        }

        if self.window.is_key_down(Key::Left) {
            *buttons |= Button::LEFT;
        }

        if self.window.is_key_down(Key::Z) {
            *buttons |= Button::B;
        }

        if self.window.is_key_down(Key::X) {
            *buttons |= Button::A;
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
        }).expect("Unable to create window")
}