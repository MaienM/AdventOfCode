use std::cmp::Ordering;

/// Chainable versions of the sort methods on [`slice`].
pub trait ChainSort<T, R> {
    /// As [`slice::sort`].
    fn sort(self) -> R
    where
        T: Ord;

    /// As [`slice::sort_by`].
    fn sort_by<F>(self, compare: F) -> R
    where
        F: FnMut(&T, &T) -> Ordering;

    /// As [`slice::sort_by_key`].
    fn sort_by_key<F, K>(self, f: F) -> R
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// As [`slice::sort_by_cached_key`].
    fn sort_by_cached_key<F, K>(self, f: F) -> R
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// As [`slice::sort_unstable`].
    fn sort_unstable(self) -> R
    where
        T: Ord;

    /// As [`slice::sort_unstable_by`].
    fn sort_unstable_by<F>(self, compare: F) -> R
    where
        F: FnMut(&T, &T) -> Ordering;

    /// As [`slice::sort_unstable_by_key`].
    fn sort_unstable_by_key<F, K>(self, f: F) -> R
    where
        F: FnMut(&T) -> K,
        K: Ord;
}
impl<I, T> ChainSort<T, <Vec<T> as IntoIterator>::IntoIter> for I
where
    I: Iterator<Item = T>,
{
    /// As [`slice::sort`]. Internally converts into [`Vec`] to perform the sort.
    fn sort(self) -> <Vec<T> as IntoIterator>::IntoIter
    where
        T: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort();
        list.into_iter()
    }

    /// As [`slice::sort_by`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by<F>(self, compare: F) -> <Vec<T> as IntoIterator>::IntoIter
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by(compare);
        list.into_iter()
    }

    /// As [`slice::sort_by_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by_key<F, K>(self, f: F) -> <Vec<T> as IntoIterator>::IntoIter
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by_key(f);
        list.into_iter()
    }

    /// As [`slice::sort_by_cached_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by_cached_key<F, K>(self, f: F) -> <Vec<T> as IntoIterator>::IntoIter
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by_cached_key(f);
        list.into_iter()
    }

    /// As [`slice::sort_unstable`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable(self) -> <Vec<T> as IntoIterator>::IntoIter
    where
        T: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable();
        list.into_iter()
    }

    /// As [`slice::sort_unstable_by`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable_by<F>(self, compare: F) -> <Vec<T> as IntoIterator>::IntoIter
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable_by(compare);
        list.into_iter()
    }

    /// As [`slice::sort_unstable_by_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable_by_key<F, K>(self, f: F) -> <Vec<T> as IntoIterator>::IntoIter
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable_by_key(f);
        list.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn sort() {
        assert_eq!(
            [-5, 4, 1, -3, 2].into_iter().sort().collect::<Vec<_>>(),
            [-5, -3, 1, 2, 4]
        );
    }

    #[test]
    fn sort_by() {
        assert_eq!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_by(Ord::cmp)
                .collect::<Vec<_>>(),
            [-5, -3, 1, 2, 4]
        );
        assert_eq!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_by(|a, b| b.cmp(a))
                .collect::<Vec<_>>(),
            [4, 2, 1, -3, -5]
        );
    }

    #[test]
    fn sort_by_key() {
        assert_eq!(
            [-5i32, 4, 1, -3, 2]
                .into_iter()
                .sort_by_key(|k| k.abs())
                .collect::<Vec<_>>(),
            [1, 2, -3, 4, -5]
        );
    }

    #[test]
    fn sort_by_cached_key() {
        assert_eq!(
            [-5i32, 4, 32, -3, 2]
                .into_iter()
                .sort_by_cached_key(ToString::to_string)
                .collect::<Vec<_>>(),
            [-3, -5, 2, 32, 4]
        );
    }

    #[test]
    fn sort_unstable() {
        assert_eq!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable()
                .collect::<Vec<_>>(),
            [-5, -3, 1, 2, 4]
        );
    }

    #[test]
    fn sort_unstable_by() {
        assert_eq!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by(Ord::cmp)
                .collect::<Vec<_>>(),
            [-5, -3, 1, 2, 4]
        );
        assert_eq!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by(|a, b| b.cmp(a))
                .collect::<Vec<_>>(),
            [4, 2, 1, -3, -5]
        );
    }

    #[test]
    fn sort_unstable_by_key() {
        assert_eq!(
            [-5i32, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by_key(|k| k.abs())
                .collect::<Vec<_>>(),
            [1, 2, -3, 4, -5]
        );
    }
}
