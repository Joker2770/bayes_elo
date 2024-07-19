
/// Result representation
///
/// # Example
///
/// ## Default actual probability
/// ```
/// use bayes_elo::result_representation::ActualProbability;
///
/// # let Default_P = ActualProbability::Default.into();
/// assert_eq!((1.0, 0.5, 0.0), Default_P);
/// ```
///
/// ## Alternative actual probability
/// ```
/// use bayes_elo::result_representation::ActualProbability;
///
/// # let Test_P = ActualProbability::Alternative { win: 0.4 , draw: 0.1, lose: 0.5 }.into();
/// assert_eq!((0.4, 0.1, 0.5), Test_P);
/// ```

const DEFAULT_ACTUAL_PROBABILITY : (f64, f64, f64) = (1.0, 0.5, 0.0);

#[derive(Debug, Clone, Copy)]
pub enum ActualProbability {
    Default,
    Alternative { win:f64, draw:f64, lose:f64},
}

impl From<ActualProbability> for (f64, f64, f64) {
    fn from(actual_p: ActualProbability) -> Self {
        match actual_p {
            ActualProbability::Default => DEFAULT_ACTUAL_PROBABILITY,
            ActualProbability::Alternative {win, draw, lose} => (win, draw, lose),
        }
    }
}

impl PartialEq for ActualProbability {
    fn eq(&self, other: &Self) -> bool {
        let (lhs, rhs): ((f64, f64, f64), (f64, f64, f64)) = ((*self).into(), (*other).into());
        lhs == rhs
    }
}

