use super::union_find::QuickFind;
use std::vec::Vec;

pub struct Perc {
    quick_union_structure: QuickFind,
    quick_union_structure_for_is_full: QuickFind,
    grid_size: usize,
    grid: Vec<Vec<bool>>,
    virtual_top_site: usize,
    virtual_bottom_site: usize,
}

impl Perc {
    pub fn new(n: usize) -> Perc {
        Perc {
            quick_union_structure: QuickFind::new(n * n + 2),
            quick_union_structure_for_is_full: QuickFind::new(n * n + 1),
            grid_size: n,
            grid: vec!(vec!(false; n); n),
            virtual_top_site: 0,
            virtual_bottom_site: n * n + 1,
        }
    }

    // opens the site (row, col) if it is not open already
    pub fn open(&mut self, row: usize, col: usize) {
        if !self.is_open(row, col) {
            let field = self.field_index(row, col);
            if row == 1 {
                self.quick_union_structure.union(self.virtual_top_site, field);
                self.quick_union_structure_for_is_full.union(self.virtual_top_site, field);
            } else if row == self.grid_size {
                self.quick_union_structure.union(field, self.virtual_bottom_site);
            }
            for (nrow, ncol) in self.neighbours_of(row, col) {
                if self.is_open(nrow, ncol) {
                    let neighbour = self.field_index(nrow, ncol);
                    self.quick_union_structure.union(neighbour, field);
                    self.quick_union_structure_for_is_full.union(neighbour, field);
                }
            }

            self.grid[row - 1][col - 1] = true;
        }
    }

    // is the site (row, col) open?
    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.grid[row - 1][col - 1]
    }

    fn field_index(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.grid_size + col
    }

    // is the site (row, col) full?
    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        self.is_open(row, col) &&
            self.quick_union_structure_for_is_full.connected(self.virtual_top_site, self.field_index(row, col))
    }

    // returns the number of open sites
    pub fn number_of_open_sites(&mut self) -> usize {
        unimplemented!()
    }

    // does the system percolate?
    pub fn percolates(&mut self) -> bool {
        self.quick_union_structure.connected(self.virtual_top_site, self.virtual_bottom_site)
    }

    fn neighbours_of(&self, i: usize, j: usize) -> std::vec::IntoIter<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if i != 1 {
            result.push((i - 1, j));
        }
        if i != self.grid_size {
            result.push((i + 1, j));
        }
        if j != 1 {
            result.push((i, j - 1));
        }
        if j != self.grid_size {
            result.push((i, j + 1));
        }
        result.into_iter()
    }
}
use std::fmt;

impl fmt::Debug for Perc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for i in 0..self.grid_size {
            for j in 0..self.grid_size {
                if self.grid[i][j] {
                    s.push_str(" ");
                } else {
                    s.push_str("█");
                }
            }
            s.push_str("\n")
        }
        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod test {
    use super::Perc;

    #[test]
    fn basics() {
        let mut p = Perc::new(5);
        assert_eq!(p.is_open(1, 2), false);
        p.open(1, 2);

        assert!(p.is_open(1, 2));
        assert_eq!(p.is_full(2, 2), false);
        p.open(2, 2);
        assert!(p.is_full(2, 2));
        p.open(3, 2);
        p.open(4, 2);

        assert_eq!(p.percolates(), false);
        p.open(5, 2);
        println!("{:#?}", p);
        assert!(p.percolates());
    }

    fn corner_case() {
        let mut p = Perc::new(1);
        assert_eq!(p.percolates(), false);
        p.open(1, 1);
        assert!(p.percolates());
    }
}
