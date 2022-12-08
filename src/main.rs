mod gol_instance;
mod generators;
mod solver;
mod solver_gpu;
mod gpu_utils;
mod consts;

use crate::{generators::gen_random, solver::iterate, solver_gpu::iterate_gpu};
use clap::Parser;
use strum_macros::EnumString;
use std::str::FromStr;

#[derive(Debug, Clone, EnumString)]
enum Approach {
    CPU,
    GPU,
}

#[derive(Parser, Debug)]
struct Cli {
    /// Width
    w: usize,
    /// Height
    h: usize,
    /// time
    t: usize,
    /// type of algorithm
    mode: String
}

fn main() {
    let args = Cli::parse();
    let mode = Approach::from_str(&args.mode).unwrap();

    let mut instance = gen_random(args.w, args.h);
    match mode {
        Approach::CPU => {
            iterate(&mut instance, args.t, false);
        },
        Approach::GPU => {
            iterate_gpu(&mut instance, args.t);
        },
    }
}
