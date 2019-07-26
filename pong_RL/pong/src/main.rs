extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::PressEvent;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

#[derive(Debug, Copy, Clone)]
pub struct GameState {

    // game state
    paddle_x: f64,
    paddle_y: f64,

    paddle_y_velocity: f64,

    ball_x: f64,
    ball_y: f64,

    window_x: f64,
    window_y: f64,

    // parameters
    wait: u32,
    velocity_x: i64,
    velocity_y: i64,

    goals_scored: u32,
    frame_capture_every_n: u32,
    frame_n: u32,

    // input
    key: i64
}

pub fn collision(ball_x: f64, ball_y: f64, paddle_x: f64, paddle_y: f64, len: f64) -> bool {
    let lower_collision = ball_x < paddle_x + len && ball_y < paddle_y + len;

    let upper_collision = ball_x < paddle_x + len && ball_y + len > paddle_y;
    
    lower_collision && upper_collision
}


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    gs: GameState,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let paddle = rectangle::square(0.0, 0.0, 50.0);
        let ball = rectangle::square(0.0, 0.0, 50.0);

        let gs = self.gs.clone();

        self.gs.window_x = args.window_size[0];
        self.gs.window_y = args.window_size[1];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let t_paddle = c.transform.trans(gs.paddle_x, gs.paddle_y);
            let t_ball = c.transform.trans(gs.ball_x, gs.ball_y);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, paddle, t_paddle, gl);
            rectangle(RED, ball, t_ball, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        // process ball (bouncing and velocity)
        // detect collision on walls
        // top
        if self.gs.ball_y < 0.0 {
            self.gs.velocity_y = 3;
        }

        // right
        if self.gs.ball_x > (self.gs.window_x - 50.0) {
            self.gs.velocity_x = -3;
        }

        // bottom
        if self.gs.ball_y > (self.gs.window_y - 50.0) {
            self.gs.velocity_y = -3;
        }

        // left
        if self.gs.ball_x < 0.0 {
            self.gs.goals_scored += 1;
            println!("goals scored: {}", self.gs.goals_scored);
            self.gs.velocity_x = 3;
        }

        // detect collision on paddle
        if collision(self.gs.ball_x, self.gs.ball_y, self.gs.paddle_x, self.gs.paddle_y, 50.0) {
            self.gs.velocity_x = 3;
        }

    
        self.gs.ball_x += (self.gs.velocity_x as f64);
        self.gs.ball_y += (self.gs.velocity_y as f64);

        if self.gs.paddle_y < 0.0 {
            self.gs.paddle_y = 0.0;
        }

        if self.gs.paddle_y + 50.0 > self.gs.window_y {
            self.gs.paddle_y = self.gs.window_y - 50.0;
        }

        self.gs.paddle_y += self.gs.paddle_y_velocity;

    }

    fn handle_press(&mut self, button: &Button) {

        // apply input on paddle
        match button {
            Button::Keyboard(Key::Up) => {
                if self.gs.paddle_y > 0.0 {
                    self.gs.paddle_y_velocity = -3.0;
                } else {
                    self.gs.paddle_y_velocity = 0.0;
                }
            }

            Button::Keyboard(Key::Down) => {
                if self.gs.paddle_y + 50.0 < self.gs.window_y {
                    self.gs.paddle_y_velocity = 3.0;
                } else {
                    self.gs.paddle_y_velocity = 0.0;
                }
            }
            _ => {
                println!("this action isn't being handled now");
            }
        }

    }

    fn handle_release(&mut self, button: &Button) {

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
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let initial_window_x = 500.0;
    let initial_window_y = 500.0;

    let initial_paddle_y = 50.0;

    let initial_ball_x = 275.0;
    let initial_ball_y = 100.0;

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
    let mut app = App {
        gl: GlGraphics::new(opengl),

        gs: GameState {
            // game state
            paddle_x: 0.0,
            paddle_y: initial_paddle_y,
            paddle_y_velocity: 0.0,

            ball_x: initial_ball_x,
            ball_y: initial_ball_y,

            window_x: initial_window_x,
            window_y: initial_window_y,

            // parameters
            wait: 10,
            velocity_x: 3,
            velocity_y: 3,

            goals_scored: 0,
            frame_capture_every_n: 10,
            frame_n: 0,

            // input
            key: 0
        }
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

        if let Some(p) = e.release_args() {
            app.handle_release(&p);
        }
    }
}