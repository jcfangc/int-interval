use core::iter::FusedIterator;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OneTwo<T> {
    One(T),
    Two(T, T),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ZeroOneTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}

/// Owned iterator for `OneTwo<T>` and `ZeroOneTwo<T>`.
#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    items: [Option<T>; 2],
    front: usize,
    back: usize,
}

impl<T> IntoIter<T> {
    #[inline]
    fn zero() -> Self {
        Self {
            items: [None, None],
            front: 0,
            back: 0,
        }
    }

    #[inline]
    fn one(value: T) -> Self {
        Self {
            items: [Some(value), None],
            front: 0,
            back: 1,
        }
    }

    #[inline]
    fn two(first: T, second: T) -> Self {
        Self {
            items: [Some(first), Some(second)],
            front: 0,
            back: 2,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        self.items[index].take()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        self.back -= 1;
        self.items[self.back].take()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<T> FusedIterator for IntoIter<T> {}

impl<T> IntoIterator for OneTwo<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::One(value) => IntoIter::one(value),
            Self::Two(first, second) => IntoIter::two(first, second),
        }
    }
}

impl<T> IntoIterator for ZeroOneTwo<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::Zero => IntoIter::zero(),
            Self::One(value) => IntoIter::one(value),
            Self::Two(first, second) => IntoIter::two(first, second),
        }
    }
}

#[cfg(test)]
mod tests_for_into_iter;
