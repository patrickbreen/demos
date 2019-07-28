extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate juggernaut;

use crate::rand::Rng;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::PressEvent;
use piston::input::*;
use glutin_window::GlutinWindow as Window;


use juggernaut::nl::NeuralLayer;
use juggernaut::nn::NeuralNetwork;
use juggernaut::activation::Sigmoid;
use juggernaut::sample::Sample;
use juggernaut::matrix::MatrixTrait;
use opengl_graphics::{ GlGraphics, OpenGL };

#[derive(Debug, Copy, Clone)]
pub struct GameState {

    // game state
    paddle_y: f64,

    paddle_y_velocity: f64,

    ball_x: f64,
    ball_y: f64,
    velocity_x: i64,
    velocity_y: i64,

    window_x: f64,
    window_y: f64,

    // parameters
    wait: u32,


    goals_scored: u32,
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
    gs: GameState,
    window: Window,
    rendering_on: bool,
}

impl Env {
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

            let t_paddle = c.transform.trans(0.0, gs.paddle_y);
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
        if collision(self.gs.ball_x, self.gs.ball_y, self.gs.paddle_y, 50.0) {
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

    fn act(&mut self, mut action: [f64; 3]) {
        // determine which action is appropriate:

        // if there isn't much divergence between max and min, then take a random action
        if (action[0] - action[0]).abs() < 0.2 && (action[1] - action[2]).abs() < 0.2 {
            action = self.sample_action_space();
        }

        // up
        if action[0] > action[1] && action[0] > action[2] {
            self.gs.paddle_y_velocity = -3.0;
        }
        // down
        else if action[1] > action[0] && action[1] > action[2] {
            self.gs.paddle_y_velocity = 3.0;
        }

        // nothing
        else {
            self.gs.paddle_y_velocity = 0.0;
        }
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

    fn step(&mut self) {

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

    fn init() -> Env {

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
        Env {
            gl: GlGraphics::new(opengl),

            gs: GameState {
                // game state
                paddle_y: initial_paddle_y,
                ball_x: initial_ball_x,
                ball_y: initial_ball_y,
                velocity_x: 3,
                velocity_y: 3,

                paddle_y_velocity: 0.0,

                window_x: initial_window_x,
                window_y: initial_window_y,

                // parameters
                wait: 10,


                goals_scored: 0,
                frame_capture_every_n: 10,
                frame_n: 0,

                // input
                key: 0
            },
            window: window,
            rendering_on: true,
        }
    }

    fn sample_action_space(&self) -> [f64; 3] {

        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, 2);

        [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ][i]
    }
}

// DQNAgent

pub struct DataStep {
    action: (f64, f64, f64),
    reward: f64,
    state: (f64, f64, f64, f64, f64),
    next_state: (f64, f64, f64, f64, f64),
}

pub struct Agent {
    mem: Vec<DataStep>,
    nn: NeuralNetwork,
    lr: f64,
    gamma: f64,
}

impl Agent {

    fn create() -> Agent {

        let mut nn = NeuralNetwork::new();
        let sig_activation = Sigmoid::new();

        // 1st layer = 2 outputs - 8 inputs:
        // (paddle_y, ball_x, ball_y, velocity_x, velocity_y, aaction_up, action_down, action_sideways)
        nn.add_layer(NeuralLayer::new(2, 8, sig_activation));

        // 2nd layer = 1 outputs (q) - 2 inputs
        nn.add_layer(NeuralLayer::new(3, 2, sig_activation));
        Agent {
            mem: Vec::new(),
            nn: nn,
            lr: 0.1,
            gamma: 0.1,
        }
    }

    fn remember(&mut self,
        action: (f64, f64, f64),
        reward: f64,
        state: (f64, f64, f64, f64, f64),
        next_state: (f64, f64, f64, f64, f64)) {
        // add arguments to memory
        // self.mem.insert
    }

    fn act(&mut self, env: &Env) -> [f64; 3] {

        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen();

        // return predicted/infered action
        let mut max_q = std::f64::NEG_INFINITY;
        let mut qs = [0.0, 0.0, 0.0];
        let mut max_i = 0;
        for i in 0..3 {
            let mut action = vec![0.0; 3];
            action[i] = 1.0;
            let mut input_data = vec![
                env.gs.paddle_y,
                env.gs.ball_x,
                env.gs.ball_y,
                env.gs.velocity_x as f64,
                env.gs.velocity_y as f64
            ];


            input_data.extend(action.iter().cloned());
            let think = self.nn.evaluate(&Sample::predict(input_data));
            let q = think.get(0, 0);
            
            qs[i] = q
        }
        qs

    }

    fn replay(&mut self) -> f64 {

        // TODO get the memory in the right format
        let mut q_convergence = 0.0;

        // for each batch,
        for (i, mem_step) in self.mem.iter().enumerate() {
            let dummy_input = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
            let dummy_reward = 1.0;

            let q_0 = self.nn.evaluate(&Sample::predict(dummy_input.clone())).get(0, 0);
            let q_1 = self.nn.evaluate(&Sample::predict(dummy_input.clone())).get(0, 0);

            let q_0_new = q_0 + (dummy_reward + self.gamma * q_1);

            let dataset = vec![
                Sample::new(dummy_input.clone(), vec![q_0_new]),
            ];

            self.nn.train(dataset, 1, self.lr);

            q_convergence += (q_0_new - q_0);
        }
        q_convergence / (self.mem.len() as f64)
    }

    fn wipe_memory(&mut self) {
        self.mem.clear();
    }
}


fn main() {

    let mut env = Env::init();
    let mut agent = Agent::create();

    for episode in 0.. 100 {

        println!("Begining episode: {}", episode);
        agent.wipe_memory();

        for time_t in 0..5000 {

            let goals_scored_before = env.gs.goals_scored;

            env.step();
            let action = agent.act(&env);

            // engender action in environment
            env.act(action);

            // save state to agent memory (-1 if goal is scored, otherwise 0)
            let reward: f64 = (goals_scored_before as f64) - (env.gs.goals_scored as f64);



        }
        println!("Episode over. Score: {}. Replaying.", env.gs.goals_scored);
        // do agent training "replay"
        let q_convergence = agent.replay();

        println!("Replay over. Q convergence: {}.", q_convergence);

    }
}