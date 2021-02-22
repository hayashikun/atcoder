use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cards: [usize; n]
    }
    cards.sort();

    let mut a = 0;
    let mut b = 0;

    loop {
        if let Some(i) = cards.pop() {
            a += i;
        } else {
            break;
        }
        if let Some(i) = cards.pop() {
            b += i;
        } else {
            break;
        }
    }
    println!("{}", a - b);
}
