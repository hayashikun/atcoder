#![allow(unused_imports)]

use libm::log10;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    if n < 1000 {
        println!("0");
        return;
    }

    let d = (log10(n as f64) / 3.0) as u128;

    let mut sum = 0;

    let mut f = 1_000;
    for i in 1..d {
        sum += (f * 1000 - f) * i;
        f *= 1_000;
    }
    sum += (n - f + 1) * d;

    println!("{}", sum)
}
