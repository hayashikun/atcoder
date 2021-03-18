#![allow(unused_imports)]

use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: isize,
        mut aa: [isize; n]
    };

    aa.sort();

    let n1 = aa.pop().unwrap();

    let mut n2 = 0;
    let mut diff = isize::max_value();

    for a in aa {
        let d = abs(a * 2 - n1);
        if d < diff {
            diff = d;
            n2 = a;
        }
    }

    println!("{} {}", n1, n2);
}
