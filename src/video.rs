use bus::{Addressable, Bus};

pub const CYCLES_PER_LINE: usize = 456;
pub const CYCLES_PER_FRAME: usize = 70224;

const ADDR_LY: u16 = 0xFF44;

#[derive(PartialEq)]
pub enum StepResult {
    None,
    InterruptVBlank
}

#[derive(Default)]
pub struct Video {
    ly: u8
}

impl Video {
    pub fn step(&mut self, cycles: usize) -> StepResult {
        let previous_line = self.ly;
        let current_line = cycles / CYCLES_PER_LINE;

        // Reset to line 0 if we've exceeded the screen size
        if current_line > 153 {
            self.ly = 0;
        } else {
            self.ly = current_line as u8;
        }

        // If we just arrived at frame 144, raise the vblank interrupt
        if previous_line == 143 && current_line == 144 {
            StepResult::InterruptVBlank
        } else {
            StepResult::None
        }
    }
}

impl Addressable for Video {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            ADDR_LY => self.ly,
            _ => { println!("Video IO read unimplemented ({:#X})", addr); 0 }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        println!("Video IO write unimplemented {:#X} -> {:#X}", val, addr);
    }
}