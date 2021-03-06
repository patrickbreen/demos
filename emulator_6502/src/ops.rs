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

fn no_arg(cpu: &mut CPU) -> u16 {
    0
}


pub fn make_op_table() -> [Instr; 256] {
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
    ops[0x0a] = Instr::new(no_arg, op_asl_acc);
    ops[0x06] = Instr::new(CPU::z_a,  op_asl);
    ops[0x16] = Instr::new(CPU::zx_a, op_asl);
    ops[0x0e] = Instr::new(CPU::a_a,  op_asl);
    ops[0x1e] = Instr::new(CPU::ax_a, op_asl);

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
    ops[0x24] = Instr::new(CPU::z,  op_bit);
    ops[0x2C] = Instr::new(CPU::a,  op_bit);

    // brk
    ops[0x00] = Instr::new(no_arg, op_brk);

    // cp
    ops[0xC9] = Instr::new(CPU::im, op_cmp);
    ops[0xC5] = Instr::new(CPU::z,  op_cmp);
    ops[0xD5] = Instr::new(CPU::zx, op_cmp);
    ops[0xCD] = Instr::new(CPU::a,  op_cmp);
    ops[0xDD] = Instr::new(CPU::ax, op_cmp);
    ops[0xD9] = Instr::new(CPU::ay, op_cmp);
    ops[0xC1] = Instr::new(CPU::ix, op_cmp);
    ops[0xD1] = Instr::new(CPU::iy, op_cmp);

    ops[0xE0] = Instr::new(CPU::im, op_cpx);
    ops[0xE4] = Instr::new(CPU::z,  op_cpx);
    ops[0xEC] = Instr::new(CPU::a,  op_cpx);

    ops[0xC0] = Instr::new(CPU::im, op_cpy);
    ops[0xC4] = Instr::new(CPU::z,  op_cpy);
    ops[0xCC] = Instr::new(CPU::a,  op_cpy);

    // dec
    ops[0xC6] = Instr::new(CPU::z_a,  op_dec);
    ops[0xD6] = Instr::new(CPU::zx_a, op_dec);
    ops[0xCE] = Instr::new(CPU::a_a,  op_dec);
    ops[0xDE] = Instr::new(CPU::ax_a, op_dec);

    ops[0xCA] = Instr::new(no_arg,  op_dex);
    ops[0x88] = Instr::new(no_arg,  op_dey);

    //eor
    ops[0x49] = Instr::new(CPU::im,  op_eor);
    ops[0x45] = Instr::new(CPU::z,  op_eor);
    ops[0x55] = Instr::new(CPU::zx,  op_eor);
    ops[0x4d] = Instr::new(CPU::a,  op_eor);
    ops[0x5d] = Instr::new(CPU::ax,  op_eor);
    ops[0x59] = Instr::new(CPU::ay,  op_eor);
    ops[0x41] = Instr::new(CPU::ix,  op_eor);
    ops[0x51] = Instr::new(CPU::iy,  op_eor);

    //flag instructions
    ops[0x18] = Instr::new(no_arg,  op_clc);
    ops[0x58] = Instr::new(no_arg,  op_cli);
    ops[0xB8] = Instr::new(no_arg,  op_clv);
    ops[0xD8] = Instr::new(no_arg,  op_cld);

    ops[0x38] = Instr::new(no_arg,  op_sec);
    ops[0x78] = Instr::new(no_arg,  op_sei);
    ops[0xF8] = Instr::new(no_arg,  op_sed);

    //inc
    ops[0xE6] = Instr::new(CPU::z_a,  op_inc);
    ops[0xF6] = Instr::new(CPU::zx_a,  op_inc);
    ops[0xEE] = Instr::new(CPU::a_a,  op_inc);
    ops[0xFE] = Instr::new(CPU::ax_a,  op_inc);

    ops[0xE8] = Instr::new(CPU::im,  op_inx);

    ops[0xC8] = Instr::new(CPU::im,  op_iny);

    //jmp
    ops[0x4C] = Instr::new(CPU::a_a,  op_jmp);
    ops[0x6C] = Instr::new(CPU::i_a,  op_jmp);

    //jsr
    ops[0x20] = Instr::new(CPU::a_a,  op_jsr);

    //ld
    ops[0xA9] = Instr::new(CPU::im,  op_lda);
    ops[0xA5] = Instr::new(CPU::z,  op_lda);
    ops[0xB5] = Instr::new(CPU::zx,  op_lda);
    ops[0xAD] = Instr::new(CPU::a,  op_lda);
    ops[0xBD] = Instr::new(CPU::ax,  op_lda);
    ops[0xB9] = Instr::new(CPU::ay,  op_lda);
    ops[0xA1] = Instr::new(CPU::ix,  op_lda);
    ops[0xB1] = Instr::new(CPU::iy,  op_lda);

    ops[0xA2] = Instr::new(CPU::im,  op_ldx);
    ops[0xA6] = Instr::new(CPU::z,  op_ldx);
    ops[0xB6] = Instr::new(CPU::zy,  op_ldx);
    ops[0xAE] = Instr::new(CPU::a,  op_ldx);
    ops[0xBE] = Instr::new(CPU::ay,  op_ldx);

    ops[0xA0] = Instr::new(CPU::im,  op_ldy);
    ops[0xA4] = Instr::new(CPU::z,  op_ldy);
    ops[0xB4] = Instr::new(CPU::zx,  op_ldy);
    ops[0xAC] = Instr::new(CPU::a,  op_ldy);
    ops[0xBC] = Instr::new(CPU::ax,  op_ldy);

    //lsr
    ops[0x4A] = Instr::new(no_arg,  op_lsra);
    ops[0x46] = Instr::new(CPU::z_a,  op_lsr);
    ops[0x56] = Instr::new(CPU::zx_a,  op_lsr);
    ops[0x4E] = Instr::new(CPU::a_a,  op_lsr);
    ops[0x5E] = Instr::new(CPU::ax_a,  op_lsr);

    //nop
    ops[0x1A] = Instr::new(no_arg,  op_nop);
    ops[0x3A] = Instr::new(CPU::im,  op_nop);
    ops[0x5A] = Instr::new(CPU::im,  op_nop);
    ops[0x7A] = Instr::new(CPU::im,  op_nop);
    ops[0xDA] = Instr::new(CPU::im,  op_nop);
    ops[0xEA] = Instr::new(CPU::im,  op_nop);
    ops[0xFA] = Instr::new(CPU::im,  op_nop);


    //ora
    ops[0x09] = Instr::new(CPU::im,  op_ora);
    ops[0x05] = Instr::new(CPU::z,  op_ora);
    ops[0x15] = Instr::new(CPU::zx,  op_ora);
    ops[0x0D] = Instr::new(CPU::a,  op_ora);
    ops[0x1D] = Instr::new(CPU::ax,  op_ora);
    ops[0x19] = Instr::new(CPU::ay,  op_ora);
    ops[0x01] = Instr::new(CPU::ix,  op_ora);
    ops[0x11] = Instr::new(CPU::iy,  op_ora);

    //p
    ops[0x48] = Instr::new(no_arg,  op_pha);
    ops[0x68] = Instr::new(no_arg,  op_pla);
    ops[0x08] = Instr::new(no_arg,  op_php);
    ops[0x28] = Instr::new(no_arg,  op_plp);

    //rol
    ops[0x2A] = Instr::new(no_arg,  op_rola);
    ops[0x26] = Instr::new(CPU::z_a,  op_rol);
    ops[0x36] = Instr::new(CPU::zx_a,  op_rol);
    ops[0x2E] = Instr::new(CPU::a_a,  op_rol);
    ops[0x3E] = Instr::new(CPU::ax_a,  op_rol);

    //ror
    ops[0x6A] = Instr::new(no_arg,  op_rora);
    ops[0x66] = Instr::new(CPU::z_a,  op_ror);
    ops[0x76] = Instr::new(CPU::zx_a,  op_ror);
    ops[0x6E] = Instr::new(CPU::a_a,  op_ror);
    ops[0x7E] = Instr::new(CPU::ax_a,  op_ror);

    //rti
    ops[0x40] = Instr::new(no_arg,  op_rti);

    //rts
    ops[0x60] = Instr::new(no_arg,  op_rts);

    //sbc
    ops[0xE9] = Instr::new(CPU::im,  op_sbc);
    ops[0xEB] = Instr::new(CPU::im,  op_sbc);
    ops[0xE5] = Instr::new(CPU::z,  op_sbc);
    ops[0xF5] = Instr::new(CPU::zx,  op_sbc);
    ops[0xED] = Instr::new(CPU::a,  op_sbc);
    ops[0xFD] = Instr::new(CPU::ax,  op_sbc);
    ops[0xF9] = Instr::new(CPU::ay,  op_sbc);
    ops[0xE1] = Instr::new(CPU::ix,  op_sbc);
    ops[0xF1] = Instr::new(CPU::iy,  op_sbc);

    //sta
    ops[0x85] = Instr::new(CPU::z_a,  op_sta);
    ops[0x95] = Instr::new(CPU::zx_a,  op_sta);
    ops[0x8D] = Instr::new(CPU::a_a,  op_sta);
    ops[0x9D] = Instr::new(CPU::ax_a,  op_sta);
    ops[0x99] = Instr::new(CPU::ay_a,  op_sta);
    ops[0x81] = Instr::new(CPU::ix_a,  op_sta);
    ops[0x91] = Instr::new(CPU::iy_a,  op_sta);

    //stx
    ops[0x86] = Instr::new(CPU::z_a,  op_stx);
    ops[0x96] = Instr::new(CPU::zy_a,  op_stx);
    ops[0x8E] = Instr::new(CPU::a_a,  op_stx);

    //sty
    ops[0x84] = Instr::new(CPU::z_a,  op_sty);
    ops[0x94] = Instr::new(CPU::zx_a,  op_sty);
    ops[0x8C] = Instr::new(CPU::a_a,  op_sty);

    //t
    ops[0xAA] = Instr::new(no_arg,  op_tax);
    ops[0x8A] = Instr::new(no_arg,  op_txa);
    ops[0xa8] = Instr::new(no_arg,  op_tay);
    ops[0x98] = Instr::new(no_arg,  op_tya);
    ops[0x9A] = Instr::new(no_arg,  op_txs);
    ops[0xBA] = Instr::new(no_arg,  op_tsx);


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

fn op_cpx(cpu: &mut CPU, src: u16) {
    let x = cpu.r.x as u16;
    cp(cpu, x, src);
}

fn op_cpy(cpu: &mut CPU, src: u16) {
    let y = cpu.r.y as u16;
    cp(cpu, y, src);
}

//dec
fn op_dec(cpu: &mut CPU, src: u16) {
    let mut v = cpu.mmu.read(src as usize);
    if v == 0 {
        v = 0xFF;
    } else {
        v -= 1;
    }
    cpu.mmu.write(src as usize, v);
    cpu.r.zn(v);
}

fn op_dex(cpu: &mut CPU, src: u16) {
    let mut v = cpu.r.x;
    if v == 0 {
        v = 0xFF;
    } else {
        v -= 1;
    }
    cpu.r.x = v;
    cpu.r.zn(v);
}

fn op_dey(cpu: &mut CPU, src: u16) {
    let mut v = cpu.r.y;
    if v == 0 {
        v = 0xFF;
    } else {
        v -= 1;
    }
    cpu.r.x = v;
    cpu.r.zn(v);
}

fn op_eor(cpu: &mut CPU, src: u16) {
    let v = cpu.r.a ^ (src as u8);
    cpu.r.a = v;
    cpu.r.zn(v);
}

// flag ops
fn op_clc(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('C', false);
}

fn op_cli(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('I', false);
}

fn op_clv(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('V', false);
}

fn op_cld(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('D', false);
}

fn op_sec(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('C', true);
}

fn op_sei(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('I', true);
}

fn op_sed(cpu: &mut CPU, src: u16) {
    cpu.r.set_flag('D', true);
}

//inc
fn op_inc(cpu: &mut CPU, src: u16) {
    let v = (cpu.mmu.read(src as usize)+1) & 0xFF;
    cpu.mmu.write(src as usize, v);
    cpu.r.zn(v);
}

fn op_inx(cpu: &mut CPU, src: u16) {
    let v = (cpu.r.x+1) & 0xFF;
    cpu.r.x = v;
    cpu.r.zn(v);
}

fn op_iny(cpu: &mut CPU, src: u16) {
    let v = (cpu.r.y+1) & 0xFF;
    cpu.r.y = v;
    cpu.r.zn(v);
}

fn op_jmp(cpu: &mut CPU, src: u16) {
    cpu.r.pc = src;
}

fn op_jsr(cpu: &mut CPU, src: u16) {
    let pc = cpu.r.pc-1;
    cpu.stack_push_word(pc);
    cpu.r.pc = src;
}

fn op_lda(cpu: &mut CPU, src: u16) {
    cpu.r.a = src as u8;
    cpu.r.zn(src as u8);
}

fn op_ldx(cpu: &mut CPU, src: u16) {
    cpu.r.x = src as u8;
    cpu.r.zn(src as u8);
}

fn op_ldy(cpu: &mut CPU, src: u16) {
    cpu.r.y = src as u8;
    cpu.r.zn(src as u8);
}

fn op_lsra(cpu: &mut CPU, src: u16) {
    let val  = cpu.r.a & 0x01 != 0;
    cpu.r.set_flag('C', val);
    let v = cpu.r.a >> 1;
    cpu.r.a = v;
    cpu.r.zn(v);
}

fn op_lsr(cpu: &mut CPU, src: u16) {
    let mut v = cpu.mmu.read(src as usize);
    let val = v & 0x01 != 0;
    cpu.r.set_flag('C', val);
    v = v >> 1;
    cpu.mmu.write(src as usize, v);
    cpu.r.zn(v);
}

fn op_nop(cpu: &mut CPU, src: u16) {
    
}

fn op_ora(cpu: &mut CPU, src: u16) {
    let a = cpu.r.a | (src as u8);
    cpu.r.a = a;
    cpu.r.zn(a);
}

fn op_pha(cpu: &mut CPU, src: u16) {
    let a = cpu.r.a;
    cpu.stack_push(a);
    cpu.r.zn(a);
}

fn op_php(cpu: &mut CPU, src: u16) {
    let p = cpu.r.p;
    cpu.stack_push(p);
    cpu.r.p = p | 0b00100000;
}

fn op_pla(cpu: &mut CPU, src: u16) {
    let a = cpu.stack_pop();
    cpu.r.a = a;
    cpu.r.zn(a);
}

fn op_plp(cpu: &mut CPU, src: u16) {
    let p = cpu.stack_pop();
    cpu.r.p = p;
    cpu.r.p = p | 0b00100000;
}

fn op_rola(cpu: &mut CPU, src: u16) {
    let v_old = cpu.r.a;
    let v_new = ((v_old << 1) + cpu.r.get_flag('C') as u8) & 0xFF;
    cpu.r.a = v_new;
    cpu.r.set_flag('C', v_old & 0x80 != 0);
    cpu.r.zn(v_new);
}

fn op_rol(cpu: &mut CPU, src: u16) {
    let v_old = cpu.mmu.read(src as usize);
    let v_new = ((v_old << 1) + cpu.r.get_flag('C') as u8) & 0xFF;
    cpu.mmu.write(src as usize, v_new);

    cpu.r.set_flag('C', v_old & 0x80 != 0);
    cpu.r.zn(v_new);
}

fn op_rora(cpu: &mut CPU, src: u16) {
    let v_old = cpu.r.a;
    let v_new = ((v_old >> 1) + (cpu.r.get_flag('C') as u8)* 0x80) & 0xFF;
    cpu.r.a = v_new;

    cpu.r.set_flag('C', v_old & 0x01 != 0);
    cpu.r.zn(v_new);
}

fn op_ror(cpu: &mut CPU, src: u16) {
    let v_old = cpu.mmu.read(src as usize);
    let v_new = ((v_old >> 1) + (cpu.r.get_flag('C') as u8)* 0x80) & 0xFF;
    cpu.mmu.write(src as usize, v_new);

    cpu.r.set_flag('C', v_old & 0x01 != 0);
    cpu.r.zn(v_new);
}

fn op_rti(cpu: &mut CPU, src: u16) {
    cpu.r.p = cpu.stack_pop();
    cpu.r.pc = cpu.stack_pop_word();
}

fn op_rts(cpu: &mut CPU, src: u16) {
    cpu.r.pc = (cpu.stack_pop_word() + 1) & 0xFFFF;
}

fn op_sbc(cpu: &mut CPU, src: u16) {
    let v1 = cpu.r.a as u16;
    let mut r: i32 = 0;
    if cpu.r.get_flag('D') {
        let d1 = cpu.from_bcd(v1);
        let d2 = cpu.from_bcd(src);
        r = d1 as i32 - d2 as i32 - (!cpu.r.get_flag('C') as i32);
    } else {
        r = v1 as i32 - src as i32 - (!cpu.r.get_flag('C') as i32);
        cpu.r.a = (r & 0xFF) as u8;
    }

    cpu.r.set_flag('C', r >= 0);
    cpu.r.set_flag('V', ((v1 ^ src) & (v1 as i32 ^ r) as u16 & 0x80) != 0);
    let a = cpu.r.a;
    cpu.r.zn(a);
}

fn op_sta(cpu: &mut CPU, src: u16) {
    cpu.mmu.write(src as usize, cpu.r.a);
}
fn op_stx(cpu: &mut CPU, src: u16) {
    cpu.mmu.write(src as usize, cpu.r.x);
}
fn op_sty(cpu: &mut CPU, src: u16) {
    cpu.mmu.write(src as usize, cpu.r.y);
}

// transfers
fn op_tax(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.a;
    cpu.r.x = v;
    cpu.r.zn(v);
}

fn op_txa(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.x;
    cpu.r.a = v;
    cpu.r.zn(v);
}

fn op_tay(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.a;
    cpu.r.y = v;
    cpu.r.zn(v);
}

fn op_tya(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.y;
    cpu.r.a = v;
    cpu.r.zn(v);
}

fn op_txs(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.x as u16;
    cpu.r.s = v;
}

fn op_tsx(cpu: &mut CPU, _src: u16) {
    let v = cpu.r.s as u8;
    cpu.r.x = v;
    cpu.r.zn(v);
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
        let src = (ops[0x0A].addr)(&mut cpu);
        (ops[0x0A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 2);

        cpu.mmu.write(0, 4);
        let src = (ops[0x06].addr)(&mut cpu);
        (ops[0x06].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0), 8);
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
        assert_eq!(cpu.stack_pop_word(), 0x1000);
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

    #[test]
    fn test_dec() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00]));
        let src = (ops[0xC6].addr)(&mut cpu);
        (ops[0xC6].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0x00), 0xFF);
    }

    #[test]
    fn test_dex() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let src = (ops[0xCA].addr)(&mut cpu);
        (ops[0xCA].code)(&mut cpu, src);
        assert_eq!(cpu.r.x, 0xFF);
    }

    #[test]
    fn test_dey() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let src = (ops[0x88].addr)(&mut cpu);
        (ops[0x88].code)(&mut cpu, src);
        assert_eq!(cpu.r.x, 0xFF);
    }

    #[test]
    fn test_eor() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x0F, 0xF0, 0xFF]));
        let src = (ops[0x49].addr)(&mut cpu);
        (ops[0x49].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x0F);

        let src = (ops[0x49].addr)(&mut cpu);
        (ops[0x49].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFF);

        let src = (ops[0x49].addr)(&mut cpu);
        (ops[0x49].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x00);
    }

    #[test]
    fn test_flag_ops() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let src = (ops[0x38].addr)(&mut cpu);
        (ops[0x38].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('C'), true);
        let src = (ops[0x78].addr)(&mut cpu);
        (ops[0x78].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('I'), true);
        let src = (ops[0xF8].addr)(&mut cpu);
        (ops[0xF8].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('D'), true);

        cpu.r.set_flag('V', true);
        let src = (ops[0x18].addr)(&mut cpu);
        (ops[0x18].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('C'), false);
        let src = (ops[0x58].addr)(&mut cpu);
        (ops[0x58].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('I'), false);
        let src = (ops[0xB8].addr)(&mut cpu);
        (ops[0xB8].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('V'), false);
        let src = (ops[0xD8].addr)(&mut cpu);
        (ops[0xD8].code)(&mut cpu, src);
        assert_eq!(cpu.r.get_flag('D'), false);
    }

    #[test]
    fn test_inc() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00,]));
        let src = (ops[0xe6].addr)(&mut cpu);
        (ops[0xe6].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0x00), 0x01);
    }

    #[test]
    fn test_inx() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let src = (ops[0xE8].addr)(&mut cpu);
        (ops[0xE8].code)(&mut cpu, src);
        assert_eq!(cpu.r.x, 0x01);
    }

    #[test]
    fn test_iny() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);
        let src = (ops[0xC8].addr)(&mut cpu);
        (ops[0xC8].code)(&mut cpu, src);
        assert_eq!(cpu.r.y, 0x01);
    }

    #[test]
    fn test_jmp() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00, 0x10]));
        let src = (ops[0x4C].addr)(&mut cpu);
        (ops[0x4C].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1000);

        let src = (ops[0x6C].addr)(&mut cpu);
        (ops[0x6C].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1000);
    }

    #[test]
    fn test_jsr() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00, 0x10]));
        let src = (ops[0x20].addr)(&mut cpu);
        (ops[0x20].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1000);
        assert_eq!(cpu.stack_pop_word(), 0x1001);
    }

    #[test]
    fn test_lda() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x01,]));
        let src = (ops[0xA9].addr)(&mut cpu);
        (ops[0xA9].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);
    }

    #[test]
    fn test_ldx() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x01,]));
        let src = (ops[0xA2].addr)(&mut cpu);
        (ops[0xA2].code)(&mut cpu, src);
        assert_eq!(cpu.r.x, 0x01);
    }

    #[test]
    fn test_ldy() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x01,]));
        let src = (ops[0xA0].addr)(&mut cpu);
        (ops[0xA0].code)(&mut cpu, src);
        assert_eq!(cpu.r.y, 0x01);
    }

    #[test]
    fn test_lsr() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x00,]));
        cpu.r.a = 0x02;

        let src = (ops[0x4A].addr)(&mut cpu);
        (ops[0x4A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x01);
        assert_eq!(cpu.r.get_flag('C'), false);

        let src = (ops[0x4A].addr)(&mut cpu);
        (ops[0x4A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x00);
        assert_eq!(cpu.r.get_flag('C'), true);

        cpu.mmu.write(0x00, 0x02);
        let src = (ops[0x46].addr)(&mut cpu);
        (ops[0x46].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0x00), 0x01);
    }


    #[test]
    fn test_ora() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec![0x0F, 0xF0, 0xFF]));

        let src = (ops[0x09].addr)(&mut cpu);
        (ops[0x09].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x0F);

        let src = (ops[0x09].addr)(&mut cpu);
        (ops[0x09].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFF);

        let src = (ops[0x09].addr)(&mut cpu);
        (ops[0x09].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFF);
    }


    #[test]
    fn test_p() {
        let ops = make_op_table();
        let mut cpu = make_cpu(None);

        cpu.r.a = 0xCC;
        let src = (ops[0x48].addr)(&mut cpu);
        (ops[0x48].code)(&mut cpu, src);
        assert_eq!(cpu.stack_pop(), 0xCC);

        cpu.r.p = 0xFF;
        let src = (ops[0x08].addr)(&mut cpu);
        (ops[0x08].code)(&mut cpu, src);
        assert_eq!(cpu.stack_pop(), 0xFF);

        cpu.r.a = 0x00;
        cpu.stack_push(0xDD);
        let src = (ops[0x68].addr)(&mut cpu);
        (ops[0x68].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xDD);

        cpu.r.p = 0x20;
        cpu.stack_push(0xFD);
        let src = (ops[0x28].addr)(&mut cpu);
        (ops[0x28].code)(&mut cpu, src);
        assert_eq!(cpu.r.p, 0xFD);
    }

    #[test]
    fn test_rol() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec!(0x00)));

        cpu.r.a = 0xFF;
        let src = (ops[0x2A].addr)(&mut cpu);
        (ops[0x2A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFE);
        assert_eq!(cpu.r.get_flag('C'), true);

        let src = (ops[0x2A].addr)(&mut cpu);
        (ops[0x2A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xFD);
        assert_eq!(cpu.r.get_flag('C'), true);

        let src = (ops[0x26].addr)(&mut cpu);
        (ops[0x26].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0x00), 0x01);
        assert_eq!(cpu.r.get_flag('C'), false);
    }

    #[test]
    fn test_ror() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec!(0x00)));

        cpu.r.a = 0xFF;
        let src = (ops[0x6A].addr)(&mut cpu);
        (ops[0x6A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0x7F);
        assert_eq!(cpu.r.get_flag('C'), true);

        let src = (ops[0x6A].addr)(&mut cpu);
        (ops[0x6A].code)(&mut cpu, src);
        assert_eq!(cpu.r.a, 0xBF);
        assert_eq!(cpu.r.get_flag('C'), true);

        let src = (ops[0x66].addr)(&mut cpu);
        (ops[0x66].code)(&mut cpu, src);
        assert_eq!(cpu.mmu.read(0x00), 0x80);
        assert_eq!(cpu.r.get_flag('C'), false);
    }


    #[test]
    fn test_rti() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec!(0x00)));

        cpu.stack_push_word(0x1234);
        cpu.stack_push(0xFD);

        let src = (ops[0x40].addr)(&mut cpu);
        (ops[0x40].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1234);
        assert_eq!(cpu.r.get_flag('N'), true);
        assert_eq!(cpu.r.get_flag('V'), true);
        assert_eq!(cpu.r.get_flag('B'), true);
        assert_eq!(cpu.r.get_flag('D'), true);
        assert_eq!(cpu.r.get_flag('I'), true);
        assert_eq!(cpu.r.get_flag('Z'), false);
        assert_eq!(cpu.r.get_flag('C'), true);
    }

    #[test]
    fn test_rts() {
        let ops = make_op_table();
        let mut cpu = make_cpu(Some(vec!(0x00)));

        cpu.stack_push_word(0x1234);
        let src = (ops[0x60].addr)(&mut cpu);
        (ops[0x60].code)(&mut cpu, src);
        assert_eq!(cpu.r.pc, 0x1235);
    }


}