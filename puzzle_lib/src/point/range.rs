use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use derive_new::new;

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
    /// let range: Point2Range<_, _> = (Point2::new(1, 2)..Point2::new(4, 5)).into();
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
    /// let range: Point2Range<_, _> = (Point2::new(1, 2)..Point2::new(4, 5)).into();
    /// assert_eq!(range.wrap(Point2::new(1, 7)), Point2::new(1, 4));
    /// assert_eq!(range.wrap(Point2::new(0, 2)), Point2::new(3, 2));
    /// ```
    fn wrap(&self, point: P) -> P;
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
        paste::paste! {
            $(#[$structmeta])*
            #[allow(clippy::redundant_field_names)]
            #[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, new)]
            pub struct $name<$([<R $var:upper>]),+> {
                $(
                    $(#[$varmeta])*
                    pub $var: [<R $var:upper>]
                ),+
            }
            impl<T, $([<R $var:upper>]),+> PointRange<$pname<T>> for $name<$([<R $var:upper>]),+>
            where
                T: PartialOrd<T>,
                $([<R $var:upper>]: RangeBounds<T> + Debug),+
            {
                fn contains(&self, point: &$pname<T>) -> bool {
                    $crate::op_chain!(&&, $(self.$var.contains(&point.$var)),+)
                }
            }
            impl<T, $([<R $var:upper>]),+> WrappablePointRange<$pname<T>> for $name<$([<R $var:upper>]),+>
            where
                Self: PointRange<$pname<T>>,
                T: PartialOrd<T>,
                $([<R $var:upper>]: WrapRange<T> + Debug),+
            {
                fn wrap(&self, mut point: $pname<T>) -> $pname<T> {
                    $(point.$var = self.$var.wrap(point.$var);)+
                    point
                }
            }

            impl<T> From<Range<$pname<T>>> for $name<$($crate::static_!($var, Range<T>)),+> {
                fn from(range: Range<$pname<T>>) -> Self {
                    Self {
                        $($var: (range.start.$var)..(range.end.$var)),+
                    }
                }
            }
            impl<T> From<RangeInclusive<$pname<T>>> for $name<$($crate::static_!($var, RangeInclusive<T>)),+>
            where
                T: Copy,
            {
                fn from(range: RangeInclusive<$pname<T>>) -> Self {
                    let start = range.start();
                    let end = range.end();
                    Self {
                        $($var: (start.$var)..=(end.$var)),+
                    }
                }
            }
            impl<T> From<RangeFrom<$pname<T>>> for $name<$($crate::static_!($var, RangeFrom<T>)),+> {
                fn from(range: RangeFrom<$pname<T>>) -> Self {
                    Self {
                        $($var: (range.start.$var)..),+
                    }
                }
            }
            impl<T> From<RangeTo<$pname<T>>> for $name<$($crate::static_!($var, RangeTo<T>)),+> {
                fn from(range: RangeTo<$pname<T>>) -> Self {
                    Self {
                        $($var: ..(range.end.$var)),+
                    }
                }
            }
            impl<T> From<RangeToInclusive<$pname<T>>> for $name<$($crate::static_!($var, RangeToInclusive<T>)),+> {
                fn from(range: RangeToInclusive<$pname<T>>) -> Self {
                    Self {
                        $($var: ..=(range.end.$var)),+
                    }
                }
            }
            impl From<RangeFull> for $name<$($crate::static_!($var, RangeFull)),+> {
                fn from(_: RangeFull) -> Self {
                    Self {
                        $($var: ..),+
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
        let range: Point2Range<_, _> = (Point2::new(1, 1)..Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
        assert_eq!(range.wrap(Point2::new(0, 7)), Point2::new(4, 3));
    }

    #[test]
    fn range_inclusive() {
        let range: Point2Range<_, _> = (Point2::new(1, 1)..=Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
        assert_eq!(range.wrap(Point2::new(0, 7)), Point2::new(5, 2));
    }

    #[test]
    fn range_from() {
        let range: Point2Range<_, _> = (Point2::new(1, 1)..).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_to() {
        let range: Point2Range<_, _> = (..Point2::new(5, 5)).into();
        assert!(range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_to_inclusive() {
        let range: Point2Range<_, _> = (..=Point2::new(5, 5)).into();
        assert!(range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
    }
}
