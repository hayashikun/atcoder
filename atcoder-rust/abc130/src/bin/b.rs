#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        n: usize,
        x: usize,
        ll: [usize; n]
    };

    let mut count = 1;
    let mut current = 0;

    for l in ll.into_iter() {
        current += l;
        if current <= x {
            count += 1;
        } else {
            break
        }
    }
    println!("{}", count);
}
