use crate::weight_conversion::WeightUnit;

#[derive(Clone, Debug)]
pub enum Formula {
    Epley,
    Brzycki,
    McGlothin,
    Kelley,
}

impl std::fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Formula::Epley => "Epley",
            Formula::Brzycki => "Brzycki",
            Formula::McGlothin => "McGlothin",
            Formula::Kelley => "Kelley",
        };
        write!(f, "{}", name)
    }
}

/// Calculate 1RM using best formula
pub fn calc(weight: WeightUnit, reps: u8) -> WeightUnit {
    let formula = best_formula(reps);

    calc_with_formula(weight, reps, formula)
}

/// Calculate 1RM using the specified formula
pub fn calc_with_formula(weight: WeightUnit, reps: u8, formula: Formula) -> WeightUnit {
    match formula {
        Formula::Epley => epley(weight, reps),
        Formula::Brzycki => brzycki(weight, reps),
        Formula::McGlothin => mcglothin(weight, reps),
        Formula::Kelley => kelley(weight, reps),
    }
}

/// Helper function to calculate one rep max with a given formula
fn calculate_1rm(weight: WeightUnit, reps: u8, formula: fn(f64, u8) -> f64) -> WeightUnit {
    let weight_in_f64 = weight.to_f64();
    let result = formula(weight_in_f64, reps);
    WeightUnit::new(result, weight.unit_type)
}

/// Calculate 1RM using the Epley formula
fn epley(weight: WeightUnit, reps: u8) -> WeightUnit {
    calculate_1rm(weight, reps, |w, r| w * (1.0 + 0.0333 * r as f64))
}

/// Calculate 1RM using the Brzycki formula
fn brzycki(weight: WeightUnit, reps: u8) -> WeightUnit {
    calculate_1rm(weight, reps, |w, r| w * (36.0 / (37.0 - r as f64)))
}

/// Calculate 1RM using the McGlothin formula
fn mcglothin(weight: WeightUnit, reps: u8) -> WeightUnit {
    calculate_1rm(weight, reps, |w, r| w * (1.0 + 0.025 * r as f64))
}

/// Calculate 1RM using the Kelley formula
fn kelley(weight: WeightUnit, reps: u8) -> WeightUnit {
    calculate_1rm(weight, reps, |w, r| w * (1.0 + 0.0278 * r as f64))
}

/// Determine best 1RM formula based on number or reps
pub fn best_formula(reps: u8) -> Formula {
    match reps {
        1..=6 => Formula::Epley,
        7..=10 => Formula::Brzycki,
        11..=15 => Formula::McGlothin,
        _ => Formula::Epley,
    }
}

/// Calculate the minimum and maximum 1RM values across all formulas
pub fn calc_range(weight: WeightUnit, reps: u8) -> (WeightUnit, WeightUnit) {
    let formulas = [
        Formula::Epley,
        Formula::Brzycki,
        Formula::McGlothin,
        Formula::Kelley,
    ];

    let results: Vec<WeightUnit> = formulas
        .iter()
        .map(|formula| calc_with_formula(weight.clone(), reps, formula.clone()))
        .collect();

    let min = results
        .iter()
        .min_by(|a, b| a.to_f64().partial_cmp(&b.to_f64()).unwrap())
        .unwrap();

    let max = results
        .iter()
        .max_by(|a, b| a.to_f64().partial_cmp(&b.to_f64()).unwrap())
        .unwrap();

    (min.clone(), max.clone())
}
