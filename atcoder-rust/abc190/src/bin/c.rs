#![allow(unused_imports)]

use std::collections::BTreeSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    };

    let mut max_ans = 0;
    for i in 0..2i32.pow(k as u32) {
        let mut dish = BTreeSet::new();

        for j in 0..k {
            if i.rotate_right(j as u32) & 1 == 1 {
                dish.insert(cd[j].0);
            } else {
                dish.insert(cd[j].1);
            }
        }

        let mut ans = 0;
        for j in 0..m {
            if dish.contains(&ab[j].0) && dish.contains(&ab[j].1) {
                ans += 1;
            }
        }
        if ans > max_ans {
            max_ans = ans;
        }
    }

    println!("{}", max_ans);
}
