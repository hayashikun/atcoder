use std::cmp::max;

use proconio::{fastout, input};

fn run(n: usize, abc: Vec<Vec<usize>>) -> usize {
    let mut hap = (abc[0][0], abc[0][1], abc[0][2]);
    for i in 1..n {
        hap = (
            max(hap.1, hap.2) + abc[i][0],
            max(hap.0, hap.2) + abc[i][1],
            max(hap.0, hap.1) + abc[i][2],
        );
    }
    return max(max(hap.0, hap.1), hap.2);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n]
    }

    println!("{}", run(n, abc));
}
