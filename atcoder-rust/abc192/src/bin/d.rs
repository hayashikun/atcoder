// base

use proconio::input;

// check m (10 -> n base) > x
fn check(mut m: u128, n: u128, x: String) -> bool {
    let mut y: Vec<u128> = Vec::new();
    // 10 base to n base
    while 0 < m {
        y.push(m % n);
        m /= n
    }
    y.reverse();

    if x.len() < y.len() {
        return true;
    } else if x.len() > y.len() {
        return false;
    }

    for (i, xc) in x.chars().into_iter().enumerate() {
        let xu: u128 = xc.to_digit(10).unwrap() as u128;
        if xu == y[i] {
            continue;
        }
        return xu < y[i];
    }
    return true;
}

fn main() {
    input! {
        x: String,
        m: u128
    };

    if x.len() == 1 {
        if x.parse::<usize>().unwrap() <= m as usize {
            println!("1")
        } else {
            println!("0")
        }
        return;
    }

    let d = x.chars().max().unwrap().to_digit(10).unwrap() as u128;

    let mut ok = d;
    let mut ng = 1e20 as u128;

    while ok + 1 < ng {
        let md = (ok + ng) / 2;
        if check(m, md, x.clone()) {
            ok = md;
        } else {
            ng = md;
        }
    }

    println!("{}", ok - d);
}
