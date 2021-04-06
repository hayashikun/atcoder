#![allow(unused_imports)]

use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        vv: [usize; n]
    };

    let mut om: HashMap<usize, usize> = HashMap::new();
    let mut em: HashMap<usize, usize> = HashMap::new();
    for i in 0..(n / 2) {
        let c = om.entry(vv[2 * i]).or_insert(0);
        *c += 1;

        let c = em.entry(vv[2 * i + 1]).or_insert(0);
        *c += 1;
    }

    let ov = om.into_iter().sorted_by_key(|&a| a.1).rev().collect_vec();
    let ev = em.into_iter().sorted_by_key(|&a| a.1).rev().collect_vec();

    let c = if ov[0].0 == ev[0].0 {
        if ov.len() == 1 && ev.len() == 1 {
            ov[0].1
        } else {
            if ov[0].1 > ev[0].1 {
                n - ov[0].1 - ev[1].1
            } else if ov[0].1 < ev[0].1 {
                n - ov[1].1 - ev[0].1
            } else {
                if ov[1].1 > ev[1].1 {
                    n - ov[1].1 - ev[0].1
                } else {
                    n - ov[0].1 - ev[1].1
                }
            }
        }
    } else {
        n - ov[0].1 - ev[0].1
    };
    println!("{}", c);
}
