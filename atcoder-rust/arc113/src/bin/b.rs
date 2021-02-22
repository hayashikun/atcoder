use proconio::input;

fn mod4_bc(b: u128, c: u128) -> u128 {
    if b == 1 {
        return 1;
    }

    let b1 = b % 10;
    let c1 = c % 10;
    return if b1 % 2 == 0 {
        if c1 % 2 == 0 {
            0
        } else {
            if c == 1 {
                b % 4
            } else {
                0
            }
        }
    } else {
        if c1 % 2 == 0 {
            (b * b) % 4
        } else {
            b % 4
        }
    };
}

fn run(a: u128, b: u128, c: u128) -> u128 {
    let a0 = a % 10;

    for i in &[0, 1, 5, 6] {
        if a0 == *i {
            return *i;
        }
    }

    return match mod4_bc(b, c) {
        1 => a0,
        2 => a0.pow(2) % 10,
        3 => a0.pow(3) % 10,
        _ => a0.pow(4) % 10,
    };
}

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
    }

    println!("{}", run(a, b, c));
}
