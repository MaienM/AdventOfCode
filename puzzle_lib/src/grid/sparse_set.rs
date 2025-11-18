use std::{collections::HashSet, fmt::Debug, ops::RangeBounds};

use inherit_methods_macro::inherit_methods;
use itertools::Itertools;

use super::{
    PointBoundaries, PointCollection, PointCollectionInsertResult, PointOnlyCollection, PointType,
};
use crate::point::{Point2, Point2Range, PointRange};

/// A collection of 2-dimensional points.
///
/// This is efficient if only a small portion of the covered region is actually used. If a
/// substantial portion of it is used a [`crate::grid::FullGrid`] is probably more efficient.
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    points: HashSet<Point2<PT>>,
}
#[inherit_methods(from = "self.points")]
impl<PT> Extend<Point2<PT>> for SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    fn extend<T: IntoIterator<Item = Point2<PT>>>(&mut self, iter: T);
}
impl<PT> PointCollection<Point2<PT>> for SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    fn contains_point(&self, point: &Point2<PT>) -> bool {
        self.points.contains(point)
    }

    fn into_iter_points(self) -> impl Iterator<Item = Point2<PT>> {
        self.points.into_iter()
    }

    fn iter_points(&self) -> impl Iterator<Item = &Point2<PT>> {
        self.points.iter()
    }

    fn area(&self) -> (Point2<PT>, Point2<PT>) {
        let x = self
            .points
            .iter()
            .minmax_by_key(|p| p.x)
            .into_option()
            .unwrap();
        let y = self
            .points
            .iter()
            .minmax_by_key(|p| p.y)
            .into_option()
            .unwrap();
        (Point2::new(x.0.x, y.0.y), Point2::new(x.1.x, y.1.y))
    }
}
impl<PT> PointOnlyCollection<Point2<PT>> for SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    fn insert(&mut self, point: Point2<PT>) -> PointCollectionInsertResult<()> {
        if self.points.insert(point) {
            PointCollectionInsertResult::Inserted
        } else {
            PointCollectionInsertResult::Replaced(())
        }
    }

    fn remove(&mut self, point: &Point2<PT>) -> bool {
        self.points.remove(point)
    }
}

// Create from variations of lists of points.
impl<PT> FromIterator<Point2<PT>> for SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    fn from_iter<I: IntoIterator<Item = Point2<PT>>>(iter: I) -> Self {
        let points = iter.into_iter().collect::<HashSet<_>>();
        points.into()
    }
}
impl<PT> From<HashSet<Point2<PT>>> for SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    fn from(points: HashSet<Point2<PT>>) -> Self {
        Self { points }
    }
}

/// A collection of 2-dimensional points within specified boundaries.
///
/// This is efficient if only a small portion of the covered region is actually used. If a
/// substantial portion of it is used a [`crate::grid::FullGrid`] is probably more efficient.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    grid: SparsePointSet<PT>,
    boundaries: R,
}
#[inherit_methods(from = "self.grid")]
impl<PT, R> Extend<Point2<PT>> for BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn extend<T: IntoIterator<Item = Point2<PT>>>(&mut self, iter: T);
}
#[inherit_methods(from = "self.grid")]
impl<PT, R> PointCollection<Point2<PT>> for BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn contains_point(&self, point: &Point2<PT>) -> bool;
    fn into_iter_points(self) -> impl Iterator<Item = Point2<PT>>;
    fn iter_points(&self) -> impl Iterator<Item = &Point2<PT>>;
    fn area(&self) -> (Point2<PT>, Point2<PT>);
}
#[inherit_methods(from = "self.grid")]
impl<PT, R> PointOnlyCollection<Point2<PT>> for BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn insert(&mut self, point: Point2<PT>) -> PointCollectionInsertResult<()> {
        if self.boundaries.contains(&point) {
            self.grid.insert(point)
        } else {
            PointCollectionInsertResult::OutOfBounds
        }
    }

    fn remove(&mut self, point: &Point2<PT>) -> bool;
}
impl<PT, R> PointBoundaries<Point2<PT>, R> for BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn boundaries(&self) -> &R {
        &self.boundaries
    }
}

// Add boundaries to unbound set or redefine boundaries of bound set.
#[inherit_methods(from = "self.grid")]
impl<PT, R> BoundedSparsePointSet<PT, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    /// Change the boundaries.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any of the points are outside the new boundaries.
    pub fn with_boundaries<RX, RY>(
        self,
        boundaries: Point2Range<RX, RY>,
    ) -> Result<BoundedSparsePointSet<PT, Point2Range<RX, RY>>, String>
    where
        RX: RangeBounds<PT> + Debug,
        RY: RangeBounds<PT> + Debug,
    {
        if let Some(invalid) = self.grid.iter_points().find(|p| !boundaries.contains(p)) {
            Err(format!("{invalid:?} not within in bounds {boundaries:?}"))
        } else {
            Ok(BoundedSparsePointSet {
                grid: self.grid,
                boundaries,
            })
        }
    }
}
impl<PT> SparsePointSet<PT>
where
    PT: PointType + 'static,
{
    /// Convert to [`BoundedSparsePointSet`] by adding boundaries.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any of the points are outside the boundaries.
    pub fn with_boundaries<RX, RY>(
        self,
        boundaries: Point2Range<RX, RY>,
    ) -> Result<BoundedSparsePointSet<PT, Point2Range<RX, RY>>, String>
    where
        RX: RangeBounds<PT> + Debug,
        RY: RangeBounds<PT> + Debug,
    {
        let bounded = BoundedSparsePointSet {
            grid: self,
            boundaries: Point2Range::from(..),
        };
        bounded.with_boundaries(boundaries)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_unordered_eq;

    #[test]
    fn into_iter_points() {
        let grid: SparsePointSet<_> = [Point2::new(1, 2), Point2::new(2, 3)].into_iter().collect();
        assert_unordered_eq!(
            grid.into_iter_points(),
            Point2::new(1, 2),
            Point2::new(2, 3),
        );
    }

    #[test]
    fn with_boundaries() {
        let grid: SparsePointSet<_> = [Point2::new(1, 2), Point2::new(2, 3)].into_iter().collect();
        assert!(
            grid.clone()
                .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
                .is_ok()
        );
        assert!(
            grid.with_boundaries((Point2::new(0, 0)..Point2::new(2, 2)).into())
                .is_err()
        );
    }

    #[test]
    fn bounded_insert() {
        let grid: SparsePointSet<_> = [Point2::new(1, 2), Point2::new(2, 3)].into_iter().collect();
        let mut grid = grid
            .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
            .unwrap();
        assert_eq!(
            grid.insert(Point2::new(3, 1)),
            PointCollectionInsertResult::Inserted
        );
        assert_eq!(
            grid.insert(Point2::new(1, 2)),
            PointCollectionInsertResult::Replaced(())
        );
        assert_eq!(
            grid.insert(Point2::new(5, 0)),
            PointCollectionInsertResult::OutOfBounds
        );
    }

    #[test]
    fn bounded_with_boundaries() {
        let grid: SparsePointSet<_> = [Point2::new(1, 2), Point2::new(2, 3)].into_iter().collect();
        let grid = grid
            .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
            .unwrap();
        assert!(
            grid.clone()
                .with_boundaries((Point2::new(0, 0)..Point2::new(6, 6)).into())
                .is_ok()
        );
        assert!(
            grid.with_boundaries((Point2::new(0, 0)..Point2::new(2, 2)).into())
                .is_err()
        );
    }

    #[test]
    fn boundaries() {
        let grid: SparsePointSet<_> = [Point2::new(1, 2), Point2::new(2, 3)].into_iter().collect();
        let grid = grid
            .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
            .unwrap();
        assert_eq!(
            grid.boundaries(),
            &(Point2::new(0, 0)..=Point2::new(3, 3)).into()
        );
    }
}
