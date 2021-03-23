#![allow(unused_imports)]

use nalgebra::min;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        aa: [usize; m]
    };

    let mut left = 0;
    let mut right = 0;

    for a in aa {
        if a < x {
            left += 1;
        } else {
            right += 1;
        }
    }
    println!("{}", min(left, right));
}
