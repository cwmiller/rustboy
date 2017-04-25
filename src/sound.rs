use bus::Addressable;

#[derive(Default)]
pub struct Sound {
}

impl Addressable for Sound {
    fn read(&self, addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, val: u8) {
    }
}