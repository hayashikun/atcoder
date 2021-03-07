use std::collections::HashMap;

use num_integer::gcd;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };

    aa.sort();
    aa.dedup();

    let min = aa[0];

    let mut map = HashMap::new();

    for a in aa {
        for i in 1.. {
            if i * i > a {
                break;
            }
            if a % i != 0 {
                continue;
            }
            if i < min {
                let p = map.entry(i).or_insert(a);
                *p = gcd(*p, a);
            }
            if a / i < min {
                let p = map.entry(a / i).or_insert(a);
                *p = gcd(*p, a);
            }
        }
    }

    let mut ans = 1;
    for (v, c) in map {
        if v == c {
            ans += 1;
        }
    }
    println!("{}", ans);
}
