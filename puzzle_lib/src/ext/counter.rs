use std::{
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
    ops::AddAssign,
};

use num::{Num, One};

/// Use a key-value map as a counter.
pub trait Counter<K, V> {
    /// Increment the given item by the given amount, starting at the given start if it doesn't
    /// exist yet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut counter: HashMap<&str, u8> = HashMap::new();
    /// counter.increment_by_starting_at("foo", 10, 5);
    /// counter.increment_by_starting_at("bar", 1, 0);
    /// counter.increment_by_starting_at("foo", 1, 100);
    /// assert_eq!(counter.get(&"foo"), Some(&16));
    /// assert_eq!(counter.get(&"bar"), Some(&1));
    /// ```
    fn increment_by_starting_at(&mut self, key: K, step: V, start: V);

    /// Increment the given item by the given amount, starting at the default value for the numeric
    /// type (probably 0) if it doesn't exist yet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut counter: HashMap<&str, u8> = HashMap::new();
    /// counter.increment_by("foo", 10);
    /// counter.increment_by("bar", 1);
    /// counter.increment_by("foo", 1);
    /// assert_eq!(counter.get(&"foo"), Some(&11));
    /// assert_eq!(counter.get(&"bar"), Some(&1));
    /// ```
    fn increment_by(&mut self, key: K, step: V)
    where
        V: Default,
    {
        self.increment_by_starting_at(key, step, V::default());
    }

    /// Increment the given item by one, starting at the default value for the numeric type
    /// (probably 0) if it doesn't exist yet.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::prelude::*;
    /// # use std::collections::HashMap;
    /// let mut counter: HashMap<&str, u8> = HashMap::new();
    /// counter.increment_one("foo");
    /// counter.increment_one("bar");
    /// counter.increment_one("foo");
    /// assert_eq!(counter.get(&"foo"), Some(&2));
    /// assert_eq!(counter.get(&"bar"), Some(&1));
    /// ```
    fn increment_one(&mut self, key: K)
    where
        V: Default + One,
    {
        self.increment_by_starting_at(key, V::one(), V::default());
    }
}
impl<K, V, H> Counter<K, V> for HashMap<K, V, H>
where
    K: Eq + Hash,
    V: Num + AddAssign,
    H: BuildHasher,
{
    fn increment_by_starting_at(&mut self, key: K, step: V, start: V) {
        *self.entry(key).or_insert(start) += step;
    }
}
impl<K, V> Counter<K, V> for BTreeMap<K, V>
where
    K: Eq + Ord + Hash,
    V: Num + AddAssign,
{
    fn increment_by_starting_at(&mut self, key: K, step: V, start: V) {
        *self.entry(key).or_insert(start) += step;
    }
}
