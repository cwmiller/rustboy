extern crate byteorder;
extern crate regex;

#[macro_use] 
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;
mod debugger;
mod gb;

use gb::Gameboy;
use debugger::Console;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut file = File::open(&file_name).unwrap();

    let mut rom: Vec<u8> = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let gb = Gameboy::new(rom);
    let mut debugger = Console::new(gb);

    debugger.init();
}