use proconio::input;

fn run(n: usize, cs: String) -> usize {
    let cs: Vec<char> = cs.chars().collect();
    let mut w = 0;
    let mut r = n - 1;

    let mut count = 0;
    while r > w {
        if cs[w] == 'R' {
            w += 1;
            continue;
        } else if cs[r] == 'W' {
            r -= 1;
            continue;
        }
        count += 1;
        w += 1;
        r -= 1;
    }

    return count;
}

fn main() {
    input! {
        n: usize,
        cs: String
    }

    println!("{}", run(n, cs));
}
