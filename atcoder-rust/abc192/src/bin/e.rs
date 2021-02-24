use std::collections::BinaryHeap;

use num_integer::div_ceil;
use proconio::{fastout, input};

/*
Dijkstra's algorithm
*/

fn run(n: usize, _m: usize, x: usize, y: usize, abtk: Vec<Vec<usize>>) -> isize {
    let s = x - 1;
    let e = y - 1;

    let mut costs: Vec<usize> = Vec::with_capacity(n);
    let mut neighbors: Vec<Vec<(usize, usize, usize)>> = Vec::with_capacity(n);

    for _ in 0..n {
        costs.push(usize::max_value());
        neighbors.push(Vec::new());
    }

    for t in abtk {
        neighbors[t[0] - 1].push((t[1] - 1, t[2], t[3]));
        neighbors[t[1] - 1].push((t[0] - 1, t[2], t[3]));
    }

    costs[s] = 0;

    // priority, index
    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    queue.push((usize::max_value(), s));

    while !queue.is_empty() {
        let (priority, current) = queue.pop().unwrap();

        // index in queue can be duplicated
        if usize::max_value() - priority > costs[current] {
            continue;
        }

        // update cost
        for n in neighbors[current].iter() {
            // neighbor cost
            let nc = div_ceil(costs[current], n.2) * n.2 + n.1;
            if nc < costs[n.0] {
                costs[n.0] = nc;
                queue.push((usize::max_value() - nc, n.0));
            }
        }
    }

    return if costs[e] == usize::max_value() {
        -1
    } else {
        costs[e] as isize
    };
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        abtk: [[usize; 4]; m]
    }

    println!("{}", run(n, m, x, y, abtk));
}
