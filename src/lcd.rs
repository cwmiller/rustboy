use bus::{Addressable, OAM_START, OAM_END, VIDEO_RAM_START, VIDEO_RAM_END};
use enum_primitive::FromPrimitive;

pub const BUFFER_WIDTH: usize = 256;
pub const BUFFER_HEIGHT: usize = 256;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;

const ADDR_LCDC: u16 = 0xFF40;
const ADDR_STAT: u16 = 0xFF41;
const ADDR_SCY: u16  = 0xFF42;
const ADDR_SCX: u16  = 0xFF43;
const ADDR_LY: u16   = 0xFF44;
const ADDR_LYC: u16  = 0xFF45;
pub const ADDR_DMA: u16  = 0xFF46;
const ADDR_BGP: u16  = 0xFF47;
const ADDR_OBP0: u16 = 0xFF48;
const ADDR_OBP1: u16 = 0xFF49;
const ADDR_WY: u16   = 0xFF4A;
const ADDR_WX: u16   = 0xFF4B;

bitflags! {
    flags Lcdc: u8 {
        const LCDC_ENABLED            = 0b1000_0000,
        const LCDC_WIN_TILE_9C        = 0b0100_0000,
        const LCDC_WIN_DISPLAY        = 0b0010_0000,
        const LCDC_UNSIGNED_TILE_DATA = 0b0001_0000,
        const LCDC_BG_TILE_9C         = 0b0000_1000,
        const LCDC_16PX_SPRITE        = 0b0000_0100,
        const LCDC_SPRITE_DISPLAY     = 0b0000_0010,
        const LCDC_BG_DISPLAY         = 0b0000_0001
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

#[derive(Copy, Clone)]
struct Palette(u8);

impl Palette {
    fn shade(&self, idx: u8) -> u8 {
        self.0 >> (idx * 2) & 0b11
    }

    fn rgb(&self, idx: u8) -> u32 {
        match idx {
            SHADE_WHITE      => 0x9CBD0F,
            SHADE_LIGHT_GRAY => 0x8CAD0F,
            SHADE_DARK_GRAY  => 0x306230,
            SHADE_BLACK      => 0x0F380F,
            _                => unreachable!()
        }
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
    bgp: Palette,
    obp0: Palette,
    obp1: Palette,
    wy: u8,
    wx: u8,
    dma: u8,

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
            bgp: Palette(0xFC),
            obp0: Palette(0xFF),
            obp1: Palette(0xFF),
            wy: 0,
            wx: 0,
            dma: 0,
            mode: Mode::Oam,
            mode_cycles: 0
        }
    }

    pub fn step(&mut self, cycles: usize, framebuffer: &mut [u32]) -> StepResult {
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

                        self.clear_framebuffer(framebuffer);

                        if self.lcdc.contains(LCDC_BG_DISPLAY) {
                            self.draw_background(framebuffer);
                        }

                        if self.lcdc.contains(LCDC_SPRITE_DISPLAY) {
                            self.draw_sprites(framebuffer);
                        }


                        self.draw_borders(framebuffer);
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

    fn clear_framebuffer(&self, framebuffer: &mut [u32]) {
        for idx in 0..BUFFER_HEIGHT*BUFFER_WIDTH {
            framebuffer[idx] = 0xFFFFFF;
        }
    }

    fn draw_background(&self, framebuffer: &mut [u32]) {
        let map_addr_base = if self.lcdc.contains(LCDC_BG_TILE_9C) {
            0x9C00
        } else {
            0x9800
        } as u16;

        let tile_addr_base = if self.lcdc.contains(LCDC_UNSIGNED_TILE_DATA) {
            0x8000
        } else {
            0x9000
        } as u16;

        for y in 0..32 {
            for x in 0..32 {
                let fb_top_left_y = y as usize * 8;
                let fb_top_left_x = x as usize * 8;

                let tile_idx = self.read(map_addr_base + ((y * 32) + x)) as u8;

                let tile_addr = if self.lcdc.contains(LCDC_UNSIGNED_TILE_DATA) {
                    (tile_addr_base + ((tile_idx as u16) * 16))
                } else {
                    let signed_idx = (tile_idx as i8) as i16;
                    ((tile_addr_base as i16) + ((signed_idx) * 16)) as u16

                };

                let tile = (0..16).map(|idx| self.read(tile_addr + (idx as u16))).collect::<Vec<u8>>();

                self.draw_tile(framebuffer, &tile, self.bgp, fb_top_left_x, fb_top_left_y, false);
            }
        }
    }

    // TODO: handle priority, 8x16 mode
    fn draw_sprites(&self, framebuffer: &mut [u32]) {
        // Sprites are 40 blocks in OAM. Each block is 32 bits
        for idx in 0..40 {
            let y = self.oam[idx * 4];
            let x = self.oam[(idx * 4) + 1];
            let pattern =  self.oam[(idx * 4) + 2];
            let flags = self.oam[(idx * 4) + 3];

            if x > 0 && y > 0 {
                let tile_addr = 0x8000 + ((pattern as u16) * 16);
                let tile = (0..16).map(|idx| self.read((tile_addr as u16) + (idx as u16))).collect::<Vec<u8>>();
                let screen_x = x.wrapping_sub(8) as usize;
                let screen_y = y.wrapping_sub(16) as usize;
                let palette = if flags & 0b10000 == 0b10000 { self.obp1 } else { self.obp0 };

                self.draw_tile(framebuffer, &tile, palette, screen_x, screen_y, true);
            }
        }
    }

    fn draw_tile(&self, framebuffer: &mut [u32], tile: &Vec<u8>, palette: Palette, x: usize, y: usize, transparent: bool) {
        for tile_y in 0..8 {
            let upper_byte = tile[(tile_y * 2) + 1];
            let lower_byte = tile[tile_y * 2];

            for tile_x in 0..8 {
                let fb_idx = ((y + tile_y) * BUFFER_WIDTH) + x + tile_x;
                if fb_idx < 65536 {
                    let shift = 7 - tile_x;
                    let mask = 1 << shift;
                    let upper_bit = (upper_byte & mask) >> shift;
                    let lower_bit = (lower_byte & mask) >> shift;
                    let shade_idx = (upper_bit << 1) | lower_bit;

                    if !(shade_idx == 0 && transparent) {
                        framebuffer[fb_idx] = palette.rgb(shade_idx);
                    }
                } else {
                    println!("Attempted tile write outside framebuffer length at index {:#X}", fb_idx);
                }
            }
        }
    }

    fn draw_borders(&self, framebuffer: &mut [u32]) {
        let scy = self.scy as usize;
        let scx = self.scx as usize;

        for y_offset in 0..LCD_HEIGHT {
            let y = if scy + y_offset >= LCD_HEIGHT {
                (scy + y_offset) - LCD_HEIGHT
            } else {
                scy + y_offset
            };

            for x_offset in 0..LCD_WIDTH {
                let x = if scx + x_offset >= LCD_WIDTH {
                    (scx + x_offset) - LCD_WIDTH
                } else {
                    scx + x_offset
                };

                if y_offset == 0 || y_offset == LCD_HEIGHT - 1 || x_offset == 0 || x_offset == LCD_WIDTH - 1 {
                    let idx = (y * BUFFER_WIDTH) + x;
                    framebuffer[idx] = 0xFF0000;
                }
            }
        }
    }
}

impl Addressable for Lcd {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START...VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize],
            OAM_START...OAM_END => self.oam[(addr - OAM_START) as usize],
            ADDR_LCDC => self.lcdc.bits,
            ADDR_STAT => 0b1000_0000 | ((self.stat.bits & 0b1111_1100) | (self.mode as u8)),
            ADDR_SCY => self.scy,
            ADDR_SCX => self.scx,
            ADDR_LY => self.ly,
            ADDR_LYC => self.lyc,
            ADDR_DMA => self.dma,
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
            ADDR_DMA => { self.dma = val }, // Actual transfer is done by the Bus object
            ADDR_BGP => self.bgp = Palette(val),
            ADDR_OBP0 => self.obp0 = Palette(val),
            ADDR_OBP1 => self.obp1 = Palette(val),
            ADDR_WY => self.wy = val,
            ADDR_WX => self.wx = val,
            _ => println!("LCD IO write unimplemented {:#X} -> {:#X}", val, addr)
        }
    }
}