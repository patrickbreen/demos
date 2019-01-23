use mmu::{Block};
use cpu::{Instr, CPU, make_cpu};


// build op table

// ops is a table of functions with offsets in the table given by an opcode (u8)
// the values in the op table are functions that borrow a CPU and no return
// value. The borrowed references to the ops have the same lifetime as the CPU

// opcode: u8 -> func(&mut CPU, u8)

// this is often called a jump table (though it isn't used much in high level code)

// TODO: this is too simplistic. Needs to track the cycle count, whether it acts
//  on values or addresses and target register if valid


fn make_op_table() -> [Instr; 256] {
    let mut ops = [Instr::new(CPU::im, op_not_implemented); 256];

    // set up op table
    // adc
    ops[0x69] = Instr::new(CPU::im, op_adc);
    ops[0x65] = Instr::new(CPU::z,  op_adc);
    ops[0x75] = Instr::new(CPU::zx, op_adc);
    ops[0x6D] = Instr::new(CPU::a,  op_adc);
    ops[0x7D] = Instr::new(CPU::ax, op_adc);
    ops[0x79] = Instr::new(CPU::ay, op_adc);
    ops[0x61] = Instr::new(CPU::ix, op_adc);
    ops[0x71] = Instr::new(CPU::iy, op_adc);

    // and
    ops[0x29] = Instr::new(CPU::im, op_and);
    ops[0x25] = Instr::new(CPU::z,  op_and);
    ops[0x35] = Instr::new(CPU::zx, op_and);
    ops[0x2D] = Instr::new(CPU::a,  op_and);
    ops[0x3D] = Instr::new(CPU::ax, op_and);
    ops[0x39] = Instr::new(CPU::ay, op_and);
    ops[0x21] = Instr::new(CPU::ix, op_and);
    ops[0x31] = Instr::new(CPU::iy, op_and);

    // asl
    ops[0x0a] = Instr::new(CPU::im, op_asl_acc);
    ops[0x06] = Instr::new(CPU::z,  op_asl);
    ops[0x16] = Instr::new(CPU::zx, op_asl);
    ops[0x0e] = Instr::new(CPU::a,  op_asl);
    ops[0x1e] = Instr::new(CPU::ax, op_asl);

    // branching
    ops[0x10] = Instr::new(CPU::im, op_bpl);
    ops[0x30] = Instr::new(CPU::im, op_bmi);
    ops[0x50] = Instr::new(CPU::im, op_bvc_f);
    ops[0x70] = Instr::new(CPU::im, op_bvc_t);
    ops[0x90] = Instr::new(CPU::im, op_bcc);
    ops[0xB0] = Instr::new(CPU::im, op_bcs);
    ops[0xD0] = Instr::new(CPU::im, op_bne);
    ops[0xF0] = Instr::new(CPU::im, op_beq);

    // bit
    ops[0x24] = Instr::new(CPU::z, op_bit);
    ops[0x2C] = Instr::new(CPU::a, op_bit);

    // brk
    ops[0x00] = Instr::new(CPU::im, op_brk);

    // cp
    ops[0xC9] = Instr::new(CPU::im, op_cmp);
    ops[0xC5] = Instr::new(CPU::z, op_cmp);
    ops[0xD5] = Instr::new(CPU::zx, op_cmp);
    ops[0xCD] = Instr::new(CPU::a, op_cmp);
    ops[0xDD] = Instr::new(CPU::ax, op_cmp);
    ops[0xD9] = Instr::new(CPU::ay, op_cmp);
    ops[0xC1] = Instr::new(CPU::ix, op_cmp);
    ops[0xD1] = Instr::new(CPU::iy, op_cmp);

    ops[0xE0] = Instr::new(CPU::im, op_cpx);
    ops[0xE4] = Instr::new(CPU::z, op_cpx);
    ops[0xEC] = Instr::new(CPU::a, op_cpx);

    ops[0xC0] = Instr::new(CPU::im, op_cpy);
    ops[0xC4] = Instr::new(CPU::z, op_cpy);
    ops[0xCC] = Instr::new(CPU::a, op_cpy);

    ops
}


// implement ops
fn op_not_implemented(cpu: &mut CPU, src: u16) {
    panic!("Error, this op is not implemented.")
}

// add - add memory to accumulator with carry
fn op_adc(cpu: &mut CPU, src: u16) {

    let v1 = cpu.r.a as u16;
    let mut r = 0;
    
    if cpu.r.get_flag('D') {
        let d1 = cpu.from_bcd(v1 as u16);
        let d2 = cpu.from_bcd(src);
        r = d1 + d2 + (cpu.r.get_flag('C') as u16);
        cpu.r.a = cpu.to_bcd((r % 100) as u16) as u8;
        cpu.r.set_flag('C', r > 99);
    } else {
        r = v1 + src + (cpu.r.get_flag('C') as u16);
        cpu.r.a = (r & 0xFF) as u8;

        cpu.r.set_flag('C', r > 0xFF);
    }
    let a = cpu.r.a;
    cpu.r.zn(a);
    cpu.r.set_flag('V', ((!(v1 ^ src)) & (v1 ^ r) & 0x80) != 0)
}

// and
fn op_and(cpu: &mut CPU, src: u16) {
    cpu.r.a = (cpu.r.a & (src as u8)) & 0xFF;
    let flag = cpu.r.a;
    cpu.r.zn(flag);
}

// asl - arithmetic shift left
fn op_asl(cpu: &mut CPU, src: u16) {
    let mut v = cpu.mmu.read(src as usize);
    println!("src {}, v {}, acc {}", src, v, cpu.r.a);
    v = v << 1;
    cpu.mmu.write(src as usize, v);

    cpu.r.set_flag('C', v > 0xFF);
    cpu.r.zn(v & 0xFF);
}

fn op_asl_acc(cpu: &mut CPU, src: u16) {
    let v = cpu.r.a << 1;
    cpu.r.a = v & 0xFF;

    cpu.r.set_flag('C', v > 0xFF);
    cpu.r.zn(v & 0xFF);
}

// Branching ops
fn op_bpl(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('N') == false {
        branch(cpu, src);
    }
}

fn op_bmi(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('N') == true {
        branch(cpu, src);
    }
}

fn op_bvc_f(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('V') == false {
        branch(cpu, src);
    }
}

fn op_bvc_t(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('V') == true {
        branch(cpu, src);
    }
}

fn op_bcc(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('C') == false {
        branch(cpu, src);
    }
}

fn op_bcs(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('C') == true {
        branch(cpu, src);
    }
}

fn op_bne(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('Z') == false {
        branch(cpu, src);
    }
}

fn op_beq(cpu: &mut CPU, src: u16) {
    if cpu.r.get_flag('Z') == true {
        branch(cpu, src);
    }
}

fn branch(cpu: &mut CPU, src: u16) {
    let o = cpu.r.pc;
    cpu.r.pc = cpu.r.pc.wrapping_add(cpu.from_twos_com(src) as u16);

    // if jumping in the first page, it takes one cycle,
    // otherwise, it takes two.
    if (o/0xFF) == (cpu.r.pc/0xFF) {
        cpu.r.cc += 1;
    } else {
        cpu.r.cc += 2;
    }
}

// bit
fn op_bit(cpu: &mut CPU, src: u16) {
    let a = (cpu.r.a as u16);
    cpu.r.set_flag('Z', a & src == 0);
    cpu.r.set_flag('N', src & 0x80 != 0);
    cpu.r.set_flag('V', src & 0x40 != 0);
}

// brk
fn op_brk(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('B', true);

    let pc = cpu.r.pc;
    cpu.stack_push_word(pc);
    
    let p = cpu.r.p;
    cpu.stack_push(p);
    cpu.r.set_flag('I', true);
    cpu.r.pc = cpu.interrupt_address("BRK".to_string());
}

fn cp(cpu: &mut CPU, r: u16, v: u16) {
    let mut o = 0;
    if v > r {
        o = (r + 0xFF - v) & 0xFF;
    } else {
        o = (r-v) & 0xFF;
    }
    cpu.r.set_flag('Z', o == 0);
    cpu.r.set_flag('C', v <= r);
    cpu.r.set_flag('N', (o & 0x80) !=0);
}

//cmp
fn op_cmp(cpu: &mut CPU, src: u16) {
    let a = cpu.r.a as u16;
    cp(cpu, a, src);
}
//cpx
fn op_cpx(cpu: &mut CPU, src: u16) {
    let x = cpu.r.x as u16;
    cp(cpu, x, src);
}
//cpy
fn op_cpy(cpu: &mut CPU, src: u16) {
    let y = cpu.r.y as u16;
    cp(cpu, y, src);
}

#[cfg(test)]
mod tests {
    // import parent scope
    use super::*;

    // ----- test all instructions -----
    // there are 56 of these instructions, plus a couple undocumented extras

    #[test]
    fn test_adc() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![1, 2, 250, 3, 100, 100]));
        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 1);

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 3);

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 253);
        assert!(cpu.r.get_flag('N'));
        cpu.r.clear_flags();

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert!(cpu.r.get_flag('C'));
        assert!(cpu.r.get_flag('Z'));
        cpu.r.clear_flags();

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert!(cpu.r.get_flag('V'));
    }

    #[test]
    fn test_adc_decimal() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x01, 0x55, 0x50]));
        cpu.r.set_flag('D', true);

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x56);

        let src = (ops[0x69].addr)(&mut cpu);
        (ops[0x69].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x06);
        assert!(cpu.r.get_flag('C'));
    }

    #[test]
    fn test_and() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0xFF, 0xFF, 0x01, 0x2]));

        cpu.r.a = 0x00;
        let src = (ops[0x29].addr)(&mut cpu);
        (ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0);

        cpu.r.a = 0xFF;
        let src = (ops[0x29].addr)(&mut cpu);
        (ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFF);

        cpu.r.a = 0x01;
        let src = (ops[0x29].addr)(&mut cpu);
        (ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);

        cpu.r.a = 0x01;
        let src = (ops[0x29].addr)(&mut cpu);
        (ops[0x29].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x00);
    }

    #[test]
    fn test_asl() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00]));

        cpu.r.a = 1;
        let src = (ops[0x0a].addr)(&mut cpu);
        (ops[0x0a].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 2);

        // TODO not sure I implemented this correctly...
        cpu.mmu.write(0, 1);
        cpu.mmu.write(1, 4);
        let src = (ops[0x06].addr)(&mut cpu);
        (ops[0x06].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(1), 8);
    }

    #[test]
    fn test_branch() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x01, 0x00, 0x00, 0xFC]));

        let src = (ops[0x10].addr)(&mut cpu);
        (ops[0x10].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1002);

        let src = (ops[0x70].addr)(&mut cpu);
        (ops[0x70].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1003);

        cpu.r.set_flag('C', true);
        let src = (ops[0xB0].addr)(&mut cpu);
        (ops[0xB0].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1000);

        let src = (ops[0xD0].addr)(&mut cpu);
        (ops[0xD0].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1002);
    }

    #[test]
    fn test_bit() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00, 0x00, 0x10]));
        cpu.mmu.write(0, 0xFF);
        cpu.r.a = 1;

        let src = (ops[0x24].addr)(&mut cpu);
        (ops[0x24].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('N'), true);
        assert_eq!(cpu.r.get_flag('V'), true);

        let src = (ops[0x2C].addr)(&mut cpu);
        (ops[0x2C].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('N'), false);
        assert_eq!(cpu.r.get_flag('V'), false);
    }

    #[test]
    fn test_brk() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let block = Block::new(0xFFFE, 0x2, true, Some(vec![0x34, 0x12]));
        cpu.mmu.add_block(&block);
        cpu.r.p = 239;

        let src = (ops[0x00].addr)(&mut cpu);
        (ops[0x00].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('B'), true);
        assert_eq!(cpu.r.get_flag('I'), true);
        assert_eq!(cpu.r.pc, 0x1234);
        assert_eq!(cpu.stack_pop(), 255);
        assert_eq!(cpu.stack_pop_word(), 0x1001);
    }

    #[test]
    fn test_cmp() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x0F, 0x10, 0x11, 0xFE, 0xFF, 0x00, 0x7F]));

        cpu.r.a = 0x10;
        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), false);
        assert_eq!(cpu.r.get_flag('N'), true);

        cpu.r.a = 0xFF;
        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), true);

        let src = (ops[0xC9].addr)(&mut cpu);
        (ops[0xC9].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), true);
    }

    #[test]
    fn test_cpx() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x0F, 0x10, 0x11]));

        cpu.r.x = 0x10;
        let src = (ops[0xE0].addr)(&mut cpu);
        (ops[0xE0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xE0].addr)(&mut cpu);
        (ops[0xE0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xE0].addr)(&mut cpu);
        (ops[0xE0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), false);
        assert_eq!(cpu.r.get_flag('N'), true);
    }

    #[test]
    fn test_cpy() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x0F, 0x10, 0x11]));

        cpu.r.y = 0x10;
        let src = (ops[0xC0].addr)(&mut cpu);
        (ops[0xC0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC0].addr)(&mut cpu);
        (ops[0xC0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), true);
        assert_eq!(cpu.r.get_flag('C'), true);
        assert_eq!(cpu.r.get_flag('N'), false);

        let src = (ops[0xC0].addr)(&mut cpu);
        (ops[0xC0].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), false);
        assert_eq!(cpu.r.get_flag('N'), true);
    }
}