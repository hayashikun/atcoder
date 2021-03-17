#![allow(unused_imports)]

use nalgebra::min;
use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: isize,
        x: Isize1,
        y: Isize1,
    };

    let mut counts = vec![0; (n - 1) as usize];

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let mut d = j - i;
            if x <= j && i <= y {
                d = min(d, abs(i - x) + 1 + abs(j - y))
            }
            counts[(d - 1) as usize] += 1;
        }
    }

    for c in counts {
        println!("{}", c);
    }
}
