use std::collections::HashMap;

use mmu::MMU;
use registers::Registers;


struct CPU <'a> {

    // a: &'a usize,

    /// The MMU, modeled here as "owned" by the CPU
    mmu: MMU,

    /// The registers of the CPU
    r: Registers,

    /// The cycle count
    cc: u32,

    // ops is a table of functions with offsets in the table given by an opcode (u8)
    // the values in the op table are functions that borrow a CPU and no return
    // value. The borrowed references to the ops have the same lifetime as the CPU

    // opcode: u8 -> func(&mut CPU)

    // this is usually called a jump table
    // TODO: might want to make this be a stand alone object that is passed to the 
    // step method. That way, CPU could derive Debug and other things that would
    // be convenient to derive. Also that would make the lifetime issue easier...
    ops: [fn(&mut CPU<'a>, u16); 256],

    stack_page: usize,
    magic: u8,
    running: bool,
    interrupts: HashMap<String, usize>,
}

impl <'a> CPU <'a> {

    // implement ops
    fn op_not_implemented(&mut self, src: u16) {
        panic!("Error, this op is not implemented.")
    }

    // add memory to accumulator with carry
    fn op_adc(&mut self, src: u16) {
        panic!("Error, this op is not implemented.")
    }

    // and
    fn op_and(&mut self, src: u16) {
        self.r.a = (self.r.a & (src as u8)) & 0xFF;
        let flag = self.r.a;
        self.r.zn(flag);
    }

    /// initialize the CPU and return it
    fn new(mmu: MMU) -> CPU<'a> {

        // I believe that these are the hard coded locations in memory that represent the
        // interrupt pins/buses.
        let mut interrupts: HashMap<String, usize> = HashMap::new();
        interrupts.insert("ABORT".to_string(), 0xFFF8);
        interrupts.insert("COP".to_string(), 0xFFF4);
        interrupts.insert("IRQ".to_string(), 0xFFFe);
        interrupts.insert("BRK".to_string(), 0xFFFe);
        interrupts.insert("NMI".to_string(), 0xFFFa);
        interrupts.insert("RESET".to_string(), 0xFFFc);



        CPU {
            mmu: mmu,
            r: Registers::new(),
            cc: 0,
            ops: [CPU::op_not_implemented; 256],

            stack_page: 0x1,
            magic: 0xEE,
            running: true,
            interrupts: interrupts,
        }
    }

    // 1) read a byte (opcode)
    // 2) decode using optable to give op function
    // 3) execute op
    fn step(&self) {

    }

    fn next_byte(&mut self) -> u8 {
        let val = self.mmu.read(self.r.pc as usize);
        self.r.pc += 1;
        val
    }

    fn next_word(&mut self) -> u16 {
        // little endian
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;

        (high << 8) + low
    }

    fn stack_push(&mut self, val: u8) {
        self.mmu.write(self.stack_page*0x100 + self.r.s as usize, val);
        self.r.s = (self.r.s - 1) & 0xFF;
    }

    fn stack_push_word(&mut self, val: u8) {
        self.stack_push(val >> 8);
        self.stack_push(val & 0xFF);
    }

    fn stack_pop(&mut self) -> u8 {
        let val = self.mmu.read(self.stack_page*0x100 + ((self.r.s as usize + 1) & 0xFF));
        self.r.s = (self.r.s + 1) & 0xFF;
        val
    }

    fn stack_pop_word(&mut self) -> u16 {
        (self.stack_pop() as u16) + ((self.stack_pop() << 8) as u16)
    }

    fn from_bcd(&self, val: u16) -> u16 {
        (((val & 0xF0) / 0x10) * 10) + (val & 0xF)
    }

    fn to_bcd(&self, val: u16) -> u16 {
        val / 10 * 16 + (val % 10)
    }

    fn from_twos_com(&self, val: u16) -> u16 {
        (val & 0x7F) - (val & 0x80)
    }

    fn interrupt_address(&mut self, interrupt: String) -> u16 {
        self.mmu.read_word(self.interrupts[&interrupt])
    }

    // ---- addressing modes ----
    // I'm making these functions return a u16 address for accuracy (as they should),
    // even though the MMU actually uses usize internally. It will be casted later.

    // Note that the width of the address bus on the target CPU (6502) is actually 16 bits.

    // zero page addressing
    fn z_a(&mut self) -> u16 {
        self.next_byte() as u16
    }

    fn zx_a(&mut self) -> u16 {
        ((self.next_byte() + self.r.x) & 0xFF) as u16
    }

    fn zy_a(&mut self) -> u16 {
        ((self.next_byte() + self.r.y) & 0xFF) as u16
    }

    // absolute addressing
    fn a_a(&mut self) -> u16 {
        self.next_word()
    }

    fn ax_a(&mut self) -> u16 {
        let op = self.next_word();
        let a = op + (self.r.x as u16);

        if op / 0xFF != a / 0xFF {
            self.cc += 1;
        }

        a & 0xFFFF
    }

    fn ay_a(&mut self) -> u16 {
        let op = self.next_word();
        let a = op + (self.r.y as u16);

        if op / 0xFF != a / 0xFF {
            self.cc += 1;
        }

        a & 0xFFFF
    }

    // indirect addressing
    fn i_a(&mut self) -> u16 {
        let i = self.next_word();
        // Doesn't carry, so if the low byte is in the XXFF position
        // Then the high byte will be XX00 rather than XY00
        let j: u16;
        if i & 0xFF == 0xFF {
            j = i - 0xFF;
        }
        else {
            j = i + 1;
        }

        (((self.mmu.read(j as usize) as u16) << 8) + self.mmu.read(i as usize) as u16) & 0xFFFF
    }


    fn ix_a(&mut self) -> u16 {
        let i = (self.next_byte() + self.r.x) & 0xFF;
        let u = self.mmu.read(((i + 1) & 0xff) as usize);
        let l = self.mmu.read(i as usize);
        (((u as u16) << 8) + l as u16) & 0xffff
    }

    fn iy_a(&mut self) -> u16 {
        let i = self.next_byte();
        let u = self.mmu.read((i as usize + 1) & 0xFF);
        let l = self.mmu.read(i as usize);
        let o = ((u as u16) << 8) + (l as u16);
        let a = o + (self.r.y as u16);

        if o / 0xFF != a / 0xFF {
            self.cc += 1;
        }


        a & 0xFFFF
        }

    // ---- read value for each addressing mode ----

    // Note that though the width of the address bus is 16 bits, the data bus is 8 bits. These
    // memory fetchs all return u8's and hence the 6502 is considered to be an 8bit processor...

    // immediate
    fn im(&mut self) -> u8 {
        self.next_byte()
    }

    // zero page addressing
    fn z(&mut self) -> u8 {
        let addr = self.z_a();
        self.mmu.read(addr as usize)
    }

    fn zx(&mut self) -> u8 {
        let addr = self.zx_a();
        self.mmu.read(addr as usize)
    }

    fn zy(&mut self) -> u8 {
        let addr = self.zy_a();
        self.mmu.read(addr as usize)
    }

    // absolute addressing
    fn a(&mut self) -> u8 {
        let addr = self.a_a();
        self.mmu.read(addr as usize)
    }

    fn ax(&mut self) -> u8 {
        let addr = self.ax_a();
        self.mmu.read(addr as usize)
    }

    fn ay(&mut self) -> u8 {
        let addr = self.ay_a();
        self.mmu.read(addr as usize)
    }

    // indirect addressing
    fn i(&mut self) -> u8 {
        let addr = self.i_a();
        self.mmu.read(addr as usize)
    }

    fn ix(&mut self) -> u8 {
        let addr = self.ix_a();
        self.mmu.read(addr as usize)
    }

    fn iy(&mut self) -> u8 {
        let addr = self.iy_a();
        self.mmu.read(addr as usize)
    }
}

#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    #[test]
    fn test_construct_cpu() {
        let mmu = MMU::new(&Vec::new());
        let cpu = CPU::new(mmu);
    }

    // test all ops
    #[test]
    fn test_op_not_implemented() {

    }

    #[test]
    fn test_adc() {

    }

        #[test]
    fn test_and() {

    }
}