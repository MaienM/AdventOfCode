use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut, RangeBounds},
};

use inherit_methods_macro::inherit_methods;
use itertools::Itertools;

use super::{
    PointBoundaries, PointCollection, PointCollectionInsertResult, PointDataCollection, PointType,
};
use crate::{
    grid::internal::PointOrRef,
    point::{Point2, Point2Range, PointRange},
};

/// A collection of 2-dimensional points with associated data.
///
/// This is efficient if only a small portion of the covered region is actually used. If a
/// substantial portion of it is used a [`crate::grid::FullGrid`] is probably more efficient.
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    cells: HashMap<Point2<PT>, D>,
}
impl<PT, D> PointCollection<Point2<PT>> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn contains_point(&self, point: &Point2<PT>) -> bool {
        self.cells.contains_key(point)
    }

    fn into_iter_points(self) -> impl Iterator<Item = Point2<PT>> {
        self.cells.into_keys()
    }

    fn iter_points(&self) -> impl Iterator<Item = &Point2<PT>> {
        self.cells.keys()
    }

    fn area(&self) -> (Point2<PT>, Point2<PT>) {
        let x = self
            .cells
            .keys()
            .minmax_by_key(|p| p.x)
            .into_option()
            .unwrap();
        let y = self
            .cells
            .keys()
            .minmax_by_key(|p| p.y)
            .into_option()
            .unwrap();
        (Point2::new(x.0.x, y.0.y), Point2::new(x.1.x, y.1.y))
    }
}
impl<PT, D> Index<Point2<PT>> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    type Output = D;

    fn index(&self, point: Point2<PT>) -> &Self::Output {
        &self.cells[&point]
    }
}
impl<PT, D> IndexMut<Point2<PT>> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn index_mut(&mut self, point: Point2<PT>) -> &mut Self::Output {
        self.cells.get_mut(&point).expect("no entry found for key")
    }
}
#[inherit_methods(from = "self.cells")]
impl<PT, D> Extend<(Point2<PT>, D)> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn extend<T: IntoIterator<Item = (Point2<PT>, D)>>(&mut self, iter: T);
}
impl<PT, D: 'static> PointDataCollection<Point2<PT>, D> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn get(&self, point: &Point2<PT>) -> Option<&D> {
        self.cells.get(point)
    }

    fn get_mut(&mut self, point: &Point2<PT>) -> Option<&mut D> {
        self.cells.get_mut(point)
    }

    fn get_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, Option<&D>)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>,
    {
        points.map(|point| (point, self.cells.get(point.resolve_ref())))
    }

    fn get_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, Option<&mut D>)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>,
    {
        points.unique().map(|point| {
            let value = self.cells.get_mut(point.resolve_ref()).map(|data| {
                let ptr = data as *mut _;
                unsafe { &mut *ptr }
            });
            (point, value)
        })
    }

    unsafe fn get_many_unchecked_mut<PR, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>,
    {
        points.map(|point| unsafe {
            let value = self.cells.get_mut(point.resolve_ref()).unwrap_unchecked();
            let ptr = value as *mut _;
            (point, &mut *ptr)
        })
    }

    fn insert(&mut self, point: Point2<PT>, data: D) -> PointCollectionInsertResult<D> {
        if let Some(old) = self.cells.insert(point, data) {
            PointCollectionInsertResult::Replaced(old)
        } else {
            PointCollectionInsertResult::Inserted
        }
    }

    fn remove(&mut self, point: &Point2<PT>) -> Option<D> {
        self.cells.remove(point)
    }

    fn into_iter_data(self) -> impl Iterator<Item = D> {
        self.cells.into_values()
    }

    fn iter_data(&self) -> impl Iterator<Item = &D> {
        self.cells.values()
    }

    fn iter_mut_data(&mut self) -> impl Iterator<Item = &mut D> {
        self.cells.values_mut()
    }

    fn into_iter_pairs(self) -> impl Iterator<Item = (Point2<PT>, D)> {
        self.cells.into_iter()
    }

    fn iter_pairs(&self) -> impl Iterator<Item = (&Point2<PT>, &D)> {
        self.cells.iter()
    }

    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&Point2<PT>, &mut D)> {
        self.cells.iter_mut()
    }
}

// Create from variations of lists of pairs of points & data.
impl<PT, D> FromIterator<(Point2<PT>, D)> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn from_iter<I: IntoIterator<Item = (Point2<PT>, D)>>(iter: I) -> Self {
        let points = iter.into_iter().collect::<HashMap<_, _>>();
        points.into()
    }
}
impl<PT, D> From<HashMap<Point2<PT>, D>> for SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    fn from(points: HashMap<Point2<PT>, D>) -> Self {
        Self { cells: points }
    }
}

/// A collection of 2-dimensional points within specified boundaries, with associated data.
///
/// This is efficient if only a small portion of the covered region is actually used. If a
/// substantial portion of it is used a [`crate::grid::FullGrid`] is probably more efficient.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoundedSparsePointMap<PT, D, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    grid: SparsePointMap<PT, D>,
    boundaries: R,
}
#[inherit_methods(from = "self.grid")]
impl<PT, D, R> PointCollection<Point2<PT>> for BoundedSparsePointMap<PT, D, R>
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
impl<PT, D, R> Index<Point2<PT>> for BoundedSparsePointMap<PT, D, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    type Output = D;
    fn index(&self, index: Point2<PT>) -> &Self::Output;
}
#[inherit_methods(from = "self.grid")]
impl<PT, D, R> IndexMut<Point2<PT>> for BoundedSparsePointMap<PT, D, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn index_mut(&mut self, index: Point2<PT>) -> &mut Self::Output;
}
#[inherit_methods(from = "self.grid")]
impl<PT, D, R> Extend<(Point2<PT>, D)> for BoundedSparsePointMap<PT, D, R>
where
    PT: PointType + 'static,
    R: PointRange<Point2<PT>>,
{
    fn extend<T: IntoIterator<Item = (Point2<PT>, D)>>(&mut self, iter: T);
}
#[inherit_methods(from = "self.grid")]
impl<PT, D, R> PointDataCollection<Point2<PT>, D> for BoundedSparsePointMap<PT, D, R>
where
    PT: PointType + 'static,
    D: 'static,
    R: PointRange<Point2<PT>>,
{
    fn insert(&mut self, point: Point2<PT>, data: D) -> PointCollectionInsertResult<D> {
        if self.boundaries.contains(&point) {
            self.grid.insert(point, data)
        } else {
            println!("Point {point:?} is out of bounds {:?}", self.boundaries);
            PointCollectionInsertResult::OutOfBounds
        }
    }

    fn get(&self, point: &Point2<PT>) -> Option<&D>;
    unsafe fn get_unchecked(&self, point: &Point2<PT>) -> &D;
    fn get_mut(&mut self, point: &Point2<PT>) -> Option<&mut D>;
    unsafe fn get_unchecked_mut(&mut self, point: &Point2<PT>) -> &mut D;
    fn get_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, Option<&D>)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    unsafe fn get_many_unchecked<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, &D)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    fn get_filter_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, &D)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    fn get_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, Option<&mut D>)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    unsafe fn get_many_unchecked_mut<PR, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    fn get_filter_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<Point2<PT>>,
        I: Iterator<Item = PR>;
    fn remove(&mut self, point: &Point2<PT>) -> Option<D>;
    fn into_iter_data(self) -> impl Iterator<Item = D>;
    fn iter_data(&self) -> impl Iterator<Item = &D>;
    fn iter_mut_data(&mut self) -> impl Iterator<Item = &mut D>;
    fn into_iter_pairs(self) -> impl Iterator<Item = (Point2<PT>, D)>;
    fn iter_pairs(&self) -> impl Iterator<Item = (&Point2<PT>, &D)>;
    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&Point2<PT>, &mut D)>;
}
impl<PT, D, R> PointBoundaries<Point2<PT>, R> for BoundedSparsePointMap<PT, D, R>
where
    PT: PointType,
    R: PointRange<Point2<PT>>,
{
    fn boundaries(&self) -> &R {
        &self.boundaries
    }
}

// Add boundaries to unbound map or redefine boundaries of bound map.
impl<PT, D, R> BoundedSparsePointMap<PT, D, R>
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
    ) -> Result<BoundedSparsePointMap<PT, D, Point2Range<RX, RY>>, String>
    where
        RX: RangeBounds<PT> + Debug,
        RY: RangeBounds<PT> + Debug,
    {
        if let Some(invalid) = self.grid.iter_points().find(|p| !boundaries.contains(p)) {
            Err(format!("{invalid:?} not within in bounds {boundaries:?}"))
        } else {
            Ok(BoundedSparsePointMap {
                grid: self.grid,
                boundaries,
            })
        }
    }
}
impl<PT, D> SparsePointMap<PT, D>
where
    PT: PointType + 'static,
{
    /// Convert to [`BoundedSparsePointMap`] by adding boundaries.
    ///
    /// # Errors
    ///
    /// Will return `Err` if any of the points are outside the boundaries.
    pub fn with_boundaries<RX, RY>(
        self,
        boundaries: Point2Range<RX, RY>,
    ) -> Result<BoundedSparsePointMap<PT, D, Point2Range<RX, RY>>, String>
    where
        RX: RangeBounds<PT> + Debug,
        RY: RangeBounds<PT> + Debug,
    {
        let bounded = BoundedSparsePointMap {
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
    fn contains_point() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_eq!(grid.contains_point(&Point2::new(1, 2)), true);
        assert_eq!(grid.contains_point(&Point2::new(2, 2)), false);
    }

    #[test]
    fn into_iter_points() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(
            grid.into_iter_points(),
            Point2::new(1, 2),
            Point2::new(2, 3),
        );
    }

    #[test]
    fn area() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 1), 4), (Point2::new(2, 0), 6)]
            .into_iter()
            .collect();
        assert_eq!(grid.area(), (Point2::new(1, 0), Point2::new(2, 1)));
    }

    #[test]
    fn index_present() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_eq!(grid[Point2::new(1, 2)], 4);
    }

    #[test]
    #[should_panic = "no entry found for key"]
    fn index_missing() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        let _ = grid[Point2::new(2, 2)];
    }

    #[test]
    fn index_mut_present() {
        let mut grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        grid[Point2::new(1, 2)] = 6;
        assert_eq!(grid.get(&Point2::new(1, 2)), Some(&6));
    }

    #[test]
    #[should_panic = "no entry found for key"]
    fn index_mut_missing() {
        let mut grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        grid[Point2::new(2, 2)] = 0;
    }

    #[test]
    fn get_many_mut() {
        let mut grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        let points: [Point2; 2] = [Point2::new(1, 2), Point2::new(2, 2)];
        assert_unordered_eq!(
            grid.get_many_mut(points.iter()),
            (&Point2::new(1, 2), Some(&mut 4)),
            (&Point2::new(2, 2), None),
        );
    }

    #[test]
    fn into_iter_data() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(grid.into_iter_data(), 4, 6);
    }

    #[test]
    fn iter_data() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(grid.iter_data(), &4, &6);
    }

    #[test]
    fn iter_mut_data() {
        let mut grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(grid.iter_mut_data(), &mut 4, &mut 6);
    }

    #[test]
    fn into_iter_pairs() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(
            grid.into_iter_pairs(),
            (Point2::new(1, 2), 4),
            (Point2::new(2, 3), 6),
        );
    }

    #[test]
    fn iter_pairs() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(
            grid.iter_pairs(),
            (&Point2::new(1, 2), &4),
            (&Point2::new(2, 3), &6),
        );
    }

    #[test]
    fn iter_mut_pairs() {
        let mut grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert_unordered_eq!(
            grid.iter_mut_pairs(),
            (&Point2::new(1, 2), &mut 4),
            (&Point2::new(2, 3), &mut 6),
        );
    }

    #[test]
    fn with_boundaries() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        assert!(
            grid.clone()
                .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
                .is_ok()
        );
        assert!(
            grid.with_boundaries((Point2::new(0, 0)..Point2::new(3, 3)).into())
                .is_err()
        );
    }

    #[test]
    fn bounded_with_boundaries() {
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
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
        let grid: SparsePointMap<_, _> = [(Point2::new(1, 2), 4), (Point2::new(2, 3), 6)]
            .into_iter()
            .collect();
        let grid = grid
            .with_boundaries((Point2::new(0, 0)..=Point2::new(3, 3)).into())
            .unwrap();
        assert_eq!(
            grid.boundaries(),
            &(Point2::new(0, 0)..=Point2::new(3, 3)).into()
        );
    }
}
