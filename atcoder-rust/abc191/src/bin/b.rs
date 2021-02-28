use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize,
        a: [usize; n],
    };

    println!("{}", a.iter().filter(|&&e| e != x).map(|e| e.to_string()).join(" "))
}
