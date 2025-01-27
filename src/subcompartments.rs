// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

use crate::compartment::Compartment;
use crate::cell::Compartments;

#[derive(Copy, Clone)]
pub struct Subcompartments {
    pub count: Compartments,
}

impl Subcompartments {
    #[inline]
    pub fn can_advance(self, c: Compartment, cs: &Compartments) -> bool {
        let i = c as usize;
        cs[i] < self.count[i]
    }
}
