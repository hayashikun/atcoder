#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use itertools::Itertools;
use nalgebra::max;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s1: Chars,
        s2: Chars,
        s3: Chars,
    };

    let s1 = s1.into_iter().map(|c| c as usize - 97).collect_vec();
    let s2 = s2.into_iter().map(|c| c as usize - 97).collect_vec();
    let s3 = s3.into_iter().map(|c| c as usize - 97).collect_vec();

    let mut cm = vec![-2; 26];
    let mut ds = vec![false; 10];

    for &c in s1.iter() {
        cm[c] = -1;
    }
    for &c in s2.iter() {
        cm[c] = -1;
    }
    for &c in s3.iter() {
        cm[c] = -1;
    }

    if cm.iter().sum::<isize>() > -42 {
        println!("UNSOLVABLE");
        return;
    }

    if max(s1.len(), s2.len()) < s3.len() {
        ds[1] = true;
        cm[s3[0] as usize] = 1;
    }

    if !rec(cm, ds, &s1, &s2, &s3) {
        println!("UNSOLVABLE");
    }
}

fn rec(cm: Vec<isize>, ds: Vec<bool>, s1: &Vec<usize>, s2: &Vec<usize>, s3: &Vec<usize>) -> bool {
    let mut ok = true;
    for (c, &cd) in cm.iter().enumerate() {
        if cd == -1 {
            ok = false;
            for (d, _) in ds.iter().enumerate().filter(|(_, &b)| !b) {
                let mut ccm = cm.clone();
                let mut cds = ds.clone();
                ccm[c as usize] = d as isize;
                cds[d] = true;
                if rec(ccm, cds, s1, s2, s3) {
                    return true;
                }
            }
            break;
        }
    }

    if ok {
        if cm[s1[0]] == 0 || cm[s2[0]] == 0 || cm[s3[0]] == 0 {
            return false;
        }

        let mut d = 1;
        let mut n1 = 0;
        for &c in s1.iter().rev() {
            n1 += cm[c] * d;
            d *= 10;
        }

        d = 1;
        let mut n2 = 0;
        for &c in s2.iter().rev() {
            n2 += cm[c] * d;
            d *= 10;
        }

        d = 1;
        let mut n3 = 0;
        for &c in s3.iter().rev() {
            n3 += cm[c] * d;
            d *= 10;
        }

        if n1 + n2 == n3 {
            println!("{}", n1);
            println!("{}", n2);
            println!("{}", n3);
            return true;
        }
    }
    return false;
}
