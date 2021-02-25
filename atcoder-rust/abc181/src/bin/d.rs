use num::range_step;
use proconio::{fastout, input};

fn run(ss: String) -> bool {
    let len = ss.len();

    let ns: Vec<usize> = ss
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    if len == 1 {
        return ns[0] == 8;
    }

    if len == 2 {
        return (ns[0] * 10 + ns[1]) % 8 == 0 || (ns[1] * 10 + ns[0]) % 8 == 0;
    }

    let mut c1: Vec<usize> = Vec::with_capacity(10);
    for _ in 0..10 {
        c1.push(0);
    }
    for n in ns {
        c1[n] += 1;
    }

    let mut c2: Vec<usize> = Vec::with_capacity(10);
    for _ in 0..10 {
        c2.push(0);
    }
    for n in range_step(112, 1000, 8) {
        let i1 = n / 100;
        let i2 = (n % 100) / 10;
        let i3 = n % 10;

        c2[i1] += 1;
        c2[i2] += 1;
        c2[i3] += 1;

        if (0..10).into_iter().all(|i| c1[i] >= c2[i]) {
            return true;
        }

        c2[i1] = 0;
        c2[i2] = 0;
        c2[i3] = 0;
    }

    return false;
}

#[fastout]
fn main() {
    input! {
        ss: String
    }

    if run(ss) {
        println!("Yes")
    } else {
        println!("No")
    }
}
