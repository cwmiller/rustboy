use bus::Addressable;
use enum_primitive::FromPrimitive;

const ADDR_DIV: u16  = 0xFF04;
const ADDR_TIMA: u16 = 0xFF05;
const ADDR_TMA: u16  = 0xFF06;
const ADDR_TAC: u16  = 0xFF07;

enum_from_primitive! {
    #[derive(Copy, Clone)]
    enum TacFrequency {
        Khz4 = 0,
        Khz262 = 1,
        Khz65 = 2,
        Khz16 = 3
    }
}

impl TacFrequency {
    fn as_cycles(&self) -> usize {
        match *self {
            TacFrequency::Khz4 => 1024,
            TacFrequency::Khz16 => 256,
            TacFrequency::Khz65 => 64,
            TacFrequency::Khz262 => 16
        }
    }
}

#[derive(Default)]
pub struct TimerResult {
    pub interrupt: bool
}

pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac_enabled: bool,
    tac_freq: TacFrequency,

    div_counter: usize,
    tima_counter: usize
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac_enabled: false,
            tac_freq: TacFrequency::Khz4,
            div_counter: 0,
            tima_counter: 0
        }
    }

    // TODO: implement odd Timer behaviors
    pub fn step(&mut self, cycles: usize) -> TimerResult {
        let mut result = TimerResult::default();

        // DIV always counts at a constant interval even if the timer is disabled
        // DIV increments once every ~256 machine cycles
        self.div_counter += cycles;

        if self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256;
        }

        // TIMA counts if TAC is enabled. It counts at an interval set by TAC.
        if self.tac_enabled {
            self.tima_counter += cycles;

            let tac_cycles = self.tac_freq.as_cycles();

            if self.tima_counter >= tac_cycles {
                // When TIMA overflows, it is reset to TMA and an interrupt is raised.
                if self.tima == 0xFF {
                    self.tima = self.tma;
                    result.interrupt = true;
                } else {
                    self.tima += 1;
                }

                self.tima_counter -= tac_cycles;
            }
        }

        result
    }
}

impl Addressable for Timer {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            ADDR_DIV => (self.div >> 8) as u8, // Retrieving DIV only returns the MSB,
            ADDR_TIMA => self.tima,
            ADDR_TMA => self.tma,
            ADDR_TAC => 0b1111_1000 | ((self.tac_enabled as u8) << 2) | (self.tac_freq as u8),
            _ => { println!("Timer read unimplemented ({:#X})", addr); 0 }
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // Writing any value to DIV resets it to 0
            ADDR_DIV => self.div = 0,
            ADDR_TIMA => self.tima = val,
            ADDR_TMA => self.tma = val,
            ADDR_TAC => {
                self.tac_enabled = (val & 0b100) == 0b100;
                self.tac_freq = TacFrequency::from_u8(val & 0b11).unwrap();
            },
            _ => println!("Timer write unimplemented {:#X} -> {:#X}", val, addr)
        }
    }
}