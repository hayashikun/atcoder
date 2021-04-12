#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::min;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut pp: [usize; a],
        mut qq: [usize; b],
        mut rr: [usize; c],
    };

    pp.sort();
    pp.reverse();
    qq.sort();
    qq.reverse();

    let mut eats: Vec<usize> = Vec::new();
    eats.extend(pp[0..x].iter());
    eats.extend(qq[0..y].iter());
    eats.sort();

    let mut sum = eats.iter().sum::<usize>();

    rr.sort();
    rr.reverse();
    for i in 0..(min(c, x + y)) {
        let ns = sum - eats[i] + rr[i];
        if ns > sum {
            sum = ns;
        }
    }

    println!("{}", sum);
}
