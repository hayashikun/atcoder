#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        n: usize
    };

    if n % 2 == 0 {
        println!("{}", n / 2 - 1);
    } else {
        println!("{}", n / 2)
    }
}
