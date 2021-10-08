use crate::T9;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Conductor {
    size: String,
    metal: Metal,
}

#[derive(Debug, PartialEq)]
pub enum Metal {
    Copper,
    Aluminum,
}

impl TryFrom<&str> for Metal {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "cu" => Ok(Self::Copper),
            "al" => Ok(Self::Aluminum),
            _ => Err("Metal must be either cu and al"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConduitType {
    PVC,
    Aluminum,
    Steel,
}

impl TryFrom<&str> for ConduitType {
    type Error = &'static str;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "pvc" => Ok(Self::PVC),
            "al" => Ok(Self::Aluminum),
            "steel" => Ok(Self::Steel),
            _ => Err("Conduit type must be either pvc, al, or steel"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Phase {
    Single,
    Three,
}

impl TryFrom<i8> for Phase {
    type Error = &'static str;

    fn try_from(item: i8) -> Result<Self, Self::Error> {
        match item {
            1 => Ok(Self::Single),
            3 => Ok(Self::Three),
            _ => Err("Phase must be either 1 or 3"),
        }
    }
}

// #[derive(Debug, PartialEq)]
// pub enum Unit {
//     Imperial,
//     Metric,
// }

pub fn calc_resistance_required() -> f64 {
    0.0
}

fn modify_resistance_temperature(resistance: f64, to_temperature: i32) -> f64 {
    const FROM_TEMPERATURE: f64 = 75.0; // Table 9 resistance values based of 75 degrees
    const A: f64 = 0.00323; // Temperature coefficient of copper @ 75 degrees. Aluminum 0.00330

    resistance * (1.0 + A * (to_temperature as f64 - FROM_TEMPERATURE))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t9_has_data() {
        assert_eq!(T9.cu_resistance_steel.get("600"), Some(&0.025));
    }

    #[test]
    fn test_calc_volts_dropped_ac() {}

    #[test]
    fn test_calc_resistance_required() {
        assert_eq!(calc_resistance_required(), 0.0)
    }

    #[test]
    fn test_modify_resistance_temperature() {
        assert_eq!(modify_resistance_temperature(2.0, 20), 1.6447)
    }
}
