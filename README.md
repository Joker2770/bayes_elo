# bayes_elo
A library for calculating Elo in balanced and unbalanced competitions or games.

## Usage

Assuming you have installed bayes_elo crate.

```rust
use bayes_elo::BayesElo;

let mut bayes_elo_instance = BayesElo::new();

/// @params winner_elo: f64 - winner's elo.
///         loser_elo: f64 - loser's elo.
///         is_winner_advantage：bool - if winner is advantage camp.
/// @return (new_winner_elo: f64, new_loser_elo: f64).
let result = bayes_elo_instance.calculate(1700.0_f64, 1200.0_f64, true);
println!("new result: {}, {}", result.0, result.1);
assert_eq!(result.0 > 1700.0_f64, true);
assert_eq!(result.1 < 1200.0_f64, true);

let new_k = bayes_elo_instance.set_k_factor(20.0f64);
println!("new k: {}", new_k);

/// @params winner_elo: f64 - winner's elo.
///         loser_elo: f64 - loser's elo.
///         is_winner_advantage：bool - if winner is advantage camp.
/// @return (new_winner_elo: f64, new_loser_elo: f64).
let result_4_draw = bayes_elo_instance.calculate_4_draw(1700.0_f64, 1200.0_f64, true);
println!("new result_4_draw: {}, {}", result_4_draw.0, result_4_draw.1);
assert_eq!(result_4_draw.0 < 1700.0_f64, true);
assert_eq!(result_4_draw.1 > 1200.0_f64, true);
```

## License

* GNU GENERAL PUBLIC LICENSE Version 3

## Reference

1.  [Bayesian-Elo](https://www.remi-coulom.fr/Bayesian-Elo/).
