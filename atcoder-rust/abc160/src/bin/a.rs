#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use regex::internal::Char;

#[fastout]
fn main() {
    input! {
        s: Chars
    };

    if s[2] == s[3] && s[4] == s[5] {
        println!("Yes")
    } else {
        println!("No")
    }
}
