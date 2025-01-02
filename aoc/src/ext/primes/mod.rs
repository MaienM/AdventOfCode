mod precomputed;

use precomputed::PRECOMPUTED;

/// Trait for numeric types for which primes can be generated.
pub trait Primes: Sized {
    /// A precomputed list of all primes that fit in a `u16`, which is sufficient to efficiently factorize any `u32`.
    const PRECOMPUTED: [Self; 6542];

    /// Calculate all primes <= the given number.
    ///
    /// This a pretty straightforward implementation of the [sieve of
    /// Erathosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
    ///
    /// Running this for `u32::MAX` would take approximately half a minute.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::prelude::*;
    /// assert_eq!(usize::primes(16), vec![2, 3, 5, 7, 11, 13]);
    /// ```
    fn primes(limit: Self) -> Vec<Self>;
}
impl Primes for usize {
    const PRECOMPUTED: [usize; 6542] = PRECOMPUTED;

    fn primes(limit: usize) -> Vec<usize>
    where
        Self: Sized,
    {
        // We really only need to consider odd numbers (and 2), so we can step by 2 and halve all
        // numbers for the is_prime list, halving the amount of memory used and speeding things up a
        // bit.

        let len = limit / 2 + 1;
        let mut is_prime = vec![true; len];
        is_prime[1] = true; // 3

        let mut primes = Vec::new();
        primes.push(2);
        for n in (3..limit).step_by(2) {
            if !is_prime[n / 2] {
                continue;
            }
            primes.push(n);
            for n in ((n * n)..limit).step_by(n * 2) {
                is_prime[n / 2] = false;
            }
        }
        primes
    }
}
