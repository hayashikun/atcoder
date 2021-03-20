#![allow(unused_imports)]

use itertools::Itertools;
use libm::log10;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let ds = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();
    let d = ds.len();
    let hd = d / 2;

    if d == 1 {
        println!("0");
        return;
    }
    if d % 2 != 0 {
        for _ in 0..hd {
            print!("9")
        }
        print!("\n");
        return;
    }


    let mut n1 = 0;
    let mut n2 = 0;

    for i in 0..hd {
        n1 += ds[i] * 10u32.pow((hd - i - 1) as u32);
        n2 += ds[hd + i] * 10u32.pow((hd - i - 1) as u32);
    }

    if n1 <= n2 {
        println!("{}", n1);
    } else {
        println!("{}", n1 - 1);
    }
}
