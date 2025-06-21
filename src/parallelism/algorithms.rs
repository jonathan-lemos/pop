use std::{cmp::min, mem::ManuallyDrop, num::NonZero, ops::Range, thread};

use crate::parallelism::{os::get_parallelism_from_os, send_sync_raw_ptr::SendSyncRawPtr};

pub struct SubrangeIterator {
    modulus: usize,
    step: usize,
    current: usize,
    end: usize,
}

impl Iterator for SubrangeIterator {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let this_start = self.current;
        let this_end = this_start
            + self.step
            + if self.modulus > 0 {
                self.modulus -= 1;
                1
            } else {
                0
            };

        self.current = this_end;

        Some(this_start..this_end)
    }
}

impl SubrangeIterator {
    pub fn from_range(range: Range<usize>, n_chunks: NonZero<usize>) -> Self {
        let range_len = match NonZero::new(range.len()) {
            Some(v) => v,
            None => {
                return Self {
                    modulus: 0,
                    step: 0,
                    current: 0,
                    end: 0,
                };
            }
        };

        let n_chunks = min(range_len, n_chunks);
        let step = range_len.get() / n_chunks.get();
        let modulus = range_len.get() % n_chunks.get();

        Self {
            modulus,
            step,
            current: range.start,
            end: range.end,
        }
    }
}

fn parallel_map_with_parallelism<T: Sync, U: Send, F: Fn(&T) -> U + Send + Sync>(
    slice: &[T],
    max_parallelism: NonZero<usize>,
    mapper: F,
) -> Vec<U> {
    let subranges = SubrangeIterator::from_range(0..slice.len(), max_parallelism);

    let mut output = Vec::<U>::new();
    output.reserve_exact(slice.len());
    unsafe { output.set_len(slice.len()) };

    let output_ptr = SendSyncRawPtr {
        ptr: output.as_mut_ptr(),
    };

    thread::scope(|s| {
        let mut threads = Vec::new();

        for subrange in subranges {
            threads.push(s.spawn(|| {
                for i in subrange {
                    unsafe { (output_ptr + i).set(mapper(&slice[i])) }
                }
            }));
        }
    });

    output
}

fn into_parallel_map_with_parallelism<T: Sync, U: Send, F: Fn(T) -> U + Send + Sync>(
    mut vec: Vec<T>,
    max_parallelism: NonZero<usize>,
    mapper: F,
) -> Vec<U> {
    let subranges = SubrangeIterator::from_range(0..vec.len(), max_parallelism);

    let mut output = Vec::<U>::new();
    output.reserve_exact(vec.len());
    unsafe { output.set_len(vec.len()) };

    let input_ptr = SendSyncRawPtr {
        ptr: vec.as_mut_ptr(),
    };

    let output_ptr = SendSyncRawPtr {
        ptr: output.as_mut_ptr(),
    };

    thread::scope(|s| {
        for subrange in subranges {
            s.spawn(|| {
                for i in subrange {
                    unsafe {
                        let output = output_ptr + i;
                        let input = input_ptr + i;
                        output.set(mapper(input.get()))
                    }
                }
            });
        }
    });

    output
}

fn into_parallel_reduce_with_parallelism<T: Send + Sync, F: Fn(T, T) -> T + Send + Sync>(
    vec: Vec<T>,
    max_parallelism: NonZero<usize>,
    reducer: F,
) -> Option<T> {
    let mut vec = vec
        .into_iter()
        .map(ManuallyDrop::new)
        .collect::<Vec<ManuallyDrop<T>>>();
    let subranges = SubrangeIterator::from_range(0..vec.len(), max_parallelism);

    let input_ptr = SendSyncRawPtr {
        ptr: vec.as_mut_ptr(),
    };

    let mut values = thread::scope(|s| {
        let threads = subranges.map(|subrange| {
            let reducer_ref = &reducer;
            s.spawn(move || {
                let mut current =
                    unsafe { ManuallyDrop::into_inner((input_ptr + subrange.start).get()) };
                for i in subrange.start + 1..subrange.end {
                    current = reducer_ref(current, unsafe {
                        ManuallyDrop::into_inner((input_ptr + i).get())
                    });
                }
                current
            })
        });

        threads
            .into_iter()
            .map(|t| ManuallyDrop::new(t.join().unwrap()))
            .collect::<Vec<ManuallyDrop<T>>>()
    });

    if values.is_empty() {
        return None;
    }
    let value_ptr = SendSyncRawPtr {
        ptr: values.as_mut_ptr(),
    };
    let mut current = ManuallyDrop::into_inner(unsafe { value_ptr.get() });
    for i in 1..values.len() {
        current = reducer(
            current,
            ManuallyDrop::into_inner(unsafe { (value_ptr + i).get() }),
        );
    }
    Some(current)
}

pub fn parallel_map<T: Sync, U: Send, F: Fn(&T) -> U + Send + Sync>(
    slice: &[T],
    mapper: F,
) -> Vec<U> {
    parallel_map_with_parallelism(slice, get_parallelism_from_os(), mapper)
}

pub fn into_parallel_map<T: Sync, U: Send, F: Fn(T) -> U + Send + Sync>(
    vec: Vec<T>,
    mapper: F,
) -> Vec<U> {
    into_parallel_map_with_parallelism(vec, get_parallelism_from_os(), mapper)
}

pub fn into_parallel_reduce<T: Send + Sync, F: Fn(T, T) -> T + Send + Sync>(
    vec: Vec<T>,
    reducer: F,
) -> Option<T> {
    into_parallel_reduce_with_parallelism(vec, get_parallelism_from_os(), reducer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subrangeiterator_no_modulus() {
        let ranges = SubrangeIterator::from_range(0..1_000_000, NonZero::new(4).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(
            ranges,
            vec![
                0..250_000,
                250_000..500_000,
                500_000..750_000,
                750_000..1_000_000
            ]
        );
    }

    #[test]
    fn test_subrangeiterator_range_smaller_than_chunks() {
        let ranges = SubrangeIterator::from_range(0..2, NonZero::new(69).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![0..1, 1..2]);
    }

    #[test]
    fn test_subrangeiterator_range_equal_to_parallelism() {
        let ranges = SubrangeIterator::from_range(0..3, NonZero::new(3).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![0..1, 1..2, 2..3]);
    }

    #[test]
    fn test_subrangeiterator_uneven_split_1() {
        let ranges = SubrangeIterator::from_range(0..100, NonZero::new(3).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![0..34, 34..67, 67..100]);
    }

    #[test]
    fn test_subrangeiterator_uneven_split_2() {
        let ranges = SubrangeIterator::from_range(0..101, NonZero::new(3).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![0..34, 34..68, 68..101]);
    }

    #[test]
    fn test_subrangeiterator_empty_range() {
        let ranges = SubrangeIterator::from_range(0..0, NonZero::new(3).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![]);
    }

    #[test]
    fn test_subrangeiterator_one_chunk() {
        let ranges = SubrangeIterator::from_range(0..10, NonZero::new(1).unwrap())
            .collect::<Vec<Range<usize>>>();

        assert_eq!(ranges, vec![0..10]);
    }

    #[test]
    fn test_parallel_map_with_parallelism() {
        let nums = (0..20).into_iter().collect::<Vec<i32>>();

        let doubled =
            parallel_map_with_parallelism(nums.as_slice(), NonZero::new(3).unwrap(), |x| x * 2);

        assert_eq!(
            doubled,
            vec![
                0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38
            ]
        );
    }

    #[test]
    fn test_into_parallel_map_with_parallelism() {
        let nums = (0..20).into_iter().collect::<Vec<i32>>();

        let doubled = into_parallel_map_with_parallelism(nums, NonZero::new(3).unwrap(), |x| x * 2);

        assert_eq!(
            doubled,
            vec![
                0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38
            ]
        );
    }

    #[test]
    fn test_into_parallel_reduce_with_parallelism() {
        let nums = (0..20).into_iter().collect::<Vec<i32>>();

        let sum =
            into_parallel_reduce_with_parallelism(nums, NonZero::new(3).unwrap(), |a, b| a + b);

        assert_eq!(sum, Some(190));
    }

    #[test]
    fn test_into_parallel_reduce_with_parallelism_empty_seq() {
        let nums: Vec<i32> = vec![];

        let sum =
            into_parallel_reduce_with_parallelism(nums, NonZero::new(3).unwrap(), |a, b| a + b);

        assert_eq!(sum, None);
    }
}
