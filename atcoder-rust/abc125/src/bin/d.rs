#![allow(unused_imports)]

use itertools::Itertools;
use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };

    let n_aa = aa
        .clone()
        .into_iter()
        .filter(|&a| a < 0)
        .sorted()
        .rev()
        .collect_vec();
    let p_aa = aa
        .clone()
        .into_iter()
        .filter(|&a| a >= 0)
        .sorted()
        .collect_vec();

    let sum = if n_aa.len() % 2 == 0 {
        -n_aa.into_iter().sum::<isize>() + p_aa.into_iter().sum::<isize>()
    } else if p_aa.len() == 0 {
        -n_aa[1..].into_iter().sum::<isize>() + n_aa[0]
    } else {
        if -n_aa[0] > p_aa[0] {
            -n_aa.into_iter().sum::<isize>() + p_aa[1..].into_iter().sum::<isize>() - p_aa[0]
        } else {
            -n_aa[1..].into_iter().sum::<isize>() + n_aa[0] + p_aa.into_iter().sum::<isize>()
        }
    };

    println!("{}", sum);
}
