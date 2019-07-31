extern crate juggernaut;


extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

mod agent;
mod env;

use agent::Agent;
use env::Env;


fn main() {

    let mut env = Env::init();
    let mut agent = Agent::create();

    for episode in 0..100 {
        env.gs.goals_scored = 0;
        env.gs.hits = 0;

        println!("Begining episode: {}", episode);
        // agent.wipe_memory();

        // track state (of previous time step)
        let mut previous_state : [f64; 5] = [
            env.gs.paddle_y,
            env.gs.ball_x,
            env.gs.ball_y,
            env.gs.velocity_x as f64,
            env.gs.velocity_y as f64,
        ];
        let mut previous_action = agent.act(&env);

        // engender action in environment
        env.act(previous_action);
        env.step();

        // 1000 time steps per episode
        for time_t in 0..1000 {

            
            let goals_scored_before = env.gs.goals_scored;
            let hits_before = env.gs.hits;

            
            let mut current_state : [f64; 5] = [
                env.gs.paddle_y,
                env.gs.ball_x,
                env.gs.ball_y,
                env.gs.velocity_x as f64,
                env.gs.velocity_y as f64,
            ];
            let mut current_action = agent.act(&env);

            // engender action in environment
            env.act(current_action);
            env.step();

            // save state to agent memory (-1 if goal is scored, otherwise 0)
            let mut reward: f64 = ((goals_scored_before as f64) - (env.gs.goals_scored as f64)) * 30.0;
            reward += ((env.gs.hits as f64) - (hits_before as f64)) * 30.0;
            // remember
            agent.remember(previous_state, previous_action, reward, current_state, current_action);

            // update previous state's
            previous_state = current_state;
            previous_action = current_action;
        }
        println!("Episode over. Score: {}. Replaying.", env.gs.goals_scored);
        // do agent training "replay"
        let q_convergence = agent.replay();

        println!("Replay over. Q convergence: {}.", q_convergence);

    }
}