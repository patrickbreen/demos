
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;


use crate::rand::Rng;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::PressEvent;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

#[derive(Debug, Copy, Clone)]
pub struct GameState {

    // game state
    pub paddle_y: f64,

    pub paddle_y_velocity: f64,

    pub ball_x: f64,
    pub ball_y: f64,
    pub velocity_x: i64,
    pub velocity_y: i64,

    pub window_x: f64,
    pub window_y: f64,

    // parameters
    wait: u32,


    pub goals_scored: u32,
    pub hits: u32,
    frame_capture_every_n: u32,
    frame_n: u32,

    // input
    key: i64
}

pub fn collision(ball_x: f64, ball_y: f64, paddle_y: f64, len: f64) -> bool {
    let lower_collision = ball_x < 0.0 + len && ball_y < paddle_y + len;

    let upper_collision = ball_x < 0.0 + len && ball_y + len > paddle_y;
    
    lower_collision && upper_collision
}


pub struct Env {
    gl: GlGraphics, // OpenGL drawing backend.
    pub gs: GameState,
    window: Window,
    rendering_on: bool,
}

impl Env {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let paddle = rectangle::square(0.0, 0.0, 10.0);
        let ball = rectangle::square(0.0, 0.0, 10.0);

        let gs = self.gs.clone();

        self.gs.window_x = args.window_size[0];
        self.gs.window_y = args.window_size[1];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let t_paddle = c.transform.trans(0.0, gs.paddle_y);
            let t_ball = c.transform.trans(gs.ball_x, gs.ball_y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, paddle, t_paddle, gl);
            rectangle(RED, ball, t_ball, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {

        // process ball (bouncing and velocity)
        // detect collision on walls
        // top
        if self.gs.ball_y < 0.0 {
            self.gs.velocity_y = 1;
        }

        // right
        if self.gs.ball_x > (self.gs.window_x - 10.0) {
            self.gs.velocity_x = -1;
        }

        // bottom
        if self.gs.ball_y > (self.gs.window_y - 10.0) {
            self.gs.velocity_y = -1;
        }

        // left
        if self.gs.ball_x < 0.0 {
            self.gs.goals_scored += 1;
            // println!("goals scored: {}", self.gs.goals_scored);
            self.gs.velocity_x = 1;
        }

        // detect collision on paddle
        if collision(self.gs.ball_x, self.gs.ball_y, self.gs.paddle_y, 10.0) {
            self.gs.velocity_x = 1;
            self.gs.hits += 1;
        }

    
        self.gs.ball_x += (self.gs.velocity_x as f64);
        self.gs.ball_y += (self.gs.velocity_y as f64);

        if self.gs.paddle_y < 0.0 {
            self.gs.paddle_y = 0.0;
        }

        if self.gs.paddle_y + 10.0 > self.gs.window_y {
            self.gs.paddle_y = self.gs.window_y - 10.0;
        }

        self.gs.paddle_y += self.gs.paddle_y_velocity;
    }

    pub fn act(&mut self, mut action: u32) {
        // up
        if action == 0 {
            self.gs.paddle_y_velocity = -2.0;
        }
        // down
        else if action == 1 {
            self.gs.paddle_y_velocity = 2.0;
        }

        // nothing
        else {
            self.gs.paddle_y_velocity = 0.0;
        }
    }

    pub fn handle_press(&mut self, button: &Button) {

        // apply input on paddle
        match button {
            Button::Keyboard(Key::Up) => {
                if self.gs.paddle_y > 0.0 {
                    self.gs.paddle_y_velocity = -1.0;
                } else {
                    self.gs.paddle_y_velocity = 0.0;
                }
            }

            Button::Keyboard(Key::Down) => {
                if self.gs.paddle_y + 10.0 < self.gs.window_y {
                    self.gs.paddle_y_velocity = 1.0;
                } else {
                    self.gs.paddle_y_velocity = 0.0;
                }
            }
            _ => {
                println!("this action isn't being handled now");
            }
        }

    }

    pub fn handle_release(&mut self, button: &Button) {

        // apply input on paddle
        match button {
            Button::Keyboard(Key::Up) => {
                self.gs.paddle_y_velocity = 0.0;
            }

            Button::Keyboard(Key::Down) => {
                self.gs.paddle_y_velocity = 0.0;
            }
            _ => {
                println!("this action isn't being handled now");
            }
        }
    }

    pub fn step(&mut self) {

        let mut has_updated = false;
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {

            if let Some(r) = e.render_args() {
                if self.rendering_on {
                    self.render(&r);
                }
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
                has_updated = true
            }

            if let Some(p) = e.press_args() {
                self.handle_press(&p);
            }

            if let Some(p) = e.release_args() {
                self.handle_release(&p);
            }

            if has_updated {
                break;
            }
        }
    }

    pub fn init() -> Env {

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        let initial_window_x = 100.0;
        let initial_window_y = 60.0;

        let initial_paddle_y = 30.0;

        let initial_ball_x = 70.0;
        let initial_ball_y = 20.0;

        // Create an Glutin window.
        let mut window: Window = WindowSettings::new(
                "pong",
                [initial_window_x, initial_window_y]
            )
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game and run it.
        Env {
            gl: GlGraphics::new(opengl),

            gs: GameState {
                // game state
                paddle_y: initial_paddle_y,
                ball_x: initial_ball_x,
                ball_y: initial_ball_y,
                velocity_x: 1,
                velocity_y: 1,

                paddle_y_velocity: 0.0,

                window_x: initial_window_x,
                window_y: initial_window_y,

                // parameters
                wait: 10,


                goals_scored: 0,
                hits: 0,
                frame_capture_every_n: 10,
                frame_n: 0,

                // input
                key: 0
            },
            window: window,
            rendering_on: true,
        }
    }

    pub fn sample_action_space(&self) -> u32 {

        let mut rng = rand::thread_rng();
        return rng.gen_range(0, 2);
    }
}