use crate::weight_conversion::{WeightUnit, WeightUnitType};

#[derive(Debug)]
pub struct Plate {
    pub weight: WeightUnit,
    pub quantity: i8,
}

pub fn calculate_plates(weight: WeightUnit) -> Vec<Plate> {
    let plate_weights = match weight.unit_type {
        WeightUnitType::Pounds => vec![45.0, 35.0, 25.0, 10.0, 5.0, 2.5],
        WeightUnitType::Kilograms => vec![50.0, 25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25],
    };

    let bar_weight = match weight.unit_type {
        WeightUnitType::Pounds => 45.0,
        WeightUnitType::Kilograms => 20.0,
    };

    let mut remaining_weight = weight.value - bar_weight;

    if remaining_weight < 0.0 {
        return Vec::new();
    }

    let mut plates: Vec<Plate> = Vec::new();

    for &plate_weight in &plate_weights {
        let mut count = 0;
        while remaining_weight >= 2.0 * plate_weight {
            remaining_weight -= 2.0 * plate_weight;
            count += 1;
        }

        if count > 0 {
            plates.push(Plate {
                weight: WeightUnit::new(plate_weight, weight.unit_type),
                quantity: count,
            })
        }
    }

    plates
}

pub fn calculate_total_weight(plates: Vec<Plate>) -> WeightUnit {
    let unit = plates[0].weight.unit_type;
    let bar_weight = match unit {
        WeightUnitType::Pounds => 45.0,
        WeightUnitType::Kilograms => 20.0,
    };

    let mut total_weight = bar_weight;
    for plate in plates {
        total_weight += 2.0 * plate.weight.value * plate.quantity as f64;
    }

    WeightUnit::new(total_weight, unit)
}
