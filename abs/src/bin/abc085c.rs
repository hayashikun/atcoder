use proconio::input;

fn main() {
    input! {
        n: isize,
        y: isize
    }
    let xyz = f_xyz(n, y);
    print!("{} {} {}", xyz.0, xyz.1, xyz.2)
}

fn f_xyz(n: isize, m: isize) -> (isize, isize, isize) {
    if m == n * 10000 {
        return (n, 0, 0);
    }
    for x in 0..n {
        let (y, z) = f_yz(n - x, m - x * 10000);
        if y >= 0 {
            return (x, y, z);
        }
    }
    return (-1, -1, -1);
}

fn f_yz(n: isize, m: isize) -> (isize, isize) {
    if m == n * 5000 {
        return (n, 0);
    }
    if n == 0 {
        return (-1, -1);
    }
    for y in 0..n {
        let z = f_z(n - y, m - y * 5000);
        if z >= 0 {
            return (y, z);
        }
    }
    return (-1, -1);
}

fn f_z(n: isize, m: isize) -> isize {
    if m == n * 1000 {
        n
    } else {
        -1
    }
}
