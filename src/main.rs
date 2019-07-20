#![feature(test)]

pub mod perc;
pub mod union_find;
pub mod perc_stats;

use std::env;
use self::perc_stats::PercStats;

fn main() {
    let mut args = env::args();
    args.next();
    let n : usize = args.next().unwrap().parse().unwrap();
    let t : usize = args.next().unwrap().parse().unwrap();
    let p = PercStats::new(n,t);
    println!("mean: {}", p.mean());
    println!("std-dev {}", p.stddev());
    println!("confidence low {}", p.confidence_low());
    println!("confidence high {}", p.confidence_high());
}

