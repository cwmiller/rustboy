#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate minifb;

mod bus;
mod cartridge;
mod debugger;
mod cpu;
mod joypad;
mod lcd;
mod rustboy;
mod serial;
mod sound;
mod timer;

use cartridge::Cartridge;
use rustboy::Rustboy;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    if env::args().len() < 2 {
        let bin_arg = env::args().nth(0).unwrap();
        let bin_path = Path::new(bin_arg.as_str());

        eprintln!("Usage: {} [ROM]", bin_path.file_name().unwrap().to_str().unwrap());
        process::exit(1);
    }

    let rom_arg = env::args().nth(1).unwrap();
    let rom_path = Path::new(rom_arg.as_str());

    if !rom_path.is_file() {
        eprintln!("File {} does not exist.", rom_arg);
        process::exit(1);
    }

    let mut cart = Cartridge::new(rom_arg.as_str());

    println!("Loaded {}", rom_path.file_name().unwrap().to_str().unwrap());
    println!("{:?}", cart);

    let mut rustboy = Rustboy::new(&mut cart);
    rustboy.run();
}