#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        a: String
    };

    if a.starts_with("YAKI") {
        println!("Yes")
    } else {
        println!("No")
    }
}
