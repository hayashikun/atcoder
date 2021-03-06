#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

fn run(n: usize, mut aa: Vec<isize>) -> isize {
    aa.sort();

    let mut a: Vec<isize> = Vec::new();
    let mut an: Vec<isize> = Vec::new();

    a.push(aa[0]);
    an.push(1);

    let mut ci = 0;
    for i in 1..n {
        if aa[i] == a[ci] {
            an[ci] += 1;
        } else {
            a.push(aa[i]);
            an.push(1);
            ci += 1;
        }
    }

    let mut sum = 0;

    for i in 1..a.len() {
        for j in 0..i {
            sum += (a[i] - a[j]).pow(2) * an[i] * an[j];
        }
    }

    return sum;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut aa: [isize; n]
    };

    println!("{}", run(n, aa));
}
