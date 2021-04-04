#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        n: usize
    };

    if n == 1 {
        println!("{}", 1.0);
    } else if n % 2 == 0 {
        println!("{}", 0.5);
    } else {
        println!("{}", (n / 2 + 1) as f64 / n as f64);
    }
}
