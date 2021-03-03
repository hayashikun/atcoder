pub fn topological_sort(nodes_num: usize, edges: Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let mut in_num = vec![0; nodes_num];
    let mut nodes: Vec<Vec<usize>> = vec![Vec::new(); nodes_num];

    for &(i, o) in edges.iter() {
        nodes[i].push(o);
        in_num[o] += 1;
    }

    let mut head = Vec::new();
    for n in 0..nodes_num {
        if in_num[n] == 0 {
            head.push(n);
        }
    }
    let head_node = head.clone();
    let mut sorted_node = Vec::new();

    while !head.is_empty() {
        let n = head.pop().unwrap();
        for &m in nodes[n].iter() {
            in_num[m] -= 1;
            if in_num[m] == 0 {
                head.push(m)
            }
        }

        sorted_node.push(n);
    }

    return (head_node, sorted_node);
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_topological_sort() {
        let start = Instant::now();
        let (h, s) = topological_sort(4, vec![(0, 1), (0, 2), (2, 1), (1, 3), (2, 3)]);
        let end = start.elapsed();
        assert_eq!(h, vec![0]);
        assert_eq!(s, vec![0, 2, 1, 3]);
        println!("topological_sort [{} us]", end.as_micros());

        let start = Instant::now();
        let (h, s) = topological_sort(6, vec![(1, 2), (3, 4), (4, 5)]);
        let end = start.elapsed();
        assert_eq!(h, vec![0, 1, 3]);
        assert_eq!(s, vec![3, 4, 5, 1, 2, 0]);
        println!("topological_sort [{} us]", end.as_micros());
    }
}
