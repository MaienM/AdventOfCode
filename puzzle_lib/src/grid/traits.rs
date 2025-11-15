use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

use num::{CheckedAdd, CheckedSub, Num};

use crate::{
    grid::internal::PointOrRef,
    point::{Point2, Point3, Point4},
};

/// Trait alias for types that can be used as a point index type.
pub trait PointType:
    Num + CheckedAdd + CheckedSub + PartialOrd + Ord + PartialEq + Eq + Hash + Copy + Debug
{
}
impl<PT> PointType for PT where
    PT: Num + CheckedAdd + CheckedSub + PartialOrd + Ord + PartialEq + Eq + Hash + Copy + Debug
{
}

/// Marker trait for types that can be used as a point type.
pub trait GridPoint: Copy + Eq + Hash {}
impl<PT> GridPoint for Point2<PT> where PT: PointType
{
}
impl<PT> GridPoint for Point3<PT> where PT: PointType
{
}
impl<PT> GridPoint for Point4<PT> where PT: PointType
{
}

/// The result of an insert operation.
#[derive(Debug, PartialEq, Eq)]
pub enum PointCollectionInsertResult<T> {
    /// Nothing was inserted because the point was outside of the boundaries of the collection.
    OutOfBounds,
    /// The value was newly inserted.
    Inserted,
    /// The value was inserted and has replaced the previous value for this point.
    Replaced(T),
}

/// A collection of points.
pub trait PointCollection<P>
where
    P: GridPoint + 'static,
{
    /// Returns `true` if the collection contains a point.
    ///
    /// Will always return `false` for a point outside the boundaries of a [`PointBoundaries`] collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.contains_point(&Point2::new(0, 1)), true);
    /// assert_eq!(grid.contains_point(&Point2::new(2, 0)), false);
    /// ```
    fn contains_point(&self, point: &P) -> bool;

    /// Iterate over the points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.into_iter_points();
    /// assert_eq!(iter.next(), Some(Point2::new(0, 0)));
    /// assert_eq!(iter.next(), Some(Point2::new(1, 0)));
    /// assert_eq!(iter.next(), Some(Point2::new(0, 1)));
    /// assert_eq!(iter.next(), Some(Point2::new(1, 1)));
    /// assert_eq!(iter.next(), Some(Point2::new(0, 2)));
    /// assert_eq!(iter.next(), Some(Point2::new(1, 2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter_points(self) -> impl Iterator<Item = P>;

    /// Iterate over refrences to the points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_points();
    /// assert_eq!(iter.next(), Some(&Point2::new(0, 0)));
    /// assert_eq!(iter.next(), Some(&Point2::new(1, 0)));
    /// assert_eq!(iter.next(), Some(&Point2::new(0, 1)));
    /// assert_eq!(iter.next(), Some(&Point2::new(1, 1)));
    /// assert_eq!(iter.next(), Some(&Point2::new(0, 2)));
    /// assert_eq!(iter.next(), Some(&Point2::new(1, 2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_points(&self) -> impl Iterator<Item = &P>;

    /// Get the area that the current points fall in.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let grid: SparsePointSet<isize> = [Point2::new(-2, 1), Point2::new(3, -1), Point2::new(0, 4)].into_iter().collect();
    /// assert_eq!(grid.area(), (Point2::new(-2, -1), Point2::new(3, 4)));
    /// ```
    fn area(&self) -> (P, P);
}

/// A collection of points without associated data.
pub trait PointOnlyCollection<P>
where
    Self: PointCollection<P>,
    P: GridPoint + 'static,
{
    /// Add a point to the collection.
    ///
    /// Returns whether the point was inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: SparsePointSet<u8> = [Point2::new(0, 1), Point2::new(0, 4)].into_iter().collect();
    /// assert_eq!(grid.contains_point(&Point2::new(2, 1)), false);
    /// assert_eq!(grid.insert(Point2::new(2, 1)), PointCollectionInsertResult::Inserted);
    /// assert_eq!(grid.contains_point(&Point2::new(2, 1)), true);
    /// assert_eq!(grid.insert(Point2::new(2, 1)), PointCollectionInsertResult::Replaced(()));
    /// ```
    fn insert(&mut self, point: P) -> PointCollectionInsertResult<()>;

    /// Remove a point from the collection.
    ///
    /// Returns whether the point was present.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: SparsePointSet<u8> = [Point2::new(0, 1), Point2::new(0, 4)].into_iter().collect();
    /// assert_eq!(grid.contains_point(&Point2::new(0, 1)), true);
    /// assert_eq!(grid.remove(&Point2::new(0, 1)), true);
    /// assert_eq!(grid.contains_point(&Point2::new(0, 1)), false);
    /// assert_eq!(grid.remove(&Point2::new(0, 1)), false);
    /// ```
    fn remove(&mut self, point: &P) -> bool;
}

/// A collection of points with associated data.
pub trait PointDataCollection<P, D>
where
    Self: PointCollection<P> + Index<P> + IndexMut<P>,
    P: GridPoint + 'static,
    D: 'static,
{
    /// Get a reference to an element, or `None` if not present/out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.get(&Point2::new(0, 1)), Some(&3));
    /// assert_eq!(grid.get(&Point2::new(0, 4)), None);
    /// assert_eq!(grid.get(&Point2::new(4, 0)), None);
    /// ```
    fn get(&self, point: &P) -> Option<&D>;

    /// Get a reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds point is undefined behavior even if the resulting
    /// reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: SparsePointMap<usize, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 2), 12)].into_iter().collect();
    /// unsafe { assert_eq!(grid.get_unchecked(&Point2::new(0, 1)), &10); }
    /// ```
    unsafe fn get_unchecked(&self, point: &P) -> &D {
        self.get(point).unwrap()
    }

    /// Get a mutable reference to an element, or `None` if not present/out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.get_mut(&Point2::new(0, 1)), Some(&mut 3));
    /// assert_eq!(grid.get_mut(&Point2::new(0, 4)), None);
    /// assert_eq!(grid.get_mut(&Point2::new(4, 0)), None);
    /// ```
    fn get_mut(&mut self, point: &P) -> Option<&mut D>;

    /// Get a mutable reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds point is undefined behavior even if the resulting
    /// reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: SparsePointMap<usize, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 2), 12)].into_iter().collect();
    /// unsafe { assert_eq!(grid.get_unchecked_mut(&Point2::new(0, 1)), &mut 10); }
    /// ```
    unsafe fn get_unchecked_mut(&mut self, point: &P) -> &mut D {
        self.get_mut(point).unwrap()
    }

    /// Get references to multiple elements at once.
    ///
    /// The resulting iterator may be in a different order and/or may skip duplicate points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(2, 2)];
    /// let pairs: Vec<(&Point2, Option<&u8>)> = grid.get_many(points.iter()).collect();
    /// assert_eq!(pairs.len(), 2);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), Some(&3))));
    /// assert!(pairs.contains(&(&Point2::new(2, 2), None)));
    /// ```
    fn get_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, Option<&D>)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>;

    /// Get references to multiple elements at once without doing bounds checking.
    ///
    /// The resulting iterator may be in a different order.
    ///
    /// For a safe alternative see [`get_many`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds point is undefined behavior even if the resulting
    /// reference is not used. Calling it with the same point multiple times is also undefined
    /// behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: SparsePointMap<usize, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 2), 12)].into_iter().collect();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(0, 2)];
    /// let pairs: Vec<(&Point2, &u8)> = unsafe { grid.get_many_unchecked(points.iter()).collect() };
    /// assert_eq!(pairs.len(), 2);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), &10)));
    /// assert!(pairs.contains(&(&Point2::new(0, 2), &12)));
    /// ```
    unsafe fn get_many_unchecked<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, &D)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        self.get_many(points).map(|(p, d)| (p, d.unwrap()))
    }

    /// Get references to multiple elements at once, dropping any points that aren't in the
    /// collection.
    ///
    /// The resulting iterator may be in a different order and/or may skip duplicate points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(2, 2)];
    /// let pairs: Vec<(&Point2, &u8)> = grid.get_filter_many(points.iter()).collect();
    /// assert_eq!(pairs.len(), 1);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), &3)));
    /// ```
    fn get_filter_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, &D)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        self.get_many(points).filter_map(|(p, d)| d.map(|d| (p, d)))
    }

    /// Get mutable references to multiple elements at once.
    ///
    /// The resulting iterator may be in a different order and/or may skip duplicate points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(2, 2)];
    /// let pairs: Vec<(&Point2, Option<&mut u8>)> = grid.get_many_mut(points.iter()).collect();
    /// assert_eq!(pairs.len(), 2);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), Some(&mut 3))));
    /// assert!(pairs.contains(&(&Point2::new(2, 2), None)));
    /// ```
    fn get_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, Option<&mut D>)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>;

    /// Get mutable references to multiple elements at once without doing bounds checking.
    ///
    /// The resulting iterator may be in a different order.
    ///
    /// For a safe alternative see [`get_many_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds point is undefined behavior even if the resulting
    /// reference is not used. Calling it with the same point multiple times is also undefined
    /// behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: SparsePointMap<usize, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 2), 12)].into_iter().collect();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(0, 2)];
    /// let pairs: Vec<(&Point2, &mut u8)> = unsafe { grid.get_many_unchecked_mut(points.iter()).collect() };
    /// assert_eq!(pairs.len(), 2);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), &mut 10)));
    /// assert!(pairs.contains(&(&Point2::new(0, 2), &mut 12)));
    /// ```
    unsafe fn get_many_unchecked_mut<PR, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>;

    /// Get mutable references to multiple elements at once, dropping any points that aren't in the
    /// collection.
    ///
    /// The resulting iterator may be in a different order and/or may skip duplicate points.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let points: [Point2; 2] = [Point2::new(0, 1), Point2::new(2, 2)];
    /// let pairs: Vec<(&Point2, &mut u8)> = grid.get_filter_many_mut(points.iter()).collect();
    /// assert_eq!(pairs.len(), 1);
    /// assert!(pairs.contains(&(&Point2::new(0, 1), &mut 3)));
    /// ```
    fn get_filter_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<P>,
        I: Iterator<Item = PR>,
    {
        self.get_many_mut(points)
            .filter_map(|(p, d)| d.map(|d| (p, d)))
    }

    /// Add a point + associated data to the collection.
    ///
    /// Replaces the existing data for the point, if any.
    ///
    /// Returns whether the point + data were inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: SparsePointMap<u8, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 2), 12)].into_iter().collect();
    /// let mut grid: BoundedSparsePointMap<u8, u8> = grid.with_boundaries((&Point2::new(0, 0), &Point2::new(2, 2))).unwrap();
    /// assert_eq!(grid.get(&Point2::new(2, 1)), None);
    /// assert_eq!(grid.insert(Point2::new(2, 1), 6), PointCollectionInsertResult::Inserted);
    /// assert_eq!(grid.get(&Point2::new(2, 1)), Some(&6));
    /// assert_eq!(grid.insert(Point2::new(0, 1), 8), PointCollectionInsertResult::Replaced(10));
    /// assert_eq!(grid.get(&Point2::new(0, 1)), Some(&8));
    /// assert_eq!(grid.insert(Point2::new(3, 0), 6), PointCollectionInsertResult::OutOfBounds);
    /// assert_eq!(grid.get(&Point2::new(3, 0)), None);
    /// ```
    fn insert(&mut self, point: P, data: D) -> PointCollectionInsertResult<D>;

    /// Remove a point from the collection.
    ///
    /// Returns `None` if the point was not present, and the data for the point if it was.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: SparsePointMap<u8, u8> = [(Point2::new(0, 1), 10), (Point2::new(0, 4), 12)].into_iter().collect();
    /// assert_eq!(grid.get(&Point2::new(0, 1)), Some(&10));
    /// assert_eq!(grid.remove(&Point2::new(0, 1)), Some(10));
    /// assert_eq!(grid.get(&Point2::new(0, 1)), None);
    /// assert_eq!(grid.remove(&Point2::new(0, 1)), None);
    /// ```
    fn remove(&mut self, point: &P) -> Option<D>;

    /// Iterate over the data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.into_iter_data();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(4));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.next(), Some(6));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter_data(self) -> impl Iterator<Item = D>;

    /// Iterate over refrences to the data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_data();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&6));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_data(&self) -> impl Iterator<Item = &D>;

    /// Iterate over mutable references to the data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_mut_data();
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// assert_eq!(iter.next(), Some(&mut 5));
    /// assert_eq!(iter.next(), Some(&mut 6));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_mut_data(&mut self) -> impl Iterator<Item = &mut D>;

    /// Iterate over pairs of the points and associated data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.into_iter_pairs();
    /// assert_eq!(iter.next(), Some((Point2::new(0, 0), 1)));
    /// assert_eq!(iter.next(), Some((Point2::new(1, 0), 2)));
    /// assert_eq!(iter.next(), Some((Point2::new(0, 1), 3)));
    /// assert_eq!(iter.next(), Some((Point2::new(1, 1), 4)));
    /// assert_eq!(iter.next(), Some((Point2::new(0, 2), 5)));
    /// assert_eq!(iter.next(), Some((Point2::new(1, 2), 6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter_pairs(self) -> impl Iterator<Item = (P, D)>;

    /// Iterate over pairs of references to the points and associated data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_pairs();
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 0), &1)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 0), &2)));
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 1), &3)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 1), &4)));
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 2), &5)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 2), &6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_pairs(&self) -> impl Iterator<Item = (&P, &D)>;

    /// Iterate over pairs of references to the points and mutable references to the associated data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_mut_pairs();
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 0), &mut 1)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 0), &mut 2)));
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 1), &mut 3)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 1), &mut 4)));
    /// assert_eq!(iter.next(), Some((&Point2::new(0, 2), &mut 5)));
    /// assert_eq!(iter.next(), Some((&Point2::new(1, 2), &mut 6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&P, &mut D)>;
}

/// A collection with strict boundaries in which all the points must fall.
pub trait PointBoundaries<P> {
    /// Get the minimum and maximum point allowed within the boundaries (both inclusive).
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.boundaries(), (&Point2::new(0, 0), &Point2::new(1, 2)));
    /// ```
    fn boundaries(&self) -> (&P, &P);

    /// Check whether a point is inside the boundaries of this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.in_boundaries(&Point2::new(0, 1)), true);
    /// assert_eq!(grid.in_boundaries(&Point2::new(3, 1)), false);
    /// ```
    fn in_boundaries(&self, point: &P) -> bool;
}
