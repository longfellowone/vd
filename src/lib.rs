#![allow(dead_code, unused_variables)]
#![feature(once_cell)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::lazy::SyncLazy;

mod ac;
mod common;
mod dc;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t9_has_data() {
        assert_eq!(T9.cu_resistance_steel.get("600"), Some(&0.025));
    }
}
