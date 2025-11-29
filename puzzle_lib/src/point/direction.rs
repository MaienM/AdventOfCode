use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use num::traits::{
    CheckedAdd, CheckedSub, One, SaturatingAdd, SaturatingSub, WrappingAdd, WrappingSub,
};

use crate::point::{Point2, Point3, Point4};

/// A helper struct combining a direction with a magnitude
///
/// This is used when taking multiple steps in a direction at one time.
pub struct DirectionWithMagnitude<D, T>(D, T);

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

        create_direction!(@operator; $name, $point, add, $($member = $($var $method)+),+);
        create_direction!(@operator; $name, $point, sub, $($member = $($var $method)+),+);
    };

    // Implements an operator (add/sub) for a point type combined with a direction (optionally multiplied by some amount), including the assign, checked, saturating, and wrapping variants.
    (@operator; $name:ty, $point:ident, $op:ident, $($member:ident = $($var:ident $method:ident)+),+ $(,)?) => {
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
                                $($var: create_direction!(@operator; call; self.$var; [($op; $method)](T::one())).into()),+,
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
                                $(create_direction!(@operator; call; self.$var; [($op; $method) _assign](T::one()));)+
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
                                $($var: create_direction!(@operator; call; self.$var; [checked_ ($op; $method)](&T::one()))?),+,
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
                                $($var: create_direction!(@operator; call; self.$var; [saturating_ ($op; $method)](&T::one()))),+,
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
                                $($var: create_direction!(@operator; call; self.$var; [wrapping_ ($op; $method)](&T::one()))),+,
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
                                $($var: create_direction!(@operator; call; self.$var; [($op; $method)](magnitude)).into()),+,
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
                                $(create_direction!(@operator; call; self.$var; [($op; $method) _assign](magnitude));)+
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
                                $($var: create_direction!(@operator; call; self.$var; [checked_ ($op; $method)](&magnitude))?),+,
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
                                $($var: create_direction!(@operator; call; self.$var; [saturating_ ($op; $method)](&magnitude))),+,
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
                                $($var: create_direction!(@operator; call; self.$var; [wrapping_ ($op; $method)](&magnitude))),+,
                                ..*self
                            },
                        )+
                    }
                }
            }
        }
    };
    (@operator; call; $expr:expr; [$($prefix:ident)? (add; add) $($suffix:ident)?]($($arg:expr),*)) => {
        create_direction!(@operator; call; $expr; [$($prefix)? (add) $($suffix)?]($($arg),*))
    };
    (@operator; call; $expr:expr; [$($prefix:ident)? (add; sub) $($suffix:ident)?]($($arg:expr),*)) => {
        create_direction!(@operator; call; $expr; [$($prefix)? (sub) $($suffix)?]($($arg),*))
    };
    (@operator; call; $expr:expr; [$($prefix:ident)? (sub; add) $($suffix:ident)?]($($arg:expr),*)) => {
        create_direction!(@operator; call; $expr; [$($prefix)? (sub) $($suffix)?]($($arg),*))
    };
    (@operator; call; $expr:expr; [$($prefix:ident)? (sub; sub) $($suffix:ident)?]($($arg:expr),*)) => {
        create_direction!(@operator; call; $expr; [$($prefix)? (add) $($suffix)?]($($arg),*))
    };
    (@operator; call; $expr:expr; [$($prefix:ident)? ($method:ident) $($suffix:ident)?]($($arg:expr),*)) => {
        paste::paste!($expr.[<$($prefix)? $method $($suffix)?>]($($arg),*))
    };
}

//
// 2-dimensional space.
//

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

//
// 3-dimensional space.
//

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

//
// 4-dimensional space.
//

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
    use pretty_assertions::assert_eq;

    use super::*;

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
}
