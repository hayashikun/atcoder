#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        ss: [Chars; h]
    };

    let mut grid = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if ss[i][j] == '#' {
                grid[i][j] = 1;
            }
        }
    }

    let mut sum = 1;

    for i in 1.. {
        if y < i || grid[x][y - i] == 1 {
            break;
        } else {
            sum += 1;
        }
    }

    for i in 1.. {
        if w - 1 < y + i || grid[x][y + i] == 1 {
            break;
        } else {
            sum += 1;
        }
    }

    for i in 1.. {
        if x < i || grid[x - i][y] == 1 {
            break;
        } else {
            sum += 1;
        }
    }

    for i in 1.. {
        if h - 1 < x + i || grid[x + i][y] == 1 {
            break;
        } else {
            sum += 1;
        }
    }

    println!("{}", sum);
}
