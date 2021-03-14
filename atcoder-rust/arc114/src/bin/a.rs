#![allow(unused_imports)]

use std::cmp::min;
use std::collections::BTreeSet;

use itertools::Itertools;
use num::integer::gcd;
use proconio::{fastout, input, marker::*};

fn check(p: usize, xs: &Vec<usize>) -> bool {
    let mut ok = true;
    for &x in xs.iter() {
        if gcd(p, x) == 1 {
            ok = false;
            break;
        }
    }
    return ok;
}

fn rec(n: usize, ns: Vec<usize>, xs: &Vec<usize>) -> usize {
    if ns.is_empty() {
        return if check(n, xs) { n } else { usize::max_value() };
    }
    return min(
        rec(n, ns[1..].to_vec(), xs),
        rec(n * ns[0], ns[1..].to_vec(), xs),
    );
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [usize; n],
    };

    let ps = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let ans = rec(1, ps, &xs);
    println!("{}", ans);
}
