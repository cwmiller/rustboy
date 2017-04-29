use bus::Addressable;

#[derive(Default)]
pub struct Serial {
}

impl Addressable for Serial {
    fn read(&self, _: u16) -> u8 {
        0
    }

    fn write(&mut self, _: u16, _: u8) {
    }
}