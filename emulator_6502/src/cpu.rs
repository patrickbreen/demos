use std::collections::HashMap;

use mmu::{MMU, Block};
use registers::Registers;


// TODO: this is too simplistic. Needs to track the cycle count, whether it acts
//  on values or addresses and target register if valid
#[derive(Copy, Clone)]
struct Instr {
    addr: fn(&mut CPU) -> u16,
    code: fn(&mut CPU, u16),
}

impl Instr {
    fn new(addr: fn(&mut CPU) -> u16, code: fn(&mut CPU, u16)) -> Instr {
        Instr {
            addr: addr,
            code: code,
        }
    }
}

struct CPU {

    /// The MMU, modeled here as "owned" by the CPU
    mmu: MMU,

    /// The registers of the CPU
    r: Registers,

    // ops is a table of functions with offsets in the table given by an opcode (u8)
    // the values in the op table are functions that borrow a CPU and no return
    // value. The borrowed references to the ops have the same lifetime as the CPU

    // opcode: u8 -> func(&mut CPU, u8)

    // this is often called a jump table (though it isn't used much in high level code)

    ops: [Instr; 256],
}

impl CPU {

    // implement ops
    fn op_not_implemented(&mut self, src: u16) {
        panic!("Error, this op is not implemented.")
    }

    // add memory to accumulator with carry
    // if 8 bit addition was all you had!!!!
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
    fn new(mmu: MMU) -> CPU {

        let mut cpu = CPU {
            mmu: mmu,
            r: Registers::new(),
            ops: [Instr::new(CPU::im, CPU::op_not_implemented); 256],
        };

        // TODO: set up op table
        // cpu.ops[0x69] = Instr::new(CPU::im, CPU::op_adc);
        // cpu.ops[0x65] = Instr::new(CPU::z, CPU::op_adc);
        // cpu.ops[0x75] = Instr::new(CPU::zx, CPU::op_adc);
        // cpu.ops[0x75] = Instr::new(CPU::zx, CPU::op_adc);

        cpu.ops[0x29] = Instr::new(CPU::im, CPU::op_and);
        cpu.ops[0x25] = Instr::new(CPU::z, CPU::op_and);
        cpu.ops[0x35] = Instr::new(CPU::zx, CPU::op_and);
        cpu.ops[0x2D] = Instr::new(CPU::a, CPU::op_and);
        cpu.ops[0x3D] = Instr::new(CPU::ax, CPU::op_and);
        cpu.ops[0x39] = Instr::new(CPU::ay, CPU::op_and);
        cpu.ops[0x21] = Instr::new(CPU::ix, CPU::op_and);
        cpu.ops[0x31] = Instr::new(CPU::iy, CPU::op_and);

        cpu
    }

    // 1) read a byte (instruction)
    // 2) decode using optable to give op function
    // 3) get argument using addressing mode if applicable
    // 4) execute op
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
        self.mmu.write(self.r.stack_page*0x100 + self.r.s as usize, val);

        // Note: rust will panic instead of wrapping (to safe for school)
        if self.r.s == 0 {
            self.r.s = 255;
        } else {
            self.r.s = (self.r.s - 1) & 0xFF;
        }
    }

    fn stack_push_word(&mut self, val: u16) {
        self.stack_push((val >> 8) as u8);
        self.stack_push((val & 0xFF) as u8);
    }

    fn stack_pop(&mut self) -> u8 {
        let val = self.mmu.read(self.r.stack_page*0x100 + ((self.r.s as usize + 1) & 0xFF));
        self.r.s = (self.r.s + 1) & 0xFF;
        val
    }

    fn stack_pop_word(&mut self) -> u16 {
        (self.stack_pop() as u16) + ((self.stack_pop() as u16) << 8)
    }

    fn from_bcd(&self, val: u16) -> u16 {
        (((val & 0xF0) / 0x10) * 10) + (val & 0xF)
    }

    fn to_bcd(&self, val: u16) -> u16 {
        val / 10 * 16 + (val % 10)
    }

    fn from_twos_com(&self, val: u16) -> i8 {
        (((val as i16) & 0x7F) - ((val as i16) & 0x80)) as i8
    }

    fn interrupt_address(&mut self, interrupt: String) -> u16 {
        self.mmu.read_word(self.r.interrupts[&interrupt])
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
            self.r.cc += 1;
        }

        a & 0xFFFF
    }

    fn ay_a(&mut self) -> u16 {
        let op = self.next_word();
        let a = op + (self.r.y as u16);

        if op / 0xFF != a / 0xFF {
            self.r.cc += 1;
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
            self.r.cc += 1;
        }

        a & 0xFFFF
        }

    // ---- read value for each addressing mode ----

    // Note that though the width of the address bus is 16 bits, the data bus is 8 bits. These
    // memory fetchs all return u8's and hence the 6502 is considered to be an 8bit processor...

    // immediate
    // the byte directly following the instruction IS the argument
    // return a u8 as a u16 for API purposes..
    fn im(&mut self) -> u16 {
        self.next_byte() as u16
    }

    // zero page addressing
    // Note this was historically used as a way to access "faster memory" on this processor.
    // The divergence of speed between registers, caches, and memory on faster processors
    // led to the loss of usefulness of zero page addressing.
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
    // The full memory location (16 bits) is used as an address to the argument byte.
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
    // The full memory location (16 bits) is used as an address to the address (16 bits),
    // which contains the location of the argument byte.
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

// Construct a simple cpu with the given bytes in ROM and
// set the pc to point to the first byte in ROM.
// TODO: expand to allow video RAM, and static program RAM, and static program ROM 
// (static data).
fn make_cpu(rom_init: Vec<u8>) -> CPU {
        let mut mmu = MMU::new(&Vec::new());
        // RAM
        mmu.add_block(&Block::new(0, 0x200, false, None));
        // ROM
        mmu.add_block(&Block::new(0x1000, 0x100, true, Some(rom_init)));

        let mut cpu = CPU::new(mmu);
        cpu.r.pc = 0x1000;
        cpu
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

    // ----- test cpu methods -----

    #[test]
    fn test_to_bcd() {
        let mut cpu = make_cpu(vec![]);

        assert_eq!(cpu.to_bcd(0), 0);
        assert_eq!(cpu.to_bcd(5), 0x05);
        assert_eq!(cpu.to_bcd(11), 0x11);
        assert_eq!(cpu.to_bcd(99), 0x99);
    }

    #[test]
    fn test_from_bcd() {
        let mut cpu = make_cpu(vec![]);

        assert_eq!(cpu.from_bcd(0), 0);
        assert_eq!(cpu.from_bcd(0x05), 5);
        assert_eq!(cpu.from_bcd(0x11), 11);
        assert_eq!(cpu.from_bcd(0x99), 99);
    }

    #[test]
    fn test_from_twos_com() {
        let mut cpu = make_cpu(vec![]);

        assert_eq!(cpu.from_twos_com(0x00), 0);
        assert_eq!(cpu.from_twos_com(0x01), 1);
        assert_eq!(cpu.from_twos_com(0x7F), 127);
        assert_eq!(cpu.from_twos_com(0xFF), -1);
        assert_eq!(cpu.from_twos_com(0x80), -128);
    }

    #[test]
    fn test_next_byte() {
        let mut cpu = make_cpu(vec![1, 2, 3]);

        assert_eq!(cpu.next_byte(), 1);
        assert_eq!(cpu.next_byte(), 2);
        assert_eq!(cpu.next_byte(), 3);
        assert_eq!(cpu.next_byte(), 0);
    }

    #[test]
    fn test_next_word() {
        let mut cpu = make_cpu(vec![1, 2, 3, 4, 5, 9, 10]);

        assert_eq!(cpu.next_word(), 0x0201);
        cpu.next_byte();
        assert_eq!(cpu.next_word(), 0x0504);
        assert_eq!(cpu.next_word(), 0x0A09);
    }

    #[test]
    fn test_stack() {
        let mut cpu = make_cpu(vec![]);

        cpu.stack_push(0x10);
        assert_eq!(cpu.stack_pop(), 0x10);
        cpu.stack_push_word(0x0510);
        assert_eq!(cpu.stack_pop_word(), 0x0510);
        assert_eq!(cpu.stack_pop(), 0x00);
        cpu.stack_push(0x00);
        cpu.stack_push_word(0x0510);
        assert_eq!(cpu.stack_pop(), 0x10);
        assert_eq!(cpu.stack_pop(), 0x05);
    }

    // ----- test addressing modes -----


    #[test]
    fn test_zeropage_addressing() {
        // set up MMU and CPU
        let mut cpu = make_cpu(vec![1, 2, 3, 4, 5]);
        assert_eq!(cpu.z_a(), 1);

        cpu.r.x = 0;
        assert_eq!(cpu.zx_a(), 2);
        cpu.r.x = 1;
        assert_eq!(cpu.zx_a(), 4);

        cpu.r.y = 0;
        assert_eq!(cpu.zy_a(), 4);
        cpu.r.y = 1;
        assert_eq!(cpu.zy_a(), 6);
    }

    #[test]
    fn test_absolute_addressing() {
        // set up MMU and CPU
        let mut cpu = make_cpu(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
        assert_eq!(cpu.a_a(), 0x0201);

        cpu.r.x = 0;
        cpu.r.cc = 0;
        assert_eq!(cpu.ax_a(), 0x0403);
        assert_eq!(cpu.r.cc, 0);
        cpu.r.x = 0xFF;
        cpu.r.cc = 0;
        assert_eq!(cpu.ax_a(), 0x0605+0xFF);
        assert_eq!(cpu.r.cc, 1);

        cpu.r.y = 0;
        cpu.r.cc = 0;
        assert_eq!(cpu.ay_a(), 0x0807);
        assert_eq!(cpu.r.cc, 0);
        cpu.r.y = 0xFF;
        cpu.r.cc = 0;
        assert_eq!(cpu.ay_a(), 0x0a09+0xFF);
        assert_eq!(cpu.r.cc, 1);

    }

    #[test]
    fn test_indirect_addressing() {
        // set up MMU and CPU
        let mut cpu = make_cpu(
            vec![
                0x06, 0x10,
                0xFF, 0x10,
                0x00, 0x00,
                0xF0, 0x00,
            ]
        );

        assert_eq!(cpu.i_a(), 0x00F0);
        assert_eq!(cpu.i_a(), 0x0600);

        cpu.r.y = 0x05;
        cpu.mmu.write(0x00, 0x21);
        cpu.mmu.write(0x01, 0x43);
        assert_eq!(cpu.iy_a(), 0x4326);

        cpu.r.x = 0x02;
        cpu.mmu.write(0x02, 0x34);
        cpu.mmu.write(0x03, 0x12);
        assert_eq!(cpu.ix_a(), 0x1234);
    }

    // ----- test all instructions -----
    // there are 56 of these instructions, plus a couple extras

    // #[test]
    // #[should_panic]
    // fn test_op_not_implemented() {
    //     let mmu = MMU::new(&Vec::new());
    //     let mut cpu = CPU::new(mmu);
    //     let op = cpu.ops[0x69];
    //     op(&mut cpu, 0);
    //     assert_eq!(cpu.r.a, 0);
    // }

    // #[test]
    // fn test_adc() {
    //     // set up MMU and CPU
    //     let mut cpu = make_cpu(vec![1, 2, 250, 3, 100, 100]);
    //     let code = cpu.ops[0x69].code;
    //     code(&mut cpu, 0);
    //     assert_eq!(cpu.r.a, 1);
    // }

    // #[test]
    // fn test_adc_decimal() {
    //     // set up MMU and CPU
    //     let mut cpu = make_cpu(vec![1, 2, 250, 3, 100, 100]);
    //     // let op = cpu.ops[0x69];
    //     // op(&mut cpu, 0);
    //     // assert_eq!(cpu.r.a, 1);
    // }

    #[test]
    fn test_and() {
        let mut cpu = make_cpu(vec![0xFF, 0xFF, 0x01, 0x2]);

        cpu.r.a = 0x00;
        let src = (cpu.ops[0x29].addr)(&mut cpu);
        (cpu.ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0);

        cpu.r.a = 0xFF;
        let src = (cpu.ops[0x29].addr)(&mut cpu);
        (cpu.ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFF);

        cpu.r.a = 0x01;
        let src = (cpu.ops[0x29].addr)(&mut cpu);
        (cpu.ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);

        cpu.r.a = 0x01;
        let src = (cpu.ops[0x29].addr)(&mut cpu);
        (cpu.ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x00);
    }

    // ----- comprehensive tests -----

    // test step

    // test ROM

}