// b^e % m
pub fn mod_ex(b: u128, e: u128, m: u128) -> u128 {
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_mod_ex() {
        for (b, e, m, c1) in vec![
            (3, 4, 5, 1),
            (3, 100, 19, 16),
            (58979, 92653, 998244353, 518090637),
            (1000000007, 1000000007, 998244353, 970844471),
        ] {
            let start = Instant::now();
            let c2 = mod_ex(b, e, m);
            let end = start.elapsed();
            assert_eq!(c1, c2);
            println!("{} ^ {} % {} = {} [{} ns]", b, e, m, c2, end.as_nanos());
        }
    }
}
