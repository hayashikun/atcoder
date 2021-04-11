#![allow(unused_imports)]

use std::iter::Enumerate;

use nalgebra::max;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        dd: [[usize; n]; n],
        q: usize,
        pp: [usize; q]
    };

    let mut sum = vec![0; n * n];

    for yi in 0..n {
        for yj in (yi + 1)..=n {
            for xi in 0..n {
                for xj in (xi + 1)..=n {
                    let mut s = 0;
                    for y in yi..yj {
                        for x in xi..xj {
                            s += dd[y][x];
                        }
                    }
                    let k = (yj - yi) * (xj - xi) - 1;
                    sum[k] = max(s, sum[k]);
                }
            }
        }
    }

    let mut ts = sum[0];
    for i in 1..(n * n) {
        ts = max(ts, sum[i]);
        sum[i] = ts;
    }

    for p in pp.into_iter() {
        println!("{}", sum[p - 1]);
    }
}
