use std::io::{stdin, stdout, Write};

use super::super::gb::Gameboy;
use super::command::Command;

pub struct Console {
    gb: Gameboy
}

impl Console {
    pub fn new(gb: Gameboy) -> Console {
        Console {
            gb: gb
        }
    }

    pub fn init(&mut self) {
        self.gb.power_on();

        loop {
            print!("{:#X}> ", self.gb.cpu.regs.pc);
            let _ = stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            match Command::parse(&input, self.gb.cpu.regs.pc) { 
                Ok(command) => match command {
                    Command::Step(steps) => self.gb.step(steps),
                    Command::Continue => { },
                    Command::Registers => { },
                    Command::Disassemble(_, _) => { },
                    Command::Memory(_, _) => { },
                    Command::Quit => break
                },
                Err(err) => println!("Error: {}", err)
            }
        }
    }
}