#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    };

    println!(
        "{} {}",
        (w * h) as f64 / 2.0,
        if w == x * 2 && h == y * 2 { 1 } else { 0 }
    );
}
