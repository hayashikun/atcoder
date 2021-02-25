use std::collections::BTreeMap;

use num::integer::sqrt;
use proconio::{fastout, input};

fn prime_factor(mut n: u128) -> BTreeMap<u128, u128> {
    let mut fs = BTreeMap::new();

    let mut count = 0;
    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    if count > 0 {
        fs.insert(2, count);
    }

    let mut i = 1;
    while i < sqrt(n) {
        i += 2;
        if n % i != 0 {
            continue;
        }
        count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        fs.insert(i, count);
    }

    if n != 1 {
        fs.insert(n, 1);
    }

    return fs;
}

#[fastout]
fn main() {
    input! {
        mut n: u128
    }

    let ps = prime_factor(n);

    let mut count = 0;
    for (_, &c) in ps.iter() {
        let mut n = 0;
        while (n * n + n) <= 2 * c {
            n += 1;
        }
        count += n - 1;
    }

    println!("{:?}", count);
}
