#![allow(unused_imports)]

use permutohedron::factorial;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    let mut m = 0.0;

    for i in 1..=n {
        m += 1.0 / (i as f64)
    }
    m *= n as f64;
    m -= 1.0;

    println!("{}", m);
}
