#![allow(unused_imports)]

use std::collections::{BTreeMap, BTreeSet};

use num::integer::sqrt;
use num_integer::gcd;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let c = gcd(a, b);

    println!("{}", prime_factors(c).len() + 1);
}

pub fn prime_factors(mut n: usize) -> BTreeSet<usize> {
    let mut fs = BTreeSet::new();

    if n % 2 == 0 {
        fs.insert(2);
        while n % 2 == 0 {
            n /= 2;
        }
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            fs.insert(i);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 2;
    }

    if n > 1 {
        fs.insert(n);
    }

    return fs;
}
