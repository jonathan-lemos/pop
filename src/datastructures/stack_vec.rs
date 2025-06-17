use std::{
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ops::{Index, Range},
};

use crate::util::ui::format_comma_separated_values;

// A vector whose memory lives entirely on the stack.
#[derive(Clone, Copy)]
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
        format_comma_separated_values(self.as_slice().into_iter(), f, |v, fmt| {
            Debug::fmt(&v, fmt)
        })?;
        f.write_str("]")
    }
}

impl<T: Display, const LENGTH: usize> Display for StackVec<T, LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        format_comma_separated_values(self.as_slice().into_iter(), f, |v, fmt| {
            Display::fmt(&v, fmt)
        })?;
        f.write_str("]")
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
}
