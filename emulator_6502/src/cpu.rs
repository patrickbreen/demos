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

    // add - add memory to accumulator with carry
    fn op_adc(&mut self, src: u16) {

        let v1 = self.r.a as u16;
        let mut r = 0;
        
        if self.r.get_flag('D') {
            let d1 = self.from_bcd(v1 as u16);
            let d2 = self.from_bcd(src);
            r = d1 + d2 + (self.r.get_flag('C') as u16);
            self.r.a = self.to_bcd((r % 100) as u16) as u8;
            self.r.set_flag('C', r > 99);
        } else {
            r = v1 + src + (self.r.get_flag('C') as u16);
            self.r.a = (r & 0xFF) as u8;

            self.r.set_flag('C', r > 0xFF);
        }
        let a = self.r.a;
        self.r.zn(a);
        self.r.set_flag('V', ((!(v1 ^ src)) & (v1 ^ r) & 0x80) != 0)
    }

    // and
    fn op_and(&mut self, src: u16) {
        self.r.a = (self.r.a & (src as u8)) & 0xFF;
        let flag = self.r.a;
        self.r.zn(flag);
    }

    // asl - arithmetic shift left
    fn op_asl(&mut self, src: u16) {
        let mut v = self.mmu.read(src as usize);
        println!("src {}, v {}, acc {}", src, v, self.r.a);
        v = v << 1;
        self.mmu.write(src as usize, v);

        self.r.set_flag('C', v > 0xFF);
        self.r.zn(v & 0xFF);
    }

    fn op_asl_acc(&mut self, src: u16) {
        let v = self.r.a << 1;
        self.r.a = v & 0xFF;

        self.r.set_flag('C', v > 0xFF);
        self.r.zn(v & 0xFF);
    }

    // Branching ops
    fn op_bpl(&mut self, src: u16) {
        if self.r.get_flag('N') == false {
            self.branch(src);
        }
    }

    fn op_bmi(&mut self, src: u16) {
        if self.r.get_flag('N') == true {
            self.branch(src);
        }
    }

    fn op_bvc_f(&mut self, src: u16) {
        if self.r.get_flag('V') == false {
            self.branch(src);
        }
    }

    fn op_bvc_t(&mut self, src: u16) {
        if self.r.get_flag('V') == true {
            self.branch(src);
        }
    }

    fn op_bcc(&mut self, src: u16) {
        if self.r.get_flag('C') == false {
            self.branch(src);
        }
    }

    fn op_bcs(&mut self, src: u16) {
        if self.r.get_flag('C') == true {
            self.branch(src);
        }
    }

    fn op_bne(&mut self, src: u16) {
        if self.r.get_flag('Z') == false {
            self.branch(src);
        }
    }

    fn op_beq(&mut self, src: u16) {
        if self.r.get_flag('Z') == true {
            self.branch(src);
        }
    }

    fn branch(&mut self, src: u16) {
        let o = self.r.pc;
        self.r.pc = self.r.pc.wrapping_add(self.from_twos_com(src) as u16);

        // if jumping in the first page, it takes one cycle,
        // otherwise, it takes two.
        if (o/0xFF) == (self.r.pc/0xFF) {
            self.r.cc += 1;
        } else {
            self.r.cc += 2;
        }
    }

    // bit
    fn op_bit(&mut self, src: u16) {
        let a = (self.r.a as u16);
        self.r.set_flag('Z', a & src == 0);
        self.r.set_flag('N', src & 0x80 != 0);
        self.r.set_flag('V', src & 0x40 != 0);
    }

    // brk
    fn op_brk(&mut self, src: u16) {
        self.r.set_flag('B', true);

        let pc = self.r.pc;
        self.stack_push_word(pc);
        
        let p = self.r.p;
        self.stack_push(p);
        self.r.set_flag('I', true);
        self.r.pc = self.interrupt_address("BRK".to_string());
    }

    /// initialize the CPU and return it
    fn new(mmu: MMU) -> CPU {

        let mut cpu = CPU {
            mmu: mmu,
            r: Registers::new(),
            ops: [Instr::new(CPU::im, CPU::op_not_implemented); 256],
        };

        // set up op table
        // adc
        cpu.ops[0x69] = Instr::new(CPU::im, CPU::op_adc);
        cpu.ops[0x65] = Instr::new(CPU::z,  CPU::op_adc);
        cpu.ops[0x75] = Instr::new(CPU::zx, CPU::op_adc);
        cpu.ops[0x6D] = Instr::new(CPU::a,  CPU::op_adc);
        cpu.ops[0x7D] = Instr::new(CPU::ax, CPU::op_adc);
        cpu.ops[0x79] = Instr::new(CPU::ay, CPU::op_adc);
        cpu.ops[0x61] = Instr::new(CPU::ix, CPU::op_adc);
        cpu.ops[0x71] = Instr::new(CPU::iy, CPU::op_adc);

        // and
        cpu.ops[0x29] = Instr::new(CPU::im, CPU::op_and);
        cpu.ops[0x25] = Instr::new(CPU::z,  CPU::op_and);
        cpu.ops[0x35] = Instr::new(CPU::zx, CPU::op_and);
        cpu.ops[0x2D] = Instr::new(CPU::a,  CPU::op_and);
        cpu.ops[0x3D] = Instr::new(CPU::ax, CPU::op_and);
        cpu.ops[0x39] = Instr::new(CPU::ay, CPU::op_and);
        cpu.ops[0x21] = Instr::new(CPU::ix, CPU::op_and);
        cpu.ops[0x31] = Instr::new(CPU::iy, CPU::op_and);

        // asl
        cpu.ops[0x0a] = Instr::new(CPU::im, CPU::op_asl_acc);
        cpu.ops[0x06] = Instr::new(CPU::z,  CPU::op_asl);
        cpu.ops[0x16] = Instr::new(CPU::zx, CPU::op_asl);
        cpu.ops[0x0e] = Instr::new(CPU::a,  CPU::op_asl);
        cpu.ops[0x1e] = Instr::new(CPU::ax, CPU::op_asl);

        // branching
        cpu.ops[0x10] = Instr::new(CPU::im, CPU::op_bpl);
        cpu.ops[0x30] = Instr::new(CPU::im, CPU::op_bmi);
        cpu.ops[0x50] = Instr::new(CPU::im, CPU::op_bvc_f);
        cpu.ops[0x70] = Instr::new(CPU::im, CPU::op_bvc_t);
        cpu.ops[0x90] = Instr::new(CPU::im, CPU::op_bcc);
        cpu.ops[0xB0] = Instr::new(CPU::im, CPU::op_bcs);
        cpu.ops[0xD0] = Instr::new(CPU::im, CPU::op_bne);
        cpu.ops[0xF0] = Instr::new(CPU::im, CPU::op_beq);

        // bit
        cpu.ops[0x24] = Instr::new(CPU::z, CPU::op_bit);
        cpu.ops[0x2C] = Instr::new(CPU::a, CPU::op_bit);

        // brk
        cpu.ops[0x00] = Instr::new(CPU::im, CPU::op_brk);

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

    fn from_twos_com(&self, val: u16) -> i16 {
        ((val as i16) & 0x7F) - ((val as i16) & 0x80)
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
    fn z(&mut self) -> u16 {
        let addr = self.z_a();
        self.mmu.read(addr as usize) as u16
    }

    fn zx(&mut self) -> u16 {
        let addr = self.zx_a();
        self.mmu.read(addr as usize) as u16
    }

    fn zy(&mut self) -> u16 {
        let addr = self.zy_a();
        self.mmu.read(addr as usize) as u16
    }

    // absolute addressing
    // The full memory location (16 bits) is used as an address to the argument byte.
    fn a(&mut self) -> u16 {
        let addr = self.a_a();
        self.mmu.read(addr as usize) as u16
    }

    fn ax(&mut self) -> u16 {
        let addr = self.ax_a();
        self.mmu.read(addr as usize) as u16
    }

    fn ay(&mut self) -> u16 {
        let addr = self.ay_a();
        self.mmu.read(addr as usize) as u16
    }

    // indirect addressing
    // The full memory location (16 bits) is used as an address to the address (16 bits),
    // which contains the location of the argument byte.
    fn i(&mut self) -> u16 {
        let addr = self.i_a();
        self.mmu.read(addr as usize) as u16
    }

    fn ix(&mut self) -> u16 {
        let addr = self.ix_a();
        self.mmu.read(addr as usize) as u16
    }

    fn iy(&mut self) -> u16 {
        let addr = self.iy_a();
        self.mmu.read(addr as usize) as u16
    }
}

// Construct a simple cpu with the given bytes in ROM and
// set the pc to point to the first byte in ROM.
// TODO: expand to allow video RAM, and static program RAM, and static program ROM 
// (static data).
fn make_cpu(rom_init: Option<Vec<u8>>) -> CPU {
        let mut mmu = MMU::new(&Vec::new());
        // RAM
        mmu.add_block(&Block::new(0, 0x200, false, None));
        // ROM
        mmu.add_block(&Block::new(0x1000, 0x100, true, rom_init));

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
        let mut cpu = make_cpu(Some(vec![]));

        assert_eq!(cpu.to_bcd(0), 0);
        assert_eq!(cpu.to_bcd(5), 0x05);
        assert_eq!(cpu.to_bcd(11), 0x11);
        assert_eq!(cpu.to_bcd(99), 0x99);
    }

    #[test]
    fn test_from_bcd() {
        let mut cpu = make_cpu(None);

        assert_eq!(cpu.from_bcd(0), 0);
        assert_eq!(cpu.from_bcd(0x05), 5);
        assert_eq!(cpu.from_bcd(0x11), 11);
        assert_eq!(cpu.from_bcd(0x99), 99);
    }

    #[test]
    fn test_from_twos_com() {
        let mut cpu = make_cpu(None);

        assert_eq!(cpu.from_twos_com(0x00), 0);
        assert_eq!(cpu.from_twos_com(0x01), 1);
        assert_eq!(cpu.from_twos_com(0x7F), 127);
        assert_eq!(cpu.from_twos_com(0xFF), -1);
        assert_eq!(cpu.from_twos_com(0x80), -128);
    }

    #[test]
    fn test_next_byte() {
        let mut cpu = make_cpu(Some(vec![1, 2, 3]));

        assert_eq!(cpu.next_byte(), 1);
        assert_eq!(cpu.next_byte(), 2);
        assert_eq!(cpu.next_byte(), 3);
        assert_eq!(cpu.next_byte(), 0);
    }

    #[test]
    fn test_next_word() {
        let mut cpu = make_cpu(Some(vec![1, 2, 3, 4, 5, 9, 10]));

        assert_eq!(cpu.next_word(), 0x0201);
        cpu.next_byte();
        assert_eq!(cpu.next_word(), 0x0504);
        assert_eq!(cpu.next_word(), 0x0A09);
    }

    #[test]
    fn test_stack() {
        let mut cpu = make_cpu(None);

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
        let mut cpu = make_cpu(Some(vec![1, 2, 3, 4, 5]));
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
        let mut cpu = make_cpu(
            Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
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
        let mut cpu = make_cpu(
            Some(vec![
                0x06, 0x10,
                0xFF, 0x10,
                0x00, 0x00,
                0xF0, 0x00,
            ])
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

    #[test]
    fn test_adc() {
        let mut cpu = make_cpu(Some(vec![1, 2, 250, 3, 100, 100]));
        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 1);

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 3);

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 253);
        assert!(cpu.r.get_flag('N'));
        cpu.r.clear_flags();

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert!(cpu.r.get_flag('C'));
        assert!(cpu.r.get_flag('Z'));
        cpu.r.clear_flags();

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert!(cpu.r.get_flag('V'));
    }

    #[test]
    fn test_adc_decimal() {
        let mut cpu = make_cpu(Some(vec![0x01, 0x55, 0x50]));
        cpu.r.set_flag('D', true);

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x56);

        let src = (cpu.ops[0x69].addr)(&mut cpu);
        (cpu.ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x06);
        assert!(cpu.r.get_flag('C'));
    }

    #[test]
    fn test_and() {
        let mut cpu = make_cpu(Some(vec![0xFF, 0xFF, 0x01, 0x2]));

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

    #[test]
    fn test_asl() {
        let mut cpu = make_cpu(Some(vec![0x00]));

        cpu.r.a = 1;
        let src = (cpu.ops[0x0a].addr)(&mut cpu);
        (cpu.ops[0x0a].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 2);

        // TODO not sure I implemented this correctly...
        cpu.mmu.write(0, 1);
        cpu.mmu.write(1, 4);
        let src = (cpu.ops[0x06].addr)(&mut cpu);
        (cpu.ops[0x06].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(1), 8);
    }

    #[test]
    fn test_branch() {
        let mut cpu = make_cpu(Some(vec![0x01, 0x00, 0x00, 0xFC]));

        let src = (cpu.ops[0x10].addr)(&mut cpu);
        (cpu.ops[0x10].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1002);

        let src = (cpu.ops[0x70].addr)(&mut cpu);
        (cpu.ops[0x70].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1003);

        cpu.r.set_flag('C', true);
        let src = (cpu.ops[0xB0].addr)(&mut cpu);
        (cpu.ops[0xB0].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1000);

        let src = (cpu.ops[0xD0].addr)(&mut cpu);
        (cpu.ops[0xD0].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1002);
    }

    #[test]
    fn test_bit() {
        let mut cpu = make_cpu(Some(vec![0x00, 0x00, 0x10]));
        cpu.mmu.write(0, 0xFF);
        cpu.r.a = 1;

        let src = (cpu.ops[0x24].addr)(&mut cpu);
        (cpu.ops[0x24].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('N'), true);
        assert_eq!(cpu.r.get_flag('V'), true);

        let src = (cpu.ops[0x2C].addr)(&mut cpu);
        (cpu.ops[0x2C].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('N'), false);
        assert_eq!(cpu.r.get_flag('V'), false);
    }

    #[test]
    fn test_brk() {
        let mut cpu = make_cpu(None);
        let block = Block::new(0xFFFE, 0x2, true, Some(vec![0x34, 0x12]));
        cpu.mmu.add_block(&block);
        cpu.r.p = 239;

        let src = (cpu.ops[0x00].addr)(&mut cpu);
        (cpu.ops[0x00].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('B'), true);
        assert_eq!(cpu.r.get_flag('I'), true);
        assert_eq!(cpu.r.pc, 0x1234);
        assert_eq!(cpu.stack_pop(), 255);
        assert_eq!(cpu.stack_pop_word(), 0x1001);

    }

    // ----- comprehensive tests -----

    // test step

    // test ROM

}