#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        cs: Chars
    };

    let mut ans = true;
    for (i, c) in cs.into_iter().enumerate() {
        if c == 'U' || c == 'D' || (i % 2 == 1 && c == 'L') || (i % 2 == 0 && c == 'R') {
            // ok
        } else {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
