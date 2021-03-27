#[allow(dead_code)]
fn bit_flag(v: Vec<usize>) -> Vec<Vec<usize>> {
    let n = v.len() as u32;

    let mut pv = Vec::new();
    for i in 0..2i32.pow(n) {
        let mut p = Vec::new();
        for j in 0..n {
            if i.rotate_right(j) & 1 == 1 {
                p.push(v[j as usize]);
            }
        }
        pv.push(p);
    }

    return pv;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn test_bit_flag() {
        let v = vec![1, 2, 3];
        let pv = bit_flag(v);
        let s = BTreeSet::from_iter(pv);
        assert_eq!(s.len(), 2i32.pow(3) as usize);
    }
}
