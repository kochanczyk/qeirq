// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

#[derive(Debug, Clone)]
pub struct Rates {
    pub c_rate: f64,
    pub e_incr: f64,
    pub i_incr: f64,
    pub r_incr: f64,
}

impl Rates {
    pub fn new(c_rate: f64, e_incr: f64, i_incr: f64, r_incr: f64) -> Self {
        Rates {
            c_rate,
            e_incr,
            i_incr,
            r_incr,
        }
    }
}
