#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize
    };

    if n == 0 {
        println!("Yes");
        return;
    }

    loop {
        if n % 10 == 0 {
            n /= 10;
        } else {
            break;
        }
    }

    let n = n.to_string().chars().collect_vec();
    let s = n.len();

    for i in 0..(s / 2) {
        if n[i] != n[s - i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
