
mod cpu;
mod mmu;
mod registers;
mod ops;
mod snake;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use ops::make_op_table;
use cpu::make_cpu;
use snake::play_snake;

fn main() {

    // read rom from file
    let args: Vec<String> = env::args().collect();
    let rom_file_path = args.get(1).expect("usage: $ cargo run <rom_file.bin>");

    if (rom_file_path == "snake") {
        play_snake();
    }

    let mut rom_file = File::open(rom_file_path).unwrap();

    let mut buffer = Vec::new();
    rom_file.read_to_end(&mut buffer).unwrap();

    // init CPU and ops
    let ops = make_op_table();
    let mut cpu = make_cpu(Some(buffer));

    // run program
    println!("Program initialized, starting cpu...");

    while(true) {
        cpu.step(ops);

        // print cpu state for debugging
        println!("opcode: {:x}", cpu.mmu.read(cpu.r.pc as usize));
        println!("cpu: {:?}", cpu.r)
    }
}
