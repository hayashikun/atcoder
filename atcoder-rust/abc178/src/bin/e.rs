#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::max;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    };

    let mut fx = xy.iter().map(|(x, y)| x + y).collect_vec();
    let mut fy = xy.iter().map(|(x, y)| x - y).collect_vec();

    fx.sort();
    fy.sort();

    let ans = max(
        fx.last().unwrap() - fx.first().unwrap(),
        fy.last().unwrap() - fy.first().unwrap(),
    );
    println!("{}", ans);
}
