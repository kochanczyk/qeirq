// QEIRQ, simulator of a monolayer of cells that hold a simple internal state and communicate when in contact
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).

use std::fs::File;
use crate::compartment::Compartment;
use crate::lattice::Lattice;
use crate::output::Output;
use crate::parameters::Parameters;
use crate::simulation::Simulation;

use rand::rngs::StdRng;
use threadpool::ThreadPool;

pub fn initialize_front(lattice: &mut Lattice, column: usize) {
    for y in 0..lattice.height {
        {
            // barrier
            let cell = &mut lattice.cells[column * lattice.height + y];
            cell.alive = false;
        }{
            // active ~ "infectious"
            debug_assert!(column + 1 < lattice.width);
            let cell = &mut lattice.cells[(column + 1) * lattice.height + y];
            if cell.alive {
                cell.compartments[Compartment::E as usize] = 0;
                cell.compartments[Compartment::I as usize] = 1;
                cell.compartments[Compartment::R as usize] = 0;
            }
        }
    }
}

pub fn set_top_edge_nonperiodic(lattice: &mut Lattice) {
    if Lattice::IMAGE_RECTANGULAR {
        for cell_i in 0..lattice.capacity {
            let (mut i, j) = (cell_i % lattice.height, cell_i / lattice.height);
            i = (i + j/2) % lattice.height;
            if 2*i + (j%2) <= 1 {
                let cell = &mut lattice.cells[cell_i];
                cell.alive = false;
                cell.compartments[Compartment::E as usize] = 0;
                cell.compartments[Compartment::I as usize] = 0;
                cell.compartments[Compartment::R as usize] = 0;
            }
        }
    } else {
        for x in 0..lattice.width {
            let cell = &mut lattice.cells[x*lattice.height];
            cell.alive = false;
            cell.compartments[Compartment::E as usize] = 0;
            cell.compartments[Compartment::I as usize] = 0;
            cell.compartments[Compartment::R as usize] = 0;
        }
    }
}

pub fn run_simulation(
    lattice: &mut Lattice,
    parameters: &Parameters,
    rng: &mut StdRng,
    tspan: (f64, f64),
    maybe_output: &Option<Output>,
    maybe_activity_horizontal_csv: &Option<File>,
    files_out_interval: f64,
    initial_frame_in_output_files: bool,
    output_workers: &ThreadPool,
) {
    Simulation::new(lattice).run(
        parameters,
        rng,
        tspan,
        maybe_output,
        maybe_activity_horizontal_csv,
        files_out_interval,
        initial_frame_in_output_files,
        output_workers
    );
}
