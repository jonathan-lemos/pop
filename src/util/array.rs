use std::mem::MaybeUninit;

pub const fn indexes<const LENGTH: usize>() -> [usize; LENGTH] {
    let mut ret: [usize; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut i = 0;
    while i < LENGTH {
        ret[i] = i;
        i += 1;
    }
    ret
}

pub fn array_map<T, U, F: FnMut(&T) -> U, const LENGTH: usize>(
    arr: &[T; LENGTH],
    mut mapper: F,
) -> [U; LENGTH] {
    let mut ret: [U; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };
    for (i, elem) in arr.iter().enumerate() {
        unsafe { ret.as_mut_ptr().add(i).write(mapper(elem)) }
    }
    ret
}

pub fn into_array_map<T, U, F: FnMut(T) -> U, const LENGTH: usize>(
    arr: [T; LENGTH],
    mut mapper: F,
) -> [U; LENGTH] {
    let mut ret: [U; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };
    for (i, elem) in arr.into_iter().enumerate() {
        unsafe { ret.as_mut_ptr().add(i).write(mapper(elem)) }
    }
    ret
}

pub fn into_array_zip<T, U, F: FnMut(T, T) -> U, const LENGTH: usize>(
    a1: [T; LENGTH],
    a2: [T; LENGTH],
    mut mapper: F,
) -> [U; LENGTH] {
    let mut ret: [U; LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };
    for (i, (a, b)) in a1.into_iter().zip(a2.into_iter()).enumerate() {
        unsafe { ret.as_mut_ptr().add(i).write(mapper(a, b)) }
    }
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonomorphizedArray<T> {
    Len0([T; 0]),
    Len1([T; 1]),
    Len2([T; 2]),
    Len3([T; 3]),
    Len4([T; 4]),
    Len5([T; 5]),
    Len6([T; 6]),
    Len7([T; 7]),
    Len8([T; 8]),
    Len9([T; 9]),
    Len10([T; 10]),
    Len11([T; 11]),
    Len12([T; 12]),
    Len13([T; 13]),
    Len14([T; 14]),
    Len15([T; 15]),
    Len16([T; 16]),
    Len17([T; 17]),
    Len18([T; 18]),
    Len19([T; 19]),
    Len20([T; 20]),
    Len21([T; 21]),
    Len22([T; 22]),
    Len23([T; 23]),
}

pub fn monomorphize<T, I: Iterator<Item = T>>(elements: I) -> Option<MonomorphizedArray<T>> {
    let elems = elements.collect::<Vec<T>>();
    let len = elems.len();

    match len {
        0 => Some(MonomorphizedArray::Len0([])),
        1 => {
            let mut ret: [T; 1] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len1(ret))
        }
        2 => {
            let mut ret: [T; 2] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len2(ret))
        }
        3 => {
            let mut ret: [T; 3] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len3(ret))
        }
        4 => {
            let mut ret: [T; 4] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len4(ret))
        }
        5 => {
            let mut ret: [T; 5] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len5(ret))
        }
        6 => {
            let mut ret: [T; 6] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len6(ret))
        }
        7 => {
            let mut ret: [T; 7] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len7(ret))
        }
        8 => {
            let mut ret: [T; 8] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len8(ret))
        }
        9 => {
            let mut ret: [T; 9] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len9(ret))
        }
        10 => {
            let mut ret: [T; 10] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len10(ret))
        }
        11 => {
            let mut ret: [T; 11] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len11(ret))
        }
        12 => {
            let mut ret: [T; 12] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len12(ret))
        }
        13 => {
            let mut ret: [T; 13] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len13(ret))
        }
        14 => {
            let mut ret: [T; 14] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len14(ret))
        }
        15 => {
            let mut ret: [T; 15] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len15(ret))
        }
        16 => {
            let mut ret: [T; 16] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len16(ret))
        }
        17 => {
            let mut ret: [T; 17] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len17(ret))
        }
        18 => {
            let mut ret: [T; 18] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len18(ret))
        }
        19 => {
            let mut ret: [T; 19] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len19(ret))
        }
        20 => {
            let mut ret: [T; 20] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len20(ret))
        }
        21 => {
            let mut ret: [T; 21] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len21(ret))
        }
        22 => {
            let mut ret: [T; 22] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len22(ret))
        }
        23 => {
            let mut ret: [T; 23] = unsafe { MaybeUninit::uninit().assume_init() };
            for (i, elem) in elems.into_iter().enumerate() {
                ret[i] = elem;
            }
            Some(MonomorphizedArray::Len23(ret))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_map() {
        let nums = [1, 2, 3];
        let strings = array_map(&nums, |x| x.to_string());

        let expected = ["1".to_string(), "2".to_string(), "3".to_string()];

        assert_eq!(strings, expected);
    }

    #[test]
    fn test_into_array_map() {
        let nums = [1, 2, 3];
        let strings = into_array_map(nums, |x| x.to_string());

        let expected = ["1".to_string(), "2".to_string(), "3".to_string()];

        assert_eq!(strings, expected);
    }

    #[test]
    fn test_into_array_zip() {
        let nums1 = [1, 2, 3];
        let nums2 = [4, 6, 8];

        let sums = into_array_zip(nums1, nums2, |a, b| a + b);
        let expected = [5, 8, 11];

        assert_eq!(sums, expected);
    }

    #[test]
    fn test_monomorphized_array() {
        assert_eq!(
            monomorphize([1, 2, 3].into_iter()),
            Some(MonomorphizedArray::Len3([1, 2, 3]))
        );

        assert_eq!(
            monomorphize([].into_iter()),
            Some(MonomorphizedArray::Len0::<i32>([]))
        );

        assert_eq!(
            monomorphize([1].into_iter()),
            Some(MonomorphizedArray::Len1([1]))
        );

        assert_eq!(
            monomorphize(1..=23),
            Some(MonomorphizedArray::Len23([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
            ]))
        );

        assert_eq!(monomorphize(1..=24), None);
    }
}
