use bus::{Addressable, OAM_START, OAM_END, VIDEO_RAM_START, VIDEO_RAM_END};

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

pub const CYCLES_PER_LINE: usize = 456;
pub const CYCLES_PER_FRAME: usize = 70224;

const ADDR_LCDC: u16 = 0xFF40;
const ADDR_STAT: u16 = 0xFF41;
const ADDR_SCY: u16 = 0xFF42;
const ADDR_LY: u16 = 0xFF44;
const ADDR_LYC: u16 = 0xFF45;
const ADDR_DMA: u16 = 0xFF46;
const ADDR_BGP: u16 = 0xFF47;
const ADDR_OBP0: u16 = 0xFF48;
const ADDR_OBP1: u16 = 0xFF49;
const ADDR_WY: u16 = 0xFF4A;
const ADDR_LX: u16 = 0xFF4B;

struct Lcdc(u8);
struct Stat(u8);
struct Bgp(u8);

#[derive(PartialEq)]
pub enum StepResult {
    None,
    InterruptVBlank
}

pub struct Lcd {
    vram: Vec<u8>,
    oam: Vec<u8>,

    lcdc: Lcdc,
    stat: Stat,
    bgp: Bgp,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8
}

impl Lcd {
    pub fn new() -> Self {
        Lcd {
            vram: vec![0; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
            oam: vec![0; (OAM_END - OAM_START) as usize + 1],

            lcdc: Lcdc(0),
            stat: Stat(0),
            bgp: Bgp(0),
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0
        }
    }

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

impl Addressable for Lcd {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START...VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize],
            OAM_START...OAM_END => self.oam[(addr - OAM_START) as usize],

            ADDR_LY => self.ly,
            _ => { println!("LCD IO read unimplemented ({:#X})", addr); 0 }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            VIDEO_RAM_START...VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize] = val,
            OAM_START...OAM_END => self.oam[(addr - OAM_START) as usize] = val,
            _ => println!("LCD IO write unimplemented {:#X} -> {:#X}", val, addr)
        }
    }
}