use std::cmp::min;

use num::abs;
use proconio::{fastout, input};

fn run(n: usize, hh: Vec<isize>) -> isize {
    let mut costs = Vec::new();
    costs.push(0);
    costs.push(abs(hh[1] - hh[0]));

    for i in 2..n {
        costs.push(min(
            abs(hh[i] - hh[i - 1]) + costs[i - 1],
            abs(hh[i] - hh[i - 2]) + costs[i - 2],
        ));
    }

    return *costs.last().unwrap();
}

#[fastout]
fn main() {
    input! {
        n: usize,
        hh: [isize; n]
    }

    println!("{}", run(n, hh));
}
