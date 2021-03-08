use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: i128,
        xypq: [(i128, i128, i128, i128); t]
    }
    run(t, xypq);
}

fn run(_t: i128, xypq: Vec<(i128, i128, i128, i128)>) {
    for (x, y, p, q) in xypq.into_iter() {
        let a = 2 * (x + y);
        let b = p + q;
        let mut ans = i128::max_value();
        for t1 in x..(x + y) {
            for t2 in p..(p + q) {
                let (r, m) = crt(vec![t1, t2], vec![a, b]);
                if m > 0 && ans > r {
                    ans = r
                }
            }
        }
        if ans == i128::max_value() {
            println!("infinity")
        } else {
            println!("{}", ans);
        }
    }
}

fn ext_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = ext_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn crt(rs: Vec<i128>, ms: Vec<i128>) -> (i128, i128) {
    let mut rem = 0;
    let mut lcm = 1;
    for (r, m) in rs.into_iter().zip(ms) {
        let (d, p, _) = ext_gcd(lcm, m);
        if (r - rem) % d != 0 {
            return (0, 0);
        }

        let m_d = m / d;

        let mut t = ((r - rem) / d * p) % m_d;
        if t < 0 {
            t += m_d;
        }
        rem += lcm * t;
        lcm *= m_d;
    }
    return (rem, lcm);
}
