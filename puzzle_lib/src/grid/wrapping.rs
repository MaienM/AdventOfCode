use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

use inherit_methods_macro::inherit_methods;

use crate::{
    grid::{
        GridPoint, PointBoundaries, PointCollection, PointCollectionInsertResult,
        PointDataCollection, PointOnlyCollection,
    },
    point::WrappablePointRange,
};

// A wrapper around a bounded grid that will wrap all points that are outside the boundaries to the
// other side.
//
// For example, if the x boundaries are `0..50` an x coordinate of `51` will be equivalent to `1`.
#[derive(Debug, Eq, Clone)]
pub struct WrappingGrid<P, R, G>
where
    P: GridPoint,
    R: WrappablePointRange<P>,
    G: PointBoundaries<P, R>,
{
    grid: G,
    boundaries: R,
    phantom: PhantomData<(P, R)>,
}

impl<P, R, G> PartialEq for WrappingGrid<P, R, G>
where
    P: GridPoint,
    R: WrappablePointRange<P>,
    G: PointBoundaries<P, R> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}
impl<P, R, G> Hash for WrappingGrid<P, R, G>
where
    P: GridPoint,
    R: WrappablePointRange<P>,
    G: PointBoundaries<P, R> + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl<P, R, G> From<G> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R>,
{
    fn from(grid: G) -> Self {
        Self {
            boundaries: grid.boundaries().clone(),
            grid,
            phantom: PhantomData,
        }
    }
}
impl<P, R, G> Deref for WrappingGrid<P, R, G>
where
    P: GridPoint,
    R: WrappablePointRange<P>,
    G: PointBoundaries<P, R> + PartialEq,
{
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<P, R, G> Extend<P> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + Extend<P>,
{
    fn extend<T: IntoIterator<Item = P>>(&mut self, iter: T) {
        self.grid
            .extend(iter.into_iter().map(|point| self.boundaries.wrap(point)));
    }
}
impl<P, D, R, G> Extend<(P, D)> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    D: 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + Extend<(P, D)>,
{
    fn extend<T: IntoIterator<Item = (P, D)>>(&mut self, iter: T) {
        self.grid.extend(
            iter.into_iter()
                .map(|(point, value)| (self.boundaries.wrap(point), value)),
        );
    }
}

impl<P, D, R, G> Index<P> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    D: 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + Index<P, Output = D>,
{
    type Output = D;

    fn index(&self, point: P) -> &Self::Output {
        G::index(&self.grid, self.boundaries.wrap(point))
    }
}
impl<P, D, R, G> IndexMut<P> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    D: 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + Index<P, Output = D> + IndexMut<P>,
{
    fn index_mut(&mut self, point: P) -> &mut Self::Output {
        G::index_mut(&mut self.grid, self.boundaries.wrap(point))
    }
}

#[inherit_methods(from = "self.grid")]
impl<P, R, G> PointCollection<P> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + PointCollection<P>,
{
    fn contains_point(&self, point: &P) -> bool {
        self.grid.contains_point(&self.boundaries.wrap(*point))
    }

    fn into_iter_points(self) -> impl Iterator<Item = P>;
    fn iter_points(&self) -> impl Iterator<Item = &P>;
    fn area(&self) -> (P, P);
}
#[inherit_methods(from = "self.grid")]
impl<P, R, G> PointOnlyCollection<P> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + PointOnlyCollection<P>,
{
    fn insert(&mut self, point: P) -> PointCollectionInsertResult<()> {
        self.grid.insert(self.boundaries.wrap(point))
    }

    fn remove(&mut self, point: &P) -> bool {
        self.grid.remove(&self.boundaries.wrap(*point))
    }
}
#[inherit_methods(from = "self.grid")]
impl<P, D, R, G> PointDataCollection<P, D> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    D: 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R> + PointDataCollection<P, D>,
    <G as std::ops::Index<P>>::Output: std::marker::Sized + 'static,
{
    fn get(&self, point: &P) -> Option<&D> {
        self.grid.get(&self.boundaries.wrap(*point))
    }

    fn get_mut(&mut self, point: &P) -> Option<&mut D> {
        self.grid.get_mut(&self.boundaries.wrap(*point))
    }

    #[allow(unused_variables)]
    fn get_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, Option<&D>)>
    where
        PR: super::internal::PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        panic!("not implemented for WrappingGrid.");
        #[allow(unreachable_code)]
        self.grid.get_many(points)
    }

    #[allow(unused_variables)]
    fn get_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, Option<&mut D>)>
    where
        PR: super::internal::PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        panic!("not implemented for WrappingGrid.");
        #[allow(unreachable_code)]
        self.grid.get_many_mut(points)
    }

    #[allow(unused_variables)]
    unsafe fn get_many_unchecked_mut<PR, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: super::internal::PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        panic!("not implemented for WrappingGrid.");
        #[allow(unreachable_code)]
        unsafe {
            self.grid.get_many_unchecked_mut(points)
        }
    }

    fn insert(&mut self, point: P, data: D) -> PointCollectionInsertResult<D> {
        self.grid.insert(self.boundaries.wrap(point), data)
    }

    fn remove(&mut self, point: &P) -> Option<D> {
        self.grid.remove(&self.boundaries.wrap(*point))
    }

    fn into_iter_data(self) -> impl Iterator<Item = D>;
    fn iter_data(&self) -> impl Iterator<Item = &D>;
    fn iter_mut_data(&mut self) -> impl Iterator<Item = &mut D>;
    fn into_iter_pairs(self) -> impl Iterator<Item = (P, D)>;
    fn iter_pairs(&self) -> impl Iterator<Item = (&P, &D)>;
    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&P, &mut D)>;
}
#[inherit_methods(from = "self.grid")]
impl<P, R, G> PointBoundaries<P, R> for WrappingGrid<P, R, G>
where
    P: GridPoint + 'static,
    R: WrappablePointRange<P> + Clone,
    G: PointBoundaries<P, R>,
{
    fn boundaries(&self) -> &R;
}

#[cfg(test)]
mod tests {
    use std::{
        hash::{DefaultHasher, Hasher},
        ops::{Range, RangeInclusive},
    };

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        grid::{BoundedSparsePointSet, FullGrid, PointDataCollection, SparsePointSet},
        point::{Point2, Point2Range},
    };

    #[allow(clippy::type_complexity)]
    fn create_grid()
    -> WrappingGrid<Point2<usize>, Point2Range<Range<usize>, Range<usize>>, FullGrid<usize>> {
        let grid: FullGrid<_> = [[1, 2], [3, 4], [5, 6]].into();
        WrappingGrid::from(grid)
    }

    #[allow(clippy::type_complexity)]
    fn create_point_grid() -> WrappingGrid<
        Point2<usize>,
        Point2Range<RangeInclusive<usize>, RangeInclusive<usize>>,
        BoundedSparsePointSet<usize, Point2Range<RangeInclusive<usize>, RangeInclusive<usize>>>,
    > {
        let grid: SparsePointSet<usize> =
            [Point2::new(1, 1), Point2::new(3, 4)].into_iter().collect();
        let grid = grid
            .with_boundaries((Point2::new(1, 1)..=Point2::new(5, 5)).into())
            .unwrap();
        WrappingGrid::from(grid)
    }

    #[test]
    fn partial_eq() {
        assert_eq!(create_grid(), create_grid());
    }

    #[test]
    fn hash() {
        let grid = create_grid();
        assert_eq!(
            {
                let mut hasher = DefaultHasher::new();
                grid.hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                grid.grid.hash(&mut hasher);
                hasher.finish()
            },
        );
    }

    #[test]
    fn deref() {
        let grid = create_grid();
        assert_eq!(grid.grid, *grid);
    }

    #[test]
    fn extend_data() {
        let mut grid = create_grid();
        grid.extend([(Point2::new(3, 1), 9)]);
        assert_eq!(grid[Point2::new(1, 1)], 9);
    }

    #[test]
    fn extend_point() {
        let mut grid = create_point_grid();
        assert!(!grid.contains_point(&Point2::new(5, 1)));
        grid.extend([Point2::new(0, 1)]);
        assert!(grid.contains_point(&Point2::new(5, 1)));
    }

    #[test]
    fn area() {
        let grid = create_grid();
        assert_eq!(grid.area(), (Point2::new(0, 0), Point2::new(2, 3)));
    }

    #[test]
    fn contains_point() {
        let grid = create_grid();
        assert!(grid.contains_point(&Point2::new(8, 4)));
    }

    #[test]
    fn index() {
        let grid = create_grid();
        assert_eq!(grid[Point2::new(3, 1)], 4);
    }

    #[test]
    fn index_mut() {
        let mut grid = create_grid();
        grid[Point2::new(1, 4)] = 6;
        assert_eq!(grid.get(&Point2::new(3, 1)), Some(&6));
    }

    #[test]
    fn get_unchecked() {
        let grid = create_grid();
        unsafe {
            assert_eq!(grid.get_unchecked(&Point2::new(2, 1)), &3);
        }
    }

    #[test]
    fn get_unchecked_mut() {
        let mut grid = create_grid();
        unsafe {
            assert_eq!(grid.get_unchecked_mut(&Point2::new(0, 4)), &mut 3);
        }
    }

    #[test]
    #[should_panic = "not implemented"]
    fn get_many() {
        let grid = create_grid();
        let _ = grid.get_many([].iter());
    }

    #[test]
    #[should_panic = "not implemented"]
    fn get_many_mut() {
        let mut grid = create_grid();
        let _ = grid.get_many_mut([].iter());
    }

    #[test]
    #[should_panic = "not implemented"]
    fn get_many_unchecked() {
        let grid = create_grid();
        let _ = unsafe { grid.get_many_unchecked([].iter()) };
    }

    #[test]
    #[should_panic = "not implemented"]
    fn get_many_unchecked_mut() {
        let mut grid = create_grid();
        let _ = unsafe { grid.get_many_unchecked_mut([].iter()) };
    }

    #[test]
    fn insert_data() {
        let mut grid = create_grid();
        assert_eq!(
            grid.insert(Point2::new(1, 1), 10),
            PointCollectionInsertResult::Replaced(4)
        );
        assert_eq!(
            grid.insert(Point2::new(3, 1), 15),
            PointCollectionInsertResult::Replaced(10),
        );
    }

    #[should_panic = "points cannot be removed from a FullGrid"]
    #[test]
    fn remove_data() {
        let mut grid = create_grid();
        assert_eq!(grid.remove(&Point2::new(3, 1)), None);
    }

    #[test]
    fn insert_point() {
        let mut grid = create_point_grid();
        assert!(!grid.contains_point(&Point2::new(5, 1)));
        grid.insert(Point2::new(0, 1));
        assert!(grid.contains_point(&Point2::new(5, 1)));
    }

    #[test]
    fn remove_point() {
        let mut grid = create_point_grid();
        assert!(grid.contains_point(&Point2::new(1, 1)));
        grid.remove(&Point2::new(6, 1));
        assert!(!grid.contains_point(&Point2::new(1, 1)));
    }
}
