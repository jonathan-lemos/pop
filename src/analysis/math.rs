use crate::parallelism::algorithms::{into_parallel_reduce, parallel_map};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SatisfactionFraction {
    pub satisfying: usize,
    pub total: usize,
}

pub fn satisfaction_ratio<T: Send + Sync, P: Fn(&T) -> bool + Send + Sync>(
    slice: &[T],
    predicate: P,
) -> SatisfactionFraction {
    if slice.is_empty() {
        return SatisfactionFraction {
            satisfying: 0,
            total: 0,
        };
    }

    let ratios = parallel_map(slice, |x| SatisfactionFraction {
        satisfying: if predicate(x) { 1 } else { 0 },
        total: 1,
    });

    into_parallel_reduce(ratios, |a, b| SatisfactionFraction {
        satisfying: a.satisfying + b.satisfying,
        total: a.total + b.total,
    })
    .unwrap()
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

    #[test]
    fn test_satisfaction_ratio_small() {
        let nums = &[1, 1, 2, 3, 5, 8, 13];

        assert_eq!(
            satisfaction_ratio(nums, |x| x % 2 == 0),
            SatisfactionFraction {
                satisfying: 2,
                total: 7
            }
        );
    }

    #[test]
    fn test_satisfaction_ratio_large() {
        let nums = (1..=10000).into_iter().collect::<Vec<i32>>();

        assert_eq!(
            satisfaction_ratio(nums.as_slice(), |x| *x <= 100),
            SatisfactionFraction {
                satisfying: 100,
                total: 10000
            }
        );
    }

    #[test]
    fn test_satisfaction_ratio_zero() {
        let nums = (1..=10000).into_iter().collect::<Vec<i32>>();

        assert_eq!(
            satisfaction_ratio(nums.as_slice(), |x| *x > 10000),
            SatisfactionFraction {
                satisfying: 0,
                total: 10000
            }
        );
    }

    #[test]
    fn test_satisfaction_ratio_empty_set() {
        let nums: &[i32] = &[];

        assert_eq!(
            satisfaction_ratio(nums, |_| true),
            SatisfactionFraction {
                satisfying: 0,
                total: 0
            }
        );
    }
}
