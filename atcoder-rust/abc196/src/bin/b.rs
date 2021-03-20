#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        s: String
    };

    println!("{}", s.split(".").collect_vec()[0]);
}
