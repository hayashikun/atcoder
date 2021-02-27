use std::cmp::min;

use proconio::{fastout, input};

fn run(n: usize, w: usize, wv: Vec<Vec<usize>>) -> usize {
    let sum_v: usize = wv.iter().map(|v| v[1]).sum();
    let mut a1 = [1_000_000_000_000; 100001];
    let mut a2 = [0; 100001];

    a1[0] = 0;
    for i in 0..n {
        let iw = wv[i][0];
        let iv = wv[i][1];
        for j in 0..(sum_v + 1) {
            if j < iv {
                a2[j] = a1[j];
            } else {
                a2[j] = min(a1[j], a1[j - iv] + iw);
            }
        }
        a1 = a2;
    }

    let mut v = 0;
    for j in 0..(sum_v + 1) {
        if a1[j] <= w && v < j {
            v = j
        }
    }

    return v;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [[usize; 2]; n]
    }

    println!("{}", run(n, w, wv));
}
