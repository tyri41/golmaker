mod gol_instance;
mod generators;

use crate::generators::{gen_small, gen_random};

fn main() {
    println!("Hello, world!");
    let instance = gen_small();
    println!("{:?}", instance);
    println!("{}", instance.show());
    let instance = gen_random(5, 7);
    println!("{}", instance.show());
}
