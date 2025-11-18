//! Helpers for points in n-dimensional space & related concepts.

use std::{
    collections::HashSet,
    fmt::Debug,
    hash::Hash,
    ops::{
        Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, RangeBounds, RangeFrom, RangeFull,
        RangeInclusive, RangeTo, RangeToInclusive, Sub, SubAssign,
    },
};

use derive_new::new;
use num::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, SaturatingAdd, SaturatingMul,
    SaturatingSub, WrappingAdd, WrappingMul, WrappingSub,
};

use crate::prelude::*;

// These traits don't exist (and they shouldn't, there's no point to them), but it makes the generation easier if they do, so we'll define them here and then never expose them.
#[allow(dead_code)]
trait SaturatingDiv {
    fn saturating_div(&self, v: &Self) -> Self;
}
#[allow(dead_code)]
trait WrappingDiv {
    fn wrapping_div(&self, v: &Self) -> Self;
}

//
// Point.
//

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

// Implements an operator (add/sub/mul/div) for a point type, including the assign, checked, saturating, and wrapping variants.
macro_rules! impl_point_operator {
    ($name:ident, $op:ident, $($var:ident),+) => {
        paste::paste! {
            // Regular.
            impl<T, R> [<$op:camel>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R>,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                type Output = Self;
                #[inline]
                fn [<$op>](self, rhs: $name<R>) -> Self {
                    Self {
                        $($var: self.$var.[<$op>](rhs.$var).into()),+
                    }
                }
            }
            // Assign.
            impl<T, R> [<$op:camel Assign>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R> + Copy,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                #[inline]
                fn [<$op _assign>](&mut self, rhs: $name<R>) {
                    *self = Self {
                        $($var: self.$var.[<$op>](rhs.$var).into()),+
                    };
                }
            }
            // Checked.
            impl<T> [<Checked $op:camel>] for $name<T>
            where
                T: [<Checked $op:camel>],
            {
                #[inline]
                fn [<checked_ $op>](&self, rhs: &Self) -> Option<Self> {
                    Some(Self {
                        $($var: self.$var.[<checked_ $op>](&rhs.$var)?),+
                    })
                }
            }
            // Saturating.
            impl<T> [<Saturating $op:camel>] for $name<T>
            where
                T: [<Saturating $op:camel>],
            {
                #[inline]
                fn [<saturating_ $op>](&self, rhs: &Self) -> Self {
                    Self {
                        $($var: self.$var.[<saturating_ $op>](&rhs.$var)),+
                    }
                }
            }
            // Wrapping.
            impl<T> [<Wrapping $op:camel>] for $name<T>
            where
                T: [<Wrapping $op:camel>],
            {
                #[inline]
                fn [<wrapping_ $op>](&self, rhs: &Self) -> Self {
                    Self {
                        $($var: self.$var.[<wrapping_ $op>](&rhs.$var)),+
                    }
                }
            }

            // Regular with scalar value.
            impl<T, R> [<$op:camel>]<R> for $name<T>
            where
                T: [<$op:camel>]<R>,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
                R: Num + Copy,
            {
                type Output = Self;
                #[inline]
                fn [<$op>](self, rhs: R) -> Self {
                    Self {
                        $($var: self.$var.[<$op>](rhs).into()),+
                    }
                }
            }
            // Assign with scalar value.
            impl<T, R> [<$op:camel Assign>]<R> for $name<T>
            where
                T: [<$op:camel>]<R> + Copy,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
                R: Num + Copy,
            {
                #[inline]
                fn [<$op _assign>](&mut self, rhs: R) {
                    *self = Self {
                        $($var: self.$var.[<$op>](rhs).into()),+
                    };
                }
            }
        }
    };
}

// Helper macro for neighbours() method.
macro_rules! impl_neighbor_diag_inner {
    (toplevel; $neighbours:ident, $base:expr, $var:ident) => {
        if let Some($var) = $base.$var.checked_sub(&T::one()) {
            $neighbours.insert(Self { $var, ..$base });
        }
        if let Some($var) = $base.$var.checked_add(&T::one()) {
            $neighbours.insert(Self { $var, ..$base });
        }
    };
    (nested; $neighbours:ident, $base:expr, $var:ident) => {
        impl_neighbor_diag_inner!(toplevel; $neighbours, $base, $var);
        $neighbours.insert($base);
    };

    ($type:ident; $neighbours:ident, $base:expr, $var:ident, $($vars:ident),*) => {
        if let Some($var) = $base.$var.checked_sub(&T::one()) {
            let base = Self { $var, ..$base };
            impl_neighbor_diag_inner!(nested; $neighbours, base, $($vars),*);
        }
        if let Some($var) = $base.$var.checked_add(&T::one()) {
            let base = Self { $var, ..$base };
            impl_neighbor_diag_inner!(nested; $neighbours, base, $($vars),*);
        }
        impl_neighbor_diag_inner!($type; $neighbours, $base, $($vars),*);
    };

    ($neighbours:ident, $base:expr, $($vars:ident),*) => {
        impl_neighbor_diag_inner!(toplevel; $neighbours, $base, $($vars),*);
    }
}

macro_rules! call_chain {
    ($fn:ident, $expr:expr $(,)?) => ($expr);
    ($fn:ident, $first:expr, $second:expr $(, $($exprs:expr),*)?) => {
        call_chain!($fn, $first.$fn($second) $(, $($exprs),*)?)
    };
}

macro_rules! and_chain {
    ($expr:expr $(,)?) => ($expr);
    ($expr:expr, $($exprs:expr),* $(,)?) => ($expr && and_chain!($($exprs),*));
}

macro_rules! expand_static {
    ($ignore:tt, $use:ty) => {
        $use
    };
}

// Generate a point class with the given name and variables.
macro_rules! create_point {
    (
        $(#[$structmeta:meta])*
        struct $name:ident {
            $(
                $(#[$varmeta:meta])*
                $var:ident
            ),+
            $(,)?
        }
    ) => {
        $(#[$structmeta])*
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, new)]
        pub struct $name<T = usize> {
            $(
                $(#[$varmeta])*
                pub $var: T
            ),+
        }

        impl_point_operator!($name, add, $($var),+);
        impl_point_operator!($name, sub, $($var),+);
        impl_point_operator!($name, mul, $($var),+);
        impl_point_operator!($name, div, $($var),+);

        impl<T> $name<T>
        where
            T: Copy
        {
            /// Convert to another type of point.
            ///
            /// # Examples.
            /// ```
            /// # use puzzle_lib::point::Point2;
            /// let point = Point2::new(1u8, 1);
            /// assert_eq!(point.cast(), Point2::new(1u16, 1));
            /// ```
            pub fn cast<O>(&self) -> $name<O>
            where
                O: From<T>
            {
                $name {
                    $($var: self.$var.into()),+
                }
            }

            /// Try to convert to another type of point.
            ///
            /// # Examples.
            /// ```
            /// # use puzzle_lib::point::Point2;
            /// let point = Point2::new(-1i8, 1);
            /// assert_eq!(point.try_cast(), Ok(Point2::new(-1i16, 1)));
            /// assert!(point.try_cast::<u8>().is_err());
            /// ```
            ///
            /// # Errors
            ///
            /// Will fail if the conversion of any of the point's coordinates fails.
            pub fn try_cast<O>(&self) -> Result<$name<O>, <O as TryFrom<T>>::Error>
            where
                O: TryFrom<T>
            {
                Ok($name {
                    $($var: self.$var.try_into()?),+
                })
            }
        }

        impl<'a, T> AbsDiff<&'a $name<T>> for &'a $name<T>
        where
            T: Copy + AbsDiff<T>,
        {
            type Output = $name<<T as AbsDiff<T>>::Output>;

            /// Get a point that represents the absolute differences of all coordinates of the two points.
            fn abs_diff(self, other: Self) -> Self::Output {
                Self::Output {
                    $($var: T::abs_diff(self.$var, other.$var)),+
                }
            }
        }

        impl<T> $name<T>
        where
            T: Copy + Add<T, Output = T>
        {
            /// Calculate the sum of all coordinates of the point.
            #[must_use]
            pub fn sum(&self) -> T {
                call_chain!(add, $(self.$var),+)
            }
        }
        impl<'a, T> $name<T>
        where
            T: Copy + AbsDiff<T> + PartialEq + 'a,
            <T as AbsDiff<T>>::Output: Copy + Add<<T as AbsDiff<T>>::Output, Output = <T as AbsDiff<T>>::Output> + Ord + One,
        {
            /// Calculate the distance between this point and another point.
            ///
            /// Diagonals are counted as a distance of two.
            #[must_use]
            pub fn distance_ortho(&'a self, other: &'a Self) -> <T as AbsDiff<T>>::Output {
                self.abs_diff(other).sum()
            }

            /// Calculate the distance between this point and another point.
            ///
            /// Diagonals are counted as a distance of one.
            #[must_use]
            pub fn distance_diag(&'a self, other: &'a Self) -> <T as AbsDiff<T>>::Output {
                let diff = self.abs_diff(other);
                call_chain!(max, $(diff.$var),+)
            }

            /// Check whether the given point is orthogontally adjacent to this one.
            pub fn adjacent_to_ortho(&'a self, other: &'a Self) -> bool {
                self.abs_diff(other).sum() == <T as AbsDiff<T>>::Output::one()
            }

            /// Check whether the given point is orthogontally or diagonally adjacent to this one.
            pub fn adjacent_to_diag(&'a self, other: &'a Self) -> bool {
                self != other && self.distance_diag(other) == <T as AbsDiff<T>>::Output::one()
            }
        }
        impl<T> $name<T>
        where
            T: Copy + Add<T, Output = T> + Sub<T, Output = T> + PartialOrd<T> + Ord + One + CheckedAdd + CheckedSub + Hash,
        {
            /// Get the orthogontal neighbours of this point.
            pub fn neighbours_ortho(&self) -> HashSet<Self> {
                let mut neighbours = HashSet::new();
                $(
                    if let Some($var) = self.$var.checked_sub(&T::one()) {
                        neighbours.insert(Self { $var, ..*self });
                    }
                    if let Some($var) = self.$var.checked_add(&T::one()) {
                        neighbours.insert(Self { $var, ..*self });
                    }
                )+
                neighbours
            }

            /// Get the orthogontal & diagonal neighbours of this point.
            pub fn neighbours_diag(&self) -> HashSet<Self> {
                let mut neighbours = HashSet::new();
                impl_neighbor_diag_inner!(neighbours, *self, $($var),+);
                neighbours
            }
        }

        paste::paste! {
            #[doc = "A range of [`crate::point::" $name "`]."]
            #[allow(clippy::redundant_field_names)]
            #[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, new)]
            pub struct [<$name Range>]<$([<R $var:upper>]),+> {
                $(
                    pub $var: [<R $var:upper>]
                ),+
            }
            impl<T, $([<R $var:upper>]),+> PointRange<$name<T>> for [<$name Range>]<$([<R $var:upper>]),+>
            where
                T: PartialOrd<T>,
                $([<R $var:upper>]: RangeBounds<T> + Debug),+
            {
                fn contains(&self, point: &$name<T>) -> bool {
                    and_chain!($(self.$var.contains(&point.$var)),+)
                }
            }

            impl<T> From<Range<$name<T>>> for [<$name Range>]<$(expand_static!($var, Range<T>)),+> {
                fn from(range: Range<$name<T>>) -> Self {
                    Self {
                        $($var: (range.start.$var)..(range.end.$var)),+
                    }
                }
            }
            impl<T> From<RangeInclusive<$name<T>>> for [<$name Range>]<$(expand_static!($var, RangeInclusive<T>)),+>
            where
                T: Copy,
            {
                fn from(range: RangeInclusive<$name<T>>) -> Self {
                    let start = range.start();
                    let end = range.end();
                    Self {
                        $($var: (start.$var)..=(end.$var)),+
                    }
                }
            }
            impl<T> From<RangeFrom<$name<T>>> for [<$name Range>]<$(expand_static!($var, RangeFrom<T>)),+> {
                fn from(range: RangeFrom<$name<T>>) -> Self {
                    Self {
                        $($var: (range.start.$var)..),+
                    }
                }
            }
            impl<T> From<RangeTo<$name<T>>> for [<$name Range>]<$(expand_static!($var, RangeTo<T>)),+> {
                fn from(range: RangeTo<$name<T>>) -> Self {
                    Self {
                        $($var: ..(range.end.$var)),+
                    }
                }
            }
            impl<T> From<RangeToInclusive<$name<T>>> for [<$name Range>]<$(expand_static!($var, RangeToInclusive<T>)),+> {
                fn from(range: RangeToInclusive<$name<T>>) -> Self {
                    Self {
                        $($var: ..=(range.end.$var)),+
                    }
                }
            }
            impl From<RangeFull> for [<$name Range>]<$(expand_static!($var, RangeFull)),+> {
                fn from(_: RangeFull) -> Self {
                    Self {
                        $($var: ..),+
                    }
                }
            }
        }
    };
}

create_point!(
    /// A point in 2-dimensional space.
    struct Point2 {
        /// The coordinate in the first dimension.
        x,
        /// The coordinate in the second dimension.
        y,
    }
);
create_point!(
    /// A point in 3-dimensional space.
    struct Point3 {
        /// The coordinate in the first dimension.
        x,
        /// The coordinate in the second dimension.
        y,
        /// The coordinate in the third dimension.
        z,
    }
);
create_point!(
    /// A point in 4-dimensional space.
    struct Point4 {
        /// The coordinate in the first dimension.
        x,
        /// The coordinate in the second dimension.
        y,
        /// The coordinate in the third dimension.
        z,
        /// The coordinate in the fourth dimension.
        w,
    }
);

impl<T> Debug for Point2<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Point(")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(")")
    }
}
impl<T> Debug for Point3<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Point(")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.z.fmt(f)?;
        f.write_str(")")
    }
}
impl<T> Debug for Point4<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Point(")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.z.fmt(f)?;
        f.write_str(", ")?;
        self.w.fmt(f)?;
        f.write_str(")")
    }
}

//
// Direction
//

/// A helper struct combining a direction with a magnitude
///
/// This is used when taking multiple steps in a direction at one time.
pub struct DirectionWithMagnitude<D, T>(D, T);

// Implements an operator (add/sub) for a point type combined with a direction (optionally multiplied by some amount), including the assign, checked, saturating, and wrapping variants.
macro_rules! impl_direction_operator {
    (call; $expr:expr; [$($prefix:ident)? ($method:ident) $($suffix:ident)?]($($arg:expr),*)) => {
        paste::paste!($expr.[<$($prefix)? $method $($suffix)?>]($($arg),*))
    };
    (call; $expr:expr; [$($prefix:ident)? (add; add) $($suffix:ident)?]($($arg:expr),*)) => {
        impl_direction_operator!(call; $expr; [$($prefix)? (add) $($suffix)?]($($arg),*))
    };
    (call; $expr:expr; [$($prefix:ident)? (add; sub) $($suffix:ident)?]($($arg:expr),*)) => {
        impl_direction_operator!(call; $expr; [$($prefix)? (sub) $($suffix)?]($($arg),*))
    };
    (call; $expr:expr; [$($prefix:ident)? (sub; add) $($suffix:ident)?]($($arg:expr),*)) => {
        impl_direction_operator!(call; $expr; [$($prefix)? (sub) $($suffix)?]($($arg),*))
    };
    (call; $expr:expr; [$($prefix:ident)? (sub; sub) $($suffix:ident)?]($($arg:expr),*)) => {
        impl_direction_operator!(call; $expr; [$($prefix)? (add) $($suffix)?]($($arg),*))
    };

    ($name:ty, $point:ident, $op:ident, $($member:ident = $($var:ident $method:ident)+),+ $(,)?) => {
        paste::paste! {
            //
            // Point & direction.
            //

            // Regular.
            impl<T> [<$op:camel>]<$name> for $point<T>
            where
                T: Add<T> + Sub<T> + One,
                <T as Add<T>>::Output: Into<T>,
                <T as Sub<T>>::Output: Into<T>,
            {
                type Output = Self;
                #[inline]
                fn [<$op>](self, direction: $name) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [($op; $method)](T::one())).into()),+,
                                ..self
                            },
                        )+
                    }
                }
            }
            // Assign.
            impl<T> [<$op:camel Assign>]<$name> for $point<T>
            where
                T: AddAssign<T> + SubAssign<T> + One,
            {
                #[inline]
                fn [<$op _assign>](&mut self, direction: $name) {
                    match direction {
                        $(
                            $name::$member => {
                                $(impl_direction_operator!(call; self.$var; [($op; $method) _assign](T::one()));)+
                            },
                        )+
                    }
                }
            }
            // Checked.
            impl<T> $point<T>
            where
                T: CheckedAdd + CheckedSub + One + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<checked_ $op _ $name:lower>](&self, direction: $name) -> Option<Self> {
                    Some(match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [checked_ ($op; $method)](&T::one()))?),+,
                                ..*self
                            },
                        )+
                    })
                }
            }
            // Saturating.
            impl<T> $point<T>
            where
                T: SaturatingAdd + SaturatingSub + One + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<saturating_ $op _ $name:lower>](&self, direction: $name) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [saturating_ ($op; $method)](&T::one()))),+,
                                ..*self
                            },
                        )+
                    }
                }
            }
            // Wrapping.
            impl<T> $point<T>
            where
                T: WrappingAdd + WrappingSub + One + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<wrapping_ $op _ $name:lower>](&self, direction: $name) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [wrapping_ ($op; $method)](&T::one()))),+,
                                ..*self
                            },
                        )+
                    }
                }
            }

            //
            // Point & Direction with magnitude.
            //

            // Regular.
            impl<T, R> [<$op:camel>]<DirectionWithMagnitude<$name, R>> for $point<T>
            where
                T: Add<R> + Sub<R>,
                <T as Add<R>>::Output: Into<T>,
                <T as Sub<R>>::Output: Into<T>,
                R: Copy,
            {
                type Output = Self;
                #[inline]
                fn [<$op>](self, DirectionWithMagnitude(direction, magnitude): DirectionWithMagnitude<$name, R>) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [($op; $method)](magnitude)).into()),+,
                                ..self
                            },
                        )+
                    }
                }
            }
            // Assign.
            impl<T, R> [<$op:camel Assign>]<DirectionWithMagnitude<$name, R>> for $point<T>
            where
                T: AddAssign<R> + SubAssign<R>,
                R: Copy,
            {
                #[inline]
                fn [<$op _assign>](&mut self, DirectionWithMagnitude(direction, magnitude): DirectionWithMagnitude<$name, R>) {
                    match direction {
                        $(
                            $name::$member => {
                                $(impl_direction_operator!(call; self.$var; [($op; $method) _assign](magnitude));)+
                            },
                        )+
                    }
                }
            }
            // Checked.
            impl<T> $point<T>
            where
                T: CheckedAdd + CheckedSub + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<checked_ $op _ $name:lower _magnitude>](&self, DirectionWithMagnitude(direction, magnitude): DirectionWithMagnitude<$name, T>) -> Option<Self> {
                    Some(match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [checked_ ($op; $method)](&magnitude))?),+,
                                ..*self
                            },
                        )+
                    })
                }
            }
            // Saturating.
            impl<T> $point<T>
            where
                T: SaturatingAdd + SaturatingSub + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<saturating_ $op _ $name:lower _magnitude>](&self, DirectionWithMagnitude(direction, magnitude): DirectionWithMagnitude<$name, T>) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [saturating_ ($op; $method)](&magnitude))),+,
                                ..*self
                            },
                        )+
                    }
                }
            }
            // Wrapping.
            impl<T> $point<T>
            where
                T: WrappingAdd + WrappingSub + Copy,
            {
                #[must_use]
                #[inline]
                pub fn [<wrapping_ $op _ $name:lower _magnitude>](&self, DirectionWithMagnitude(direction, magnitude): DirectionWithMagnitude<$name, T>) -> Self {
                    match direction {
                        $(
                            #[allow(clippy::needless_update)]
                            $name::$member => Self {
                                $($var: impl_direction_operator!(call; self.$var; [wrapping_ ($op; $method)](&magnitude))),+,
                                ..*self
                            },
                        )+
                    }
                }
            }
        }
    };
}

macro_rules! create_direction {
    (
        $(#[$enummeta:meta])*
        enum $name:ident for $point:ident {
            $(
                $(#[$membermeta:meta])*
                $member:ident = $($var:ident $method:ident)+
            ),+
            $(,)?
        }
    ) => {
        $(#[$enummeta])*
        #[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $(
                $(#[$membermeta])*
                $member
            ),+
        }

        impl<T> Mul<T> for $name {
            type Output = DirectionWithMagnitude<$name, T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                DirectionWithMagnitude(self, rhs)
            }
        }

        impl_direction_operator!($name, $point, add, $($member = $($var $method)+),+);
        impl_direction_operator!($name, $point, sub, $($member = $($var $method)+),+);
    };
}

create_direction! {
    /// An orthogontal direction in 2-dimensional space.
    enum Direction2 for Point2 {
        /// Movement in the X dimension towards positive infinity.
        East = x add,
        /// Movement in the X dimension towards negative infinity.
        West = x sub,
        /// Movement in the Y dimension towards positive infinity.
        South = y add,
        /// Movement in the Y dimension towards negative infinity.
        North = y sub,
    }
}
create_direction! {
    /// An orthogontal or diagonal direction in 2-dimensional space.
    enum Direction2X for Point2 {
        /// Movement in the X dimension towards negative infinity.
        West = x sub,
        /// Movement in the X & Y dimension towards negative infinity.
        NorthWest = y sub x sub,
        /// Movement in the Y dimension towards negative infinity.
        North = y sub,
        /// Movement in the X dimension towards positive infinity and in the Y dimension towards negative infinity.
        NorthEast = y sub x add,
        /// Movement in the X dimension towards positive infinity.
        East = x add,
        /// Movement in the X & Y dimension towards positive infinity.
        SouthEast = y add x add,
        /// Movement in the Y dimension towards positive infinity.
        South = y add,
        /// Movement in the X dimension towards negative infinity and in the Y dimension towards positive infinity.
        SouthWest = y add x sub,
    }
}
create_direction! {
    /// An orthogontal direction in 3-dimensional space.
    enum Direction3 for Point3 {
        /// Movement in the X dimension towards positive infinity.
        Right = x add,
        /// Movement in the X dimension towards negative infinity.
        Left = x sub,
        /// Movement in the Y dimension towards positive infinity.
        Back = y add,
        /// Movement in the Y dimension towards negative infinity.
        Front = y sub,
        /// Movement in the Z dimension towards positive infinity.
        Up = z add,
        /// Movement in the Z dimension towards negative infinity.
        Down = z sub,
    }
}
create_direction! {
    /// An orthogontal direction in 4-dimensional space.
    enum Direction4 for Point4 {
        /// Movement in the X dimension towards positive infinity.
        Right = x add,
        /// Movement in the X dimension towards negative infinity.
        Left = x sub,
        /// Movement in the Y dimension towards positive infinity.
        Back = y add,
        /// Movement in the Y dimension towards negative infinity.
        Front = y sub,
        /// Movement in the Z dimension towards positive infinity.
        Up = z add,
        /// Movement in the Z dimension towards negative infinity.
        Down = z sub,
        /// Movement in the W dimension towards positive infinity.
        Ana = w add,
        /// Movement in the W dimension towards negative infinity.
        Kata = w sub,
    }
}

#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn add() {
        assert_eq!(Point2::new(10, 5) + Point2::new(8, 7), Point2::new(18, 12));
        assert_eq!(
            Point3::new(10, 5, 7) + Point3::new(8, 7, 1),
            Point3::new(18, 12, 8)
        );
    }

    #[test]
    fn add_assign() {
        let mut point = Point2::new(10, 5);
        point += Point2::new(8, 7);
        assert_eq!(point, Point2::new(18, 12));

        let mut point = Point3::new(10, 5, 7);
        point += Point3::new(8, 7, 1);
        assert_eq!(point, Point3::new(18, 12, 8));
    }

    #[test]
    fn add_checked() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(
            point.checked_add(&Point2::new(8, 2)),
            Some(Point2::new(18, 7))
        );
        assert_eq!(point.checked_add(&Point2::new(8, 254)), None);

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.checked_add(&Point3::new(8, 2, 4)),
            Some(Point3::new(18, 7, 11))
        );
        assert_eq!(point.checked_add(&Point3::new(8, 254, 4)), None);
    }

    #[test]
    fn add_saturating() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(point.saturating_add(&Point2::new(8, 2)), Point2::new(18, 7));
        assert_eq!(
            point.saturating_add(&Point2::new(8, 254)),
            Point2::new(18, 255)
        );

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.saturating_add(&Point3::new(8, 2, 4)),
            Point3::new(18, 7, 11)
        );
        assert_eq!(
            point.saturating_add(&Point3::new(8, 254, 4)),
            Point3::new(18, 255, 11)
        );
    }

    #[test]
    fn add_wrapping() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(point.wrapping_add(&Point2::new(8, 2)), Point2::new(18, 7));
        assert_eq!(point.wrapping_add(&Point2::new(8, 254)), Point2::new(18, 3));

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.wrapping_add(&Point3::new(8, 2, 4)),
            Point3::new(18, 7, 11)
        );
        assert_eq!(
            point.wrapping_add(&Point3::new(8, 254, 4)),
            Point3::new(18, 3, 11)
        );
    }

    #[test]
    fn sub() {
        assert_eq!(Point2::new(10, 5) - Point2::new(8, 2), Point2::new(2, 3));
        assert_eq!(
            Point3::new(10, 5, 7) - Point3::new(8, 2, 1),
            Point3::new(2, 3, 6)
        );
    }

    #[test]
    fn sub_assign() {
        let mut point = Point2::new(10, 5);
        point -= Point2::new(8, 2);
        assert_eq!(point, Point2::new(2, 3));

        let mut point = Point3::new(10, 5, 7);
        point -= Point3::new(8, 2, 1);
        assert_eq!(point, Point3::new(2, 3, 6));
    }

    #[test]
    fn sub_checked() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(
            point.checked_sub(&Point2::new(8, 2)),
            Some(Point2::new(2, 3))
        );
        assert_eq!(point.checked_sub(&Point2::new(8, 7)), None);

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.checked_sub(&Point3::new(8, 2, 4)),
            Some(Point3::new(2, 3, 3))
        );
        assert_eq!(point.checked_sub(&Point3::new(8, 7, 4)), None);
    }

    #[test]
    fn sub_saturating() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(point.saturating_sub(&Point2::new(8, 2)), Point2::new(2, 3));
        assert_eq!(point.saturating_sub(&Point2::new(8, 7)), Point2::new(2, 0));

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.saturating_sub(&Point3::new(8, 2, 4)),
            Point3::new(2, 3, 3)
        );
        assert_eq!(
            point.saturating_sub(&Point3::new(8, 7, 4)),
            Point3::new(2, 0, 3)
        );
    }

    #[test]
    fn sub_wrapping() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(point.wrapping_sub(&Point2::new(8, 2)), Point2::new(2, 3));
        assert_eq!(point.wrapping_sub(&Point2::new(8, 7)), Point2::new(2, 254));

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.wrapping_sub(&Point3::new(8, 2, 4)),
            Point3::new(2, 3, 3)
        );
        assert_eq!(
            point.wrapping_sub(&Point3::new(8, 7, 4)),
            Point3::new(2, 254, 3)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(Point2::new(10, 5) * Point2::new(2, 3), Point2::new(20, 15));
        assert_eq!(
            Point3::new(10, 5, 7) * Point3::new(2, 3, 4),
            Point3::new(20, 15, 28)
        );
    }

    #[test]
    fn mul_assign() {
        let mut point = Point2::new(10, 5);
        point *= Point2::new(2, 3);
        assert_eq!(point, Point2::new(20, 15));

        let mut point = Point3::new(10, 5, 7);
        point *= Point3::new(2, 3, 4);
        assert_eq!(point, Point3::new(20, 15, 28));
    }

    #[test]
    fn mul_checked() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(
            point.checked_mul(&Point2::new(8, 2)),
            Some(Point2::new(80, 10))
        );
        assert_eq!(point.checked_mul(&Point2::new(8, 100)), None);

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.checked_mul(&Point3::new(8, 2, 4)),
            Some(Point3::new(80, 10, 28))
        );
        assert_eq!(point.checked_mul(&Point3::new(8, 100, 4)), None);
    }

    #[test]
    fn mul_saturating() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(
            point.saturating_mul(&Point2::new(8, 2)),
            Point2::new(80, 10)
        );
        assert_eq!(
            point.saturating_mul(&Point2::new(8, 100)),
            Point2::new(80, 255)
        );

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.saturating_mul(&Point3::new(8, 2, 4)),
            Point3::new(80, 10, 28)
        );
        assert_eq!(
            point.saturating_mul(&Point3::new(8, 100, 4)),
            Point3::new(80, 255, 28)
        );
    }

    #[test]
    fn mul_wrapping() {
        let point = Point2::<u8>::new(10, 5);
        assert_eq!(point.wrapping_mul(&Point2::new(8, 2)), Point2::new(80, 10));
        assert_eq!(
            point.wrapping_mul(&Point2::new(8, 100)),
            Point2::new(80, 244)
        );

        let point = Point3::<u8>::new(10, 5, 7);
        assert_eq!(
            point.wrapping_mul(&Point3::new(8, 2, 4)),
            Point3::new(80, 10, 28)
        );
        assert_eq!(
            point.wrapping_mul(&Point3::new(8, 100, 4)),
            Point3::new(80, 244, 28)
        );
    }

    #[test]
    fn div() {
        assert_eq!(Point2::new(20, 15) / Point2::new(2, 3), Point2::new(10, 5));
        assert_eq!(
            Point3::new(20, 15, 28) / Point3::new(2, 3, 4),
            Point3::new(10, 5, 7)
        );
    }

    #[test]
    fn div_assign() {
        let mut point = Point2::new(20, 15);
        point /= Point2::new(2, 3);
        assert_eq!(point, Point2::new(10, 5));

        let mut point = Point3::new(20, 15, 28);
        point /= Point3::new(2, 3, 4);
        assert_eq!(point, Point3::new(10, 5, 7));
    }

    #[test]
    fn div_checked() {
        let point = Point2::<u8>::new(10, 9);
        assert_eq!(
            point.checked_div(&Point2::new(2, 3)),
            Some(Point2::new(5, 3))
        );
        assert_eq!(
            point.checked_div(&Point2::new(3, 2)),
            Some(Point2::new(3, 4))
        );
        assert_eq!(point.checked_div(&Point2::new(8, 0)), None);

        let point = Point3::<u8>::new(10, 9, 7);
        assert_eq!(
            point.checked_div(&Point3::new(2, 3, 1)),
            Some(Point3::new(5, 3, 7))
        );
        assert_eq!(
            point.checked_div(&Point3::new(3, 2, 4)),
            Some(Point3::new(3, 4, 1))
        );
        assert_eq!(point.checked_div(&Point3::new(8, 0, 1)), None);
    }

    #[test]
    fn sum() {
        assert_eq!(Point2::new(10, 5).sum(), 15);
        assert_eq!(Point2::new(10, -5).sum(), 5);
        assert_eq!(Point3::new(10, 5, 8).sum(), 23);
        assert_eq!(Point3::new(10, -5, 3).sum(), 8);
    }

    #[test]
    fn abs_diff() {
        assert_eq!(
            Point2::new(10i8, 5).abs_diff(&Point2::new(2, 20)),
            Point2::new(8, 15)
        );
        assert_eq!(
            Point3::new(10i8, 5, 3).abs_diff(&Point3::new(2, 20, -3)),
            Point3::new(8, 15, 6)
        );
    }

    #[test]
    fn distance_ortho() {
        assert_eq!(Point2::new(10i8, 5).distance_ortho(&Point2::new(2, 20)), 23);
        assert_eq!(
            Point3::new(10i8, 5, 3).distance_ortho(&Point3::new(2, 20, -3)),
            29
        );
    }

    #[test]
    fn distance_diag() {
        assert_eq!(Point2::new(10i8, 5).distance_diag(&Point2::new(2, 20)), 15);
        assert_eq!(
            Point3::new(10i8, 5, 3).distance_diag(&Point3::new(2, 20, -3)),
            15
        );
    }

    #[test]
    fn adjacent_to_ortho() {
        let point: Point2<u8> = Point2::new(10, 5);

        assert_eq!(point.adjacent_to_ortho(&Point2::new(10, 4)), true);
        assert_eq!(point.adjacent_to_ortho(&Point2::new(10, 6)), true);
        assert_eq!(point.adjacent_to_ortho(&Point2::new(9, 5)), true);

        assert_eq!(point.adjacent_to_ortho(&point), false);
        assert_eq!(point.adjacent_to_ortho(&Point2::new(9, 4)), false);
        assert_eq!(point.adjacent_to_ortho(&Point2::new(10, 3)), false);

        let point: Point3<u8> = Point3::new(10, 5, 8);

        assert_eq!(point.adjacent_to_ortho(&Point3::new(10, 5, 7)), true);
        assert_eq!(point.adjacent_to_ortho(&Point3::new(10, 6, 8)), true);
        assert_eq!(point.adjacent_to_ortho(&Point3::new(9, 5, 8)), true);

        assert_eq!(point.adjacent_to_ortho(&point), false);
        assert_eq!(point.adjacent_to_ortho(&Point3::new(11, 6, 8)), false);
        assert_eq!(point.adjacent_to_ortho(&Point3::new(12, 5, 8)), false);
    }

    #[test]
    fn adjacent_to_diag() {
        let point: Point2<u8> = Point2::new(10, 5);

        assert_eq!(point.adjacent_to_diag(&Point2::new(10, 4)), true);
        assert_eq!(point.adjacent_to_diag(&Point2::new(10, 6)), true);
        assert_eq!(point.adjacent_to_diag(&Point2::new(9, 5)), true);
        assert_eq!(point.adjacent_to_diag(&Point2::new(9, 4)), true);

        assert_eq!(point.adjacent_to_diag(&point), false);
        assert_eq!(point.adjacent_to_diag(&Point2::new(10, 3)), false);

        let point: Point3<u8> = Point3::new(10, 5, 8);

        assert_eq!(point.adjacent_to_diag(&Point3::new(10, 5, 7)), true);
        assert_eq!(point.adjacent_to_diag(&Point3::new(10, 6, 8)), true);
        assert_eq!(point.adjacent_to_diag(&Point3::new(9, 5, 8)), true);
        assert_eq!(point.adjacent_to_diag(&Point3::new(11, 6, 8)), true);

        assert_eq!(point.adjacent_to_diag(&point), false);
        assert_eq!(point.adjacent_to_diag(&Point3::new(12, 5, 8)), false);
    }

    macro_rules! assert_eq_points {
        (sort; $set:expr) => {
            {
                let mut list: Vec<_> = $set.into_iter().collect();
                list.sort_unstable();
                list
            }
        };
        ($actual:expr, $expected:expr $(,)?) => {
            assert_eq!(
                assert_eq_points!(sort; $actual),
                assert_eq_points!(sort; $expected),
            );
        };
    }

    #[test]
    fn neighbours_ortho() {
        assert_eq_points!(
            Point2::<u8>::new(10, 5).neighbours_ortho(),
            hash_set![
                Point2::new(9, 5),
                Point2::new(10, 4),
                Point2::new(10, 6),
                Point2::new(11, 5),
            ]
        );
        assert_eq_points!(
            Point2::<u8>::new(0, 255).neighbours_ortho(),
            hash_set![Point2::new(1, 255), Point2::new(0, 254)]
        );

        assert_eq_points!(
            Point3::<u8>::new(10, 5, 8).neighbours_ortho(),
            hash_set![
                Point3::new(9, 5, 8),
                Point3::new(10, 4, 8),
                Point3::new(10, 5, 7),
                Point3::new(10, 5, 9),
                Point3::new(10, 6, 8),
                Point3::new(11, 5, 8),
            ]
        );
        assert_eq_points!(
            Point3::<u8>::new(0, 5, 255).neighbours_ortho(),
            hash_set![
                Point3::new(0, 4, 255),
                Point3::new(0, 5, 254),
                Point3::new(0, 6, 255),
                Point3::new(1, 5, 255),
            ]
        );
    }

    #[test]
    fn neighbours_diag() {
        assert_eq_points!(
            Point2::<u8>::new(10, 5).neighbours_diag(),
            hash_set![
                Point2::new(9, 4),
                Point2::new(9, 5),
                Point2::new(9, 6),
                Point2::new(10, 4),
                Point2::new(10, 6),
                Point2::new(11, 4),
                Point2::new(11, 5),
                Point2::new(11, 6),
            ]
        );
        assert_eq_points!(
            Point2::<u8>::new(0, 255).neighbours_diag(),
            hash_set![
                Point2::new(0, 254),
                Point2::new(1, 254),
                Point2::new(1, 255),
            ]
        );

        assert_eq_points!(
            Point3::<u8>::new(10, 5, 8).neighbours_diag(),
            hash_set![
                Point3::new(9, 4, 7),
                Point3::new(9, 4, 8),
                Point3::new(9, 4, 9),
                Point3::new(9, 5, 7),
                Point3::new(9, 5, 8),
                Point3::new(9, 5, 9),
                Point3::new(9, 6, 7),
                Point3::new(9, 6, 8),
                Point3::new(9, 6, 9),
                Point3::new(10, 4, 7),
                Point3::new(10, 4, 8),
                Point3::new(10, 4, 9),
                Point3::new(10, 5, 7),
                Point3::new(10, 5, 9),
                Point3::new(10, 6, 7),
                Point3::new(10, 6, 8),
                Point3::new(10, 6, 9),
                Point3::new(11, 4, 7),
                Point3::new(11, 4, 8),
                Point3::new(11, 4, 9),
                Point3::new(11, 5, 7),
                Point3::new(11, 5, 8),
                Point3::new(11, 5, 9),
                Point3::new(11, 6, 7),
                Point3::new(11, 6, 8),
                Point3::new(11, 6, 9),
            ]
        );
        assert_eq_points!(
            Point3::<u8>::new(0, 5, 255).neighbours_diag(),
            hash_set![
                Point3::new(0, 4, 254),
                Point3::new(0, 4, 255),
                Point3::new(0, 5, 254),
                Point3::new(0, 6, 254),
                Point3::new(0, 6, 255),
                Point3::new(1, 4, 254),
                Point3::new(1, 4, 255),
                Point3::new(1, 5, 254),
                Point3::new(1, 5, 255),
                Point3::new(1, 6, 254),
                Point3::new(1, 6, 255),
            ]
        );
    }

    #[test]
    fn format() {
        assert_eq!(format!("{:?}", Point2::new(10, 12)), "Point(10, 12)");
        assert_eq!(format!("{:?}", Point3::new(10, 12, 3)), "Point(10, 12, 3)");
        assert_eq!(
            format!("{:?}", Point4::new(10, 12, 3, 80)),
            "Point(10, 12, 3, 80)"
        );

        assert_eq!(
            format!("{:.1?}", Point2::new(10.123, 12.0)),
            "Point(10.1, 12.0)"
        );
    }

    #[test]
    fn add_direction() {
        assert_eq!(Point2::new(10, 5) + Direction2::East, Point2::new(11, 5));
        assert_eq!(Point2::new(10, 5) + Direction2::West, Point2::new(9, 5));
        assert_eq!(
            Point3::new(10, 5, 7) + Direction3::Up,
            Point3::new(10, 5, 8)
        );
        assert_eq!(
            Point3::new(10, 5, 7) + Direction3::Down,
            Point3::new(10, 5, 6)
        );
    }

    #[test]
    fn add_assign_direction() {
        let mut point = Point2::new(10, 5);
        point += Direction2::East;
        assert_eq!(point, Point2::new(11, 5));
        point += Direction2::North;
        assert_eq!(point, Point2::new(11, 4));

        let mut point = Point3::new(10, 5, 7);
        point += Direction3::Up;
        assert_eq!(point, Point3::new(10, 5, 8));
        point += Direction3::Left;
        assert_eq!(point, Point3::new(9, 5, 8));
    }

    #[test]
    fn checked_add_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_add_direction2(Direction2::East),
            Some(Point2::new(11, 5))
        );
        assert_eq!(
            Point2::<u8>::new(0, 5).checked_add_direction2(Direction2::West),
            None
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_add_direction3(Direction3::Up),
            Some(Point3::new(10, 5, 8))
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 0).checked_add_direction3(Direction3::Down),
            None
        );
    }

    #[test]
    fn saturating_add_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_add_direction2(Direction2::East),
            Point2::new(11, 5)
        );
        assert_eq!(
            Point2::<u8>::new(0, 5).saturating_add_direction2(Direction2::West),
            Point2::new(0, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_add_direction3(Direction3::Up),
            Point3::new(10, 5, 8)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 0).saturating_add_direction3(Direction3::Down),
            Point3::new(10, 5, 0)
        );
    }

    #[test]
    fn wrapping_add_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_add_direction2(Direction2::East),
            Point2::new(11, 5)
        );
        assert_eq!(
            Point2::<u8>::new(0, 5).wrapping_add_direction2(Direction2::West),
            Point2::new(255, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_add_direction3(Direction3::Up),
            Point3::new(10, 5, 8)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 0).wrapping_add_direction3(Direction3::Down),
            Point3::new(10, 5, 255)
        );
    }

    #[test]
    fn add_direction_magnitude() {
        assert_eq!(
            Point2::new(10, 5) + Direction2::East * 3,
            Point2::new(13, 5)
        );
        assert_eq!(Point2::new(10, 5) + Direction2::West * 3, Point2::new(7, 5));
        assert_eq!(
            Point3::new(10, 5, 7) + Direction3::Up * 3,
            Point3::new(10, 5, 10)
        );
        assert_eq!(
            Point3::new(10, 5, 7) + Direction3::Down * 3,
            Point3::new(10, 5, 4)
        );
    }

    #[test]
    fn add_assign_direction_magnitude() {
        let mut point = Point2::new(10, 5);
        point += Direction2::East * 3;
        assert_eq!(point, Point2::new(13, 5));
        point += Direction2::North * 3;
        assert_eq!(point, Point2::new(13, 2));

        let mut point = Point3::new(10, 5, 7);
        point += Direction3::Up * 3;
        assert_eq!(point, Point3::new(10, 5, 10));
        point += Direction3::Left * 3;
        assert_eq!(point, Point3::new(7, 5, 10));
    }

    #[test]
    fn checked_add_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_add_direction2_magnitude(Direction2::East * 3),
            Some(Point2::new(13, 5))
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_add_direction2_magnitude(Direction2::West * 255),
            None
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_add_direction3_magnitude(Direction3::Up * 3),
            Some(Point3::new(10, 5, 10))
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_add_direction3_magnitude(Direction3::Down * 255),
            None
        );
    }

    #[test]
    fn saturating_add_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_add_direction2_magnitude(Direction2::East * 3),
            Point2::new(13, 5)
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_add_direction2_magnitude(Direction2::West * 255),
            Point2::new(0, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_add_direction3_magnitude(Direction3::Up * 3),
            Point3::new(10, 5, 10)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_add_direction3_magnitude(Direction3::Down * 255),
            Point3::new(10, 5, 0)
        );
    }

    #[test]
    fn wrapping_add_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_add_direction2_magnitude(Direction2::East * 3),
            Point2::new(13, 5)
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_add_direction2_magnitude(Direction2::West * 255),
            Point2::new(11, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_add_direction3_magnitude(Direction3::Up * 3),
            Point3::new(10, 5, 10)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_add_direction3_magnitude(Direction3::Down * 255),
            Point3::new(10, 5, 8)
        );
    }

    #[test]
    fn sub_direction() {
        assert_eq!(Point2::new(10, 5) - Direction2::East, Point2::new(9, 5));
        assert_eq!(Point2::new(10, 5) - Direction2::West, Point2::new(11, 5));
        assert_eq!(
            Point3::new(10, 5, 7) - Direction3::Up,
            Point3::new(10, 5, 6)
        );
        assert_eq!(
            Point3::new(10, 5, 7) - Direction3::Down,
            Point3::new(10, 5, 8)
        );
    }

    #[test]
    fn sub_assign_direction() {
        let mut point = Point2::new(10, 5);
        point -= Direction2::East;
        assert_eq!(point, Point2::new(9, 5));
        point -= Direction2::North;
        assert_eq!(point, Point2::new(9, 6));

        let mut point = Point3::new(10, 5, 7);
        point -= Direction3::Up;
        assert_eq!(point, Point3::new(10, 5, 6));
        point -= Direction3::Left;
        assert_eq!(point, Point3::new(11, 5, 6));
    }

    #[test]
    fn checked_sub_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_sub_direction2(Direction2::East),
            Some(Point2::new(9, 5))
        );
        assert_eq!(
            Point2::<u8>::new(255, 5).checked_sub_direction2(Direction2::West),
            None
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_sub_direction3(Direction3::Up),
            Some(Point3::new(10, 5, 6))
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 255).checked_sub_direction3(Direction3::Down),
            None
        );
    }

    #[test]
    fn saturating_sub_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_sub_direction2(Direction2::East),
            Point2::new(9, 5)
        );
        assert_eq!(
            Point2::<u8>::new(255, 5).saturating_sub_direction2(Direction2::West),
            Point2::new(255, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_sub_direction3(Direction3::Up),
            Point3::new(10, 5, 6)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 255).saturating_sub_direction3(Direction3::Down),
            Point3::new(10, 5, 255)
        );
    }

    #[test]
    fn wrapping_sub_direction() {
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_sub_direction2(Direction2::East),
            Point2::new(9, 5)
        );
        assert_eq!(
            Point2::<u8>::new(255, 5).wrapping_sub_direction2(Direction2::West),
            Point2::new(0, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_sub_direction3(Direction3::Up),
            Point3::new(10, 5, 6)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 255).wrapping_sub_direction3(Direction3::Down),
            Point3::new(10, 5, 0)
        );
    }

    #[test]
    fn sub_direction_magnitude() {
        assert_eq!(Point2::new(10, 5) - Direction2::East * 3, Point2::new(7, 5));
        assert_eq!(
            Point2::new(10, 5) - Direction2::West * 3,
            Point2::new(13, 5)
        );
        assert_eq!(
            Point3::new(10, 5, 7) - Direction3::Up * 3,
            Point3::new(10, 5, 4)
        );
        assert_eq!(
            Point3::new(10, 5, 7) - Direction3::Down * 3,
            Point3::new(10, 5, 10)
        );
    }

    #[test]
    fn sub_assign_direction_magnitude() {
        let mut point = Point2::new(10, 5);
        point -= Direction2::East * 3;
        assert_eq!(point, Point2::new(7, 5));
        point -= Direction2::North * 3;
        assert_eq!(point, Point2::new(7, 8));

        let mut point = Point3::new(10, 5, 7);
        point -= Direction3::Up * 3;
        assert_eq!(point, Point3::new(10, 5, 4));
        point -= Direction3::Left * 3;
        assert_eq!(point, Point3::new(13, 5, 4));
    }

    #[test]
    fn checked_sub_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_sub_direction2_magnitude(Direction2::East * 3),
            Some(Point2::new(7, 5))
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).checked_sub_direction2_magnitude(Direction2::West * 255),
            None
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_sub_direction3_magnitude(Direction3::Up * 3),
            Some(Point3::new(10, 5, 4))
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).checked_sub_direction3_magnitude(Direction3::Down * 255),
            None
        );
    }

    #[test]
    fn saturating_sub_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_sub_direction2_magnitude(Direction2::East * 3),
            Point2::new(7, 5)
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).saturating_sub_direction2_magnitude(Direction2::West * 255),
            Point2::new(255, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_sub_direction3_magnitude(Direction3::Up * 3),
            Point3::new(10, 5, 4)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).saturating_sub_direction3_magnitude(Direction3::Down * 255),
            Point3::new(10, 5, 255)
        );
    }

    #[test]
    fn wrapping_sub_direction_magnitude() {
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_sub_direction2_magnitude(Direction2::East * 3),
            Point2::new(7, 5)
        );
        assert_eq!(
            Point2::<u8>::new(10, 5).wrapping_sub_direction2_magnitude(Direction2::West * 255),
            Point2::new(9, 5)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_sub_direction3_magnitude(Direction3::Up * 3),
            Point3::new(10, 5, 4)
        );
        assert_eq!(
            Point3::<u8>::new(10, 5, 7).wrapping_sub_direction3_magnitude(Direction3::Down * 255),
            Point3::new(10, 5, 6)
        );
    }

    #[test]
    fn add_scalar() {
        assert_eq!(Point2::new(10, 5) + 3, Point2::new(13, 8));
        assert_eq!(Point3::new(10, 5, 7) + 4, Point3::new(14, 9, 11));
    }

    #[test]
    fn add_assign_scalar() {
        let mut point = Point2::new(10, 5);
        point += 3;
        assert_eq!(point, Point2::new(13, 8));

        let mut point = Point3::new(10, 5, 7);
        point += 4;
        assert_eq!(point, Point3::new(14, 9, 11));
    }

    #[test]
    fn sub_scalar() {
        assert_eq!(Point2::new(10, 5) - 3, Point2::new(7, 2));
        assert_eq!(Point3::new(10, 5, 7) - 4, Point3::new(6, 1, 3));
    }

    #[test]
    fn sub_assign_scalar() {
        let mut point = Point2::new(10, 5);
        point -= 3;
        assert_eq!(point, Point2::new(7, 2));

        let mut point = Point3::new(10, 5, 7);
        point -= 4;
        assert_eq!(point, Point3::new(6, 1, 3));
    }

    #[test]
    fn mul_scalar() {
        assert_eq!(Point2::new(10, 5) * 3, Point2::new(30, 15));
        assert_eq!(Point3::new(10, 5, 7) * 4, Point3::new(40, 20, 28));
    }

    #[test]
    fn mul_assign_scalar() {
        let mut point = Point2::new(10, 5);
        point *= 3;
        assert_eq!(point, Point2::new(30, 15));

        let mut point = Point3::new(10, 5, 7);
        point *= 4;
        assert_eq!(point, Point3::new(40, 20, 28));
    }

    #[test]
    fn div_scalar() {
        assert_eq!(Point2::new(20, 15) / 3, Point2::new(6, 5));
        assert_eq!(Point3::new(20, 15, 28) / 4, Point3::new(5, 3, 7));
    }

    #[test]
    fn div_assign_scalar() {
        let mut point = Point2::new(20, 15);
        point /= 3;
        assert_eq!(point, Point2::new(6, 5));

        let mut point = Point3::new(20, 15, 28);
        point /= 4;
        assert_eq!(point, Point3::new(5, 3, 7));
    }

    #[test]
    fn range() {
        let range: Point2Range<_, _> = (Point2::new(1, 1)..Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(!range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
    }

    #[test]
    fn range_inclusive() {
        let range: Point2Range<_, _> = (Point2::new(1, 1)..=Point2::new(5, 5)).into();
        assert!(!range.contains(&Point2::new(0, 0)));
        assert!(range.contains(&Point2::new(2, 1)));
        assert!(range.contains(&Point2::new(5, 2)));
        assert!(!range.contains(&Point2::new(6, 3)));
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
