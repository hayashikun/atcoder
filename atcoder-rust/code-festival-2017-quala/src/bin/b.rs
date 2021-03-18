#![allow(unused_imports)]

use std::mem::swap;

use num::integer::sqrt;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: isize,
        mut m: isize,
        k: isize
    };

    if m < n {
        swap(&mut n, &mut m);
    }

    for x in 0..=m {
        let t1 = n * x - k;
        let t2 = 2 * x - m;
        if t2 == 0 {
            if t1 == 0 {
                println!("Yes");
                return;
            }
        } else if t1 % t2 == 0 && 0 <= t1 / t2 && t1 / t2 <= n {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
