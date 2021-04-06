#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: Chars
    };

    let ans = n.iter().map(|&c| if c == '9' { "1" } else { "9" }).join("");

    println!("{}", ans);
}
