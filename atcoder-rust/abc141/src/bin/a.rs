#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: String
    };

    let a = match s.as_str() {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        "Rainy" => "Sunny",
        _ => "",
    };
    println!("{}", a);
}
