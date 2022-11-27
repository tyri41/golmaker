mod gol_instance;
mod generators;
mod solver;
mod solver_gpu;
mod gpu_utils;
mod consts;

use solver_gpu::run_gpu;

// use crate::{generators::gen_random, solver::iterate};

fn main() {
    // let mut instance = gen_random(7, 7);
    // iterate(&mut instance, 10, true);

    run_gpu();
}
