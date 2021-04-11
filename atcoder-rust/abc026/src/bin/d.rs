#![allow(unused_imports)]

use std::f64::consts::PI;

use libm::sin;
use proconio::{fastout, input, marker::*};
use num::abs;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };

    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let mut d = 1.0;
    let mut lt = 0.0;
    let mut rt = 200.0;
    let mut t = 0.0;
    while abs(d) > 0.00_000_1 {
        t = (lt + rt) / 2.0;
        d = a * t + b * sin(c * t * PI) - 100.0;
        if d > 0.0 {
            rt = t;
        } else {
            lt = t;
        }
    }
    println!("{}", t);
}
