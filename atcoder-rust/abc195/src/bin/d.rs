#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

fn run(
    _n: usize,
    m: usize,
    q: usize,
    mut wv: Vec<(usize, usize)>,
    x: Vec<usize>,
    lr: Vec<(usize, usize)>,
) {
    for i in 0..q {
        let mut a_box = Vec::new();
        for j in 0..lr[i].0 {
            a_box.push(x[j]);
        }
        for j in (lr[i].1 + 1)..m {
            a_box.push(x[j]);
        }
        a_box.sort();

        wv.sort_by_key(|&e| e.1);
        wv.reverse();

        let mut sum = 0;
        for (w, v) in wv.clone() {
            let mut a_idx = usize::max_value();
            for (j, &s) in a_box.iter().enumerate() {
                if s >= w {
                    sum += v;
                    a_idx = j;
                    break;
                }
            }

            if a_idx != usize::max_value() {
                a_box.remove(a_idx);
            }
        }

        println!("{}", sum);
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(usize, usize); n],
        x: [usize; m],
        lr: [(Usize1, Usize1); q],
    };

    run(n, m, q, wv, x, lr);
}
