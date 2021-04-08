#![allow(unused_imports)]

use std::collections::BinaryHeap;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n]
    };

    let mut heap = BinaryHeap::new();
    for a in aa.into_iter() {
        heap.push(a);
    }

    for _ in 0..m {
        let a = heap.pop().unwrap();
        heap.push(a / 2);
    }

    println!("{}", heap.into_iter().sum::<usize>());
}
