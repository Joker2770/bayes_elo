/*
A library for calculating Elo in balanced and not balanced competitions or games.
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