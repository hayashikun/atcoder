#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        s: Chars
    };

    if check(a, b, s) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check(a: usize, b: usize, s: Vec<char>) -> bool {
    for i in 0..a {
        if !s[i].is_numeric() {
            return false;
        }
    }

    if s[a] != '-' {
        return false;
    }

    for i in (a + 1)..(a + b + 1) {
        if !s[i].is_numeric() {
            return false;
        }
    }

    return true;
}
