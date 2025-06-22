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
}
