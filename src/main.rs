
mod robot_env;
mod agent;

use robot_env::{GridWorld, Action};
use agent::QLearningAgent;
use std::{thread, time::Duration};

fn main() {
    println!("
--- Reinforcement Learning Robotics Simulation (GridWorld) ---
");

    let grid_size = 10;
    let start_position = 0;
    let goal_position = 9;
    let num_episodes = 200;

    let mut env = GridWorld::new(grid_size, start_position, goal_position);
    let mut agent = QLearningAgent::new(grid_size, 0.1, 0.9, 0.1);

    for episode in 0..num_episodes {
        let mut state = env.reset(start_position);
        let mut done = false;
        let mut total_reward = 0.0;

        let mut steps_in_episode = 0;
        while !done && steps_in_episode < 100 { 
            let action = agent.choose_action(state);
            let (next_state, reward, is_done) = env.step(action);

            agent.update_q_table(state, action, reward, next_state, is_done);

            state = next_state;
            total_reward += reward;
            done = is_done;
            steps_in_episode += 1;
        }
        println!("Episode {} finished in {} steps with total reward: {:.2}", episode + 1, steps_in_episode, total_reward);
    }

    println!("
Training complete. Final Q-table (first few states):");
    for i in 0..std::cmp::min(5, grid_size) {
        let q_values = agent.get_q_values(i);
        println!("State {}: Left Q={:.2}, Right Q={:.2}", i, q_values[0], q_values[1]);
    }

    println!("
--- Simulation Complete ---
");

    println!("
--- Demonstrating Learned Policy ---");
    let mut state = env.reset(start_position);
    let mut done = false;
    let mut path = vec![state];
    while !done {
        let action = agent.get_best_action(state);
        let (next_state, _, is_done) = env.step(action);
        state = next_state;
        path.push(state);
        done = is_done;
    }
    println!("Learned path to goal: {:?}", path);
    println!("--- Demonstration Complete ---");
}
