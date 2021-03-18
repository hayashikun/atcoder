#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

fn run(n: usize, k: usize, aa: Vec<usize>) {
    let mut count = 0;
    for i in 0..n {
        let mut sum = 0;
        let mut j = i;
        while j < n {
            sum += aa[j];
            if sum >= k {
                break;
            }
            j += 1;
        }
        if j == n {
            break;
        }
        count += n - j;
    }
    println!("{}", count);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };

    run(n, k, aa);
}
