pub fn n_choose_r(n: usize, r: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n < r {
        return 0;
    }
    if r == 0 {
        return 1;
    }

    let numerator: usize = ((n - r + 1)..=n).product();
    let denominator: usize = (1..=r).product();
    numerator / denominator
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
