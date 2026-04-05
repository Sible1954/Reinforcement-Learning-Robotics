
use rand::Rng;
use crate::robot_env::Action;

pub struct QLearningAgent {
    q_table: Vec<Vec<f64>>,
    alpha: f64,
    gamma: f64,
    epsilon: f64,
    num_states: usize,
    num_actions: usize,
    rng: rand::rngs::ThreadRng,
}

impl QLearningAgent {
    pub fn new(num_states: usize, alpha: f64, gamma: f64, epsilon: f64) -> Self {
        QLearningAgent {
            q_table: vec![vec![0.0; 2]; num_states],
            alpha,
            gamma,
            epsilon,
            num_states,
            num_actions: 2,
            rng: rand::thread_rng(),
        }
    }

    pub fn choose_action(&mut self, state: usize) -> Action {
        if self.rng.gen::<f64>() < self.epsilon {
            if self.rng.gen::<f64>() < 0.5 {
                Action::Left
            } else {
                Action::Right
            }
        } else {
            self.get_best_action(state)
        }
    }

    pub fn get_best_action(&self, state: usize) -> Action {
        let q_values = &self.q_table[state];
        if q_values[Action::Left.to_usize()] > q_values[Action::Right.to_usize()] {
            Action::Left
        } else {
            Action::Right
        }
    }

    pub fn update_q_table(&mut self, state: usize, action: Action, reward: f64, next_state: usize, done: bool) {
        let action_idx = action.to_usize();
        let old_q_value = self.q_table[state][action_idx];
        
        let max_next_q = if done { 
            0.0 
        } else { 
            *self.q_table[next_state].iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0) 
        };
        
        let new_q_value = old_q_value + self.alpha * (reward + self.gamma * max_next_q - old_q_value);
        self.q_table[state][action_idx] = new_q_value;
    }

    pub fn get_q_values(&self, state: usize) -> &[f64] {
        &self.q_table[state]
    }
}
