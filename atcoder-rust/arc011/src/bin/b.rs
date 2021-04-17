#![allow(unused_imports)]

use std::collections::HashMap;
use std::iter::FromIterator;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        ww: [String; n]
    };

    let mut nums = Vec::new();
    for i in 0..n {
        let s = ww[i]
            .to_lowercase()
            .chars()
            .into_iter()
            .map(|c| match c {
                'z' => "0",
                'r' => "0",
                'b' => "1",
                'c' => "1",
                'd' => "2",
                'w' => "2",
                't' => "3",
                'j' => "3",
                'f' => "4",
                'q' => "4",
                'l' => "5",
                'v' => "5",
                's' => "6",
                'x' => "6",
                'p' => "7",
                'm' => "7",
                'h' => "8",
                'k' => "8",
                'n' => "9",
                'g' => "9",
                _ => "",
            })
            .join("");
        if !s.is_empty() {
            nums.push(s);
        }
    }
    println!("{}", nums.iter().join(" "));
}
