use bus::Addressable;

#[derive(Default)]
pub struct Sound {
}

impl Addressable for Sound {
    fn read(&self, _: u16) -> u8 {
        0
    }

    fn write(&mut self, _: u16, _: u8) {
    }
}