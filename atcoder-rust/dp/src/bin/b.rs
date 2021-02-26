use std::cmp::min;

use num::abs;
use proconio::{fastout, input};

fn run(n: usize, k: usize, hh: Vec<isize>) -> isize {
    let mut costs = Vec::new();
    costs.push(0);

    for i in 1..n {
        costs.push(
            (0..min(k, i))
                .into_iter()
                .map(|j| abs(hh[i] - hh[i - 1 - j]) + costs[i - 1 - j])
                .min()
                .unwrap(),
        );
    }

    return *costs.last().unwrap();
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        hh: [isize; n]
    }

    println!("{}", run(n, k, hh));
}
