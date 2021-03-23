#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m]
    };

    let mut li = 0;
    let mut ri = n - 1;
    for (l, r) in lr {
        if li < l {
            li = l;
        }
        if r < ri {
            ri = r;
        }
    }

    if li > ri {
        println!("{}", 0);
    } else {
        println!("{}", ri - li + 1);
    }
}
