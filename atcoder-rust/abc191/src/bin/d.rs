use num::integer::{div_ceil, sqrt};
use num_integer::div_floor;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64
    };

    let x = (10000.0 * x).round() as isize;
    let y = (10000.0 * y).round() as isize;
    let r = (10000.0 * r).round() as isize;

    let mut sum = 0;
    for cx in (x - r) / 10000..=(x + r) / 10000 {
        let c2 = r.pow(2);
        let a2 = (cx * 10000 - x).pow(2);
        if a2 > c2 {
            continue;
        }
        let b = sqrt(c2 - a2);

        let y1 = div_floor(y + b, 10000);
        let y2 = div_ceil(y - b, 10000);

        sum += y1 - y2 + 1;
    }

    println!("{}", sum)
}
