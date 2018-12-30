use bus::Addressable;

bitflags! {
    struct Pin: u8 {
        const PIN_15 = 0b10_0000;
        const PIN_14 = 0b01_0000;
        const PIN_13 = 0b00_1000;
        const PIN_12 = 0b00_0100;
        const PIN_11 = 0b00_0010;
        const PIN_10 = 0b00_0001;
    }
}

#[derive(Default)]
pub struct StepResult {
    pub interrupt: bool
}


bitflags! {
    pub struct Button: u8 {
        const START     = 0b1000_0000;
        const SELECT    = 0b0100_0000;
        const UP        = 0b0010_0000;
        const RIGHT     = 0b0001_0000;
        const DOWN      = 0b0000_1000;
        const LEFT      = 0b0000_0100;
        const A         = 0b0000_0010;
        const B         = 0b0000_0001;
    }
}

pub struct Joypad {
    pins: Pin
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            pins: Pin::empty()
        }
    }

    pub fn step(&mut self, buttons: Button) -> StepResult {
        let mut result = StepResult::default();
        let previous = self.pins;

        self.pins.bits = previous.bits & 0b11_0000;

        // The GB uses a 2x4 matrix for detecting button presses.
        // It sets PIN_14 when reading the dpad and PIN_15 when reading the other buttons.
        if self.pins.contains(Pin::PIN_14) {
            if buttons.contains(Button::UP) {
                self.pins |= Pin::PIN_12;
            }

            if buttons.contains(Button::RIGHT) {
                self.pins |= Pin::PIN_10;
            }

            if buttons.contains(Button::DOWN) {
                self.pins |= Pin::PIN_13;
            }

            if buttons.contains(Button::LEFT) {
                self.pins |= Pin::PIN_11;
            }
        } else if self.pins.contains(Pin::PIN_15) {
            if buttons.contains(Button::START) {
                self.pins |= Pin::PIN_13;
            }

            if buttons.contains(Button::SELECT) {
                self.pins |= Pin::PIN_12;
            }

            if buttons.contains(Button::A) {
                self.pins |= Pin::PIN_10;
            }

            if buttons.contains(Button::B) {
                self.pins |= Pin::PIN_11;
            }
        }

        // An interrupt is generated if any pin 10-13 gets triggered
        for pin in [Pin::PIN_10, Pin::PIN_11, Pin::PIN_12, Pin::PIN_13].iter() {
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
        // Only allow bits 5 and 4 to be written to
        self.pins.bits = (!val & 0b11_0000) | (self.pins.bits & 0b00_1111)
    }
}