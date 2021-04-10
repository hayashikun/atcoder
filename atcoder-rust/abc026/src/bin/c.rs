#![allow(unused_imports)]

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        bb: [Usize1; n - 1]
    };

    let mut sub = vec![Vec::new(); n];
    for (i, b) in bb.into_iter().enumerate() {
        sub[b].push(i + 1);
    }

    let mut salary = vec![0; n];
    println!("{}", rec(0, &mut salary, &sub));
}

fn rec(i: usize, salary: &mut Vec<usize>, sub: &Vec<Vec<usize>>) -> usize {
    if salary[i] > 0 {
        return salary[i];
    }

    if sub[i].len() == 0 {
        salary[i] = 1;
        return 1;
    }

    let ss = sub[i].iter().map(|&s| rec(s, salary, sub)).collect_vec();

    let s = ss.iter().max().unwrap() + ss.iter().min().unwrap() + 1;
    salary[i] = s;
    return s;
}
