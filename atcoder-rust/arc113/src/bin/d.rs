use proconio::input;

// b^e % m
fn mod_ex(b: u128, e: u128, m: u128) -> u128 {
    if e == 0 {
        return 1;
    }
    if b == 0 || b == 1 {
        return b;
    }
    let mut c = mod_ex(b, e / 2, m);
    c = (c * c) % m;
    if e % 2 == 1 {
        c = (c * b) % m;
    }
    return c;
}

fn run(n: u128, m: u128, k: u128) -> u128 {
    let mc = 998244353;
    if n == 1 {
        return mod_ex(k, m, mc);
    } else if m == 1 {
        return mod_ex(k, n, mc);
    }
    let mut a = 0;

    // mod_ex(x - 1, n, mc)
    let mut mex = 0;
    for x in 1..(k + 1) {
        let t = mod_ex(x, n, mc);
        a = (a + (mc + t - mex) * mod_ex(k - x + 1, m, mc)) % mc;
        mex = t;
    }
    return a;
}

fn main() {
    input! {
        n: u128,
        m: u128,
        k: u128,
    }

    println!("{}", run(n, m, k));
}
