//! Extension methods for numbers.

mod primes;

use std::{iter::successors, ops::DivAssign};

use num::Integer;
use primes::PRIMES;

const LAST_PRIME: u32 = PRIMES[PRIMES.len() - 1];

/// Extension methods for [`num::Integer`].
pub trait IntegerExt {
    /// Get the [prime factorision](https://en.wikipedia.org/wiki/Integer_factorization) of this number.
    ///
    /// This is implemented using [trial division](https://en.wikipedia.org/wiki/Trial_division),
    /// which is a simple algorithm that has worst-case performance of `O(2^(n/2))` (worst case
    /// being that the number is a prime number or the square of a prime number).
    ///
    /// This is optimized somewhat by using a list of known primes before falling back to all odd
    /// nubmers after the last known prime. The length of this precomputed list has been chosen to
    /// be optimal for up to 32-bit unsigned integers; for larger numbers a larger table can be
    /// used at the cost of this list having a larger footprint in the binary & in memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc::utils::ext::num::IntegerExt;
    /// assert_eq!(7u8.factorize(), vec![7]);
    /// assert_eq!(8u8.factorize(), vec![2, 2, 2]);
    /// assert_eq!(9u8.factorize(), vec![3, 3]);
    /// assert_eq!(210u8.factorize(), vec![2, 3, 5, 7]);
    /// ```
    fn factorize(&self) -> Vec<Self>
    where
        Self: Sized;
}

impl<I> IntegerExt for I
where
    I: Integer + TryFrom<u32> + DivAssign<I> + Clone,
{
    fn factorize(&self) -> Vec<I> {
        // Create infinite iterator that first goes through the list of known primes, and once that
        // runs out just steps through all odd numbers after the last known prime.
        let two = I::one() + I::one();
        let primes = PRIMES
            .iter()
            .copied()
            .map(I::try_from)
            .filter_map(Result::ok)
            .chain(successors(I::try_from(LAST_PRIME).ok(), |p| {
                Some(p.clone() + two.clone())
            }));

        let mut factors = Vec::new();
        let mut remaining = self.clone();
        for prime in primes {
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
