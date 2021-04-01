#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        r: usize,
        mut ss: [usize; n-1]
    };

    ss.sort();
    ss.reverse();

    let req_sum = k * r;
    let cs = ss[0..(k-1)].iter().sum::<usize>();


    if (k == n && cs >= req_sum) || (k != n && cs + ss[k - 1] >= req_sum) {
        println!("{}", 0);
        return;
    }

    if cs + m < req_sum {
        println!("{}", -1);
        return;
    }
    println!("{}", req_sum - cs);
}
