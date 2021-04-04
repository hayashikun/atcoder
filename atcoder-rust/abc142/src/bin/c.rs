#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };

    let s = aa
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, a)| a)
        .map(|(i, _)| (i + 1).to_string())
        .join(" ");

    println!("{}", s);
}
