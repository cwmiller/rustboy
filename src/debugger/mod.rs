mod command;

use self::command::Command;
use bus::{Addressable, Bus};
use cpu::{Cpu, decode};
use std::io::{stdin, stdout, Write};
use std::process;

enum State {
    Running,
    BreakAfter(usize)
}

pub struct Debugger {
    state: State
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            state: State::Running
        }
    }

    #[inline(always)]
    pub fn should_break(&mut self, cpu: &Cpu) -> bool {
        match self.state {
            State::Running => false,
            State::BreakAfter(steps) => {
                if steps == 0 {
                    true
                } else {
                    self.state = State::BreakAfter(steps - 1);
                    false
                }
            }
        }
    }

    pub fn brk(&mut self, cpu: &mut Cpu, bus: &mut Bus) {
        loop {
            print!("{:#X}> ", cpu.regs.pc());
            let _ = stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if input.len() > 0 {
                match Command::parse(&input, cpu.regs.pc()) { 
                    Ok(command) => match command {
                        Command::Continue => {
                            self.state = State::Running;
                            break;
                        },
                        Command::Disassemble(addr, count) => self.cmd_disassemble(&bus, addr, count),
                        Command::Help => {
                            println!("Command\t\t\tDescription");
                            println!("c\t\t\tContinue Execution");
                            println!("d [addr] [count]\tDisassemble");
                            println!("h\t\t\tHelp");
                            println!("m [addr] [count]\tInspect Memory");
                            println!("q\t\t\tQuit");
                            println!("r\t\t\tInspect Registers");
                            println!("s [count]\t\tStep Instructions");

                        }
                        Command::Memory(_, _) => println!("Not yet implemented."),
                        Command::Quit => process::exit(0),
                        Command::Registers => println!("{:?}", cpu.regs),
                        Command::Step(steps) => {
                            self.state = State::BreakAfter(steps - 1);
                            break;
                        }
                    },
                    Err(err) => println!("Error: {}", err)
                }
            }
        }
    }

    fn cmd_disassemble(&self, bus: &Bus, addr: u16, count: usize) {
        let mut current = addr;
        let mut prefixed = false;

        for _ in 0..count {
            let base_addr = current;
            let opcode = bus.read(current);
            let decoded_instruction;
            let mut length = 1;
            

            {
                let mut imm8 = || { 
                    let byte = bus.read(addr.wrapping_add(length));
                    length = length + 1;
                    byte
                };

                decoded_instruction = decode(opcode, prefixed, &mut imm8);
            }

            let mut raw = "0x".to_string();
            for i in 0..length {
                raw = raw + &format!("{:02X}", bus.read(addr + i));
            }

            if decoded_instruction.is_some() {
                let instruction = decoded_instruction.unwrap();
                println!("{:#06X}\t{}\t\t{}", base_addr, raw, instruction);
            }

            current = current + length;
        }
    }
}