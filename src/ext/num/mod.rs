//! Extension methods for numbers.

mod primes;

use std::ops::DivAssign;

use num::Integer;
use primes::PRIMES;

/// Trait for numeric types for which primes can be generated.
pub trait PrimeGen {
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
    fn primes(limit: Self) -> Vec<Self>
    where
        Self: Sized;
}
impl PrimeGen for usize {
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

/// Trait for numeric types that can be factorized.
pub trait Factorize {
    /// Get the [prime factorision](https://en.wikipedia.org/wiki/Integer_factorization) of this number.
    ///
    /// This is implemented using [trial division](https://en.wikipedia.org/wiki/Trial_division),
    /// which is a simple algorithm that has worst-case performance of `O(2^(n/2))` (worst case
    /// being that the number is a prime number or the square of a prime number).
    ///
    /// This is optimized somewhat by using a list of known primes before falling back to all odd
    /// nubmers after the last known prime. The length of this precomputed list has been chosen to
    /// be optimal for up to 32-bit unsigned integers; for numbers with larger factors a list of
    /// primes can be created using [`PrimeGen::primes`] and passed to
    /// [`Factorize::factorize_with_primes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::prelude::*;
    /// assert_eq!(7u8.factorize(), vec![7]);
    /// assert_eq!(8u8.factorize(), vec![2, 2, 2]);
    /// assert_eq!(9u8.factorize(), vec![3, 3]);
    /// assert_eq!(210u8.factorize(), vec![2, 3, 5, 7]);
    /// ```
    fn factorize(&self) -> Vec<Self>
    where
        Self: Sized;

    /// As [`Factorize::factorize`], but using the given list of known-primes instead of the
    /// default. This list can be as long or short as you want, but it _must_ start at `2` and not
    /// have any gaps (e.g. if it contains `7` it _must_ also contain `3` and `5`).
    fn factorize_with_primes<'a, P, Iter>(&self, primes: Iter) -> Vec<Self>
    where
        Self: TryFrom<P>,
        P: Copy + 'a,
        Iter: Iterator<Item = &'a P>;
}
impl<I> Factorize for I
where
    I: Integer + TryFrom<u32> + DivAssign<I> + Clone,
{
    fn factorize(&self) -> Vec<I> {
        self.factorize_with_primes(PRIMES.iter())
    }

    fn factorize_with_primes<'a, P, Iter>(&self, primes: Iter) -> Vec<Self>
    where
        I: TryFrom<P>,
        P: Copy + 'a,
        Iter: Iterator<Item = &'a P>,
    {
        let mut primes = primes.map(|p| I::try_from(*p)).filter_map(Result::ok);

        let two = I::one() + I::one();
        let mut factors = Vec::new();
        let mut remaining = self.clone();
        let mut prime = I::zero();
        loop {
            prime = primes.next().unwrap_or_else(|| prime + two.clone());
            loop {
                let (div, rem) = remaining.div_rem(&prime);
                if !rem.is_zero() {
                    break;
                }
                remaining = div;
                factors.push(prime.clone());
            }

            if remaining.is_one() {
                break;
            }

            // If `prime > sqrt(remaining)` number we know there's only one factor remaining, and
            // it's `remaining`.
            //
            // (Because if there were another factor (`f`) where `f > sqrt(remaining)` and `f <
            // remaining` there would be a cofactor (`remaining / f`) which would be `< prime` and
            // have at least one more factor, but we've already checked all possible factors `<
            // prime` so this cannot be true.
            if prime.clone() * prime.clone() > remaining {
                factors.push(remaining);
                break;
            }
        }
        factors
    }
}
