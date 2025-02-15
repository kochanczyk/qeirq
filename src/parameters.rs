// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Parameters {
    pub e_subcompartments_count: u8,
    pub i_subcompartments_count: u8,
    pub r_subcompartments_count: u8,
    pub c_rate: f64,
    pub e_forward_rate: f64,
    pub i_forward_rate: f64,
    pub r_forward_rate: f64,
}

impl Parameters {
    pub fn from_json_file(params_filename: &String) -> Self {
        let contents = fs::read_to_string(params_filename).expect("☠ 🕮 JSON");
        from_str(&contents).unwrap()
    }
}
