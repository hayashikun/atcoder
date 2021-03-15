#![allow(unused_imports)]

use std::collections::BTreeSet;

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};

pub fn mod_ex(b: u128, e: u128, m: u128) -> u128 {
    if e == 0 {
        return 1;
    }
    if b == 0 || b == 1 {
        return b;
    }
    let mut c = mod_ex(b, e / 2, m);
    c = (c * c) % m;
    if e % 2 == 1 {
        c = (c * b) % m;
    }
    return c;
}

fn run(n: usize, fs: Vec<usize>) {
    let mut uf = UnionFind::new(n);
    for (i, f) in fs.into_iter().enumerate() {
        uf.union(i, f);
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }

    println!("{}", mod_ex(2, set.len() as u128, 998244353) - 1)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        fs: [Usize1; n]
    };

    run(n, fs);
}
