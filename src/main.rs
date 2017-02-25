extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::Read;

mod bus;
mod cpu;
mod gb;
mod registers;

use gb::Gameboy;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut file = File::open(&file_name).unwrap();

    let mut rom: Vec<u8> = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut gb = Gameboy::new(rom);
    gb.power_on();
}