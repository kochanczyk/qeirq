// -------------------------------------------------------------------------------------------------
// QEIRQ -- a simulator of a monolayer of cells that hold a simple internal state and communicate
// when in contact.
//
// This code features the research article:
//
//               "Information transmission in a cell monolayer: a numerical study"
//
//                                 by Nałęcz-Jawecki et al.
//                             PLOS Computational Biology, 2025
//
// For more info, see file README.md.
//
// Copyright (2024-2025) https://github.com/kochanczyk/qeirq/CONTRIBUTORS.md.
// Licensed under the BSD 3-clause license (https://opensource.org/license/bsd-3-clause).
// -------------------------------------------------------------------------------------------------

mod cell;
mod commands;
mod compartment;
mod event;
mod lattice;
mod output;
mod parameters;
mod protocol;
mod randomness;
mod rates;
mod simulation;
mod subcompartments;
mod units;

use lattice::Lattice;
use output::Output;
use parameters::Parameters;
use protocol::Protocol;
use randomness::initialize_generator;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// A JSON-formatted file with both the kinetic and the structural model parameters
    parameters_file: String,

    /// A file containing commands that may be used as external event triggers
    protocol_file: String,

    /// Reactor width
    #[clap(long = "width", short = 'W', action)]
    width: usize,

    /// Reactor height
    #[clap(long = "height", short = 'H', action)]
    height: usize,

    /// Write out the full reactor state (file per every output time point)
    #[clap(long = "states-out", short = 'S', action)]
    states: bool,

    /// Write out the number of E+I cells in every reactor column (single file)
    #[clap(long = "activity-out", short = 'A', action)]
    activity: bool,

    /// Generate a PNG image of the reactor (file per every output time point)
    #[clap(long = "images-out", short = 'I', action)]
    images: bool,

    /// Random seed
    #[clap(long = "seed", short = 's', default_value_t = 123)]
    seed: u128,
}

fn execute_protocol() {
    let args = Args::parse();

    let protocol = Protocol::from_text_file(&args.protocol_file);
    let parameters = Parameters::from_json_file(&args.parameters_file);

    let mut generator = initialize_generator(args.seed, false);
    let mut lattice = Lattice::new(args.width, args.height, &mut generator);

    let output = Output {
        all_states: args.states,
        active_states: args.activity,
        images: args.images,
    };

    protocol.execute(&mut lattice, &parameters, &mut generator, output);
}

fn main() {
    execute_protocol();
}
