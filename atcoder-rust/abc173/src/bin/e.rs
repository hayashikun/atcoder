use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut aa: [isize; n]
    }
    aa.sort_by_key(|a| a.abs());

    if k % 2 == 0 || aa.iter().any(|&a| a > 0) {
        aa.reverse();
        loop {
            if aa[..k].iter().all(|&a| a > 0) {
                break;
            }

            let neg_idx: Vec<usize> = aa
                .iter()
                .enumerate()
                .filter(|&(_, &a)| a < 0)
                .take(2)
                .map(|(i, _)| i)
                .collect();
            if neg_idx.len() == 1 {
                aa.remove(neg_idx[0]);
            } else {
                aa.push(aa[neg_idx[0]] * aa[neg_idx[1]]);
                k -= 1;
                aa.sort_by_key(|a| a.abs());
                aa.reverse();
            }
        }
    }
    let div = 1_000_000_007;
    let mut c = 1;
    for a in aa[..k].iter() {
        c *= a;
        if c.abs() > div {
            c %= div;
        }
    }
    if c < 0 {
        c += div;
    }
    println!("{}", c);
}
