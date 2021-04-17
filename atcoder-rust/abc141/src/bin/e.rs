#![allow(unused_imports)]

use nalgebra::{max, min};
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut ans = 0;
    for l in 0..n {
        // Z-Algorithm
        let size = n - l;
        let mut z = vec![0; size];
        z[0] = size;
        let mut i = 1;
        let mut j = 0;
        while i < size {
            while i + j < size && s[l + j] == s[l + i + j] {
                j += 1;
            }
            z[i] = j;

            if j == 0 {
                i += 1;
                continue;
            }

            let mut k = 1;
            while k < j && k + z[k] < j {
                z[i + k] = z[k];
                k += 1;
            }
            i += k;
            j -= k;
        }
        for i in 1..size {
            ans = max(ans, min(i, z[i]));
        }
    }
    println!("{}", ans);
}
