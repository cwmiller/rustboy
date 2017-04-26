mod command;

use self::command::Command;
use bus::{Addressable, Bus};
use cpu::{Cpu, Instruction, decode};
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::process;

enum State {
    Running,
    BreakAfter(usize)
}

pub struct Debugger {
    breakpoints: HashSet<u16>,
    state: State
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: HashSet::new(),
            state: State::Running
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
            print!("{:#X}> ", cpu.regs.pc());
            let _ = stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            if input.len() > 0 {
                match Command::parse(&input, cpu.regs.pc()) { 
                    Ok(command) => match command {
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
                        Command::Disassemble(addr, count) => self.cmd_disassemble(&bus, addr, count),
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
                raw = raw + &format!("{:02X}", bus.read(base_addr + i));
            }

            if decoded_instruction.is_some() {
                let instruction = decoded_instruction.unwrap();
                let padding = iter::repeat(" ").take(10 - (length as usize * 2)).collect::<String>();
                
                println!("{:#06X}\t{}{}{}", base_addr, raw, padding, instruction);

                prefixed = match instruction {
                    Instruction::Prefix => true,
                    _ => false
                };
            }

            current = current + length;
        }
    }
}