#![allow(unused_imports)]

use num::integer::sqrt;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize
    };

    let tn = n * 2;
    let mut ans = 0;
    for i in 1..=sqrt(tn) {
        if tn % i == 0 {
            if (i + tn / i) % 2 == 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans * 2);
}
