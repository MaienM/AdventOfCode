use std::{collections::HashMap, hash::Hash, sync::mpsc};

use threadpool::ThreadPool;

pub trait IterExt<T> {
    /// Like [`Iterator::filter_map`], but with the calls being executed inside threads from a threadpool.
    fn threaded_filter_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> Option<R> + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static;

    /// Like [`Iterator::filter`], but with the calls being executed inside threads from a threadpool.
    fn threaded_filter<F>(self, pool_size: usize, f: F) -> impl Iterator<Item = T>
    where
        F: Fn(&T) -> bool + Send + Copy + 'static,
        T: Send + 'static;

    /// Like [`Iterator::map`], but with the calls being executed inside threads from a threadpool.
    fn threaded_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static;

    /// Count how often each item occurs.
    fn count_occurences(self) -> HashMap<T, usize>
    where
        T: Eq + PartialEq + Hash;
}
impl<I, T> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn threaded_filter_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> Option<R> + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let pool = ThreadPool::new(pool_size);
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

    fn threaded_filter<F>(self, pool_size: usize, f: F) -> impl Iterator<Item = T>
    where
        F: Fn(&T) -> bool + Send + Copy + 'static,
        T: Send + 'static,
    {
        let pool = ThreadPool::new(pool_size);
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

    fn threaded_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let pool = ThreadPool::new(pool_size);
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn threaded_filter_map() {
        assert_eq!(
            (0..200)
                .threaded_filter_map(5, |v| if v % 2 == 0 { Some(v * 2) } else { None })
                .collect::<Vec<_>>(),
            (0..400).step_by(4).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn threaded_filter() {
        assert_eq!(
            (0..200)
                .threaded_filter(5, |v| v % 2 == 0)
                .collect::<Vec<_>>(),
            (0..200).step_by(2).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn threaded_map() {
        assert_eq!(
            (0..200).threaded_map(5, |v| v * 2).collect::<Vec<_>>(),
            (0..400).step_by(2).collect::<Vec<_>>(),
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
}
