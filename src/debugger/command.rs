/*
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Command {
    AddBreakPoint(u16),
    Continue,
    Disassemble(u16, usize),
    Help,
    ListBreakPoints,
    Memory(u16, usize),
    Quit,
    RemoveBreakPoint(u16),
    Registers,
    Step(usize)
}

fn parse_str(val: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^0[xb]").unwrap();
    }

    if RE.is_match(val) {
        match &val[..2] {
            "0x" => usize::from_str_radix(&val[2..], 16),
            "0b" => usize::from_str_radix(&val[2..], 2),
            _ => usize::from_str(val)
        }
    } else {
        usize::from_str(val)
    }.unwrap()
}

impl Command {
    pub fn parse(line: &String, pc: u16) -> Result<Self, String> {
        let trimmed = line.trim();

        match trimmed.chars().nth(0).unwrap() {
            'b' => {
                if trimmed.len() == 1 {
                    Ok(Command::ListBreakPoints)
                } else {
                    match trimmed.chars().nth(1).unwrap() {
                        'a' => {
                            lazy_static! {
                                static ref RE: Regex = Regex::new(r"ba ([0-9a-fA-Fx]+)").unwrap();
                            }
                            if RE.is_match(trimmed) {
                                let caps = RE.captures(trimmed).unwrap();
                                let addr = caps.get(1).map_or(0, |m| parse_str(m.as_str()) as u16);

                                Ok(Command::AddBreakPoint(addr))
                            } else {
                                Err(String::from("Usage: ba [addr]"))
                            }
                        },
                        'r' => {
                            lazy_static! {
                                static ref RE: Regex = Regex::new(r"br ([0-9a-fx]+)").unwrap();
                            }
                            if RE.is_match(trimmed) {
                                let caps = RE.captures(trimmed).unwrap();
                                let addr = caps.get(1).map_or(0, |m| parse_str(m.as_str()) as u16);

                                Ok(Command::RemoveBreakPoint(addr))
                            } else {
                                Err(String::from("Usage: br [addr]"))
                            }
                        },
                        _ => Err(String::from("Unknown command"))
                    }
                }
            },
            'c' => Ok(Command::Continue),
            'd' => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"d(?:is)? ?([0-9a-fx]+)? ?([0-9a-fx]+)?").unwrap();
                }
                if RE.is_match(trimmed) {
                    let caps = RE.captures(trimmed).unwrap();
                    let addr = caps.get(1).map_or(pc, |m| parse_str(m.as_str()) as u16);
                    let length = caps.get(2).map_or(10, |m| parse_str(m.as_str()));

                    Ok(Command::Disassemble(addr, length))
                } else {
                    Err(String::from("Usage: d [addr] [length]"))
                }
            },
            'h' => Ok(Command::Help),
            'm' => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"m(?:em)? ([0-9a-fx]+) ?([0-9a-fx]+)?").unwrap();
                }
                if RE.is_match(trimmed) {
                    let caps = RE.captures(trimmed).unwrap();
                    let addr = caps.get(1).map_or(0, |m| parse_str(m.as_str()) as u16);
                    let length = caps.get(2).map_or(1, |m| parse_str(m.as_str()));

                    Ok(Command::Memory(addr, length))
                } else {
                    Err(String::from("Usage: m [addr] [length]"))
                }
            }
            'q' => Ok(Command::Quit),
            'r' => Ok(Command::Registers),
            's' => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"s(?:tep)? ?([0-9a-fx]+)?").unwrap();
                }
                if RE.is_match(trimmed) {
                    let caps = RE.captures(trimmed).unwrap();
                    let count = caps.get(1).map_or(1, |m| parse_str(m.as_str()));

                    Ok(Command::Step(count))
                } else {
                    Err(String::from("Usage: s [count]"))
                }
            },
            _ => Err(String::from("Unknown command"))
        }
    }
}
*/