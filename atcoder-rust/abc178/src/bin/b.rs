#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use nalgebra::max;

#[fastout]
fn main() {
    input!{
        a: isize,
        b: isize,
        c: isize,
        d: isize
    };

    println!("{}", max(max(a * c, a * d), max(b * c, b * d)));
}
