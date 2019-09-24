extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate rand;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

use self::rand::Rng;

use std::{thread, time};
use std::fs::File;
use std::io;
use std::io::prelude::*;

use ops::make_op_table;
use cpu::{CPU, Instr};
use mmu::{Block, MMU};


fn make_snake_cpu(rom_init: Option<Vec<u8>>) -> CPU {
        let mut mmu = MMU::new(&Vec::new());
        // RAM
        mmu.add_block(&Block::new(0, 0x600, false, None));
        // ROM
        mmu.add_block(&Block::new(0x600, 0x1000, true, rom_init));

        let mut cpu = CPU::new(mmu);
        cpu.r.pc = 0x600;

        // since for some reason we're reading 0xff and 0xfe as direct memory access,
        // have the stack start at 0xfc instead of 0xff
        cpu.r.s = 0xfc;
        cpu
}


pub struct SnakeApp {
    gl: GlGraphics,
    ops: [Instr; 256],
    cpu: CPU,
}

impl SnakeApp {
    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const CYAN:  [f32; 4] = [0.0, 242.0/256.0, 1.0, 1.0];
        const PURPLE:  [f32; 4] = [195.0/256.0, 0.0, 1.0, 1.0];
        const GREEN:  [f32; 4] = [64.0/256.0, 1.0, 0.0, 1.0];
        const BLUE:  [f32; 4] = [0.0, 26.0/256.0, 1.0, 1.0];
        const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        const ORANGE:  [f32; 4] = [255.0/256.0, 162.0/256.0, 0.0, 1.0];
        const BROWN:  [f32; 4] = [156.0/256.0, 90.0/256.0, 40.0/256.0, 1.0];
        const LIGHT_RED:  [f32; 4] = [255.0/256.0, 117.0/256.0, 117.0/256.0, 1.0];
        const DARK_GREY:  [f32; 4] = [92.0/256.0, 92.0/256.0, 92.0/256.0, 1.0];
        const GREY:  [f32; 4] = [135.0/256.0, 135.0/256.0, 135.0/256.0, 1.0];
        const LIGHT_GREEN:  [f32; 4] = [147.0/256.0, 255.0/256.0, 120.0/256.0, 1.0];
        const LIGHT_BLUE:  [f32; 4] = [130.0/256.0, 130.0/256.0, 255.0/256.0, 1.0];
        const LIGHT_GREY:  [f32; 4] = [194.0/256.0, 194.0/256.0, 194.0/256.0, 1.0];

        let colors = [BLACK, WHITE, RED, CYAN, PURPLE, GREEN, BLUE, YELLOW,
            ORANGE, BROWN, LIGHT_RED, DARK_GREY, GREY, LIGHT_GREEN, LIGHT_BLUE, LIGHT_GREY];

        // access the memory
        let ram = &self.cpu.mmu.blocks[0].memory.clone();

        // Print out key global variables
        let appleL = ram[0];
        let appleH = ram[1];
        let snakeHeadL = ram[16];
        let snakeHeadH = ram[17];
        let snakeBodyStart = ram[18];
        let snakeDirection = ram[2];
        let snakeLength = ram[3];

        let start = 0x200;

        let square = rectangle::square(0.0, 0.0, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let mut i = 0;
            for j in 0..32 {
                for k in 0..32 {
                    let next_byte = &ram[start + i];
                    let transform = c.transform.trans(10.0*k as f64, 10.0*j as f64);
                    rectangle(colors[*next_byte as usize], square, transform, gl);
                    i += 1;
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        let mut rng = rand::thread_rng();
        
        for i in 0..15 {

            // set 0xfe to random byte
            self.cpu.mmu.blocks[0].memory[0xfe] = rng.gen_range(0, 16);

            self.cpu.step(self.ops);
        }

    }

    fn handle_press(&mut self, button: &Button) {

        // apply input on paddle
        let ram = &mut self.cpu.mmu.blocks[0].memory;
        match button {
            Button::Keyboard(Key::Up) => {
                ram[0xff] = 0x77;
            }

            Button::Keyboard(Key::Down) => {
                ram[0xff] = 0x73;
            }

            Button::Keyboard(Key::Left) => {
                ram[0xff] = 0x61;
            }

            Button::Keyboard(Key::Right) => {
                ram[0xff] = 0x64;
            }
            _ => {
                // println!("this action isn't being handled now");
            }
        }

    }
}


pub fn play_snake() {
    let mut rom_file = File::open("/home/q/Desktop/demos/emulator_6502/snake.bin").unwrap();

    let mut buffer = Vec::new();
    rom_file.read_to_end(&mut buffer).unwrap();

    // init GUI
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [320, 320]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = SnakeApp {
        gl: GlGraphics::new(opengl),
        ops: make_op_table(),
        cpu: make_snake_cpu(Some(buffer)),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            app.handle_press(&p);
        }
    }

    panic!("snake game is over");
}