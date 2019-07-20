extern crate rand;
extern crate test;
extern crate rayon;

use test::stats::Stats;
use rand::Rng;
use super::perc::Perc;
use rayon::prelude::*;

pub struct PercStats {
    rs: std::vec::Vec<f64>,
    trials: usize,
}

impl PercStats {
    // perform independent trials on an n-by-n grid
    pub fn new(grid_size: usize, trials: usize) -> PercStats {
        let rs = (0..trials).into_par_iter().map(|_| {
            let mut rng = rand::thread_rng();
            let mut p = Perc::new(grid_size);
            let mut count: usize = 0;
            while !p.percolates() {
                let r = rng.gen_range(0, grid_size * grid_size);
                let row = r / grid_size + 1;
                let col = r % grid_size + 1;
                if !p.is_open(row, col) {
                    p.open(row, col);
                    count += 1;
                }
            }
            (count / grid_size) as f64
        }).collect();

        PercStats { rs, trials }
    }

    // sample mean of percolation threshold
    pub fn mean(&self) -> f64 {
        self.rs.mean()
    }

    // sample standard deviation of percolation threshold
    pub fn stddev(&self) -> f64 {
        self.rs.std_dev()
    }

    // low endpoint of 95% confidence interval
    // mean - 1.96 * sd / sqrt t
    pub fn confidence_low(&self) -> f64 {
        self.mean() - 1.96 * self.stddev() / (self.trials as f64).sqrt()
    }

    // high endpoint of 95% confidence interval
    pub fn confidence_high(&self) -> f64 {
        self.mean() + 1.96 * self.stddev() / (self.trials as f64).sqrt()
    }
}