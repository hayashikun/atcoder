use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let mut count = 0;
    for a in 1..(k + 1) {
        for b in 1..(k / a + 1) {
            for c in 1..(k / a / b + 1) {
                if a * b * c <= k {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
