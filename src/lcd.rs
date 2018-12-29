use bus::{Addressable, OAM_START, OAM_END, VIDEO_RAM_START, VIDEO_RAM_END};
use enum_primitive::FromPrimitive;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

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

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    enum Shade {
        White       = 0,
        LightGray   = 1,
        DarkGray    = 2,
        Black       = 3
    }
}

const CYCLES_PER_OAM_READ: usize = 80;
const CYCLES_PER_TRANSFER: usize = 172;
const CYCLES_PER_HBLANK: usize = 204;
const CYCLES_PER_LINE: usize = 456;

#[derive(Copy, Clone)]
struct Palette(u8);

impl Palette {
    fn rgb(&self, shade: Shade) -> u32 {
        match shade {
            Shade::White        => 0x9CBD0F,
            Shade::LightGray    => 0x8CAD0F,
            Shade::DarkGray     => 0x306230,
            Shade::Black        => 0x0F380F
        }
    }
}

#[derive(Default)]
pub struct StepResult {
    pub int_vblank: bool,
    pub int_stat: bool
}

pub struct Lcd {
    vram: [u8; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
    oam: [u8; (OAM_END - OAM_START) as usize + 1],
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
    mode_cycles: usize,
    draw_pending: bool
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            vram: [0; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
            oam: [0; (OAM_END - OAM_START) as usize + 1],
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
            mode_cycles: 0,
            draw_pending: false
        }
    }

    pub fn step(&mut self, cycles: usize, screen_buffer: &mut [u32]) -> StepResult {
        let mut result = StepResult::default();

        if self.lcdc.contains(LCDC_ENABLED) {
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
                        self.draw_pending = true;
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
                            self.mode_cycles = 0;
                            result.int_vblank = true;

                            if self.lcdc.contains(LCDC_SPRITE_DISPLAY) {
                                self.draw_sprites(screen_buffer);
                            }
                        } else { 
                            self.mode = Mode::Oam;
                        };

                        self.check_coincidence(&mut result);
                    } else {
                        if self.ly < 144 && self.draw_pending {
                            if self.lcdc.contains(LCDC_BG_DISPLAY) {
                                self.draw_background_line(screen_buffer);
                            }

                            self.draw_pending = false;
                        }
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
        }

        result
    }

    fn draw_background_line(&self, screen_buffer: &mut [u32]) {
        let map_y = self.scy.wrapping_add(self.ly);
        let map_tile_y = (map_y / 8) as u16;
        let tile_row = map_y & 7;

        let map_addr_base = if self.lcdc.contains(LCDC_BG_TILE_9C) {
            0x9C00
        } else {
            0x9800
        } as u16;

        for screen_x in 0..SCREEN_WIDTH {
            let map_x = self.scx.wrapping_add(screen_x as u8);
            let map_tile_x = (map_x / 8) as u16;
            let tile_column = map_x & 7;

            let tile_idx_addr = map_addr_base + (map_tile_y * 32) + map_tile_x;
            let tile_idx = self.vram[(tile_idx_addr - VIDEO_RAM_START) as usize];
            let tile_addr = self.bg_tile_address(tile_idx);
            let shade = self.tile_pixel_shade(tile_addr, tile_row, tile_column);

            screen_buffer[(self.ly as usize * SCREEN_WIDTH) + screen_x] = self.bgp.rgb(shade);
        }
    }

    fn draw_sprites(&self, screen_buffer: &mut [u32]) {
        // Sprites are 40 blocks in OAM. Each block is 32 bits
        for idx in 0..40 {
            let y = self.oam[idx * 4] as usize;
            let x = self.oam[(idx * 4) + 1] as usize;
            let pattern =  self.oam[(idx * 4) + 2];
            let flags = self.oam[(idx * 4) + 3];

            if x >= 8 && y >= 16 {
                let tile_addr = self.sprite_tile_address(pattern);
                let screen_x = x.wrapping_sub(8) as usize;
                let screen_y = y.wrapping_sub(16) as usize;
                let palette = if flags & 0b10000 == 0b10000 { self.obp1 } else { self.obp0 };

                for row in 0..8 as usize {
                    for column in 0..8 as usize {
                        let idx = ((screen_y + row) * SCREEN_WIDTH) + screen_x + column;
                        let shade = self.tile_pixel_shade(tile_addr, row as u8, column as u8);

                        if idx < screen_buffer.len() && shade != Shade::White {
                            screen_buffer[idx] = palette.rgb(shade);
                        }
                    }
                }
            }
        }
    }

    fn tile_pixel_shade(&self, address: u16, row: u8, column: u8) -> Shade {
        let upper_byte = self.vram[((address + (row * 2) as u16) + 1 - VIDEO_RAM_START) as usize];
        let lower_byte = self.vram[((address + (row * 2) as u16) - VIDEO_RAM_START) as usize];

        let shift = 7 - column;
        let mask = 1 << shift;
        let upper_bit = (upper_byte & mask) >> shift;
        let lower_bit = (lower_byte & mask) >> shift;

        Shade::from_u8((upper_bit << 1) | lower_bit).unwrap()
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

    fn bg_tile_address(&self, tile_index: u8) -> u16 {
        let tile_addr_base = if self.lcdc.contains(LCDC_UNSIGNED_TILE_DATA) {
            0x8000
        } else {
            0x9000
        } as u16;

        if self.lcdc.contains(LCDC_UNSIGNED_TILE_DATA) {
            (tile_addr_base + ((tile_index as u16) * 16))
        } else {
            let signed_index = (tile_index as i8) as i16;
            ((tile_addr_base as i16) + ((signed_index) * 16)) as u16
        }
    }

    fn sprite_tile_address(&self, tile_index: u8) -> u16 {
        0x8000 + ((tile_index as u16) * 16)
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