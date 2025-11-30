use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use derive_new::new;
use itertools::{ConsTuples, MapInto, Product, iproduct};
use num::Bounded;

use crate::{
    point::{Point2, Point3, Point4},
    prelude::*,
};

pub trait PointRange<P>: Debug {
    /// Returns `true` if `point` is contained in the range.
    ///
    /// # Examples.
    /// ```
    /// # use puzzle_lib::point::{Point2,Point2Range,PointRange};
    /// let range: Point2Range<_> = (Point2::new(1, 2)..Point2::new(4, 5)).into();
    /// assert!(range.contains(&Point2::new(1, 3)));
    /// assert!(!range.contains(&Point2::new(4, 3)));
    /// ```
    fn contains(&self, point: &P) -> bool;
}
pub trait WrappablePointRange<P>: PointRange<P> {
    /// Wrap the point to fall within the range.
    ///
    /// # Examples.
    ///
    /// ```
    /// # use puzzle_lib::point::{Point2,Point2Range,WrappablePointRange};
    /// let range: Point2Range<_> = (Point2::new(1, 2)..Point2::new(4, 5)).into();
    /// assert_eq!(range.wrap(Point2::new(1, 7)), Point2::new(1, 4));
    /// assert_eq!(range.wrap(Point2::new(0, 2)), Point2::new(3, 2));
    /// ```
    fn wrap(&self, point: P) -> P;
}

trait NormalizeRange<T> {
    /// Convert the range to a [`Range`].
    ///
    /// Note that since the resulting type exludes the top of the range any range that would
    /// normally include that value will exclude it after conversion. This is fine for our
    /// purposes, but it's not technically equivalent to the input.
    fn to_range(self) -> Range<T>;
}
impl<T> NormalizeRange<T> for Range<T> {
    fn to_range(self) -> Range<T> {
        self
    }
}
impl<T> NormalizeRange<T> for RangeInclusive<T>
where
    T: Copy + SaturatingAdd + One,
{
    fn to_range(self) -> Range<T> {
        *self.start()..self.end().saturating_add(&T::one())
    }
}
impl<T> NormalizeRange<T> for RangeFrom<T>
where
    T: Bounded,
{
    fn to_range(self) -> Range<T> {
        self.start..T::max_value()
    }
}
impl<T> NormalizeRange<T> for RangeTo<T>
where
    T: Bounded,
{
    fn to_range(self) -> Range<T> {
        T::min_value()..self.end
    }
}
impl<T> NormalizeRange<T> for RangeToInclusive<T>
where
    T: Bounded + SaturatingAdd + One,
{
    fn to_range(self) -> Range<T> {
        T::min_value()..self.end.saturating_add(&T::one())
    }
}
impl<T> NormalizeRange<T> for RangeFull
where
    T: Bounded + SaturatingAdd + One,
{
    fn to_range(self) -> Range<T> {
        T::min_value()..T::max_value()
    }
}

/// Generate the type that will be returned by the [`iproduct!`] macro.
macro_rules! iproduct_type {
    ([ $second:ty, $first:ty ];) => (Product<$first, $second>);
    ([ $last:ty, $($items:ty),+ ];) => {
        ConsTuples<
            Product<
                iproduct_type!([ $($items),+ ];),
                $last,
            >
        >
    };
    ([ $($items:ty),+ ]; $next:ty $(, $($rest:ty),+)?) => {
        iproduct_type!([ $next, $($items),+ ]; $($($rest),+)?)
    };
    ($first:ty, $($rest:ty),+ $(,)?) => {
        iproduct_type!([ $first ]; $($rest),+)
    };
}

macro_rules! create_point_range {
    (
        $(#[$structmeta:meta])*
        struct $name:ident for $pname:ident {
            $(
                $(#[$varmeta:meta])*
                $var:ident
            ),+
            $(,)?
        }
    ) => {
        $(#[$structmeta])*
        #[allow(clippy::redundant_field_names)]
        #[derive(Debug, Clone, Eq, Hash, PartialEq, new)]
        pub struct $name<T> {
            $(
                $(#[$varmeta])*
                pub $var: Range<T>
            ),+
        }

        impl<T> PointRange<$pname<T>> for $name<T>
        where
            T: PartialOrd<T> + Debug,
        {
            fn contains(&self, point: &$pname<T>) -> bool {
                $crate::op_chain!(&&, $(self.$var.contains(&point.$var)),+)
            }
        }

        impl<T> WrappablePointRange<$pname<T>> for $name<T>
        where
            Self: PointRange<$pname<T>>,
            T: PartialOrd<T>,
            Range<T>: WrapRange<T> + Debug,
        {
            fn wrap(&self, mut point: $pname<T>) -> $pname<T> {
                $(point.$var = self.$var.wrap(point.$var);)+
                point
            }
        }

        impl<T> IntoIterator for $name<T>
        where
            Self: PointRange<$pname<T>>,
            T: Clone,
            Range<T>: WrapRange<T> + IntoIterator<Item = T> + Debug,
            <Range<T> as IntoIterator>::IntoIter: Clone,
        {
            type Item = $pname<T>;
            type IntoIter = MapInto<
                iproduct_type!($($crate::static_!($var, <Range<T> as IntoIterator>::IntoIter)),+),
                $pname<T>,
            >;

            fn into_iter(self) -> Self::IntoIter {
                iproduct!($(self.$var),+).map_into::<$pname<T>>()
            }
        }

        impl<T> From<Range<$pname<T>>> for $name<T> {
            fn from(range: Range<$pname<T>>) -> Self {
                Self {
                    $($var: range.start.$var..range.end.$var),+
                }
            }
        }
        impl<T> From<RangeInclusive<$pname<T>>> for $name<T>
        where
            T: Copy,
            RangeInclusive<T>: NormalizeRange<T>,
        {
            fn from(range: RangeInclusive<$pname<T>>) -> Self {
                let start = range.start();
                let end = range.end();
                Self {
                    $($var: (start.$var..=end.$var).to_range()),+
                }
            }
        }
        impl<T> From<RangeFrom<$pname<T>>> for $name<T>
        where
            RangeFrom<T>: NormalizeRange<T>,
        {
            fn from(range: RangeFrom<$pname<T>>) -> Self {
                Self {
                    $($var: (range.start.$var..).to_range()),+
                }
            }
        }
        impl<T> From<RangeTo<$pname<T>>> for $name<T>
        where
            RangeTo<T>: NormalizeRange<T>,
        {
            fn from(range: RangeTo<$pname<T>>) -> Self {
                Self {
                    $($var: (..range.end.$var).to_range()),+
                }
            }
        }
        impl<T> From<RangeToInclusive<$pname<T>>> for $name<T>
        where
            RangeToInclusive<T>: NormalizeRange<T>,
        {
            fn from(range: RangeToInclusive<$pname<T>>) -> Self {
                Self {
                    $($var: (..=range.end.$var).to_range()),+
                }
            }
        }
        impl<T> From<RangeFull> for $name<T>
        where
            RangeFull: NormalizeRange<T>,
        {
            fn from(_: RangeFull) -> Self {
                Self {
                    $($var: (..).to_range()),+
                }
            }
        }
        paste::paste! {
            impl<T, $([<R $var:upper>]),+> From<($([<R $var:upper>]),+)> for $name<T>
            where
                $([<R $var:upper>]: NormalizeRange<T>),+
            {
                fn from(value: ($([<R $var:upper>]),+)) -> Self {
                    let ($($var),+) = value;
                    Self {
                        $($var: $var.to_range()),+
                    }
                }
            }
    }
    };
}

//
// 2-dimensional space.
//

create_point_range! {
    /// A range of points in 2-dimensional space.
    struct Point2Range for Point2 {
        /// The range in the first dimension.
        x,
        /// The range in the second dimension.
        y,
    }
}

//
// 3-dimensional space.
//

create_point_range! {
    /// A range of points in 3-dimensional space.
    struct Point3Range for Point3 {
        /// The range in the first dimension.
        x,
        /// The range in the second dimension.
        y,
        /// The range in the third dimension.
        z,
    }
}

//
// 4-dimensional space.
//
create_point_range! {
    /// A range of points in 4-dimensional space.
    struct Point4Range for Point4 {
        /// The range in the first dimension.
        x,
        /// The range in the second dimension.
        y,
        /// The range in the third dimension.
        z,
        /// The range in the fourth dimension.
        w,
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn range() {
        let range: Point2Range<_> = (Point2::new(1, 1)..Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
        assert_eq!(range.wrap(Point2::new(0, 7)), Point2::new(4, 3));
    }

    #[test]
    fn range_inclusive() {
        let range: Point2Range<_> = (Point2::new(1, 1)..=Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
        assert_eq!(range.wrap(Point2::new(0, 7)), Point2::new(5, 2));
    }

    #[test]
    fn range_from() {
        let range: Point2Range<_> = (Point2::new(1, 1)..).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_to() {
        let range: Point2Range<_> = (..Point2::new(5, 5)).into();
        assert!(range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_to_inclusive() {
        let range: Point2Range<_> = (..=Point2::new(5, 5)).into();
        assert!(range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_mixed() {
        let range: Point2Range<_> = (0..5, 0..=5).into();
        assert!(range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(range.contains(&Point2::new(4, 5)));
    }

    #[test]
    fn into_iter() {
        let range: Point2Range<_> = (Point2::new(0, 10)..Point2::new(2, 12)).into();
        let points = range.into_iter().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point2::new(0, 10),
                Point2::new(0, 11),
                Point2::new(1, 10),
                Point2::new(1, 11),
            ]
        );

        let range: Point3Range<_> = (Point3::new(0, 10, 20)..Point3::new(2, 12, 22)).into();
        let points = range.into_iter().collect::<Vec<_>>();
        assert_eq!(
            points,
            vec![
                Point3::new(0, 10, 20),
                Point3::new(0, 10, 21),
                Point3::new(0, 11, 20),
                Point3::new(0, 11, 21),
                Point3::new(1, 10, 20),
                Point3::new(1, 10, 21),
                Point3::new(1, 11, 20),
                Point3::new(1, 11, 21),
            ]
        );
    }
}
