// TODO (still TLE)
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        cs: [usize; n],
        lr: [[usize; 2]; q]
    }

    let mut cn = Vec::with_capacity(n);
    unsafe {
        cn.set_len(n);
    }
    for i in 0..n {
        cn[i] = 0;
    }
    for c in cs.clone() {
        cn[c - 1] += 1
    }

    let mut block: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        if cn[cs[i] - 1] < 2 {
            continue;
        }
        cn[cs[i] - 1] -= 1;
        for j in (i + 1)..n {
            if cs[i] == cs[j] {
                block.push((i + 1, j + 1));
                break;
            }
        }
    }

    for i in 0..q {
        let l = lr[i][0];
        let r = lr[i][1];

        let mut count = 0;
        for &(bs, be) in block.iter() {
            if be <= r && l <= bs {
                count += 1;
            }
            if r <= bs {
                break;
            }
        }
        println!("{}", r - l + 1 - count);
    }
}
