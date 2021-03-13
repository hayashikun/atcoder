#![allow(unused_imports)]

use num_integer::{div_ceil, div_floor};
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        w: usize,
    };

    let w = w * 1000;
    let ans1 = div_floor(w, a);
    let ans2 = div_ceil(w, b);

    if ans1 < ans2 {
        println!("UNSATISFIABLE")
    } else {
        println!("{} {}", ans2, ans1);
    }
}
