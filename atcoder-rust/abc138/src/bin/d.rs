#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, usize); q]
    };

    let mut neighbors = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        neighbors[ab[i].0].push(ab[i].1);
        neighbors[ab[i].1].push(ab[i].0);
    }

    let mut children = vec![Vec::new(); n];
    rec1(0, 0, &neighbors, &mut children);

    let mut count = vec![0; n];
    for (p, x) in px.into_iter() {
        count[p] += x;
    }

    let mut sum = vec![0; n];
    rec2(0, &children, &count, 0, &mut sum);
    println!("{}", sum.into_iter().map(|n| n.to_string()).join(" "));
}

fn rec1(i: usize, f: usize, neighbors: &Vec<Vec<usize>>, children: &mut Vec<Vec<usize>>) {
    for &j in neighbors[i].iter() {
        if j == f {
            continue
        }
        children[i].push(j);
        rec1(j, i, neighbors, children);
    }
}

fn rec2(i: usize, children: &Vec<Vec<usize>>, count: &Vec<usize>, current: usize, sum: &mut Vec<usize>) {
    sum[i] = current + count[i];
    for &j in children[i].iter() {
        rec2(j, children, count, sum[i], sum);
    }
}
