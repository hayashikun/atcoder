use std::cmp::max;

use proconio::{fastout, input};

fn run(n: usize, w: usize, wv: Vec<Vec<usize>>) -> usize {
    let mut a1 = [0; 100001];
    let mut a2 = [0; 100001];
    for i in 0..n {
        let iw = wv[i][0];
        let iv = wv[i][1];
        for j in 0..(w + 1) {
            if j < iw {
                a2[j] = a1[j];
            } else {
                a2[j] = max(a1[j], a1[j - iw] + iv);
            }
        }
        a1 = a2;
    }

    return a1[w];
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
