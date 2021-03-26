#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        aa: [usize; n]
    };

    let mut md = aa[0] + k - aa[n - 1];

    for i in 1..n {
        let d = aa[i] - aa[i - 1];
        if md < d {
            md = d;
        }
    }

    println!("{}", k - md)
}
