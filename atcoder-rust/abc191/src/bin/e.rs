use std::collections::BinaryHeap;

use proconio::marker::Usize1;
use proconio::{fastout, input};

const MAX_COST: usize = 1_000_000_000;

pub fn calc_cost(n: usize, s: usize, neighbors: Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut costs = vec![MAX_COST; n];
    let mut nearest = vec![s; n];

    costs[s] = 0;

    // priority, index
    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    queue.push((MAX_COST, s));

    while !queue.is_empty() {
        let (priority, current) = queue.pop().unwrap();
        let cost = costs[current];

        // index in queue can be duplicated
        if MAX_COST - priority > cost {
            continue;
        }

        // update cost
        for n in neighbors[current].iter() {
            // neighbor cost
            let nc = cost + n.1;
            if nc < costs[n.0] {
                costs[n.0] = nc;
                nearest[n.0] = current;
                queue.push((MAX_COST - nc, n.0));
            }
        }
    }
    return costs;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m]
    };

    let mut costs = vec![MAX_COST; n];

    let mut go: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    let mut back: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for r in abc {
        if r.0 == r.1 {
            if r.2 < costs[r.0] {
                costs[r.0] = r.2;
            }
        } else {
            go[r.0].push((r.1, r.2));
            back[r.1].push((r.0, r.2));
        }
    }

    for i in 0..n {
        let go_cost = calc_cost(n, i, go.clone());
        let back_cost = calc_cost(n, i, back.clone());

        for j in 0..n {
            if i == j {
                continue;
            }
            let cost = go_cost[j] + back_cost[j];
            if cost < costs[i] {
                costs[i] = cost;
            }
        }
    }

    for c in costs {
        if c == MAX_COST {
            println!("-1")
        } else {
            println!("{}", c);
        }
    }
}
