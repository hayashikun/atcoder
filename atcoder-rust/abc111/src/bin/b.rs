#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize
    };

    loop {
        let d1 = n % 10;
        let d2 = (n % 100) / 10;
        let d3 = n / 100;

        if (d1 == d2) && (d2 == d3) {
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
