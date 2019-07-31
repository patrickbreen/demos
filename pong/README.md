
# Pong Reinforcement Learning game (and environment) demonstration

### Running
Run the Reinforcement Learning Pong game with:
`cargo run` or a more optimized build with `cargo run --release`

It should converge to perfect-scoring solution within about 25 episodes. I've seen it converge within 3 episodes. It is variable.

### Testing

There is no testing, because most of the non-trivial functionality is non-deterministic.


### Explanation
I implemented a form of the Q-learning algorithm to show a simple demonstration of Reinforcement Learning (RL).

I used the juggernaut library for neural nets (simple feed forward MLP). Its a unmaintained library, and I don't recommend it, but there aren't many options on the rust plaform, and it satisfies my simple needs.

I used the piston library to build a very simplistic 2-D game environment.

I could have done this much faster and less painfully in python, but thats not how I roll (right now).