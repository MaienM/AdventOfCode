use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use derive_new::new;

use crate::utils::abs_diff;

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

// Helper macro for neighbours() method.
macro_rules! impl_neighbor_diag_inner {
    (toplevel; $neighbours:ident, $base:expr, $var:ident) => {
        if let Some($var) = $base.$var.checked_sub(1) {
            $neighbours.insert(Self { $var, ..$base });
        }
        if let Some($var) = $base.$var.checked_add(1) {
            $neighbours.insert(Self { $var, ..$base });
        }
    };
    (nested; $neighbours:ident, $base:expr, $var:ident) => {
        impl_neighbor_diag_inner!(toplevel; $neighbours, $base, $var);
        $neighbours.insert($base);
    };

    ($type:ident; $neighbours:ident, $base:expr, $var:ident, $($vars:ident),*) => {
        if let Some($var) = $base.$var.checked_sub(1) {
            let base = Self { $var, ..$base };
            impl_neighbor_diag_inner!(nested; $neighbours, base, $($vars),*);
        }
        if let Some($var) = $base.$var.checked_add(1) {
            let base = Self { $var, ..$base };
            impl_neighbor_diag_inner!(nested; $neighbours, base, $($vars),*);
        }
        impl_neighbor_diag_inner!($type; $neighbours, $base, $($vars),*);
    };

    ($neighbours:ident, $base:expr, $($vars:ident),*) => {
        impl_neighbor_diag_inner!(toplevel; $neighbours, $base, $($vars),*);
    }
}

// Implements methods that rely on the type contained in the point being an integer of some kind.
macro_rules! impl_integer_methods {
    ($name:ident, $type:ty, $($var:ident),+) => {
        impl $name<$type> {
            /// Check whether the given point is orthogontally adjacent to this one.
            pub fn adjacent_to_ortho(&self, other: &Self) -> bool {
                self.abs_diff(other).sum() == 1
            }

            /// Check whether the given point is orthogontally or diagonally adjacent to this one.
            pub fn adjacent_to_diag(&self, other: &Self) -> bool {
                self != other && self.distance_diag(other) == 1
            }

            /// Get the orthogontal neighbours of this point.
            pub fn neighbours_ortho(&self) -> HashSet<Self> {
                let mut neighbours = HashSet::new();
                $(
                    if let Some($var) = self.$var.checked_sub(1) {
                        neighbours.insert(Self { $var, ..*self });
                    }
                    if let Some($var) = self.$var.checked_add(1) {
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
        #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, new)]
        pub struct $name<T = usize> {
            $(pub $var: T),+
        }

        impl_operator!($name, add, $($var),+);
        impl_operator!($name, sub, $($var),+);
        impl_operator!($name, mul, $($var),+);
        impl_operator!($name, div, $($var),+);

        impl_integer_methods!($name, u8, $($var),+);
        impl_integer_methods!($name, u16, $($var),+);
        impl_integer_methods!($name, u32, $($var),+);
        impl_integer_methods!($name, u64, $($var),+);
        impl_integer_methods!($name, u128, $($var),+);
        impl_integer_methods!($name, usize, $($var),+);
        impl_integer_methods!($name, i8, $($var),+);
        impl_integer_methods!($name, i16, $($var),+);
        impl_integer_methods!($name, i32, $($var),+);
        impl_integer_methods!($name, i64, $($var),+);
        impl_integer_methods!($name, i128, $($var),+);
        impl_integer_methods!($name, isize, $($var),+);

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
            /// Diagonals are counted as a distance of two.
            #[must_use]
            pub fn distance_ortho(&self, other: &Self) -> T {
                self.abs_diff(other).sum()
            }

            /// Calculate the distance between this point and another point.
            ///
            /// Diagonals are counted as a distance of one.
            #[must_use]
            pub fn distance_diag(&self, other: &Self) -> T {
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
    fn distance_ortho() {
        assert_eq!(Point2::new(10, 5).distance_ortho(&Point2::new(2, 20)), 23);
        assert_eq!(
            Point3::new(10, 5, 3).distance_ortho(&Point3::new(2, 20, -3)),
            29
        );
    }

    #[test]
    fn distance_diag() {
        assert_eq!(Point2::new(10, 5).distance_diag(&Point2::new(2, 20)), 15);
        assert_eq!(
            Point3::new(10, 5, 3).distance_diag(&Point3::new(2, 20, -3)),
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
}
