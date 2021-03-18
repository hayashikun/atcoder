#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

fn run(n: usize, m: usize, mut aa: Vec<usize>, mut bc: Vec<(usize, usize)>) {
    aa.sort();
    bc.sort_by_key(|&e| e.1);
    bc.reverse();
    let mut j = 0;
    let mut ok = false;
    for i in 0..m {
        for _ in 0..bc[i].0 {
            if j < n && aa[j] < bc[i].1 {
                aa[j] = bc[i].1;
                j += 1;
            } else {
                ok = true;
            }
        }
        if ok {
            break;
        }
    }
    let ans: usize = aa.iter().sum();
    println!("{}", ans);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        bc: [(usize, usize); m]
    };

    run(n, m, aa, bc);
}
