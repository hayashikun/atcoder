#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        aa: [Usize1; q]
    };

    let mut counts = vec![0; n];
    aa.iter().for_each(|&a| counts[a] += 1);

    for c in counts {
        println!("{}", if k + c > q { "Yes" } else { "No" });
    }
}
