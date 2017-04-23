use regex::Regex;
use std::str::FromStr;

pub enum Command {
    Continue,
    Disassemble(u16, usize),
    Help,
    Memory(u16, usize),
    Quit,
    Registers,
    Step(usize)
}

fn parse_str(val: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^0[xXb]").unwrap();
    }

    if RE.is_match(val) {
        let prefix = &val[..2];
        match prefix {
            "0x" => usize::from_str_radix(&val[2..], 16),
            "0b" => usize::from_str_radix(&val[2..], 2),
            _ => usize::from_str(&val)
        }
    } else {
        usize::from_str(&val)
    }.unwrap()
}

impl Command {
    pub fn parse(line: &String, pc: u16) -> Result<Self, String> {
        match line.chars().nth(0).unwrap() {
            'c' => Ok(Command::Continue),
            'd' => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"d(?:is)? ?([0-9a-fx]+)? ?([0-9a-fx]+)?").unwrap();
                }
                if RE.is_match(line) {
                    let caps = RE.captures(line).unwrap();
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
                if RE.is_match(line) {
                    let caps = RE.captures(line).unwrap();
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
                if RE.is_match(line) {
                    let caps = RE.captures(line).unwrap();
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