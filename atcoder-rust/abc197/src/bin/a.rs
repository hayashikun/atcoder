#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};


#[fastout]
fn main() {
    input!{
        s: Chars
    };

    for i in 1..s.len() {
        print!("{}", s[i]);
    }
    print!("{}", s[0]);
}
