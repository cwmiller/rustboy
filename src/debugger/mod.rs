/*
mod command;

use self::command::Command;
use bus::{Addressable, Bus};
use cpu::{Cpu, decode};
use fnv::FnvHashSet;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::process;

enum State {
    Running,
    BreakAfter(usize)
}

pub struct Debugger {
    breakpoints: FnvHashSet<u16>,
    state: State,
    previous_command: Command
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: FnvHashSet::default(),
            state: State::Running,
            previous_command: Command::Continue
        }
    }

    #[inline(always)]
    pub fn should_break(&mut self, cpu: &Cpu) -> bool {
        match self.state {
            State::Running => {
                self.breakpoints.contains(&cpu.regs.pc())
            },
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
            self.print_disassembly(&bus, cpu.regs.pc(), 1);

            print!("> ");
            let _ = stdout().flush();

            let mut input = String::new();

            if let Ok(_) = stdin().read_line(&mut input) {
                let command = if input.trim().len() == 0 {
                    Ok(self.previous_command)
                } else {
                    Command::parse(&input, cpu.regs.pc())
                };

                match command {
                    Ok(command) => {
                        self.previous_command = command;

                        match command {
                            Command::AddBreakPoint(addr) => {
                                self.breakpoints.insert(addr);
                            },
                            Command::ListBreakPoints => {
                                for addr in &self.breakpoints {
                                    println!("{:#X}", *addr);
                                }
                            },
                            Command::Continue => {
                                self.state = State::Running;
                                break;
                            },
                            Command::Disassemble(addr, count) => self.print_disassembly(&bus, addr, count),
                            Command::Help => {
                                println!("Command\t\t\tDescription");
                                println!("b\t\t\tList Break Points");
                                println!("ba [addr]\t\tAdd Break Point");
                                println!("br [addr]\t\tRemove Break Point");
                                println!("c\t\t\tContinue Execution");
                                println!("d [addr] [count]\tDisassemble");
                                println!("h\t\t\tHelp");
                                println!("m [addr] [count]\tInspect Memory");
                                println!("q\t\t\tQuit");
                                println!("r\t\t\tInspect Registers");
                                println!("s [count]\t\tStep Instructions");
                            }
                            Command::Memory(addr, count) => {
                                print!("{:#04X} = ", addr);
                                for i in 0..count {
                                    print!("{:02X} ", bus.read(addr.wrapping_add(i as u16)));
                                }
                                println!();
                            },
                            Command::Quit => process::exit(0),
                            Command::RemoveBreakPoint(addr) => {
                                self.breakpoints.remove(&addr);
                            }
                            Command::Registers => println!("{:?}", cpu.regs),
                            Command::Step(steps) => {
                                self.state = State::BreakAfter(steps - 1);
                                break;
                            }
                        };
                    },
                    Err(err) => println!("Error: {}", err)
                }
            }
        }
    }

    fn print_disassembly(&self, bus: &Bus, addr: u16, count: usize) {
        let mut instruction_addr = addr;

        for _ in 0..count {
            let mut opcode = bus.read(instruction_addr);
            let mut prefixed = false;

            let (decoded_instruction, length) = {
                let mut length = 1;

                if opcode == 0xCB {
                    opcode = bus.read(instruction_addr.wrapping_add(1));
                    length = length + 1;
                    prefixed = true;
                }

                ({
                    let mut next = || {
                        let byte = bus.read(addr.wrapping_add(length));
                        length = length + 1;
                        byte
                    };

                    decode(opcode, prefixed, &mut next)
                }, length)
            };

            if let Some(instruction) = decoded_instruction {
                let hex = "0x".to_string() + &(0..length).map(|offset| format!("{:02X}", bus.read(instruction_addr + offset))).collect::<String>();
                let padding = iter::repeat(" ").take(10 - (length as usize * 2)).collect::<String>();
                
                println!("{:#06X}\t{}{}{}", instruction_addr, hex, padding, instruction);
            }

            instruction_addr = instruction_addr + length;
        }
    }
}
*/