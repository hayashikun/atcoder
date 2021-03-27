#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use std::f64::consts::PI;
use libm::{cos, sin};

#[fastout]
fn main() {
    input!{
        n: usize,
        x0: usize,
        y0: usize,
        xh: usize,
        yh: usize,
    };

    let x0 = x0 as f64;
    let y0 = y0 as f64;
    let xh = xh as f64;
    let yh = yh as f64;

    let xc = (x0 + xh) / 2.0;
    let yc = (y0 + yh) / 2.0;

    let a = 2.0 * PI / n as f64;
    let sin_a = sin(a);
    let cos_a = cos(a);

    let x1 = xc + cos_a * (x0 - xc) - sin_a * (y0 - yc);
    let y1 = yc + sin_a * (x0 - xc) + cos_a * (y0 - yc);

    println!("{} {}", x1, y1);
}
