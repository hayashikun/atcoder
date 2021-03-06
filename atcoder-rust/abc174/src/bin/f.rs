use std::ops::{AddAssign, Sub};

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        cs: [Usize1; n],
        lr: [(Usize1, Usize1); q]
    }
    let lr = lr
        .into_iter()
        .enumerate()
        .sorted_by_key(|&(_, (_, r))| r)
        .collect_vec();

    let mut idx = vec![n; n];
    let mut bit = vec![0; n + 1];
    let mut ans = vec![0; q];
    let mut cr = 0;

    for &(i, (l, r)) in lr.iter() {
        for j in cr..=r {
            if idx[cs[j]] != n {
                bit.add(idx[cs[j]], -1);
            }
            idx[cs[j]] = j;
            bit.add(idx[cs[j]], 1);
        }

        cr = r + 1;
        ans[i] = if l > 0 {
            bit.sum(r) - bit.sum(l - 1)
        } else {
            bit.sum(r)
        }
    }

    for a in ans {
        println!("{}", a);
    }
}

trait BIT<T: Copy + Clone + Sub + AddAssign> {
    fn add(&mut self, idx: usize, w: T);
    fn sum(&self, idx: usize) -> T;
}

impl<T: Copy + Clone + Sub<Output = T> + AddAssign> BIT<T> for [T] {
    fn add(&mut self, idx: usize, w: T) {
        let mut j = (idx + 1) as i64;
        while j < self.len() as i64 {
            self[j as usize] += w;
            j += j & -j;
        }
    }

    fn sum(&self, idx: usize) -> T {
        let mut i = (idx + 1) as i64;
        let mut sum = self[0];
        while i != 0 {
            sum += self[i as usize];
            i -= i & -i;
        }
        sum
    }
}
