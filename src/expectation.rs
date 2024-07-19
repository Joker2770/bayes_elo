pub mod elo_expectation {
    pub fn get_elo_expectation(delta: f64) -> f64 {
        1.0 / (1.0 + (10.0_f64.powf(delta / 400.0)))
    }
}
