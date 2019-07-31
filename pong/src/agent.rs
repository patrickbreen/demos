extern crate juggernaut;
extern crate rand;


use juggernaut::nl::NeuralLayer;
use juggernaut::nn::NeuralNetwork;
use juggernaut::activation::Sigmoid;
use juggernaut::sample::Sample;
use juggernaut::matrix::MatrixTrait;

use crate::rand::Rng;


use crate::env::Env;


// DQNAgent

#[derive(Debug)]
pub struct DataStep {
    previous_state: [f64; 5],
    previous_action: u32,
    reward: f64,
    current_state: [f64; 5],
    current_action: u32,

}

pub struct Agent {
    mem: Vec<DataStep>,
    nn: NeuralNetwork,
    lr: f64,
    gamma: f64,
}

impl Agent {

    pub fn create() -> Agent {

        let mut nn = NeuralNetwork::new();
        let sig_activation = Sigmoid::new();

        // 1st layer = 2 outputs - 8 inputs:
        // (paddle_y, ball_x, ball_y, velocity_x, velocity_y, aaction_up, action_down, action_sideways)
        nn.add_layer(NeuralLayer::new(4, 1, sig_activation));

        nn.add_layer(NeuralLayer::new(3, 4, sig_activation));


        nn.set_shuffle_data(true);
        Agent {
            mem: Vec::new(),
            nn: nn,
            lr: 0.5,
            gamma: 0.01,
        }
    }

    pub fn remember(&mut self,
        previous_state: [f64; 5],
        previous_action: u32,
        reward: f64,
        current_state: [f64; 5],
        current_action: u32) {
        // add arguments to memory
        self.mem.push( DataStep {
            previous_state: previous_state,
            previous_action: previous_action,
            reward: reward,
            current_state: current_state,
            current_action: current_action,
        });
    }

    pub fn act(&mut self, env: &Env) -> u32 {


        let mut input_data = vec![
            env.gs.paddle_y,
            env.gs.ball_x,
            env.gs.ball_y,
            env.gs.velocity_x as f64,
            env.gs.velocity_y as f64
        ];

        let input_feature = vec![env.gs.paddle_y - env.gs.ball_y];

        let think = self.nn.evaluate(&Sample::predict(input_feature));
        

        // if there isn't much divergence between max and min,
        // or with probability .1, then take a random action

        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen();

        // println!("{:?}", think);

        if (think.get(0,0) - think.get(0,1)).abs() < 0.1 && (think.get(0,1) - think.get(0,2)).abs() < 0.1 || theta < 0.01 {
            return env.sample_action_space();
        }
        
        // otherwise determine which action is appropriate:

        if (think.get(0,0) > think.get(0,1)) && (think.get(0,0) > think.get(0,2)) {
            return 0;
        }
        else if (think.get(0,1) > think.get(0,0)) && (think.get(0,1) > think.get(0,2)) {
            return 1;
        }
        return 2;
    }

    pub fn replay(&mut self) -> f64 {

        let mut q_convergence = 0.0;

        let mut dataset = Vec::new();

        // for each batch,
        for (i, mem_step) in self.mem.iter().enumerate() {

            let previous_state = mem_step.previous_state[0] - mem_step.previous_state[2];
            let current_state = mem_step.current_state[0] - mem_step.current_state[2];

            let mut q_0 = self.nn.evaluate(&Sample::predict(vec![previous_state])).row(0).clone();
            let q_1 = self.nn.evaluate(&Sample::predict(vec![current_state])).row(0).clone();
            q_convergence -= q_0[mem_step.previous_action as usize];
            // let q_0_new = q_0 + (mem_step.reward + self.gamma * q_1);

            // println!("q_0 before: {:?}", q_0);
            // println!("mem_step before: {:?}", mem_step);

            for i in 0..3 {

                if i == mem_step.previous_action as usize {
                    q_0[i] += mem_step.reward;
                } else {
                    q_0[i] += (-mem_step.reward) / 2.0;
                }

                q_0[i] += self.gamma * q_1[i];
            }

            // println!("q_0 after: {:?}", q_0);
            // panic!();
            
            q_convergence += q_0[mem_step.previous_action as usize];
            
            let example = Sample::new(vec![previous_state], q_0.to_vec());
            dataset.push(example);
        }
        self.nn.train(dataset, 100, 0.1);
        q_convergence / (self.mem.len() as f64)
    }

    pub fn wipe_memory(&mut self) {
        self.mem.clear();
    }
}