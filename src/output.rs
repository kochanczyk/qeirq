// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

pub const STATE_FILE_NAME_PREFIX: &str = "state_";
pub const LATTICE_IMAGE_FILE_NAME_PREFIX: &str = "lattice_";
pub const ACTIVITY_COLUMN_SUM_FILE_NAME: &str = "activity_column_sum.csv";

#[derive(Clone, Copy)]
pub struct Output {
    pub all_states: bool,    // one text file per output time point
    pub active_states: bool, // one text file per whole simulation
    pub images: bool,        // one image file per output time point
}

impl Output {
    #[inline]
    pub fn any_of_lattice(&self) -> bool { self.all_states || self.images }
}
