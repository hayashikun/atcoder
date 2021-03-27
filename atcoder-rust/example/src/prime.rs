use std::collections::BTreeMap;

use num::integer::sqrt;

//noinspection ALL
#[allow(dead_code)]
pub fn prime_factors(mut n: u128) -> BTreeMap<u128, u128> {
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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_prime_factors() {
        for n in vec![64, 2021, 1000000007] {
            let start = Instant::now();
            let fs = prime_factors(n);
            let end = start.elapsed();
            let p: u128 = fs
                .iter()
                .map(|(n, &c)| n.pow(c as u32))
                .fold(1, |a, b| a * b);
            assert_eq!(n, p);
            println!("factors of {}: {:?} [{} us]", n, fs, end.as_micros());
        }
    }
}
