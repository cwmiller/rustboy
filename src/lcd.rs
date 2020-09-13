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
const ADDR_BGP: u16  = 0xFF47;
const ADDR_OBP0: u16 = 0xFF48;
const ADDR_OBP1: u16 = 0xFF49;
const ADDR_WY: u16   = 0xFF4A;
const ADDR_WX: u16   = 0xFF4B;

bitflags! {
    struct Lcdc: u8 {
        const LCDC_ENABLED            = 0b1000_0000;
        const LCDC_WIN_TILE_9C        = 0b0100_0000;
        const LCDC_WIN_DISPLAY        = 0b0010_0000;
        const LCDC_UNSIGNED_TILE_DATA = 0b0001_0000;
        const LCDC_BG_TILE_9C         = 0b0000_1000;
        const LCDC_8X16_SPRITE        = 0b0000_0100;
        const LCDC_SPRITE_DISPLAY     = 0b0000_0010;
        const LCDC_BG_DISPLAY         = 0b0000_0001;
    }
}

// STAT is mostly flags, but the lower 2 bits are the current mode
// Mode is will stored separately so the Stat struct is only treated as bitflags.
bitflags! {
    struct Stat: u8 {
        const STAT_COINCIDENCE_INT   = 0b0100_0000;
        const STAT_OAM_INT           = 0b0010_0000;
        const STAT_VBLANK_INT        = 0b0001_0000;
        const STAT_HBLANK_INT        = 0b0000_1000;
        const STAT_COINCIDENCE_EQUAL = 0b0000_0100;
    }
}

// The LCD driver is always in one of four states:
// HBlank - Current line has finished rendering
// VBlank - All visible lines have been rendered
// Oam - Driver is reading data from OAM
// Transfer - Current line is being rendered using OAM & VRAM
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    pub enum Mode {
        HBlank   = 0,
        VBlank   = 1,
        Oam      = 2,
        Transfer = 3
    }
}

const CYCLES_PER_OAM_READ: usize = 80;
const CYCLES_PER_TRANSFER: usize = 172;
const CYCLES_PER_HBLANK: usize = 204;
const CYCLES_PER_LINE: usize = 456;



// DMG can only display four beautiful shades of color
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    enum Shade {
        White     = 0,
        LightGray = 1,
        DarkGray  = 2,
        Black     = 3
    }
}

type ColorIndex = u8;

// Color pallete is stored in an 8 bit value where each 2 bytes indicate the shade for each color index
// aabbccdd
// dd is the shade for color 0
// cc is the shade for color 1
// bb is the shade for color 2
// aa is the shade for color 3
#[derive(Copy, Clone)]
struct Palette(u8);

impl Palette {
    fn rgb(&self, idx: ColorIndex) -> u32 {
        if idx > 3 {
            panic!("Color index cannot exceed 3");
        }

        // Shade can be retrieved by the shifting right
        let shade_index = (self.0 >> (idx * 2)) & 0b11;

        match Shade::from_u8(shade_index).unwrap() {
            Shade::White        => 0x9CBD0F,
            Shade::LightGray    => 0x8CAD0F,
            Shade::DarkGray     => 0x306230,
            Shade::Black        => 0x0F380F
        }
    }
}

// Represents an OAM (Sprite data)
#[derive(Copy, Clone, Debug)]
struct OamEntry {
    y: u8,
    x: u8,
    tile: u8,
    attrs: OamAttr
}

impl OamEntry {
    pub fn new() -> Self {
        Self { 
            y: 0,
            x: 0,
            tile: 0,
            attrs: OamAttr::empty()
        }
    }
}

bitflags! {
    struct OamAttr: u8 {
        const OAM_ATTR_OBJ_PRIORITY     = 0b1000_0000;
        const OAM_ATTR_Y_FLIP           = 0b0100_0000;
        const OAM_ATTR_X_FLIP           = 0b0010_0000;
        const OAM_ATTR_PALETTE_DMG      = 0b0001_0000;
        const OAM_ATTR_TILE_BANK_CGB    = 0b0000_1000;
        const OAM_ATTR_PALETTE_CGB      = 0b0000_0111;
    }
}

#[derive(Default)]
pub struct StepResult {
    pub int_vblank: bool,
    pub int_stat: bool
}

pub struct Lcd {
    vram: [u8; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
    oam: [OamEntry; (OAM_END - OAM_START) as usize + 1],
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
    mode: Mode,
    mode_cycles: usize,
    draw_pending: bool
}

impl Lcd {
    pub fn new() -> Self {
        Self {
            vram: [0; (VIDEO_RAM_END - VIDEO_RAM_START) as usize + 1],
            oam: [OamEntry::new(); (OAM_END - OAM_START) as usize + 1],
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
            mode: Mode::Oam,
            mode_cycles: 0,
            draw_pending: false
        }
    }

    pub fn step(&mut self, cycles: usize, screen_buffer: &mut [u32]) -> StepResult {
        let mut result = StepResult::default();

        if self.lcdc.contains(Lcdc::LCDC_ENABLED) {
            let previous_mode = self.mode;
            self.mode_cycles += cycles;

            match self.mode {
                Mode::Oam => {
                    // Process starts with the LCD controller reading information from OAM.
                    // This lasts 77-83 cycles
                    if self.mode_cycles >= CYCLES_PER_OAM_READ {
                        // Move onto transfer stage
                        self.mode = Mode::Transfer;
                        self.mode_cycles -= CYCLES_PER_OAM_READ;
                        self.draw_pending = true;
                    }
                },
                Mode::Transfer => {
                    // Data is being actively read by the LCD driver from OAM & VRAM.
                    // This lasts 169-175 cycles
                    if self.mode_cycles >= CYCLES_PER_TRANSFER {
                        // Once transfer is complete, enter HBLANK
                        self.mode = Mode::HBlank;
                        self.mode_cycles -= CYCLES_PER_TRANSFER;
                    } else if self.draw_pending {
                        self.draw_background_current_line(screen_buffer);
                        self.draw_window_current_line(screen_buffer);
                        self.draw_sprites_current_line(screen_buffer);

                        self.draw_pending = false;
                    }
                },
                Mode::HBlank => {
                    // LCD has reached the end of a line.
                    // HBlank phase lasts 201-207 cycles
                    if self.mode_cycles >= CYCLES_PER_HBLANK {
                        // LCD is ready to begin the next line
                        self.ly += 1;
                        self.mode_cycles -= CYCLES_PER_HBLANK;

                        // If the LCD has line 144, then enter VBLANK. Else, enter OAM read.
                        if self.ly == 144 { 
                            self.mode = Mode::VBlank;
                            result.int_vblank = true;
                        } else { 
                            self.mode = Mode::Oam;

                            self.check_coincidence(&mut result);
                        };
                    }
                },
                Mode::VBlank => {
                    // LCD is writing to VBlank lines
                    if self.mode_cycles >= CYCLES_PER_LINE {
                        self.mode_cycles -= CYCLES_PER_LINE;
                        self.ly += 1;

                        if self.ly > 153 {
                            // Finished with VBlank. Enter OAM with the first line.
                            self.ly = 0;
                            self.mode = Mode::Oam;

                            self.check_coincidence(&mut result);
                        }
                    } 
                }
            }

            // Check if a new mode has been entered during this step.
            // If so, an interrupt will be raised if the respective flag is set in the STAT register
            if self.mode != previous_mode {
                let raise = 
                    self.stat.contains(Stat::STAT_HBLANK_INT) && self.mode == Mode::HBlank
                    || self.stat.contains(Stat::STAT_VBLANK_INT) && self.mode == Mode::VBlank
                    || self.stat.contains(Stat::STAT_OAM_INT) && (self.mode == Mode::Transfer || self.mode == Mode::VBlank);

                if raise {
                    result.int_stat = true;
                }
            }
        }

        result
    }

    // Draws the background for the current line specified in LY
    fn draw_background_current_line(&self, screen_buffer: &mut [u32]) {
        if self.lcdc.contains(Lcdc::LCDC_BG_DISPLAY) {
            // Background can be scrolled via the SCX and SCY registers
            let map_y = self.scy.wrapping_add(self.ly);

            for screen_x in 0..SCREEN_WIDTH as u8 {
                let map_x = self.scx.wrapping_add(screen_x);

                self.draw_bg_tile_pixel(map_x, map_y, screen_x, self.ly, screen_buffer);
            }
        }
    }

    // Draws the window for the current line specified in LY
    fn draw_window_current_line(&self, screen_buffer: &mut [u32]) {
        if self.lcdc.contains(Lcdc::LCDC_WIN_DISPLAY) {
            // Window cannot scroll. The top-left is specified by the WY and WX registers. 
            // They are in relation to the top-left of the physical screen.
            // Only display if WX=0..166, WY=0..143
            if self.wx < 166 && self.wy < 143 {
                if self.ly >= self.wy {
                    let map_y = self.ly - self.wy;

                    // X position is really WX - 7
                    let window_x = (self.wx as isize) - 7;

                    for screen_x in 0..SCREEN_WIDTH as u8 {
                        let screen_x_i = screen_x as isize;

                        if window_x <= screen_x_i {
                            let map_x_i = screen_x_i - window_x;

                            if map_x_i >= 0 && map_x_i < 256 {
                                self.draw_win_tile_pixel(map_x_i as u8, map_y, screen_x, self.ly, screen_buffer);
                            }
                        }
                    }
                }
            }
        }
    }

    // Draws a background tile at the given coordinates.
    // map_x and map_y are the coordinates in the tile map
    // screen_x and screen_y are the coordinates of the physical screen
    fn draw_bg_tile_pixel(&self, map_x: u8, map_y: u8, screen_x: u8, screen_y: u8, screen_buffer: &mut [u32]) {
        let map_addr_base = if self.lcdc.contains(Lcdc::LCDC_BG_TILE_9C) {
            0x9C00
        } else {
            0x9800
        } as u16;

        self.draw_tile_pixel(map_addr_base, map_x, map_y, screen_x, screen_y, screen_buffer);
    }

    // Draws a window tile at the given coordinates.
    // map_x and map_y are the coordinates in the tile map
    // screen_x and screen_y are the coordinates of the physical screen
    fn draw_win_tile_pixel(&self, map_x: u8, map_y: u8, screen_x: u8, screen_y: u8, screen_buffer: &mut [u32]) {
        let map_addr_base = if self.lcdc.contains(Lcdc::LCDC_WIN_TILE_9C) {
            0x9C00
        } else {
            0x9800
        } as u16;

        self.draw_tile_pixel(map_addr_base, map_x, map_y, screen_x, screen_y, screen_buffer);
    }

    // Draws a tile at the given coordinates.
    // map_addr_base is either 0x9C00 or 0x9800
    // map_x and map_y are the coordinates in the tile map
    // screen_x and screen_y are the coordinates of the physical screen
    fn draw_tile_pixel(&self, map_addr_base: u16, map_x: u8, map_y: u8, screen_x: u8, screen_y: u8, screen_buffer: &mut [u32]) {
        // Each tile is 8x8 pixels.
        // Get the X,Y positions of the tile boundaries
        let map_tile_y = (map_y / 8) as u16;
        let map_tile_x = (map_x / 8) as u16;

        // Get the X,Y coordinates within the tile for the given map X,Y
        let tile_row = map_y & 7;
        let tile_column = map_x & 7;

        // Background Tile Table is a 32x32 byte array containing the the index of each tile
        let tile_idx_addr = map_addr_base + (map_tile_y * 32) + map_tile_x;
        let tile_idx = self.vram[(tile_idx_addr - VIDEO_RAM_START) as usize];

        // The tile index is used to get the address in the Tile Pattern Table that contains the tile pixel data
        let tile_addr = self.tile_address(tile_idx);

        // Read the Tile Pattern Table to get what color pixel to display at the given screen coordinates
        let color = self.tile_pixel_color(tile_addr, tile_row, tile_column);

        screen_buffer[(screen_y as usize * SCREEN_WIDTH) + screen_x as usize] = self.bgp.rgb(color);
    }

    // Draws sprites on the current line
    fn draw_sprites_current_line(&self, screen_buffer: &mut [u32]) {
        if self.lcdc.contains(Lcdc::LCDC_SPRITE_DISPLAY) {
            let screen_y = self.ly as isize;

            // Sprites can be 8x8 or 8x16
            let sprite_height = if self.lcdc.contains(Lcdc::LCDC_8X16_SPRITE) { 16 } else { 8 };

            // Find all sprites that will be visible on the current line
            let line_sprite_idxs = (0..40 as usize).filter(|&idx| {
                let entry = &self.oam[idx as usize];

                // X,Y values are offset by 8,16
                let y = (entry.y as isize) - 16;
                let x = (entry.x as isize) - 8;

                let end_y = y + (sprite_height - 1);

               return x >= 0 && x < 160 && screen_y >= y && screen_y <= end_y;
            }).collect::<Vec<_>>();

            for screen_x in 0..SCREEN_WIDTH as isize { 
                // Sprites are 40 blocks in OAM. Each block is 32 bits
                for idx in &line_sprite_idxs {
                    let entry = &self.oam[*idx];

                    // X,Y values are offset by 8,16
                    let y = (entry.y as isize) - 16;
                    let x = (entry.x as isize) - 8;

                    let end_x = x + 7;

                    if screen_x >= x && screen_x <= end_x {
                        let tile_addr = self.sprite_tile_address(entry.tile);
                
                        // In DMG, the sprite can use one of two palletes
                        let palette = if entry.attrs.contains(OamAttr::OAM_ATTR_PALETTE_DMG) { 
                            self.obp1
                        } else { 
                            self.obp0
                        };

                        let row = if entry.attrs.contains(OamAttr::OAM_ATTR_Y_FLIP) {
                            (screen_y - y - sprite_height).abs()
                        } else {
                            screen_y - y
                        };

                        // Sprite can also be horizontally flipped
                        let column = if entry.attrs.contains(OamAttr::OAM_ATTR_X_FLIP) {
                            (screen_x - x - 7).abs()
                        } else {
                            screen_x - x
                        };

                        let color = self.tile_pixel_color(tile_addr, row as u8, column as u8);

                        // Color 0 is hidden for sprites
                        if color != 0 {
                            let screen_buffer_idx = (screen_y as usize * SCREEN_WIDTH) + screen_x as usize;

                            screen_buffer[screen_buffer_idx] = palette.rgb(color);
                        }
                    }
                }
            }
        }
    }

    // Gets the color of a pixel within a tile
    fn tile_pixel_color(&self, address: u16, row: u8, column: u8) -> ColorIndex {
        let upper_byte = self.vram[((address + (row * 2) as u16) + 1 - VIDEO_RAM_START) as usize];
        let lower_byte = self.vram[((address + (row * 2) as u16) - VIDEO_RAM_START) as usize];

        
        let shift = 7 - column;
        let upper_bit = (upper_byte >> shift) & 0b1;
        let lower_bit = (lower_byte >> shift) & 0b1;

        (upper_bit << 1) | lower_bit
    }

    // A cycle can raise a STAT interrupt when LY matches LYC
    fn check_coincidence(&mut self, result: &mut StepResult) {
        if self.stat.contains(Stat::STAT_COINCIDENCE_INT) {
            if self.ly == self.lyc {
                self.stat.insert(Stat::STAT_COINCIDENCE_EQUAL);
                result.int_stat = true;
            } else {
                self.stat.remove(Stat::STAT_COINCIDENCE_EQUAL);
            }
        } else {
            self.stat.remove(Stat::STAT_COINCIDENCE_EQUAL);
        }
    }

    // Gets the memory address for the given tile
    fn tile_address(&self, tile_index: u8) -> u16 {
        // Tile data can be in two spots. Either 0x8000 where the patterns are indexed with unsigned numbers,
        // or 0x9000 where the indexes are signed 
        let tile_addr_base = if self.lcdc.contains(Lcdc::LCDC_UNSIGNED_TILE_DATA) {
            0x8000
        } else {
            0x9000
        } as u16;

        if self.lcdc.contains(Lcdc::LCDC_UNSIGNED_TILE_DATA) {
            tile_addr_base + ((tile_index as u16) * 16)
        } else {
            let signed_index = (tile_index as i8) as i16;
            ((tile_addr_base as i16) + ((signed_index) * 16)) as u16
        }
    }

    // Gets the memory address for the given sprite tile index
    // Sprite pattern numbers are always unsigned starting in address 0x8000
    #[inline(always)]
    fn sprite_tile_address(&self, tile_index: u8) -> u16 {
        0x8000 + ((tile_index as u16) * 16)
    }
}

impl Addressable for Lcd {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            VIDEO_RAM_START..=VIDEO_RAM_END => self.vram[(addr - VIDEO_RAM_START) as usize],
            OAM_START..=OAM_END => {
                let start = (addr - OAM_START) as usize;
                let entry_idx = start / 4;
                let offset = start % 4;
                let entry = &self.oam[entry_idx];

                match offset {
                    0 => entry.y,
                    1 => entry.x,
                    2 => entry.tile,
                    3 => entry.attrs.bits(),
                    _ => unreachable!()
                }
            },
            ADDR_LCDC => self.lcdc.bits,
            ADDR_STAT => 0b1000_0000 | ((self.stat.bits & 0b0111_1100) | (self.mode as u8)),
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
            VIDEO_RAM_START..=VIDEO_RAM_END => {
                // Only allow write if LCD is disabled or in Mode 00 (HBlank), 01 (VBLANK), or 10 (OAM)
                if !self.lcdc.contains(Lcdc::LCDC_ENABLED) || self.mode != Mode::Transfer {
                    self.vram[(addr - VIDEO_RAM_START) as usize] = val;
                } else {
                    println!("Attempted VRAM write during mode {}. Mode cycles: {}", self.mode as u8, self.mode_cycles);
                }
            },
            OAM_START..=OAM_END => {
                // Only allow write if LCD is disabled or in Mode 00 (HBlank) or 01 (VBLANK)
                if !self.lcdc.contains(Lcdc::LCDC_ENABLED) || self.mode == Mode::HBlank || self.mode == Mode::VBlank {
                    let start = (addr - OAM_START) as usize;
                    let entry_idx = start / 4;
                    let offset = start % 4;
                    let mut entry = &mut self.oam[entry_idx];

                    match offset {
                        0 => { entry.y = val },
                        1 => { entry.x = val },
                        2 => { entry.tile = val },
                        3 => { entry.attrs = OamAttr::from_bits(val).unwrap() },
                        _ => unreachable!()
                    }
                }
            },
            ADDR_LCDC => {
                self.lcdc = Lcdc::from_bits(val).unwrap();

                // Reset internal counters if display disabled
                // LCDC mode goes back to 0 (HBlank)
                if !self.lcdc.contains(Lcdc::LCDC_ENABLED) {
                    self.mode_cycles = 0;
                    self.ly = 0;
                    self.mode = Mode::HBlank;

                }
            },
            ADDR_STAT => self.stat.bits = val & 0b1111_1100,
            ADDR_SCY => self.scy = val,
            ADDR_SCX => self.scx = val,
            ADDR_LY => (), // read-only
            ADDR_LYC => self.lyc = val,
            ADDR_BGP => self.bgp = Palette(val),
            ADDR_OBP0 => self.obp0 = Palette(val),
            ADDR_OBP1 => self.obp1 = Palette(val),
            ADDR_WY => self.wy = val,
            ADDR_WX => self.wx = val,
            _ => println!("LCD IO write unimplemented {:#X} -> {:#X}", val, addr)
        }
    }
}