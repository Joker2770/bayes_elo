# bayes_elo

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/72cf25d0c0944860929596f61d2d8749)](https://app.codacy.com/gh/Joker2770/bayes_elo?utm_source=github.com&utm_medium=referral&utm_content=Joker2770/bayes_elo&utm_campaign=Badge_Grade)
[![Rust](https://github.com/Joker2770/bayes_elo/actions/workflows/rust.yml/badge.svg)](https://github.com/Joker2770/bayes_elo/actions/workflows/rust.yml)

A library for calculating Elo in balanced and unbalanced competitions or games.

## Usage

Assuming you have installed bayes_elo crate.

```rust
use bayes_elo::BayesElo;

let mut bayes_elo_instance = BayesElo::new();

/// @params winner_elo: f64 - winner's elo.
///         loser_elo: f64 - loser's elo.
///         is_winner_advantage: bool - if winner is advantage camp.
/// @return (new_winner_elo: f64, new_loser_elo: f64).
let result = bayes_elo_instance.calculate(1700.0_f64, 1200.0_f64, true);
println!("new result: {}, {}", result.0, result.1);
assert_eq!(result.0 > 1700.0_f64, true);
assert_eq!(result.1 < 1200.0_f64, true);

let new_k = bayes_elo_instance.set_k_factor(20.0f64);
println!("new k: {}", new_k);

/// @params first_player_elo: f64 - first-player's elo.
///         second_player_elo: f64 - second-player's elo.
///         is_first_player_advantage: bool - if first-player is advantage camp.
/// @return (new_first_elo: f64, new_second_elo: f64).
let result_4_draw = bayes_elo_instance.calculate_4_draw(1700.0_f64, 1200.0_f64, true);
println!("new result_4_draw: {}, {}", result_4_draw.0, result_4_draw.1);
assert_eq!(result_4_draw.0 < 1700.0_f64, true);
assert_eq!(result_4_draw.1 > 1200.0_f64, true);
```

## License

* GNU GENERAL PUBLIC LICENSE Version 3

## Reference

1.  [Bayesian-Elo](https://www.remi-coulom.fr/Bayesian-Elo/).
