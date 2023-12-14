use std::sync::mpsc;

use threadpool::ThreadPool;

pub trait IterExt<T> {
    /// Like [`Iterator::map`], but with the calls being executed inside threads from a threadpool.
    fn threaded_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static;
}
impl<I, T> IterExt<T> for I
where
    I: Iterator<Item = T>,
{
    fn threaded_map<F, R>(self, pool_size: usize, f: F) -> impl Iterator<Item = R>
    where
        F: Fn(T) -> R + Send + Copy + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let pool = ThreadPool::new(pool_size);
        let (tx, rx) = mpsc::channel::<(usize, R)>();
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
    fn threaded_map() {
        assert_eq!(
            (0..200).threaded_map(5, |v| v * 2).collect::<Vec<_>>(),
            (0..400).step_by(2).collect::<Vec<_>>(),
        );
    }
}
