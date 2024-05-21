// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under GPLv3 (https://www.gnu.org/licenses/gpl-3.0.html).

pub enum Compartment {
    // The "quiescent" compartment (Q) is implicit.
    E, // ~ "exposed"
    I, // ~ "infectious"
    R, // ~ "resistant"
}
pub const NONQ_COMPARTMENTS_COUNT: usize = 3;
