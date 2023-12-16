use std::{cmp::Ordering, collections::HashMap, hash::Hash, sync::mpsc};

use threadpool::ThreadPool;

pub trait IterExt<T> {
    /// Count how often each item occurs.
    fn count_occurences(self) -> HashMap<T, usize>
    where
        T: Eq + PartialEq + Hash;

    /// As [`slice::sort`]. Internally converts into [`Vec`] to perform the sort.
    fn sort(self) -> impl Iterator<Item = T>
    where
        T: Ord;

    /// As [`slice::sort_by`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by<F>(self, compare: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T, &T) -> Ordering;

    /// As [`slice::sort_by_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// As [`slice::sort_by_cached_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_by_cached_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// As [`slice::sort_unstable`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable(self) -> impl Iterator<Item = T>
    where
        T: Ord;

    /// As [`slice::sort_unstable_by`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable_by<F>(self, compare: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T, &T) -> Ordering;

    /// As [`slice::sort_unstable_by_key`]. Internally converts into [`Vec`] to perform the sort.
    fn sort_unstable_by_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord;

    /// Like [`Iterator::filter`], but with the calls being executed inside threads from a threadpool.
    fn threaded_filter<F>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = T>
    where
        F: Fn(&T) -> bool + Send + Copy + 'static,
        T: Send + 'static;

    /// Like [`Iterator::filter_map`], but with the calls being executed inside threads from a threadpool.
    fn threaded_filter_map<F, R>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> Option<R> + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static;

    /// Like [`Iterator::map`], but with the calls being executed inside threads from a threadpool.
    fn threaded_map<F, R>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static;
}
impl<I, T> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn count_occurences(self) -> HashMap<T, usize>
    where
        T: Eq + PartialEq + Hash,
    {
        let mut map = HashMap::new();
        for item in self {
            map.entry(item).and_modify(|c| *c += 1).or_insert(1);
        }
        map
    }

    fn sort(self) -> impl Iterator<Item = T>
    where
        T: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort();
        list.into_iter()
    }

    fn sort_by<F>(self, compare: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by(compare);
        list.into_iter()
    }

    fn sort_by_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by_key(f);
        list.into_iter()
    }

    fn sort_by_cached_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_by_cached_key(f);
        list.into_iter()
    }

    fn sort_unstable(self) -> impl Iterator<Item = T>
    where
        T: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable();
        list.into_iter()
    }

    fn sort_unstable_by<F>(self, compare: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable_by(compare);
        list.into_iter()
    }

    fn sort_unstable_by_key<F, K>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut list: Vec<_> = self.collect();
        list.sort_unstable_by_key(f);
        list.into_iter()
    }

    fn threaded_filter<F>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = T>
    where
        F: Fn(&T) -> bool + Send + Copy + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let mut len = 0;
        for (idx, item) in self.enumerate() {
            len += 1;
            let tx = tx.clone();
            pool.execute(move || {
                let result = f(&item);
                tx.send((idx, result, item)).unwrap();
            });
        }

        let mut results: Vec<_> = rx.iter().take(len).collect();
        results.sort_unstable_by_key(|(idx, _, _)| *idx);
        results
            .into_iter()
            .filter(|(_, m, _)| *m)
            .map(|(_, _, v)| v)
    }

    fn threaded_filter_map<F, R>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> Option<R> + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let mut len = 0;
        for (idx, item) in self.enumerate() {
            len += 1;
            let tx = tx.clone();
            pool.execute(move || {
                let result = f(item);
                tx.send((idx, result)).unwrap();
            });
        }

        let mut results: Vec<_> = rx.iter().take(len).collect();
        results.sort_unstable_by_key(|(idx, _)| *idx);
        results.into_iter().filter_map(|(_, v)| v)
    }

    fn threaded_map<F, R>(self, pool: &ThreadPool, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let mut len = 0;
        for (idx, item) in self.enumerate() {
            len += 1;
            let tx = tx.clone();
            pool.execute(move || {
                let result = f(item);
                tx.send((idx, result)).unwrap();
            });
        }

        let mut results: Vec<_> = rx.iter().take(len).collect();
        results.sort_unstable_by_key(|(idx, _)| *idx);
        results.into_iter().map(|(_, v)| v)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn sort() {
        assert!([-5, 4, 1, -3, 2].into_iter().sort().collect::<Vec<_>>() == [-5, -3, 1, 2, 4]);
    }

    #[test]
    fn sort_by() {
        assert!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_by(Ord::cmp)
                .collect::<Vec<_>>()
                == [-5, -3, 1, 2, 4]
        );
        assert!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_by(|a, b| b.cmp(a))
                .collect::<Vec<_>>()
                == [4, 2, 1, -3, -5]
        );
    }

    #[test]
    fn sort_by_key() {
        assert!(
            [-5i32, 4, 1, -3, 2]
                .into_iter()
                .sort_by_key(|k| k.abs())
                .collect::<Vec<_>>()
                == [1, 2, -3, 4, -5]
        );
    }

    #[test]
    fn sort_by_cached_key() {
        assert!(
            [-5i32, 4, 32, -3, 2]
                .into_iter()
                .sort_by_cached_key(ToString::to_string)
                .collect::<Vec<_>>()
                == [-3, -5, 2, 32, 4]
        );
    }

    #[test]
    fn sort_unstable() {
        assert!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable()
                .collect::<Vec<_>>()
                == [-5, -3, 1, 2, 4]
        );
    }

    #[test]
    fn sort_unstable_by() {
        assert!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by(Ord::cmp)
                .collect::<Vec<_>>()
                == [-5, -3, 1, 2, 4]
        );
        assert!(
            [-5, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by(|a, b| b.cmp(a))
                .collect::<Vec<_>>()
                == [4, 2, 1, -3, -5]
        );
    }

    #[test]
    fn sort_unstable_by_key() {
        assert!(
            [-5i32, 4, 1, -3, 2]
                .into_iter()
                .sort_unstable_by_key(|k| k.abs())
                .collect::<Vec<_>>()
                == [1, 2, -3, 4, -5]
        );
    }

    #[test]
    fn count_occurences() {
        let counts = ["foo", "foo", "bar", "foo", "baz", "bar"]
            .into_iter()
            .count_occurences();
        assert_eq!(counts.len(), 3);
        assert_eq!(counts["foo"], 3);
        assert_eq!(counts["bar"], 2);
        assert_eq!(counts["baz"], 1);
    }

    #[test]
    fn threaded_filter_map() {
        assert_eq!(
            (0..200)
                .threaded_filter_map(&ThreadPool::new(5), |v| if v % 2 == 0 {
                    Some(v * 2)
                } else {
                    None
                })
                .collect::<Vec<_>>(),
            (0..400).step_by(4).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn threaded_filter() {
        assert_eq!(
            (0..200)
                .threaded_filter(&ThreadPool::new(5), |v| v % 2 == 0)
                .collect::<Vec<_>>(),
            (0..200).step_by(2).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn threaded_map() {
        assert_eq!(
            (0..200)
                .threaded_map(&ThreadPool::new(5), |v| v * 2)
                .collect::<Vec<_>>(),
            (0..400).step_by(2).collect::<Vec<_>>(),
        );
    }
}
