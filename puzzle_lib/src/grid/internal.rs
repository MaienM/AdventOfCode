use std::hash::Hash;

use derive_new::new;

use super::{PointBoundaries, PointType};
use crate::{grid::GridPoint, point::Point2};

#[derive(Debug, Eq, PartialEq, Clone, new)]
pub struct PointBoundariesImpl<P>(P, P);
impl<PT> PointBoundaries<Point2<PT>> for PointBoundariesImpl<Point2<PT>>
where
    PT: PointType,
{
    fn boundaries(&self) -> (&Point2<PT>, &Point2<PT>) {
        (&self.0, &self.1)
    }

    fn in_boundaries(&self, point: &Point2<PT>) -> bool {
        self.0.x <= point.x && point.x <= self.1.x && self.0.y <= point.y && point.y <= self.1.y
    }
}
impl<PT> From<(&Point2<PT>, &Point2<PT>)> for PointBoundariesImpl<Point2<PT>>
where
    PT: PointType,
{
    fn from(value: (&Point2<PT>, &Point2<PT>)) -> Self {
        let (x1, x2) = if value.0.x > value.1.x {
            (value.1.x, value.0.x)
        } else {
            (value.0.x, value.1.x)
        };
        let (y1, y2) = if value.0.y > value.1.y {
            (value.1.y, value.0.y)
        } else {
            (value.0.y, value.1.y)
        };
        Self(Point2::new(x1, y1), Point2::new(x2, y2))
    }
}

pub trait PointOrRef<P>: Copy + Eq + Hash
where
    P: GridPoint,
{
    fn resolve_val(self) -> P;
    fn resolve_ref(&self) -> &P;
}
impl<P> PointOrRef<P> for P
where
    P: GridPoint,
{
    #[inline]
    fn resolve_val(self) -> P {
        self
    }

    #[inline]
    fn resolve_ref(&self) -> &P {
        self
    }
}
impl<P> PointOrRef<P> for &P
where
    P: GridPoint,
{
    #[inline]
    fn resolve_val(self) -> P {
        *self
    }

    #[inline]
    fn resolve_ref(&self) -> &P {
        self
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn in_boundaries() {
        let boundaries: PointBoundariesImpl<Point2<i8>> =
            PointBoundariesImpl(Point2::new(-10, -5), Point2::new(5, 10));

        // Check somewhere in the middle, on each of the 4 edges, and on each of the 4 corners.
        assert_eq!(boundaries.in_boundaries(&Point2::new(0, 0)), true);

        assert_eq!(boundaries.in_boundaries(&Point2::new(-10, 0)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(0, -5)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(0, 10)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(5, 0)), true);

        assert_eq!(boundaries.in_boundaries(&Point2::new(-10, -5)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(-10, 10)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(5, -5)), true);
        assert_eq!(boundaries.in_boundaries(&Point2::new(5, 10)), true);

        // Check just outside each of the 4 edges and each of the 4 corners (on both axis).
        assert_eq!(boundaries.in_boundaries(&Point2::new(-11, 0)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(0, -6)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(0, 11)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(6, 0)), false);

        assert_eq!(boundaries.in_boundaries(&Point2::new(-11, -5)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(-10, -6)), false);

        assert_eq!(boundaries.in_boundaries(&Point2::new(-11, 10)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(-10, 11)), false);

        assert_eq!(boundaries.in_boundaries(&Point2::new(6, -5)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(5, -6)), false);

        assert_eq!(boundaries.in_boundaries(&Point2::new(6, 10)), false);
        assert_eq!(boundaries.in_boundaries(&Point2::new(5, 11)), false);
    }

    #[test]
    fn from_tuple() {
        let boundaries: PointBoundariesImpl<Point2<i8>> =
            (&Point2::new(-10, -5), &Point2::new(5, 10)).into();
        assert_eq!(boundaries.0, Point2::new(-10, -5));
        assert_eq!(boundaries.1, Point2::new(5, 10));

        assert_eq!(
            PointBoundariesImpl::from((&Point2::new(5, 10), &Point2::new(-10, -5))),
            boundaries,
        );
        assert_eq!(
            PointBoundariesImpl::from((&Point2::new(-10, 10), &Point2::new(5, -5))),
            boundaries,
        );
        assert_eq!(
            PointBoundariesImpl::from((&Point2::new(5, -5), &Point2::new(-10, 10))),
            boundaries,
        );
    }

    #[test]
    fn point_or_ref() {
        let value = Point2::new(2, 4);
        assert_eq!(value.resolve_val(), value);
        assert_eq!(value.resolve_ref(), &value);

        let reference = &value;
        assert_eq!(reference.resolve_val(), value);
        assert_eq!(reference.resolve_ref(), &value);
    }
}
