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

pub mod elo_expectation {
    /// f(Delta) = 1 / (1 + 10^(Delta/400))
    pub fn get_elo_expectation(delta: f64) -> f64 {
        1.0 / (1.0 + (10.0_f64.powf(delta / 400.0)))
    }
}
