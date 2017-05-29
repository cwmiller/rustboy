use bus::Addressable;
use minifb::Key;

bitflags! {
    flags Pins: u8 {
        const PIN_15 = 0b100000,
        const PIN_14 = 0b010000,
        const PIN_13 = 0b001000,
        const PIN_12 = 0b000100,
        const PIN_11 = 0b000010,
        const PIN_10 = 0b000001
    }
}

#[derive(Default)]
pub struct StepResult {
    pub interrupt: bool
}

pub struct Joypad {
    pins: Pins
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            pins: Pins::empty()
        }
    }

    pub fn step(&mut self, keys: Vec<Key>) -> StepResult {
        let mut result = StepResult::default();
        let previous = self.pins;

        self.pins.bits = previous.bits & 0b11_0000;

        for key in keys.iter() {
            let pins = match *key {
                Key::Enter if self.pins.contains(PIN_15) => PIN_13,         // Start
                Key::LeftShift
                | Key::RightShift if self.pins.contains(PIN_15) => PIN_12,  // Select
                Key::Up if self.pins.contains(PIN_14) => PIN_12,            // Up
                Key::Right if self.pins.contains(PIN_14) => PIN_10,         // Right
                Key::Down if self.pins.contains(PIN_14) => PIN_13,          // Down
                Key::Left if self.pins.contains(PIN_14) => PIN_11,          // Left
                Key::X if self.pins.contains(PIN_15) => PIN_10,             // A
                Key::Z if self.pins.contains(PIN_15) => PIN_11,             // B
                _ => Pins::empty()
            };

            self.pins.bits = self.pins.bits | pins.bits;
        }

        // An interrupt is generated if any pin 10-13 gets triggered
        for pin in [PIN_10, PIN_11, PIN_12, PIN_13].iter() {
            if !previous.contains(*pin) && self.pins.contains(*pin) {
                result.interrupt = true;
            }
        }

        result
    }
}

// When reading and writing memory, the pins are low when selected.
impl Addressable for Joypad {
    fn read(&self, _: u16) -> u8 {
        !self.pins.bits | 0b1100_0000
    }

    fn write(&mut self, _: u16, val: u8) {
        self.pins.bits = !val & 0b11_1111;
    }
}