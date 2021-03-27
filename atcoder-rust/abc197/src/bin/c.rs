#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use std::ops::BitXor;

#[fastout]
fn main() {
    input!{
        n: usize,
        aa: [usize; n],
    };

    if n == 1 {
        println!("{}", aa[0]);
        return;
    }

    let mut ans = usize::max_value();
    for i in 1..2i32.pow(n as u32 - 1) {
        let mut b = 0;
        let mut a = aa[0];

        for j in 1..n {
            if i.rotate_right((j - 1) as u32) & 1 == 1 {
                b ^= a;
                a = aa[j];
            } else {
                a |= aa[j];
            }
        }
        b ^= a;

        if b < ans {
            ans = b;
        }
    }

    println!("{}", ans);
}
