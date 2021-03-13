#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        m: usize,
        h: usize
    };

    if h % m == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
