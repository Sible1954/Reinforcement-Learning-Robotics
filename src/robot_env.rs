
pub enum Action {
    Left = 0,
    Right = 1,
}

impl Action {
    pub fn from_i8(val: i8) -> Option<Action> {
        match val {
            -1 => Some(Action::Left),
            1 => Some(Action::Right),
            _ => None,
        }
    }

    pub fn to_i8(&self) -> i8 {
        match self {
            Action::Left => -1,
            Action::Right => 1,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Action::Left => 0,
            Action::Right => 1,
        }
    }
}

pub struct GridWorld {
    pub agent_position: usize,
    pub goal_position: usize,
    pub grid_size: usize,
}

impl GridWorld {
    pub fn new(grid_size: usize, start_pos: usize, goal_pos: usize) -> Self {
        assert!(start_pos < grid_size && goal_pos < grid_size);
        GridWorld {
            agent_position: start_pos,
            goal_position: goal_pos,
            grid_size,
        }
    }

    pub fn reset(&mut self, start_pos: usize) -> usize {
        self.agent_position = start_pos;
        self.agent_position
    }

    pub fn step(&mut self, action: Action) -> (usize, f64, bool) {
        let mut new_position = self.agent_position as i8 + action.to_i8();

        if new_position < 0 {
            new_position = 0;
        } else if new_position >= self.grid_size as i8 {
            new_position = (self.grid_size - 1) as i8;
        }

        self.agent_position = new_position as usize;

        let reward = if self.agent_position == self.goal_position {
            10.0
        } else {
            -1.0
        };

        let done = self.agent_position == self.goal_position;

        (self.agent_position, reward, done)
    }

    pub fn render(&self) {
        let mut grid_display = String::new();
        for i in 0..self.grid_size {
            if i == self.agent_position {
                grid_display.push_str(" A ");
            } else if i == self.goal_position {
                grid_display.push_str(" G ");
            } else {
                grid_display.push_str(" - ");
            }
        }
        println!("Current Grid: [{}]", grid_display);
    }
}
