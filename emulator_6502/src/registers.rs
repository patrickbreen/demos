use std::collections::HashMap;

/// This represents the registers of the 6502 CPU. This, holds (most of) the state of the CPU.
///
/// Because this struct derives `Debug` it can be easily printed for debugging using:
///
/// `println!("{:?}", registers);`
///
/// Most of the logic in this type is restricted to getting and setting the fields in the flag
/// bitfield.
#[derive(Debug)]
pub struct Registers {
    /// The accumulator
    pub a: u8,

    /// A general purpose register
    pub x: u8,

    /// A general purpose register
    pub y: u8,

    /// The stack pointer (this only uses 9 bits)
    pub s: u16,

    /// The program counter
    pub pc: u16,

    /// The flag bitfield N|V|1|B|D|I|Z|C
    pub p: u8,

    /// An internal conversion of the flag field label and the flag mask
    fm: HashMap<char, u8>,

    pub stack_page: usize,
    pub magic: u8,
    pub running: bool,
    pub interrupts: HashMap<String, usize>,

    /// The cycle count
    pub cc: u32,
}

impl Registers {

    pub fn new() -> Registers {
        let mut map = HashMap::new();

        map.insert('N', 128);
        map.insert('V', 64);
        map.insert('B', 16);
        map.insert('D', 8);
        map.insert('I', 4);
        map.insert('Z', 2);
        map.insert('C', 1);

        // I believe that these are the hard coded locations in memory that represent the
        // interrupt pins/buses.
        let mut interrupts: HashMap<String, usize> = HashMap::new();
        interrupts.insert("ABORT".to_string(), 0xFFF8);
        interrupts.insert("COP".to_string(), 0xFFF4);
        interrupts.insert("IRQ".to_string(), 0xFFFe);
        interrupts.insert("BRK".to_string(), 0xFFFe);
        interrupts.insert("NMI".to_string(), 0xFFFa);
        interrupts.insert("RESET".to_string(), 0xFFFc);

        Registers {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFF,
            pc: 0,
            fm: map,
            p: 0b00100100,
            stack_page: 0x1,
            magic: 0xEE,
            running: true,
            interrupts: interrupts,
            cc: 0,
        }
    }

    pub fn get_flag(&self, flag: char) -> bool {
        (self.p & self.fm.get(&flag).unwrap()) != 0
    }

    pub fn set_flag(&mut self, flag: char, value: bool) {
        if value {
            self.p = self.p | self.fm.get(&flag).unwrap();
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn clear_flag(&mut self, flag: char) {
        self.p = self.p & (255 - self.fm.get(&flag).unwrap());
    }

    pub fn clear_flags(&mut self) {
        self.p = 0;
    }

    pub fn zn(&mut self, value: u8) {
        self.set_flag('Z', value == 0);
        self.set_flag('N', (value & 0x80) != 0);
    }    
}

#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test_flags() {
        let mut r = Registers::new();
        r.set_flag('N', true);
        assert_eq!(r.get_flag('N'), true);

        r.clear_flag('N');
        assert_eq!(r.get_flag('N'), false);

        r.set_flag('Z', true);
        assert_eq!(r.get_flag('Z'), true);

        r.clear_flag('Z');
        assert_eq!(r.get_flag('Z'), false);

        // this flag should already be set by the constructor
        assert_eq!(r.get_flag('I'), true);

        // clear all flags
        r.clear_flags();
        assert_eq!(r.get_flag('I'), false);
    }
}