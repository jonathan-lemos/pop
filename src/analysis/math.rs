use std::collections::HashMap;

fn n_choose_r_memoized(n: usize, r: usize, mut memo: &mut HashMap<(usize, usize), usize>) -> usize {
    match memo.get(&(n, r)) {
        Some(v) => return *v,
        None => {}
    };

    // No ways to chose 4 elements from a set of 3.
    if n < r {
        return 0;
    }
    // One way to choose 0 elements. The empty set.
    if r == 0 {
        return 1;
    }
    // 3 ways to pick 1 element out of a set of 3.
    if r == 1 {
        return n;
    }

    let value =
        n_choose_r_memoized(n - 1, r, &mut memo) + n_choose_r_memoized(n - 1, r - 1, &mut memo);
    memo.insert((n, r), value);
    value
}

pub fn n_choose_r(n: usize, r: usize) -> usize {
    n_choose_r_memoized(n, r, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_choose_r() {
        assert_eq!(n_choose_r(3, 4), 0);
        assert_eq!(n_choose_r(3, 0), 1);
        assert_eq!(n_choose_r(3, 1), 3);
        assert_eq!(n_choose_r(3, 2), 3);
        assert_eq!(n_choose_r(3, 3), 1);
        assert_eq!(n_choose_r(5, 2), 10);
        assert_eq!(n_choose_r(52, 7), 133_784_560);
    }
}
