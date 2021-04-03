#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut tt: [usize; n]
    };

    tt.sort();
    println!("{}", tt.first().unwrap());
}
