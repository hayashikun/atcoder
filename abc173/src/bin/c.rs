use proconio::input;

fn main() {
    input! {
        h: usize,
        _: usize,
        k: usize,
        ss: [String; h],
    }
    let grid = ss
        .into_iter()
        .map(|s| s.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    println!("{}", check_v(&grid, k, 0));
}

fn check_v(grid: &Vec<Vec<usize>>, k: usize, v: usize) -> usize {
    let sum: usize = grid.iter().map(|v| -> usize { v.iter().sum() }).sum();
    if sum < k {
        return 0;
    }
    let mut total = 0;
    total += check_h(grid, k, 0);
    for i in v..grid[0].len() {
        let ng: Vec<Vec<usize>> = grid
            .to_owned()
            .into_iter()
            .map(|r| -> Vec<usize> {
                r.iter()
                    .enumerate()
                    .filter(|&(n, _)| n != i)
                    .map(|(_, &e)| e)
                    .collect()
            })
            .collect();
        total += check_v(&ng, k, i);
    }
    return total;
}

fn check_h(grid: &Vec<Vec<usize>>, k: usize, h: usize) -> usize {
    let sum: usize = grid.iter().map(|v| -> usize { v.iter().sum() }).sum();
    if sum < k {
        return 0;
    }
    let mut total = if sum == k { 1 } else { 0 };
    for i in h..grid.len() {
        let ng = grid
            .to_owned()
            .into_iter()
            .enumerate()
            .filter(|&(n, _)| n != i)
            .map(|(_, e)| e)
            .collect();
        total += check_h(&ng, k, i);
    }
    return total;
}
