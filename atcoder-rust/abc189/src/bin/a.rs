#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        ccc: Chars
    };

    if ccc[0] == ccc[1] && ccc[0] == ccc[2] {
        println!("Won")
    } else {
        println!("Lost")
    }
}
