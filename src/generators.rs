use crate::gol_instance::GolInstance;
use rand::Rng;

#[allow(dead_code)]
pub fn gen_small() -> GolInstance {
    GolInstance {
        h: 3,
        w: 3,
        cells: vec![vec![0; 3], vec![1; 3], vec![0; 3]]
    }
}

#[allow(dead_code)]
pub fn gen_random(w: usize, h: usize) -> GolInstance {
    let mut rng = rand::thread_rng();
    let mut cells = Vec::with_capacity(h);
    for _i in 0..h {
        let mut row = Vec::with_capacity(w);
        for  _j in 0..w {
            row.push(rng.gen_range(0..2) as i32);
        }
        cells.push(row);
    }

    GolInstance { h, w, cells }
}