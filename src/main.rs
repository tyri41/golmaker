mod gol_instance;
mod generators;
mod solver;

use crate::{generators::gen_random, solver::iterate};

fn main() {
    let mut instance = gen_random(7, 7);
    iterate(&mut instance, 10, true);
}
