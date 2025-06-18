use log::warn;
use std::{
    cmp::min,
    num::NonZero,
    ops::Range,
    thread::{self, available_parallelism},
};

use crate::parallelism::os::get_parallelism_from_os;

fn divide_and_conquer_with_parallelism<
    R: Send,
    F: Fn(Range<usize>) -> R + Send + Sync,
    P: FnOnce() -> NonZero<usize>,
>(
    range: Range<usize>,
    func: F,
    get_parallelism: P,
) -> Vec<R> {
    let width = match NonZero::new(range.len()) {
        Some(w) => w,
        None => return vec![],
    };

    let parallelism = min(get_parallelism(), width);

    let step = width.get() / parallelism.get();
    let mut modulus = width.get() % parallelism.get();

    thread::scope(|s| {
        let mut threads = Vec::new();
        threads.reserve_exact(parallelism.get());

        let mut last = range.start;

        for _ in 0..parallelism.get() {
            let mut this_width = step;
            if modulus > 0 {
                modulus -= 1;
                this_width += 1;
            }
            let this_end = last + this_width;

            let this_range = last..this_end;
            last = this_end;

            threads.push(s.spawn(|| func(this_range)));
        }

        threads.into_iter().map(|x| x.join().unwrap()).collect()
    })
}

// Divides the given range as evenly as possible into a number of subranges equal to the current
// machine's CPU's, then runs the given function on each subrange in parallel.
pub fn divide_and_conquer<R: Send, F: Fn(Range<usize>) -> R + Send + Sync>(
    range: Range<usize>,
    func: F,
) -> Vec<R> {
    divide_and_conquer_with_parallelism(range, func, get_parallelism_from_os)
}

pub fn map_reduce<A, B, M, T, R>(elems: &[A], mapper: M, seed: T, mut reducer: R) -> T
where
    A: Sync,
    B: Send,
    M: Fn(&A) -> B + Send + Sync,
    R: FnMut(T, Vec<B>) -> T,
{
    let func = |range: Range<usize>| {
        let mut ret = Vec::new();
        ret.reserve_exact(range.len());

        for i in range {
            ret.push(mapper(&elems[i]));
        }

        ret
    };

    let chunks = divide_and_conquer(0..elems.len(), func);
    let mut accumulator = seed;
    for chunk in chunks {
        accumulator = reducer(accumulator, chunk);
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_evenly() {
        let range = 0..1_000_000;
        let get_parallelism = || NonZero::new(4).unwrap();
        let func = |r| r;

        let mut results = divide_and_conquer_with_parallelism(range, func, get_parallelism);
        results.sort_by_key(|r| r.start);

        assert_eq!(
            results,
            vec![
                0..250_000,
                250_000..500_000,
                500_000..750_000,
                750_000..1_000_000
            ]
        );
    }

    #[test]
    fn test_split_less_than_parallelism() {
        let range = 0..2;
        let get_parallelism = || NonZero::new(69).unwrap();
        let func = |r| r;

        let mut results = divide_and_conquer_with_parallelism(range, func, get_parallelism);
        results.sort_by_key(|r| r.start);

        assert_eq!(results, vec![0..1, 1..2]);
    }

    #[test]
    fn test_split_equal_to_parallelism() {
        let range = 0..3;
        let get_parallelism = || NonZero::new(3).unwrap();
        let func = |r| r;

        let mut results = divide_and_conquer_with_parallelism(range, func, get_parallelism);
        results.sort_by_key(|r| r.start);

        assert_eq!(results, vec![0..1, 1..2, 2..3]);
    }

    #[test]
    fn test_split_unevenly_1() {
        let range = 0..100;
        let get_parallelism = || NonZero::new(3).unwrap();
        let func = |r| r;

        let mut results = divide_and_conquer_with_parallelism(range, func, get_parallelism);
        results.sort_by_key(|r| r.start);

        assert_eq!(results, vec![0..34, 34..67, 67..100]);
    }

    #[test]
    fn test_split_unevenly_2() {
        let range = 0..101;
        let get_parallelism = || NonZero::new(3).unwrap();
        let func = |r| r;

        let mut results = divide_and_conquer_with_parallelism(range, func, get_parallelism);
        results.sort_by_key(|r| r.start);

        assert_eq!(results, vec![0..34, 34..68, 68..101]);
    }

    #[test]
    fn test_map_reduce() {
        let range = (0..100).into_iter().collect::<Vec<i32>>();
        let sum_of_squares = map_reduce(
            range.as_slice(),
            |x| x * x,
            0,
            |a, b| a + b.iter().sum::<i32>(),
        );

        assert_eq!(sum_of_squares, 328_350);
    }
}
