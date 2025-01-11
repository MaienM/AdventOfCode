use std::{collections::HashMap, hash::Hash};

use super::Counter;

/// Count the number of occurences of each element in a collection.
pub trait CountOccurences<T> {
    /// Count how often each item occurs.
    fn count_occurences(self) -> HashMap<T, usize>
    where
        T: Eq + PartialEq + Hash;
}
impl<I, T> CountOccurences<T> for I
where
    I: Iterator<Item = T>,
{
    fn count_occurences(self) -> HashMap<T, usize>
    where
        T: Eq + PartialEq + Hash,
    {
        let mut map = HashMap::new();
        for item in self {
            map.increment_one(item);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

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
