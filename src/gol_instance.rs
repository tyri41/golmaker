#[derive(Debug)]
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
}