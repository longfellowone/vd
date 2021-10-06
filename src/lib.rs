#![allow(dead_code, unused_variables)]
#![feature(once_cell)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::lazy::SyncLazy;

// Use match on metal/phase/conduit/unit

// vd::single_phase ::three_phase -> crate::vd
// vd::dc

// References
// http://profwagner.com/4520/4520-PPT10.pdf
// https://pdhonline.com/courses/e426/e426content.pdf
// https://github.com/MasonMcGarrity/Voltage_Drop_Calculator/blob/master/main.py#L282
// https://github.com/Zclarkwilliams/Voltage-Drop-Excel-Calculator/blob/master/Code/Main_Rev2.vba

pub static T9: SyncLazy<Table9> = SyncLazy::new(|| {
    serde_cbor::from_slice(include_bytes!(concat!(env!("OUT_DIR"), "\\t9.cbor"))).unwrap()
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Table9 {
    pub reactance_pvc_al: BTreeMap<String, f64>,
    pub reactance_steel: BTreeMap<String, f64>,
    pub cu_resistance_pvc: BTreeMap<String, f64>,
    pub cu_resistance_al: BTreeMap<String, f64>,
    pub cu_resistance_steel: BTreeMap<String, f64>,
    pub al_resistance_pvc: BTreeMap<String, f64>,
    pub al_resistance_al: BTreeMap<String, f64>,
    pub al_resistance_steel: BTreeMap<String, f64>,
}

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

#[derive(Debug, PartialEq)]
pub struct MinConductorSizeAC {
    pub phase: Phase,
    pub metal: Metal,
    pub conduit_type: ConduitType,
    pub voltage: i32,
    pub current: i32,
    pub max_vd_percentage: f64,
    pub parallel_sets: i32,
    pub length: i32,
    pub termination_temperature: i32,
    pub power_factor: f64,
}

// Delete?
// Remove args that have no defaults (phase, conduit, metal,sets)
pub struct InputArgsAC<'a> {
    pub length: i32,
    pub phase: i8,
    pub metal: &'a str,
    pub conduit_type: &'a str,
    pub voltage: i32,
    pub current: i32,
    pub max_vd_percentage: f64,
    pub parallel_sets: i32,
}

// Change to input voltage, current, and vd percentage mandatory, use builder for remaining
// Make generic accept ENUM of AC or DC args, use impl on input args?
impl MinConductorSizeAC {
    pub fn new(args: InputArgsAC) -> Self {
        MinConductorSizeAC {
            phase: Phase::try_from(args.phase).unwrap(),
            metal: Metal::try_from(args.metal).unwrap(),
            conduit_type: ConduitType::try_from(args.conduit_type).unwrap(),
            voltage: args.voltage,
            current: args.current,
            max_vd_percentage: args.max_vd_percentage,
            parallel_sets: args.parallel_sets,
            length: args.length,
            power_factor: 0.9,
            termination_temperature: 75,
        }
    }

    pub fn calculate(self: Self) -> Conductor {
        Conductor {
            size: "250".to_string(),
            metal: Metal::Copper,
        }
    }
}

// Single phase max distance
// V*(VDP/100)/(2*((I*(Re/1000)*PF)+(I*(X/1000)*SIN(ACOS(PF)))))
pub struct MaxDistance {}

// Maybe use approximate than double check?
// https://pdhonline.com/courses/e426/e426content.pdf see page49/57
// Or just use vd function recursively until match
// https://stackoverflow.com/questions/49599833/how-to-find-next-smaller-key-in-btreemap-btreeset
// https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
pub fn min_conductor_size(length_ft: i32, voltage: i32, current: i32) -> Conductor {
    Conductor {
        size: "500mcm".to_string(),
        metal: Metal::Copper,
    }
}

// Estimated, does not account for error
// https://pdhonline.com/courses/e426/e426content.pdf see page 36/57
pub fn volts_dropped_ac(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let power_factor: f64 = 0.9; // PF of 0.85 most common
    let theta = f64::acos(power_factor); // Power factor angle
    let multiplier = f64::sqrt(3.0); // (3.0_f64).sqrt() for line-to-line voltage drop, Multiply for 2 instead for line-to-neutral
    let resistance = 0.054; // R
    let reactance = 0.052; // X
    let impedance = (resistance * power_factor) + (reactance * theta.sin()); // Effective Z, Addition based on assumed lagging PF

    multiplier * current as f64 * impedance * length_ft as f64 / 1000.0
    // VD % = result / voltage * 100
}

// Requires use of different table in NEC for resistance values?
pub fn volts_dropped_dc(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let resistance = 0.0847; // R

    2.0 * current as f64 * resistance * length_ft as f64 / 1000.0
}

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
    fn t9_has_data() {
        assert_eq!(T9.cu_resistance_steel.get("600"), Some(&0.025));
    }

    #[test]
    fn test_min_conductor_size_calculate() {
        let min_conductor_size = MinConductorSizeAC::new(InputArgsAC {
            length: 200,
            phase: 3,
            metal: "cu",
            conduit_type: "pvc",
            voltage: 208,
            current: 160,
            max_vd_percentage: 0.03,
            parallel_sets: 1,
        });

        assert_eq!(
            min_conductor_size.calculate(),
            Conductor {
                size: "250".to_string(),
                metal: Metal::Copper
            }
        )
    }

    #[test]
    fn test_calc_volts_dropped_ac() {
        // For 250mcm Copper/Steel
        assert_eq!(volts_dropped_ac(400, 208, 160), 7.899955731920339)
    }

    #[test]
    fn test_volts_dropped_dc() {}

    #[test]
    fn test_calc_resistance_required() {
        assert_eq!(calc_resistance_required(), 0.0)
    }

    #[test]
    fn test_modify_resistance_temperature() {
        assert_eq!(modify_resistance_temperature(1.2, 20), 0.98682)
    }
    #[test]
    fn test_min_conductor_size() {
        assert_eq!(
            min_conductor_size(155, 208, 160),
            Conductor {
                size: "500mcm".to_string(),
                metal: Metal::Copper
            }
        )
    }
}
