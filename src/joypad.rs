use bus::Addressable;

#[derive(Default)]
pub struct Joypad {
}

impl Addressable for Joypad {
    fn read(&self, _: u16) -> u8 {
        0xFF
    }

    fn write(&mut self, _: u16, _: u8) {
    }
}