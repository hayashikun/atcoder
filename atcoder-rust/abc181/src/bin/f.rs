// TODO
#![allow(unused_imports)]

use std::cmp::min;

use itertools::Itertools;
use libm::sqrt;
use num::traits::Pow;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    println!("{}", run(n, xy));
}

fn run(n: usize, xy: Vec<(isize, isize)>) -> f64 {
    // length, xy from, xy to
    let mut edges: Vec<(f64, usize, usize)> = Vec::new();

    for i in 0..n {
        let xi = xy[i].0 as f64;
        let yi = xy[i].1 as f64;
        edges.push((100.0 - yi, i, n));
        edges.push((yi + 100.0, i, n + 1));
        for j in (i + 1)..n {
            let xj = xy[j].0 as f64;
            let yj = xy[j].1 as f64;
            let r = sqrt((xi - xj).pow(2) + (yi - yj).pow(2));
            edges.push((r, i, j));
        }
    }
    edges.sort_by(|&a, &b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(n + 2);

    for e in edges {
        uf.union(e.1, e.2);

        if uf.equiv(n, n + 1) {
            return e.0 / 2.0;
        }
    }

    return 0.0;
}
