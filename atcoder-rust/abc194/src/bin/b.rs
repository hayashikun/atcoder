#![allow(unused_imports)]

use std::cmp::{max, min};

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

fn run(_n: usize, ab: Vec<(usize, usize)>) -> usize {
    let ab_am = ab
        .clone()
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, (a, _))| a)
        .collect_vec();
    let ab_bm = ab
        .clone()
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, (_, b))| b)
        .collect_vec();

    if ab_am[0].0 != ab_bm[0].0 {
        return max((ab_am[0].1).0, (ab_bm[0].1).1);
    }

    let t1 = (ab_am[0].1).0 + (ab_bm[0].1).1;
    let t2 = min(
        max((ab_am[1].1).0, (ab_bm[0].1).1),
        max((ab_am[0].1).0, (ab_bm[1].1).1),
    );

    return min(t1, t2);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n]
    };
    println!("{}", run(n, ab));
}
