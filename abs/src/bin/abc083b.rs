use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize
    }
    let mut r = 0;
    for m in 1..n + 1 {
        let s = d_sum(m);
        if a <= s && s <= b {
            r += m;
        }
    }
    print!("{}", r);
}

fn d_sum(d: usize) -> usize {
    if d < 10 {
        d
    } else {
        d % 10 + d_sum(d / 10)
    }
}
