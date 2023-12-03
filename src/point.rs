use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use derive_new::new;

use crate::abs_diff;

// Implements an operator (add/sub/mul/div) for a point type, including the assign variant of the operator.
macro_rules! impl_operator {
    ($name:ident, $op:ident, $($var:ident),+) => {
        paste::paste! {
            impl<T, R> [<$op:camel>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R>,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                type Output = Self;
                #[must_use]
                fn [<$op>](self, rhs: $name<R>) -> Self {
                    Self {
                        $($var: self.$var.[<$op>](rhs.$var).into()),+
                    }
                }
            }
            impl<T, R> [<$op:camel Assign>]<$name<R>> for $name<T>
            where
                T: [<$op:camel>]<R> + Copy,
                <T as [<$op:camel>]<R>>::Output: Into<T>,
            {
                fn [<$op _assign>](&mut self, rhs: $name<R>) {
                    *self = Self {
                        $($var: self.$var.[<$op>](rhs.$var).into()),+
                    };
                }
            }
        }
    };
}

macro_rules! call_chain {
    ($fn:ident, $expr:expr $(,)?) => ($expr);
    ($fn:ident, $first:expr, $second:expr $(, $($exprs:expr),*)?) => {
        call_chain!($fn, $first.$fn($second) $(, $($exprs),*)?)
    };
}

// Generate a point class with the given name and variables.
macro_rules! create_point {
    ($name:ident, $($var:ident),+) => {
        #[derive(Clone, Copy, Eq, Hash, PartialEq, new)]
        pub struct $name<T = usize> {
            $(pub $var: T),+
        }

        impl_operator!($name, add, $($var),+);
        impl_operator!($name, sub, $($var),+);
        impl_operator!($name, mul, $($var),+);
        impl_operator!($name, div, $($var),+);

        impl<T> $name<T>
            where T: Copy + Add<T, Output = T> + Sub<T, Output = T> + PartialOrd<T> + Ord
        {
            /// Calculate the sum of all coordinates of the point.
            #[must_use]
            pub fn sum(&self) -> T {
                call_chain!(add, $(self.$var),+)
            }

            /// Calculate the distance between this point and another point.
            ///
            /// Diagonals are counted as a distance of one.
            #[must_use]
            pub fn distance(&self, other: &Self) -> T {
                let diff = self.abs_diff(other);
                call_chain!(max, $(diff.$var),+)
            }

            /// Get a point that represents the absolute differences of all coordinates of the two points.
            #[must_use]
            pub fn abs_diff(&self, other: &Self) -> Self {
                Self {
                    $($var: abs_diff(self.$var, other.$var)),+
                }
            }
        }
    };
}

create_point!(Point2, x, y);
create_point!(Point3, x, y, z);

impl<T: Debug> Debug for Point2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Point({:?}, {:?})", self.x, self.y))
    }
}
impl<T: Debug> Debug for Point3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Point({:?}, {:?}, {:?})", self.x, self.y, self.z))
    }
}

#[cfg(test)]
mod tests {
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
    fn sum() {
        assert_eq!(Point2::new(10, 5).sum(), 15);
        assert_eq!(Point2::new(10, -5).sum(), 5);
        assert_eq!(Point3::new(10, 5, 8).sum(), 23);
        assert_eq!(Point3::new(10, -5, 3).sum(), 8);
    }

    #[test]
    fn abs_diff() {
        assert_eq!(
            Point2::new(10, 5).abs_diff(&Point2::new(2, 20)),
            Point2::new(8, 15)
        );
        assert_eq!(
            Point3::new(10, 5, 3).abs_diff(&Point3::new(2, 20, -3)),
            Point3::new(8, 15, 6)
        );
    }

    #[test]
    fn distance() {
        assert_eq!(Point2::new(10, 5).distance(&Point2::new(2, 20)), 15);
        assert_eq!(Point3::new(10, 5, 3).distance(&Point3::new(2, 20, -3)), 15);
    }
}
