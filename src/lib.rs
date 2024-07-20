/*
A library for calculating Elo in balanced and unbalanced competitions or games.
Copyright (C) 2024  joker2770

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::delta::Delta;
use crate::elo::EloRankDelta;
use crate::probability::Probability;
use crate::result_representation::ActualProbability;

mod delta;
mod elo;
mod expectation;
mod probability;
pub mod result_representation;

pub struct BayesElo {
    k_factor: f64,
    result_duty_cycle: (f64, f64, f64),
    elo_advantage: f64,
    elo_draw: f64,
}

impl BayesElo {
    /// Create new object.
    pub fn new() -> Self {
        BayesElo {
            k_factor: 32.0,
            result_duty_cycle: ActualProbability::Default.into(),
            elo_advantage: 32.8,
            elo_draw: 97.3,
        }
    }

    /// Set K factor coefficient.
    pub fn set_k_factor(&mut self, k: f64) -> f64 {
        self.k_factor = k;
        self.k_factor
    }

    /// Set result duty cycle.
    pub fn set_result_duty_cycle(&mut self, actual_p: ActualProbability) -> (f64, f64, f64) {
        self.result_duty_cycle = actual_p.into();
        self.result_duty_cycle
    }

    /// Set elo advantage.
    pub fn set_elo_advantage(&mut self, advantage: f64) -> f64 {
        self.elo_advantage = advantage;
        self.elo_advantage
    }

    /// Set elo draw.
    pub fn set_elo_draw(&mut self, draw: f64) -> f64 {
        self.elo_draw = draw;
        self.elo_draw
    }

    /// winner_elo - winner's elo.
    /// loser_elo - loser's elo.
    /// is_winner_advantage - if winner is advantage camp.
    pub fn calculate(
        &self,
        winner_elo: f64,
        loser_elo: f64,
        is_winner_advantage: bool,
    ) -> (f64, f64) {
        let delta_4_winner = Delta {
            opponent_elo: loser_elo,
            current_elo: winner_elo,
            elo_advantage: self.elo_advantage,
            elo_draw: self.elo_draw,
            is_advantage_camp: is_winner_advantage,
        };

        let delta_4_loser = Delta {
            opponent_elo: winner_elo,
            current_elo: loser_elo,
            elo_advantage: self.elo_advantage,
            elo_draw: self.elo_draw,
            is_advantage_camp: !is_winner_advantage,
        };

        let p_w_e = delta_4_winner.get_probability();
        let p_l_e = delta_4_loser.get_probability();

        let winner_elo_rank_delta = EloRankDelta {
            w_r: self.result_duty_cycle.0,
            w_e: p_w_e,
            k: self.k_factor,
        };

        let loser_elo_rank_delta = EloRankDelta {
            w_r: self.result_duty_cycle.2,
            w_e: p_l_e,
            k: self.k_factor,
        };

        let new_winner_elo = winner_elo + winner_elo_rank_delta.get_elo_rank_delta();
        let new_loser_elo = loser_elo + loser_elo_rank_delta.get_elo_rank_delta();

        (new_winner_elo, new_loser_elo)
    }

    /// first_player_elo - first-player's elo.
    /// second_player_elo - second-player's elo.
    /// is_first_player_advantage - if first-player is advantage camp.
    pub fn calculate_4_draw(
        &self,
        first_player_elo: f64,
        second_player_elo: f64,
        is_first_player_advantage: bool,
    ) -> (f64, f64) {
        let delta_4_first = Delta {
            opponent_elo: second_player_elo,
            current_elo: first_player_elo,
            elo_advantage: self.elo_advantage,
            elo_draw: self.elo_draw,
            is_advantage_camp: is_first_player_advantage,
        };

        let delta_4_second = Delta {
            opponent_elo: first_player_elo,
            current_elo: second_player_elo,
            elo_advantage: self.elo_advantage,
            elo_draw: self.elo_draw,
            is_advantage_camp: !is_first_player_advantage,
        };

        let p_w_e = delta_4_first.get_probability();
        let p_l_e = delta_4_second.get_probability();

        let first_elo_rank_delta = EloRankDelta {
            w_r: self.result_duty_cycle.1,
            w_e: p_w_e,
            k: self.k_factor,
        };

        let second_elo_rank_delta = EloRankDelta {
            w_r: self.result_duty_cycle.1,
            w_e: p_l_e,
            k: self.k_factor,
        };

        let new_first_elo = first_player_elo + first_elo_rank_delta.get_elo_rank_delta();
        let new_second_elo = second_player_elo + second_elo_rank_delta.get_elo_rank_delta();

        (new_first_elo, new_second_elo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bayes_elo_instance = BayesElo::new();
        let result = bayes_elo_instance.calculate(1700.0_f64, 1200.0_f64, true);
        println!("new result: {}, {}", result.0, result.1);
        assert_eq!(result.0 > 1700.0_f64, true);
        assert_eq!(result.1 < 1200.0_f64, true);

        let new_k = bayes_elo_instance.set_k_factor(20.0f64);
        println!("new k: {}", new_k);
        let result_4_draw = bayes_elo_instance.calculate_4_draw(1700.0_f64, 1200.0_f64, true);
        println!(
            "new result_4_draw: {}, {}",
            result_4_draw.0, result_4_draw.1
        );
        assert_eq!(result_4_draw.0 < 1700.0_f64, true);
        assert_eq!(result_4_draw.1 > 1200.0_f64, true);
    }

    #[test]
    fn set_duty_cycle() {
        let mut bayes_elo_instance = BayesElo::new();
        bayes_elo_instance.set_result_duty_cycle(ActualProbability::Alternative {
            win: 0.99,
            draw: 0.01,
            lose: 0.0,
        });
        println!(
            "result_duty_cycle: {}, {}, {}",
            bayes_elo_instance.result_duty_cycle.0,
            bayes_elo_instance.result_duty_cycle.1,
            bayes_elo_instance.result_duty_cycle.2
        );
        assert_eq!(bayes_elo_instance.result_duty_cycle.0, 0.99);
        assert_eq!(bayes_elo_instance.result_duty_cycle.1, 0.01);
        assert_eq!(bayes_elo_instance.result_duty_cycle.2, 0.0);
    }
}
