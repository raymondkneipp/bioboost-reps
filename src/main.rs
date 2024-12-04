mod components;
mod one_rep_max;
mod plates;
mod weight_conversion;

use components::app::App;
use one_rep_max::Formula;
use plates::{calculate_plates, calculate_total_weight};
use weight_conversion::{WeightUnit, WeightUnitType};

fn main() {
    console_error_panic_hook::set_once();

    println!("Welcome to Reps!");

    let weight = WeightUnit::new(225, WeightUnitType::Pounds);

    println!("{}", weight);
    println!("{}", weight.to_kilograms());

    for reps in 1..=16 {
        print!("{} for {} reps\t", weight, reps);
        print!(
            "Epley: {}\t",
            one_rep_max::calc_with_formula(weight, reps, Formula::Epley)
        );
        print!(
            "Brzychi: {}\t",
            one_rep_max::calc_with_formula(weight, reps, Formula::Brzycki)
        );
        print!(
            "McGlothin: {}\t",
            one_rep_max::calc_with_formula(weight, reps, Formula::McGlothin)
        );
        print!(
            "Kelley: {}\t",
            one_rep_max::calc_with_formula(weight, reps, Formula::Kelley)
        );

        let best = one_rep_max::best_formula(reps);
        print!("Best: {}:\t{}\n", best, one_rep_max::calc(weight, reps));
    }

    let deadlift = WeightUnit::new(315, WeightUnitType::Pounds).to_kilograms();
    let reps = 15;
    println!("Deadlift of {} for {} reps", deadlift, reps);
    println!("Best Formula 1RM: {}", one_rep_max::calc(deadlift, reps));
    println!("Best Formula: {}", one_rep_max::best_formula(reps));
    println!(
        "Epley 1RM: {}",
        one_rep_max::calc_with_formula(deadlift, reps, Formula::Epley)
    );
    println!(
        "Brzycki 1RM: {}",
        one_rep_max::calc_with_formula(deadlift, reps, Formula::Brzycki)
    );
    println!(
        "McGlothin 1RM: {}",
        one_rep_max::calc_with_formula(deadlift, reps, Formula::McGlothin)
    );
    println!(
        "Kelley 1RM: {}",
        one_rep_max::calc_with_formula(deadlift, reps, Formula::Kelley)
    );

    let (min, max) = one_rep_max::calc_range(deadlift, reps);
    println!("{} / {} / {}", min, one_rep_max::calc(deadlift, reps), max);

    for i in [deadlift, min, one_rep_max::calc(deadlift, reps), max] {
        println!("\nTo get close to but not over {}", i);
        let p = calculate_plates(i);

        for plate in &p {
            println!("\t{}x {} on each side", plate.quantity, plate.weight,);
        }

        println!("\tActual weight: {}\n", calculate_total_weight(p));
    }

    leptos::mount::mount_to_body(App)
}
