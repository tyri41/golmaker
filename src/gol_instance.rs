#[derive(Debug, PartialEq, Clone)]
pub struct GolInstance {
    pub h: usize,
    pub w: usize,
    pub cells: Vec<Vec<i32>>,
}

impl GolInstance {
    #[allow(dead_code)]
    pub fn show(self: &GolInstance) -> String {
        let vbar = format!("O{}O\n", "-".repeat(self.w as usize));
        let mut ret = vbar.clone();
        for line in &self.cells {
            ret.push('|');
            for c in line {
                if *c > 0 {
                    ret.push('#');
                } else {
                    ret.push(' ');
                }
            }
            ret.push_str("|\n");
        }
        ret.push_str(&vbar);
        ret
    }

    #[allow(dead_code)]
    pub fn flatten(self: &GolInstance) -> Vec<i32> {
        self.cells.clone().into_iter().flatten().collect()
    }

    #[allow(dead_code)]
    pub fn update_flat(self: &mut GolInstance, data: Vec<i32>) {
        self.cells = data.chunks(self.w).map(|l| l.to_vec()).collect();
    }
}


#[allow(dead_code)]
pub fn from_flat(w: usize, data: Vec<i32>) -> GolInstance {
    let h = data.len() / w;
    GolInstance { 
        h, w,
        cells: data.chunks(w).map(|l| l.to_vec()).collect()
    }
}