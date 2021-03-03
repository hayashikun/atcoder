use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        gr: [String; h]
    };

    let gr: Vec<Vec<usize>> = gr
        .iter()
        .map(|s| s.chars().map(|c| if c == '.' { 0 } else { 1 }).collect())
        .collect();

    let mut corner = 0;
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            if (gr[y][x] + gr[y + 1][x] + gr[y][x + 1] + gr[y + 1][x + 1]) % 2 == 1 {
                corner += 1;
            }
        }
    }

    println!("{}", corner);
}
