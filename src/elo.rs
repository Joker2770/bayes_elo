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
