// TODO
#![allow(unused_imports)]

use std::cmp::min;

use itertools::Itertools;
use libm::sqrt;
use num_traits::Pow;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let xy = xy
        .into_iter()
        .sorted_by_key(|p| p.1)
        .map(|p| (p.0 as f64, p.1 as f64))
        .collect_vec();
    let mut min_dis = 200.0;

    for i in 0..n {
        for j in (i + 1)..n {
            if xy[j].1 - xy[i].1 > min_dis {
                break;
            }
            let dis = sqrt((xy[j].0 - xy[i].0).pow(2) + (xy[j].1 - xy[i].1).pow(2));
            if dis < min_dis {
                min_dis = dis;
            }
        }
    }

    println!("{}", min_dis);
}
