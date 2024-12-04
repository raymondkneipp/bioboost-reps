/// Conversion constant: 1 pound = 0.45359237 kilograms
const POUNDS_TO_KILOGRAMS: f64 = 0.45359237;

/// Enum to represent different unit types
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeightUnitType {
    Pounds,
    Kilograms,
}

/// Enum to represent weight in either Pounds or Kilograms
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WeightUnit {
    pub value: f64,
    pub unit_type: WeightUnitType,
}

impl WeightUnit {
    /// Convert the weight to `f64` (value in the current unit)
    pub fn to_f64(&self) -> f64 {
        self.value
    }

    pub fn to_string(&self) -> String {
        format!("{:.2} {}", self.value, self.get_abbreviation())
    }

    /// Convert to Kilograms if the weight is in Pounds
    pub fn to_kilograms(&self) -> WeightUnit {
        match self.unit_type {
            WeightUnitType::Pounds => WeightUnit {
                value: self.value * POUNDS_TO_KILOGRAMS,
                unit_type: WeightUnitType::Kilograms,
            },
            WeightUnitType::Kilograms => *self,
        }
    }

    /// Convert to Pounds if the weight is in Kilograms
    pub fn to_pounds(&self) -> WeightUnit {
        match self.unit_type {
            WeightUnitType::Kilograms => WeightUnit {
                value: self.value / POUNDS_TO_KILOGRAMS,
                unit_type: WeightUnitType::Pounds,
            },
            WeightUnitType::Pounds => *self,
        }
    }

    /// Get the unit as a string (either "Pounds" or "Kilograms")
    pub fn get_unit(&self) -> &str {
        match self.unit_type {
            WeightUnitType::Pounds => "Pounds",
            WeightUnitType::Kilograms => "Kilograms",
        }
    }

    pub fn get_abbreviation(&self) -> &str {
        match self.unit_type {
            WeightUnitType::Pounds => "lbs",
            WeightUnitType::Kilograms => "kg",
        }
    }

    /// Create a new `WeightUnit` from a value and a unit type
    pub fn new<T>(value: T, unit_type: WeightUnitType) -> Self
    where
        T: Into<f64>,
    {
        let value_in_f64: f64 = value.into(); // Convert to f64

        if value_in_f64 < 0.0 {
            panic!("Weight cannot be negative.")
        }
        WeightUnit {
            value: value_in_f64,
            unit_type,
        }
    }
}

impl std::fmt::Display for WeightUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.value, self.get_abbreviation())
    }
}
