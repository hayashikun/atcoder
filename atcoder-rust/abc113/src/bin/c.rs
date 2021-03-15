// TODO
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

fn run(n: usize, m: usize, py: Vec<(usize, usize)>) {
    let ip = py
        .iter()
        .enumerate()
        .sorted_by_key(|&(_, &(_, y))| y)
        .map(|(i, &(p, _))| (i, p))
        .collect_vec();

    let mut city = vec![1; n];
    let mut ans = vec!["".to_string(); m];
    for (i, p) in ip {
        ans[i] = format!("{:>06}{:>06}", p + 1, city[p]);
        city[p] += 1
    }

    for a in ans {
        println!("{}", a);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut py: [(Usize1, usize); m]
    };

    run(n, m, py);
}
