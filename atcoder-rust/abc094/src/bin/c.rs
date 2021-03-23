#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        mut xx: [usize; n]
    };

    let mut sxx = xx.clone();
    sxx.sort();
    let m1 = sxx[n / 2];
    let m2 = sxx[n / 2 - 1];

    for x in xx {
        if x > m2 {
            println!("{}", m2);
        } else {
            println!("{}", m1);
        }
    }
}
