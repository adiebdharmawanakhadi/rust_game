use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Game {
    score: u32,
    player_position: (f32, f32),
}

impl Game {
    fn new() -> Self {
        Self {
            score: 0,
            player_position: (0.0, 0.0),
        }
    }

    fn update(&mut self) {
        self.score += 1;
        self.player_position.0 += 0.1;
    }
}

fn main() {
    let mut game = Game::new();

    // Update the game logic here
}