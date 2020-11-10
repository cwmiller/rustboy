use bus::Addressable;

const ADDR_SB: u16 = 0xFF01;
const ADDR_SC: u16 = 0xFF02;

// Internal clock for the serial port runs at 8192Hz
const INT_CLOCK_CYCLES: usize = 8192;

bitflags! {
    struct Sc: u8 {
        const SC_START_TRANSFER    = 0b1000_0000;   // Setting bit enables transfer. Bit is reset upon completion.
        const SC_FAST_SPEED_GBC    = 0b0000_0010;
        const SC_CLOCK             = 0b0000_0001;   // Setting bit enables internal clock.
    }
}

#[derive(Default)]
pub struct SerialResult {
    pub interrupt: bool         // Interrupt is raised after a byte is transferred
}

pub struct Serial {
    sb: u8,                     // SB stores the byte of data to be transferred out the serial port. It is replaced with a byte coming from the other side.
    sc: Sc,                     // SC controls the serial interface
    transfer_bit: usize,        // The current bit being transferred out of the serial connection
    transfer_bit_cycles: usize  // Running tally of clock cycles for the current bit transfer
}

impl Serial {
    pub fn new() -> Self {
        Self { 
            sb: 0,
            sc: Sc::empty(),
            transfer_bit: 0,
            transfer_bit_cycles: 0
        }
    }

    pub fn step(&mut self, cycles: usize) -> SerialResult {
        let mut interrupt = false;

        // When a serial transfer is initiated and the GB is using an internal clock, a bit is transferred at a rate of 8192Hz. 
        // Nothing is done when using an external clock. I don't see any other GB around here!
        if self.sc.contains(Sc::SC_CLOCK) && self.sc.contains(Sc::SC_START_TRANSFER) {
            self.transfer_bit_cycles += cycles;

            // On every tick of the serial clock, shift a bit off of SB
            // In a real GB, the bit from the other GB would be shifted in
            if self.transfer_bit_cycles >= INT_CLOCK_CYCLES {
                self.sb = self.sb << 1;

                // When all 8 bits are transferred, raise an interrupt and update SC to disable transfer
                if self.transfer_bit == 7 {
                    interrupt = true;

                    self.transfer_bit = 0;
                    self.sc.remove(Sc::SC_START_TRANSFER);
                } else {
                    self.transfer_bit += 1;
                }
            }
        }

        SerialResult {
            interrupt: interrupt
        }
    }
}

impl Addressable for Serial {
    fn read(&self, addr: u16) -> u8 {
        return match addr {
            ADDR_SB => self.sb,
            ADDR_SC => self.sc.bits(),
            _ => unreachable!()
        }
    }

    fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            ADDR_SB => self.sb = byte,
            ADDR_SC => self.sc = Sc::from_bits(byte & 0b1000_0001).unwrap(),
            _ => unreachable!()
        };
    }
}