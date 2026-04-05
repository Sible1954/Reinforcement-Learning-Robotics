
use std::{thread, time::Duration};
use rand::Rng;

// Define a simple environment for reinforcement learning
struct GridWorld {
    agent_position: usize,
    goal_position: usize,
    grid_size: usize,
}

impl GridWorld {
    fn new(grid_size: usize, start_pos: usize, goal_pos: usize) -> Self {
        GridWorld {
            agent_position: start_pos,
            goal_position: goal_pos,
            grid_size,
        }
    }

    fn reset(&mut self, start_pos: usize) -> usize {
        self.agent_position = start_pos;
        self.agent_position
    }

    fn step(&mut self, action: i8) -> (usize, f64, bool) {
        // action: -1 for left, 1 for right
        let mut new_position = self.agent_position as i8 + action;

        // Keep agent within grid boundaries
        if new_position < 0 {
            new_position = 0;
        } else if new_position >= self.grid_size as i8 {
            new_position = (self.grid_size - 1) as i8;
        }

        self.agent_position = new_position as usize;

        let reward = if self.agent_position == self.goal_position {
            10.0 // Positive reward for reaching the goal
        } else {
            -1.0 // Negative reward for each step
        };

        let done = self.agent_position == self.goal_position;

        (self.agent_position, reward, done)
    }

    fn render(&self) {
        let mut grid = vec!["-"; self.grid_size];
        grid[self.agent_position] = "A";
        grid[self.goal_position] = "G";
        println!("Current Grid: {}", grid.join(" "));
    }
}

// Simple Q-learning agent
struct QLearningAgent {
    q_table: Vec<Vec<f64>>,
    alpha: f64, // Learning rate
    gamma: f64, // Discount factor
    epsilon: f64, // Exploration-exploitation trade-off
    num_states: usize,
    num_actions: usize,
    rng: rand::rngs::ThreadRng,
}

impl QLearningAgent {
    fn new(num_states: usize, num_actions: usize, alpha: f64, gamma: f64, epsilon: f64) -> Self {
        QLearningAgent {
            q_table: vec![vec![0.0; num_actions]; num_states],
            alpha,
            gamma,
            epsilon,
            num_states,
            num_actions,
            rng: rand::thread_rng(),
        }
    }

    fn choose_action(&mut self, state: usize) -> i8 {
        if self.rng.gen::<f64>() < self.epsilon {
            // Explore: choose a random action
            if self.rng.gen::<f64>() < 0.5 {
                -1 // Left
            } else {
                1 // Right
            }
        } else {
            // Exploit: choose the best action from Q-table
            let q_values = &self.q_table[state];
            if q_values[0] > q_values[1] {
                -1
            } else {
                1
            }
        }
    }

    fn update_q_table(&mut self, state: usize, action_idx: usize, reward: f64, next_state: usize, done: bool) {
        let old_q_value = self.q_table[state][action_idx];
        let max_next_q = if done { 0.0 } else { *self.q_table[next_state].iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0) };
        
        let new_q_value = old_q_value + self.alpha * (reward + self.gamma * max_next_q - old_q_value);
        self.q_table[state][action_idx] = new_q_value;
    }
}

fn main() {
    println!("\n--- Reinforcement Learning Robotics Simulation (GridWorld) ---\n");

    let grid_size = 10;
    let start_position = 0;
    let goal_position = 9;
    let num_episodes = 100;

    let mut env = GridWorld::new(grid_size, start_position, goal_position);
    let mut agent = QLearningAgent::new(grid_size, 2, 0.1, 0.9, 0.1);

    for episode in 0..num_episodes {
        let mut state = env.reset(start_position);
        let mut done = false;
        let mut total_reward = 0.0;

        println!("\nEpisode {}:", episode + 1);
        env.render();

        while !done {
            let action = agent.choose_action(state);
            let action_idx = if action == -1 { 0 } else { 1 }; // Map -1 to 0, 1 to 1
            let (next_state, reward, is_done) = env.step(action);

            agent.update_q_table(state, action_idx, reward, next_state, is_done);

            state = next_state;
            total_reward += reward;
            done = is_done;

            env.render();
            thread::sleep(Duration::from_millis(100)); // Simulate real-time steps
        }
        println!("Episode {} finished with total reward: {}\n", episode + 1, total_reward);
    }

    println!("Training complete. Final Q-table (simplified view):");
    for (i, row) in agent.q_table.iter().enumerate() {
        println!("State {}: Left Q={:.2}, Right Q={:.2}", i, row[0], row[1]);
    }

    println!("\n--- Simulation Complete ---\n");
}
