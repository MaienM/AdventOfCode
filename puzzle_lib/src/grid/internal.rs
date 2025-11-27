use std::hash::Hash;

use crate::grid::GridPoint;

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
    use crate::point::Point2;

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
