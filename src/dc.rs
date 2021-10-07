// Requires use of different table in NEC for resistance values?
pub fn volts_dropped_dc(length_ft: i32, voltage: i32, current: i32) -> f64 {
    let resistance = 0.0847; // R

    2.0 * current as f64 * resistance * length_ft as f64 / 1000.0
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_calc_volts_dropped() {}
}
