#![allow(dead_code)]
#[macro_use(s)]
#[macro_use(stack)]
extern crate ndarray;

use ndarray::{Array, Array2, Axis};
use proconio::input;
use proconio::marker::Chars;
use std::cmp::{Ordering, Reverse};

fn main() {
    abc173_d();
}

fn abc173_d() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }
    aa.sort_by_key(|&x| Reverse(x));
    println!("{:?}", aa);
    let mut comfort: usize = 0;
    let mut circle: Vec<usize> = Vec::new();
    for a in aa {
        let diff = circle.iter().map(|c| c - a);
        let max = diff.max().unwrap_or(0).clone();
        if let Some((idx, _)) = diff.enumerate().find(|(i, d)| *d == max) {
            circle.insert(idx, a);
        } else {
            circle.push(a)
        }
    }
    println!("{:?}", circle);
}

fn abc173_c() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        ss: [Chars; h],
    }
    let grid = Array::from_shape_vec(
        (h, w),
        ss.concat()
            .iter()
            .map(|&c| if c == '#' { 1 } else { 0 })
            .collect(),
    )
    .unwrap();
    println!("{}", check_v(&grid, k, 0));

    fn check_v(grid: &Array2<usize>, k: usize, v: usize) -> usize {
        let sum = grid.sum();
        if sum < k {
            return 0;
        }
        let mut total = 0;
        total += check_h(&grid, k, 0);
        for i in v..grid.shape()[1] {
            total += check_v(
                &stack![
                    Axis(1),
                    grid.slice(s![.., ..i]),
                    grid.slice(s![.., (i + 1)..])
                ],
                k,
                i,
            );
        }
        return total;
    }

    fn check_h(grid: &Array2<usize>, k: usize, h: usize) -> usize {
        let sum = grid.sum();
        if sum < k {
            return 0;
        }
        let mut total = if sum == k { 1 } else { 0 };
        for i in h..grid.shape()[0] {
            total += check_h(
                &stack![
                    Axis(0),
                    grid.slice(s![..i, ..]),
                    grid.slice(s![(i + 1).., ..])
                ],
                k,
                i,
            );
        }
        return total;
    }
}

fn abc173_b() {
    input!(n: u32);
    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;
    for _ in 0..n {
        input!(s: String);
        match &*s {
            "AC" => ac += 1,
            "WA" => wa += 1,
            "TLE" => tle += 1,
            "RE" => re += 1,
            _ => (),
        }
    }
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}

fn abc173_a() {
    input!(n: u16);
    if n % 1000 == 0 {
        println!("0")
    } else {
        println!("{}", ((n / 1000) + 1) * 1000 - n)
    }
}
