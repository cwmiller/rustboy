use std::io::{stdin, stdout, Write};

use gb::Gameboy;
use super::command::Command;
use super::disassembler::disassemble;

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
                    Command::Continue => println!("Not yet implemented."),
                    Command::Registers => println!("Not yet implemented."),
                    Command::Disassemble(pc, count) => self.cmd_disassemble(pc, count),
                    Command::Memory(_, _) => println!("Not yet implemented."),
                    Command::Quit => break
                },
                Err(err) => println!("Error: {}", err)
            }
        }
    }

    fn cmd_disassemble(&self, pc: u16, count: usize) {
        let bus = &self.gb.cpu.bus;
        let mut current = pc;
        let mut prefixed = false;

        for _ in 0..count {
            let addr = current;
            let mnemonic = disassemble(bus, &mut current, &mut prefixed);

            println!("{:#X}\t{}", addr, mnemonic);
        }
    }
}