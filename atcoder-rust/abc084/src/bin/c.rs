#![allow(unused_imports)]

use num_integer::div_ceil;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1]
    };

    for i in 0..(n - 1) {
        let mut t = 0;
        for j in i..(n - 1) {
            t = if t <= csf[j].1 {
                csf[j].1
            } else {
                div_ceil(t, csf[j].2) * csf[j].2
            };
            t += csf[j].0;
        }
        println!("{}", t);
    }
    println!("{}", 0);
}
