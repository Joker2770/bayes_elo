#[derive(Debug)]
pub struct EloRankDelta {
    /// Actual result of competition.
    pub w_r: f64,
    /// Expectation probability for win
    pub w_e: f64,
    /// Float coefficient
    pub k: f64,
}

impl EloRankDelta {
    pub fn get_elo_rank_delta(&self) -> f64 {
        self.k * (self.w_r - self.w_e)
    }
}
