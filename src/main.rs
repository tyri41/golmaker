mod gol_instance;
mod generators;
mod solver;
mod solver_gpu;
mod gpu_utils;
mod consts;


use crate::{generators::gen_random, solver::iterate, solver_gpu::iterate_gpu};

fn main() {
    let w = 500;
    let h = 500;
    let t = 30;
    let instance = gen_random(w, h);
    // println!("Base:");
    // println!("{}", instance.show());

    let mut inst_cpu = instance.clone();
    let mut inst_gpu = instance.clone();
    assert_eq!(inst_cpu, inst_gpu);

    println!("CPU based:");
    iterate(&mut inst_cpu, t, false);
    // println!("{}", inst_cpu.show());

    println!("GPU based:");
    // iterate_gpu_debug(&mut inst_gpu, t, true);
    iterate_gpu(&mut inst_gpu, t);
    // println!("{}", inst_gpu.show());

    assert_eq!(inst_cpu, inst_gpu);
    println!("gol match!");
}
