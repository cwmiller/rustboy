use bus::{Addressable, OAM_START, OAM_END, VIDEO_RAM_START, VIDEO_RAM_END};
use enum_primitive::FromPrimitive;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

const ADDR_LCDC: u16 = 0xFF40;
const ADDR_STAT: u16 = 0xFF41;
const ADDR_SCY: u16  = 0xFF42;
const ADDR_SCX: u16  = 0xFF43;
const ADDR_LY: u16   = 0xFF44;
const ADDR_LYC: u16  = 0xFF45;
const ADDR_DMA: u16  = 0xFF46;
const ADDR_BGP: u16  = 0xFF47;
const ADDR_OBP0: u16 = 0xFF48;
const ADDR_OBP1: u16 = 0xFF49;
const ADDR_WY: u16   = 0xFF4A;
const ADDR_WX: u16   = 0xFF4B;

bitflags! {
    flags Lcdc: u8 {
        const LCDC_ENABLED          = 0b1000_0000,
        const LCDC_TILE_TBL_9C      = 0b0100_0000,
        const LCDC_WIN_ENABLED      = 0b0010_0000,
        const LCDC_SIGNED_TILE_DATA = 0b0001_0000,
        const LCDC_BG_TILE_TBL_9C   = 0b0000_1000,
        const LCDC_16PX_SPRITE      = 0b0000_0100,
        const LCDC_SPRITE_DISPLAY   = 0b0000_0010,
        const LCDC_BG_DISPLAY       = 0b0000_0001
    }
}

// STAT is mostly flags, but the lower 2 bits are the current mode
// Mode is will stored separately so the Stat struct is only treated as bitflags.
bitflags! {
    flags Stat: u8 {
        const STAT_COINCIDENCE_INT   = 0b0100_0000,
        const STAT_OAM_INT           = 0b0010_0000,
        const STAT_VBLANK_INT        = 0b0001_0000,
        const STAT_HBLANK_INT        = 0b0000_1000,
        const STAT_COINCIDENCE_EQUAL = 0b0000_0100
    }
}

enum_from_primitive! {
#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    VBlank   = 0,
    HBlank   = 1,
    Oam      = 2,
    Transfer = 3
}
}

const CYCLES_PER_OAM_READ: usize = 80;
const CYCLES_PER_TRANSFER: usize = 172;
const CYCLES_PER_HBLANK: usize = 204;
const CYCLES_PER_LINE: usize = 456;

const SHADE_WHITE: u8      = 0;
const SHADE_LIGHT_GRAY: u8 = 1;
const SHADE_DARK_GRAY: u8  = 2;
const SHADE_BLACK: u8      = 3;

struct Pallete(u8);

impl Pallete {
    fn shade(&self, idx: u8) -> u8 {
        self.0 >> (idx * 2)
    }

    fn set_shade(&mut self, idx: u8, shade: u8) {
        self.0 = self.0 | shade << (idx * 2);
    }
}

#[derive(Default)]
pub struct StepResult {
    pub int_vblank: bool,
    pub int_stat: bool
}

pub struct Lcd {
    vram: Vec<u8>,
    oam: Vec<u8>,
    lcdc: Lcdc,
    stat: Stat,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: Pallete,
    obp0: Pallete,
    obp1: Pallete,
    wy: u8,
    wx: u8,

    mode: Mode,
    mode_cycles: usize
}

impl Lcd {
    pub fn new() -> Self {
        Lcd {
            vram: vec![0; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
            oam: vec![0; (OAM_END - OAM_START) as usize + 1],
            lcdc: Lcdc::from_bits(0x91).unwrap(),
            stat: Stat::empty(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: Pallete(0xFC),
            obp0: Pallete(0xFF),
            obp1: Pallete(0xFF),
            wy: 0,
            wx: 0,
            mode: Mode::Oam,
            mode_cycles: 0
        }
    }

    pub fn step(&mut self, cycles: usize) -> StepResult {
        let mut result = StepResult::default();
        let previous_mode = self.mode;

        self.mode_cycles = self.mode_cycles + cycles;

        match self.mode {
            Mode::Oam => {
                // Process starts with the LCD controller reading information from OAM.
                // This lasts 77-83 cycles
                if self.mode_cycles >= CYCLES_PER_OAM_READ {
                    // Move onto transfer stage
                    self.mode = Mode::Transfer;
                    self.mode_cycles = 0;
                }
            },
            Mode::Transfer => {
                // Data is being actively read by the LCD driver from OAM & VRAM.
                // This lasts 169-175 cycles
                if self.mode_cycles >= CYCLES_PER_TRANSFER {
                    // Once transfer is complete, enter HBLANK
                    self.mode = Mode::HBlank;
                    self.mode_cycles = 0;
                }
            },
            Mode::HBlank => {
                // LCD has reached the end of a line.
                // HBlank phase lasts 201-207 cycles
                if self.mode_cycles >= CYCLES_PER_HBLANK {
                    // LCD is ready to begin the next line
                    self.ly = self.ly + 1;
                    self.mode_cycles = 0;

                    // If the LCD has line 144, then enter VBLANK. Else, enter OAM read.
                    if self.ly == 144 { 
                        self.mode = Mode::VBlank;
                        result.int_vblank = true;
                    } else { 
                        self.mode = Mode::Oam;
                    };

                    self.check_coincidence(&mut result);
                }
            },
            Mode::VBlank => {
                // LCD is writing to VBlank lines
                if self.mode_cycles >= CYCLES_PER_LINE {
                    self.mode_cycles = 0;
                    self.ly = self.ly + 1;

                    if self.ly > 153 {
                        // Finished with VBlank. Enter OAM with the first line.
                        self.ly = 0;
                        self.mode = Mode::Oam;
                    }

                    self.check_coincidence(&mut result);
                } 
            }
        }

        // Check if a new mode has been entered during this step.
        // If so, an interrupt will be raised if the respective flag is set in the STAT register
        if self.mode != previous_mode {
            let flag = match self.mode {
                Mode::VBlank   => Some(STAT_VBLANK_INT),
                Mode::HBlank   => Some(STAT_HBLANK_INT),
                Mode::Oam      => Some(STAT_OAM_INT),
                Mode::Transfer => None
            };

            if flag.is_some() && self.stat.contains(flag.unwrap()) {
                result.int_stat = true;
            }
        }

        result
        
    }

    fn check_coincidence(&mut self, result: &mut StepResult) {
        if self.stat.contains(STAT_COINCIDENCE_INT) {
            if self.ly == self.lyc {
                self.stat.insert(STAT_COINCIDENCE_EQUAL);
                result.int_stat = true;
            } else {
                self.stat.remove(STAT_COINCIDENCE_EQUAL);
            }
        } else {
            self.stat.remove(STAT_COINCIDENCE_EQUAL);
        }
    }
}

impl Addressable for Lcd {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START...VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize],
            OAM_START...OAM_END => self.oam[(addr - OAM_START) as usize],
            ADDR_LCDC => self.lcdc.bits,
            ADDR_STAT => (self.stat.bits & 0b1111_1100) | (self.mode as u8),
            ADDR_SCY => self.scy,
            ADDR_SCX => self.scx,
            ADDR_LY => self.ly,
            ADDR_LYC => self.lyc,
            ADDR_BGP => self.bgp.0,
            ADDR_OBP0 => self.obp0.0,
            ADDR_OBP1 => self.obp1.0,
            ADDR_WY => self.wy,
            ADDR_WX => self.wx,
            _ => { println!("LCD IO read unimplemented ({:#X})", addr); 0 }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            VIDEO_RAM_START...VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize] = val,
            OAM_START...OAM_END => self.oam[(addr - OAM_START) as usize] = val,
            ADDR_LCDC => self.lcdc.bits = val,
            ADDR_STAT => {
                self.stat.bits = val & 0b1111_1100;
                self.mode = Mode::from_u8(val & 0b11).unwrap();
            },
            ADDR_SCY => self.scy = val,
            ADDR_SCX => self.scx = val,
            ADDR_LY => (), // read-only
            ADDR_LYC => self.lyc = val,
            ADDR_DMA => println!("DMA unimplemented"),
            ADDR_BGP => self.bgp = Pallete(val),
            ADDR_OBP0 => self.obp0 = Pallete(val),
            ADDR_OBP1 => self.obp1 = Pallete(val),
            ADDR_WY => self.wy = val,
            ADDR_WX => self.wx = val,
            _ => println!("LCD IO write unimplemented {:#X} -> {:#X}", val, addr)
        }
    }
}