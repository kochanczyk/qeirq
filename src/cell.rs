// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

use crate::compartment::Compartment;
use crate::compartment::NONQ_COMPARTMENTS_COUNT;

pub type Compartments = [u8; NONQ_COMPARTMENTS_COUNT];
pub const QUIESCENT_CELL: Compartments = [0; NONQ_COMPARTMENTS_COUNT];

#[derive(Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub compartments: Compartments,
}

impl Cell {
    #[inline]
    pub fn is_zero(c: Compartment, cs: &Compartments) -> bool {
        let i = c as usize;
        cs[i] == 0
    }
}
