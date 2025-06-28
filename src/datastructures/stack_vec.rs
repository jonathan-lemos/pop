use crate::ui::output::format_separated_values;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ops::{Index, Range};

// A vector whose memory lives entirely on the stack.
pub struct StackVec<T, const LENGTH: usize> {
    elems: [T; LENGTH],
    length: usize,
}

impl<T, const LENGTH: usize> StackVec<T, LENGTH> {
    pub fn new() -> Self {
        Self {
            elems: unsafe { MaybeUninit::zeroed().assume_init() },
            length: 0,
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.elems[0..self.length]
    }

    pub fn as_slice(&self) -> &[T] {
        &self.elems[0..self.length]
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        StackVecIterator {
            vec: &self,
            position: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // Does nothing if the vector is full.
    pub fn push(&mut self, element: T) {
        if self.length >= LENGTH {
            return;
        }
        self.elems[self.length] = element;
        self.length += 1;
    }

    pub fn reset(&mut self) {
        self.length = 0;
    }
}

impl<T, const LENGTH: usize, const ARR_LENGTH: usize> From<[T; ARR_LENGTH]>
    for StackVec<T, LENGTH>
{
    fn from(value: [T; ARR_LENGTH]) -> Self {
        const { assert!(LENGTH >= ARR_LENGTH) }

        let mut ret = Self::new();
        for elem in value {
            ret.push(elem);
        }

        ret
    }
}

impl<T: Clone, const LENGTH: usize, const ARR_LENGTH: usize> From<&[T; ARR_LENGTH]>
    for StackVec<T, LENGTH>
{
    fn from(value: &[T; ARR_LENGTH]) -> Self {
        const { assert!(LENGTH >= ARR_LENGTH) }

        let mut ret = Self::new();
        for elem in value {
            ret.push(elem.clone());
        }

        ret
    }
}

impl<T: Clone, const LENGTH: usize> Clone for StackVec<T, LENGTH> {
    fn clone(&self) -> Self {
        let mut ret = Self::new();
        for (i, elem) in self.iter().enumerate() {
            ret.elems[i] = elem.clone();
        }
        ret.length = self.length;
        ret
    }
}

impl<T: Copy, const LENGTH: usize> Copy for StackVec<T, LENGTH> {}

impl<T, const LENGTH: usize> Index<usize> for StackVec<T, LENGTH> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.length);
        &self.elems[index]
    }
}

impl<T, const LENGTH: usize> Index<Range<usize>> for StackVec<T, LENGTH> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        assert!(index.end <= self.length);
        &self.elems[index]
    }
}

impl<T: Debug, const LENGTH: usize> Debug for StackVec<T, LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        format_separated_values(self.as_slice().into_iter(), ", ", f, |v, fmt| {
            Debug::fmt(&v, fmt)
        })?;
        f.write_str("]")
    }
}

impl<T: Display, const LENGTH: usize> Display for StackVec<T, LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        format_separated_values(self.as_slice().into_iter(), ", ", f, |v, fmt| {
            Display::fmt(&v, fmt)
        })?;
        f.write_str("]")
    }
}

pub struct StackVecIterator<'a, T, const LENGTH: usize> {
    vec: &'a StackVec<T, LENGTH>,
    position: usize,
}

impl<'a, T, const LENGTH: usize> Iterator for StackVecIterator<'a, T, LENGTH> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.vec.len() {
            None
        } else {
            let val = Some(&self.vec[self.position]);
            self.position += 1;
            val
        }
    }
}

impl<T: PartialEq, const LENGTH: usize> PartialEq for StackVec<T, LENGTH> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq, const LENGTH: usize> Eq for StackVec<T, LENGTH> {}

impl<T: PartialOrd, const LENGTH: usize> PartialOrd for StackVec<T, LENGTH> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (a, b) in self.iter().zip(other.iter()) {
            match a.partial_cmp(b) {
                Some(Ordering::Equal) => {}
                x => return x,
            }
        }

        Some(self.len().cmp(&other.len()))
    }
}

impl<T: Ord, const LENGTH: usize> Ord for StackVec<T, LENGTH> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (a, b) in self.iter().zip(other.iter()) {
            match a.cmp(b) {
                Ordering::Equal => {}
                x => return x,
            }
        }

        self.len().cmp(&other.len())
    }
}

impl<T: Hash, const LENGTH: usize> Hash for StackVec<T, LENGTH> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for elem in self.iter() {
            elem.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::datastructures::stack_vec::*;

    #[test]
    fn test_stack_vec() {
        let mut vec = StackVec::<i32, 3>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
        assert_eq!(vec[0..2], [1, 2]);
        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_stack_vec_from() {
        let v1 = StackVec::<i32, 3>::from([1, 2]);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_from_ref() {
        let v1 = StackVec::<i32, 3>::from(&[1, 2]);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_clone() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let v2 = v1.clone();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_copy() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let v2 = v1;
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_mut_slice() {
        let mut vec = StackVec::<i32, 3>::new();
        vec.push(3);
        vec.push(2);
        vec.push(1);

        vec.as_mut_slice().sort();

        assert_eq!(vec.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_stack_vec_reset() {
        let mut vec = StackVec::<i32, 3>::new();
        vec.push(1);
        vec.push(2);
        vec.reset();
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.as_slice(), &[3, 4, 5]);
    }

    #[test]
    fn test_stack_vec_eq() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_not_eq() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(3);
        v2.push(4);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_stack_vec_not_eq_different_lengths() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_stack_vec_not_eq_different_lengths_2() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);
        v1.push(3);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_stack_vec_not_eq_different_second_elem() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(3);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_stack_vec_not_eq_different_order() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(2);
        v2.push(1);

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_stack_vec_eq_empty() {
        let v1 = StackVec::<i32, 3>::new();
        let v2 = StackVec::<i32, 3>::new();

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_stack_vec_iter() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let vec = v1.iter().map(|x| *x).collect::<Vec<i32>>();

        assert_eq!(vec, vec![1, 2]);
    }

    #[test]
    fn test_stack_vec_empty_iter() {
        let v1 = StackVec::<i32, 3>::new();
        let vec = v1.iter().map(|x| *x).collect::<Vec<i32>>();

        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_stack_vec_ord_eq() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert!(v1.cmp(&v2) == Ordering::Equal);
    }

    #[test]
    fn test_stack_vec_ord_first_element_less() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(3);
        v2.push(4);

        assert!(v1.cmp(&v2) == Ordering::Less);
    }

    #[test]
    fn test_stack_vec_ord_first_element_greater() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(3);
        v1.push(4);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert!(v1.cmp(&v2) == Ordering::Greater);
    }

    #[test]
    fn test_stack_vec_ord_second_element_less() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(3);

        assert!(v1.cmp(&v2) == Ordering::Less);
    }

    #[test]
    fn test_stack_vec_ord_second_element_greater() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(3);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert!(v1.cmp(&v2) == Ordering::Greater);
    }

    #[test]
    fn test_stack_vec_ord_first_shorter() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);
        v2.push(3);

        assert!(v1.cmp(&v2) == Ordering::Less);
    }

    #[test]
    fn test_stack_vec_ord_first_longer() {
        let mut v1 = StackVec::<i32, 3>::new();
        v1.push(1);
        v1.push(2);
        v1.push(3);

        let mut v2 = StackVec::<i32, 3>::new();
        v2.push(1);
        v2.push(2);

        assert!(v1.cmp(&v2) == Ordering::Greater);
    }
}
