#![allow(unused_imports)]

use std::cmp::max;

use nalgebra::min;
use proconio::{fastout, input, marker::*};

fn run(n: usize, m: usize, aa: Vec<usize>) {
    let &ma = aa.iter().max().unwrap();
    let mut length: Vec<isize> = vec![-1; ma + 1];
    let mut starts: Vec<isize> = vec![-1; ma + 1];

    for i in 0..n {
        let l = i as isize - starts[aa[i]] as isize;
        length[aa[i]] = max(length[aa[i]], l);
        starts[aa[i]] = i as isize;
    }

    for i in 0..=ma {
        if starts[i] != -1 {
            length[i] = max(length[i], n as isize - starts[i]);
        }
    }

    for i in 0..=ma {
        if length[i] > m as isize || length[i] < 0 {
            println!("{}", i);
            return;
        }
    }

    println!("{}", ma + 1);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n]
    };

    run(n, m, aa);
}
