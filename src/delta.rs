use crate::{expectation::elo_expectation, probability::Probability};

#[derive(Debug)]
pub struct Delta {
    /// Opponent Elo.
    pub opponent_elo: f64,
    /// Current Elo.
    pub current_elo: f64,
    /// Elo advantage.
    pub elo_advantage: f64,
    /// elo draw.
    pub elo_draw: f64,
    /// The camp flag.
    pub is_advantage_camp: bool,
}

impl Delta {
    fn get_delta(&self) -> f64 {
        if self.is_advantage_camp {
            self.opponent_elo - self.current_elo - self.elo_advantage + self.elo_draw
        } else {
            self.opponent_elo - self.current_elo + self.elo_advantage + self.elo_draw
        }
    }
}

impl Probability for Delta {
    fn get_probability(&self) -> f64 {
        elo_expectation::get_elo_expectation(self.get_delta())
    }
}