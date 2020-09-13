#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate clap;
#[macro_use]
extern crate enum_primitive;
extern crate log;
extern crate minifb;

mod bus;
mod cartridge;
mod debugger;
mod cpu;
mod joypad;
mod lcd;
mod logger;
mod rustboy;
mod serial;
mod sound;
mod timer;

use rustboy::RustboyOptions;
use cartridge::Cartridge;
use clap::{Arg, App};
use logger::{Logger};
use log::{error, info, LevelFilter};
use rustboy::Rustboy;
use std::path::Path;
use std::process;

use minifb::{Scale};

static LOGGER: Logger = Logger;

fn main() {
    let matches = App::new("Rustboy")
        .arg(Arg::with_name("scale")
            .long("scale")
            .value_name("SCALE")
            .default_value("4")
            .help("Sets the scale the display runs at (1, 2, 4, 8, 16, 32)")
            .takes_value(true))

        .arg(Arg::with_name("ROM")
            .help("Sets the ROM filename to run")
            .required(true)
            .index(1))

        .arg(Arg::with_name("unlock-fps")
            .long("unlock-fps")
            .multiple(false)
            .help("Disable limiting to 60fps"))

        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Set verbosity level. (1 - 3)"))

        .get_matches();

    // Configure logging
    let log_level = match matches.occurrences_of("v") {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug
    };

    log::set_logger(&LOGGER).map(|()| log::set_max_level(log_level)).unwrap();

    let rom_arg = matches.value_of("ROM").unwrap();
    let rom_path = Path::new(rom_arg);

    if !rom_path.is_file() {
        error!("File {} does not exist.", rom_arg);

        process::exit(1);
    }

    let mut cart = Cartridge::new(rom_arg);

    info!("Loaded {}", rom_path.file_name().unwrap().to_str().unwrap());
    info!("{:?}", cart);

    let scale = match matches.value_of("scale").unwrap() {
        "1" => Scale::X1,
        "2" => Scale::X2,
        "4" => Scale::X4,
        "8" => Scale::X8,
        "16" => Scale::X16,
        "32" => Scale::X32,
        _ => {
            error!("Invalid scale setting");
            process::exit(1);
        }
    };

    let options = RustboyOptions {
        scale: scale,
        unlock_fps: matches.is_present("unlock-fps")
    };

    let mut rustboy = Rustboy::new(&mut cart, options);
    rustboy.run();
}