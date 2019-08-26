// extern crate piston;
// extern crate graphics;
// extern crate glutin_window;
// extern crate opengl_graphics;
// extern crate rand;


// use crate::rand::Rng;

// use piston::window::WindowSettings;
// use piston::event_loop::*;
// use piston::PressEvent;
// use piston::input::*;
// use glutin_window::GlutinWindow as Window;
// use opengl_graphics::{ GlGraphics, OpenGL };

extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use self::piston_window::*;
use self::vecmath::*;


use std::fs::File;
use std::io;
use std::io::prelude::*;

use ops::make_op_table;
use cpu::CPU;
use mmu::{Block, MMU};


fn make_snake_cpu(rom_init: Option<Vec<u8>>) -> CPU {
        let mut mmu = MMU::new(&Vec::new());
        // RAM
        mmu.add_block(&Block::new(0, 0x5ff, false, None));
        // ROM
        mmu.add_block(&Block::new(0x600, 0x1000, true, rom_init));

        let mut cpu = CPU::new(mmu);
        cpu.r.pc = 0x600;
        cpu
}

// TODO
// init the cpu
// render (draw out all the pixels)
// step the cpu
// update(poll for input)

fn draw_screen() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (200, 200);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                last_pos = None
            }
        };
        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = last_pos {
                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                    let distance = vec2_len(vec2_sub(p, pos)) as u32;

                    for i in 0..distance {
                        let diff_x = x - last_x;
                        let diff_y = y - last_y;
                        let delta = i as f32 / distance as f32;
                        let new_x = (last_x + (diff_x * delta)) as u32;
                        let new_y = (last_y + (diff_y * delta)) as u32;
                        if new_x < width && new_y < height {
                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                        };
                    };
                };

                last_pos = Some(pos)
            };

        }
    }
}


pub fn play_snake() {
    let mut rom_file = File::open("snake.bin").unwrap();

    let mut buffer = Vec::new();
    rom_file.read_to_end(&mut buffer).unwrap();

    // init CPU and ops
    let ops = make_op_table();
    let mut cpu = make_snake_cpu(Some(buffer));

    // run program
    println!("Program initialized, starting cpu...");

    draw_screen();

    // while(true) {
    //     cpu.step(ops);

    //     // print cpu state for debugging
    //     println!("opcode: {:x}", cpu.mmu.read(cpu.r.pc as usize));
    //     println!("cpu: {:?}", cpu.r)
    // }

    panic!("snake game is over");
}