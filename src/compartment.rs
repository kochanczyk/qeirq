// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

pub enum Compartment {
    // The "quiescent" compartment (Q) is implicit.
    E, // ~ "exposed"
    I, // ~ "infectious"
    R, // ~ "resistant"
}
pub const NONQ_COMPARTMENTS_COUNT: usize = 3;
