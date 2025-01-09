mod precomputed;

use num::Integer;
use precomputed::PRECOMPUTED;

/// Trait for numeric types for which primes can be generated.
pub trait Primes: Sized {
    /// A precomputed list of all primes that fit in a `u16`, which is sufficient to efficiently
    /// factorize any `u32`.
    const PRECOMPUTED: [Self; 6542];

    /// Calculate all primes <= the given number.
    ///
    /// This is just a convenience method which creates an empty list, calls `extend_primes` with
    /// this list and the given limit, and then returns this list.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// assert_eq!(usize::primes(16), vec![2, 3, 5, 7, 11, 13]);
    /// ```
    fn primes(limit: Self) -> Vec<Self>;

    /// Extend the given list of primes up to <= the given number.
    ///
    /// The given list _must_ start at `2`, not have any gaps (e.g. if it contains `7` it _must_
    /// also contain `3` and `5`), and be sorted.
    ///
    /// This a pretty straightforward implementation of the [sieve of
    /// Erathosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
    ///
    /// Running this starting with `primes = Vec::new()` and `limit = u32::MAX` would take
    /// approximately half a minute.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// let mut primes = vec![2, 3, 5];
    /// usize::extend_primes(&mut primes, 16);
    /// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13]);
    /// ```
    fn extend_primes(primes: &mut Vec<Self>, limit: Self);
}
impl Primes for usize {
    const PRECOMPUTED: [usize; 6542] = PRECOMPUTED;

    fn primes(limit: usize) -> Vec<usize>
    where
        Self: Sized,
    {
        let mut primes = Vec::new();
        Self::extend_primes(&mut primes, limit);
        primes
    }

    fn extend_primes(primes: &mut Vec<Self>, limit: Self) {
        // Ensure the first two primes are present so we can be sure we start at an odd number and
        // can always step by 2.
        if primes.is_empty() {
            primes.push(2);
        }
        if primes.len() == 1 {
            primes.push(3);
        }

        // We start at the last number in the current list + 2 and then step by 2 (since we only
        // need to consider odd numbers), so the `is_prime` list only need to cover the odd numbers
        // in this range.
        let start = *primes.last().unwrap() + 2;
        let len = (limit - start) / 2 + 1;
        let mut is_prime = vec![true; len]; // the indexes in this are (num - start) / 2

        // Mark multiples of existing primes.
        for prime in primes.iter().skip(1) {
            let mut pstart = prime * prime;
            if pstart < start {
                pstart = start.next_multiple_of(*prime);
                if pstart.is_even() {
                    pstart += prime;
                }
            }
            for n in (pstart..limit).step_by(prime * 2) {
                is_prime[(n - start) / 2] = false;
            }
        }

        // Step through odd numbers in the range and (if they've not yet been marked) add them to
        // the list of primes and mark their multiples.
        for n in (start..limit).step_by(2) {
            if !is_prime[(n - start) / 2] {
                continue;
            }
            primes.push(n);
            for n in ((n * n)..limit).step_by(n * 2) {
                is_prime[(n - start) / 2] = false;
            }
        }
    }
}
