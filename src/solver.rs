use crate::gol_instance::GolInstance;

// replace gol.cells with next iteration of GOL (with loop boundaries)
pub fn step(gol: &mut GolInstance) {
    let difs: [(i32, i32); 8] = [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

    let mut cells = Vec::with_capacity(gol.h);
    // loop over all cells
    for i in 0..gol.h {
        let mut row = Vec::with_capacity(gol.w);
        for  j in 0..gol.w {
            // calc new value for some cell
            let mut neigs = 0;
            for (dj, di) in difs {
                let ii = (i as i32 + di).rem_euclid(gol.h as i32) as usize;
                let jj = (j as i32 + dj).rem_euclid(gol.w as i32) as usize;
                neigs += gol.cells[ii][jj];
            }
            let is_alive = gol.cells[i][j] > 0;
            if is_alive {
                if neigs >= 2 && neigs <= 3 {
                    row.push(1);
                } else {
                    row.push(0);
                }
            } else {
                if neigs == 3 {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
        }
        cells.push(row);
    }

    gol.cells = cells;
}

#[allow(dead_code)]
pub fn iterate(gol: &mut GolInstance, t: usize, prt: bool) {
    for _i in 0..t {
        step(gol);
        if prt {
            println!("{}", gol.show());
        }
    }
}