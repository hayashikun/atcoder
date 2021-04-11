#![allow(unused_imports)]

use std::collections::BTreeSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        cc: [Usize1; n],
        ab: [(Usize1, Usize1); n - 1]
    };

    let mut goods = Vec::new();

    let mut tree = vec![Vec::new(); n];
    for (e1, e2) in ab.into_iter() {
        tree[e1].push(e2);
        tree[e2].push(e1);
    }

    let mut counts = vec![0; *cc.iter().max().unwrap() + 1];

    rec(0, 0, &cc, &tree, &mut counts, &mut goods);

    goods.sort();
    for a in goods {
        println!("{}", a + 1);
    }
}

fn rec(
    a: usize,
    fa: usize,
    cc: &Vec<usize>,
    tree: &Vec<Vec<usize>>,
    counts: &mut Vec<usize>,
    goods: &mut Vec<usize>,
) {
    let c = cc[a];
    if counts[c] == 0 {
        goods.push(a);
    }
    counts[c] += 1;
    for &b in tree[a].iter() {
        if b != fa {
            rec(b, a, cc, tree, counts, goods);
        }
    }
    counts[c] -= 1;
}
