use crate::T9;
// use serde::{Deserialize, Serialize};
// vd::single_phase ::three_phase -> crate::vd

// Estimated volts dropped, does not account for error
// https://pdhonline.com/courses/e426/e426content.pdf see page 36/57
pub fn evd(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let power_factor: f64 = 0.9; // PF of 0.85 most common
    let theta = f64::acos(power_factor); // Power factor angle
    let multiplier = f64::sqrt(3.0); // (3.0_f64).sqrt() for line-to-line voltage drop, Multiply for 2 instead for line-to-neutral
    let resistance = T9.cu_resistance_steel.get("12").unwrap(); // R
    let reactance = T9.reactance_steel.get("12").unwrap(); // X
    let impedance = (resistance * power_factor) + (reactance * theta.sin()); // Effective Z, Addition based on assumed lagging PF

    let result = multiplier * current as f64 * impedance * length_ft as f64 / 1000.0;

    println!("volts dropped: {:?}", result);

    println!("#12 R: {:?}", T9.cu_resistance_steel.get("12"));
    println!("#12 X: {:?}", T9.reactance_steel.get("12"));

    result
    // VD % = result / voltage * 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t9_has_data() {
        assert_eq!(T9.cu_resistance_steel.get("600"), Some(&0.025));
    }

    #[test]
    fn test_calc_volts_dropped() {
        // For 250mcm Copper/Steel
        assert_eq!(evd(144, 120, 12), 5.4760844064104415)
    }

    // #[test]
    // fn test_min_conductor_size_calculate() {
    //     let min_conductor_size = MinConductocmdrSizeAC::new(MinConductorSizeAcParams {
    //         length: 200,
    //         phase: 3,
    //         metal: "cu",
    //         conduit_type: "pvc",
    //         voltage: 208,
    //         current: 160,
    //         max_vd_percentage: 0.03,
    //         parallel_sets: 1,
    //     });
    //
    //     assert_eq!(
    //         min_conductor_size.calculate(),
    //         Conductor {
    //             size: "250".to_string(),
    //             metal: Metal::Copper
    //         }
    //     )
    // }

    // #[test]
    // fn test_min_conductor_size() {
    //     assert_eq!(
    //         min_conductor_size(155, 208, 160),
    //         Conductor {
    //             size: "500mcm".to_string(),
    //             metal: Metal::Copper
    //         }
    //     )
    // }
}
