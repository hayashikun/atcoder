use std::collections::BinaryHeap;

// n nodes
// from s to e
// edges (e1, e2, cost) (e1 <-> e2)
#[allow(dead_code)]
pub fn dijkstra(
    n: usize,
    s: usize,
    e: usize,
    edges: Vec<(usize, usize, usize)>,
) -> (isize, Vec<usize>) {
    let mut costs = vec![usize::max_value(); n];
    let mut nearest = vec![s; n];

    // neighbors[from] = (to, cost)
    let mut neighbors: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for r in edges {
        neighbors[r.0].push((r.1, r.2));
        neighbors[r.1].push((r.0, r.2));
    }

    costs[s] = 0;

    // priority, index
    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    queue.push((usize::max_value(), s));

    while !queue.is_empty() {
        let (priority, current) = queue.pop().unwrap();
        let cost = costs[current];

        // index in queue can be duplicated
        if usize::max_value() - priority > cost {
            continue;
        }

        // update cost
        for n in neighbors[current].iter() {
            // neighbor cost
            let nc = cost + n.1;
            if nc < costs[n.0] {
                costs[n.0] = nc;
                nearest[n.0] = current;
                queue.push((usize::max_value() - nc, n.0));
            }
        }
    }

    if costs[e] == usize::max_value() {
        return (-1, vec![]);
    };

    let mut routes = Vec::new();
    let mut current = e;
    while current != s {
        routes.push(current);
        current = nearest[current];
    }
    routes.push(s);
    return (costs[e] as isize, routes.into_iter().rev().collect());
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_dijkstra() {
        let start = Instant::now();
        let (cost, routes) = dijkstra(
            8,
            0,
            7,
            vec![
                (0, 1, 2),
                (0, 2, 7),
                (0, 3, 2),
                (1, 4, 2),
                (1, 5, 4),
                (2, 5, 2),
                (2, 6, 3),
                (3, 6, 5),
                (4, 5, 1),
                (5, 7, 6),
                (6, 7, 2),
            ],
        );
        let end = start.elapsed();
        assert_eq!(cost, 9);
        assert_eq!(routes.len(), 4);
        println!(
            "case 1: {} cost, {} step [{} us]",
            cost,
            routes.len(),
            end.as_micros()
        );

        let start = Instant::now();
        let (cost, routes) = dijkstra(
            8,
            0,
            7,
            vec![
                (0, 1, 2),
                (0, 2, 7),
                (0, 3, 2),
                (1, 4, 2),
                (1, 5, 4),
                (2, 5, 2),
                (4, 5, 1),
                (6, 7, 2),
            ],
        );
        let end = start.elapsed();
        assert_eq!(cost, -1);
        assert_eq!(routes.len(), 0);
        println!(
            "case 2: {} cost, {} step [{} us]",
            cost,
            routes.len(),
            end.as_micros()
        );
    }
}
