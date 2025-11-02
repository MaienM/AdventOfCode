use std::{
    fmt::Debug,
    mem::{self},
    ops::{Index, IndexMut},
};

use inherit_methods_macro::inherit_methods;
use itertools::Itertools;

use super::{PointBoundaries, PointCollection, PointCollectionInsertResult, PointDataCollection};
use crate::{grid::internal::PointBoundariesImpl, point::Point2};

/// A 2-dimensional grid with all points present & some arbitrary data stored for each point.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FullGrid<D> {
    points: Vec<Point2<usize>>,
    cells: Vec<D>,
    width: usize,
    height: usize,
    boundaries: PointBoundariesImpl<Point2<usize>>,
}
impl<D> PointCollection<Point2<usize>> for FullGrid<D> {
    fn contains_point(&self, point: &Point2<usize>) -> bool {
        self.boundaries.in_boundaries(point)
    }

    fn into_iter_points(self) -> impl Iterator<Item = Point2<usize>> {
        self.points.into_iter()
    }

    fn iter_points(&self) -> impl Iterator<Item = &Point2<usize>> {
        self.points.iter()
    }

    fn area(&self) -> (Point2<usize>, Point2<usize>) {
        (Point2::new(0, 0), Point2::new(self.width, self.height))
    }
}
impl<D> Index<Point2<usize>> for FullGrid<D> {
    type Output = D;

    fn index(&self, point: Point2<usize>) -> &Self::Output {
        assert!(point.x < self.width, "x is out of bounds");
        assert!(point.y < self.height, "y is out of bounds");
        unsafe { self.cells.get_unchecked(point.y * self.width + point.x) }
    }
}
impl<D> IndexMut<Point2<usize>> for FullGrid<D> {
    fn index_mut(&mut self, point: Point2<usize>) -> &mut Self::Output {
        assert!(point.x < self.width, "x is out of bounds");
        assert!(point.y < self.height, "y is out of bounds");
        unsafe { self.cells.get_unchecked_mut(point.y * self.width + point.x) }
    }
}
impl<D: 'static> PointDataCollection<Point2<usize>, D> for FullGrid<D> {
    fn get(&self, point: &Point2<usize>) -> Option<&D> {
        if point.x < self.width {
            self.cells.get(point.y * self.width + point.x)
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self, point: &Point2<usize>) -> &D {
        // Note that this can return a valid reference to the wrong element when out of bounds.
        // E.g. for a 2x2 grid getting (2, 0) will result in returning the element for (1, 1).
        unsafe { self.cells.get_unchecked(point.y * self.width + point.x) }
    }

    fn get_mut(&mut self, point: &Point2<usize>) -> Option<&mut D> {
        if point.x < self.width {
            self.cells.get_mut(point.y * self.width + point.x)
        } else {
            None
        }
    }

    unsafe fn get_unchecked_mut(&mut self, point: &Point2<usize>) -> &mut D {
        // Note that this can return a valid reference to the wrong element when out of bounds.
        // E.g. for a 2x2 grid getting (2, 0) will result in returning the element for (1, 1).
        unsafe { self.cells.get_unchecked_mut(point.y * self.width + point.x) }
    }

    fn get_many<'a, I>(&self, points: I) -> impl Iterator<Item = (&'a Point2<usize>, Option<&D>)>
    where
        I: Iterator<Item = &'a Point2<usize>>,
    {
        points.map(|point| (point, self.get(point)))
    }

    unsafe fn get_many_unchecked<'a, I>(
        &self,
        points: I,
    ) -> impl Iterator<Item = (&'a Point2<usize>, &D)>
    where
        I: Iterator<Item = &'a Point2<usize>>,
    {
        unsafe { points.map(|point| (point, self.get_unchecked(point))) }
    }

    fn get_many_mut<'a, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (&'a Point2<usize>, Option<&mut D>)>
    where
        I: Iterator<Item = &'a Point2<usize>>,
    {
        let slice: *mut [D] = self.cells.as_mut_slice();
        points.into_iter().map(move |point| {
            let value = unsafe {
                if point.x < self.width && point.y < self.height {
                    Some((&mut *slice).get_unchecked_mut(point.y * self.width + point.x))
                } else {
                    None
                }
            };
            (point, value)
        })
    }

    unsafe fn get_many_unchecked_mut<'a, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (&'a Point2<usize>, &mut D)>
    where
        I: Iterator<Item = &'a Point2<usize>>,
    {
        // Note that this can return a valid reference to the wrong element when out of bounds.
        // E.g. for a 2x2 grid getting (2, 0) will result in returning the element for (1, 1).
        let slice: *mut [D] = self.cells.as_mut_slice();
        points.into_iter().map(move |point| {
            let value = unsafe { (&mut *slice).get_unchecked_mut(point.y * self.width + point.x) };
            (point, value)
        })
    }

    fn insert(&mut self, point: Point2<usize>, mut data: D) -> PointCollectionInsertResult<D> {
        if let Some(value) = self.get_mut(&point) {
            mem::swap(value, &mut data);
            PointCollectionInsertResult::Replaced(data)
        } else {
            PointCollectionInsertResult::OutOfBounds
        }
    }

    fn remove(&mut self, point: &Point2<usize>) -> Option<D> {
        if self.in_boundaries(point) {
            panic!("points cannot be removed from a FullGrid");
        } else {
            None
        }
    }

    fn into_iter_data(self) -> impl Iterator<Item = D> {
        self.cells.into_iter()
    }

    fn iter_data(&self) -> impl Iterator<Item = &D> {
        self.cells.iter()
    }

    fn iter_mut_data(&mut self) -> impl Iterator<Item = &mut D> {
        self.cells.iter_mut()
    }

    fn into_iter_pairs(self) -> impl Iterator<Item = (Point2<usize>, D)> {
        self.points.into_iter().zip(self.cells)
    }

    fn iter_pairs(&self) -> impl Iterator<Item = (&Point2<usize>, &D)> {
        self.points.iter().zip(self.cells.iter())
    }

    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&Point2<usize>, &mut D)> {
        self.points.iter().zip(self.cells.iter_mut())
    }
}
#[inherit_methods(from = "self.boundaries")]
impl<D> PointBoundaries<Point2<usize>> for FullGrid<D> {
    fn boundaries(&self) -> (&Point2<usize>, &Point2<usize>);
    fn in_boundaries(&self, point: &Point2<usize>) -> bool;
}

// Create empty.
impl<D> FullGrid<D>
where
    D: Default,
{
    /// Create a new grid of the given dimensions with all points set to the default value of the
    /// contained type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = FullGrid::new_default(2, 2);
    /// assert_eq!(grid.get(&Point2::new(0, 1)), Some(&0));
    /// assert_eq!(grid.get(&Point2::new(2, 1)), None);
    /// ```
    pub fn new_default(width: usize, height: usize) -> Self {
        assert!(width > 0, "must have width > 0");
        assert!(height > 0, "must have height > 0");

        let mut cells = Vec::with_capacity(width * height);
        cells.resize_with(width * height, D::default);

        Self {
            points: (0..height)
                .flat_map(|y| (0..width).map(move |x| Point2::new(x, y)))
                .collect(),
            cells,
            width,
            height,
            boundaries: PointBoundariesImpl::new(
                Point2::new(0, 0),
                Point2::new(width - 1, height - 1),
            ),
        }
    }
}

// Create from variations of lists of lists.
impl<II, D> FromIterator<II> for FullGrid<D>
where
    II: Iterator<Item = D>,
{
    fn from_iter<OI: IntoIterator<Item = II>>(iter: OI) -> Self {
        let cells = iter
            .into_iter()
            .map(Iterator::collect::<Vec<_>>)
            .collect::<Vec<_>>();
        cells.into()
    }
}
impl<D> From<Vec<Vec<D>>> for FullGrid<D> {
    fn from(cells: Vec<Vec<D>>) -> Self {
        let height = cells.len();
        assert!(height > 0, "must have height > 0");

        let width = cells[0].len();
        assert!(width > 0, "must have width > 0");
        for (i, row) in cells.iter().enumerate().skip(1) {
            assert_eq!(
                row.len(),
                width,
                "Rows have different lengths (row 0 vs row {i})"
            );
        }

        Self {
            points: (0..height)
                .flat_map(|y| (0..width).map(move |x| Point2::new(x, y)))
                .collect(),
            cells: cells.into_iter().flatten().collect(),
            width,
            height,
            boundaries: PointBoundariesImpl::new(
                Point2::new(0, 0),
                Point2::new(width - 1, height - 1),
            ),
        }
    }
}
impl<D, const R: usize, const C: usize> From<[[D; R]; C]> for FullGrid<D> {
    fn from(cells: [[D; R]; C]) -> Self {
        cells
            .into_iter()
            .map(std::convert::Into::into)
            .collect::<Vec<_>>()
            .into()
    }
}

// Get dimensions.
impl<D> FullGrid<D> {
    /// Get the width of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.width(), 2);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }
}

// By-row iterators.
impl<D> FullGrid<D>
where
    D: 'static,
{
    /// Iterate over the rows.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.into_iter_rows();
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![1, 2]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![3, 4]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![5, 6]));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn into_iter_rows(self) -> impl Iterator<Item: Iterator<Item = D>> {
        self.cells
            .into_iter()
            .batching(move |iter| Some(iter.take(self.width).collect::<Vec<_>>().into_iter()))
            .take(self.height)
    }

    /// Iterate over refrences to the rows.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_rows();
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&1, &2]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&3, &4]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&5, &6]));
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter_rows(&self) -> impl Iterator<Item: Iterator<Item = &D>> {
        self.cells.chunks(self.width).map(|row| row.iter())
    }

    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let mut iter = grid.iter_mut_rows();
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&mut 1, &mut 2]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&mut 3, &mut 4]));
    /// assert_eq!(iter.next().map(Iterator::collect), Some(vec![&mut 5, &mut 6]));
    /// assert!(iter.next().is_none());
    /// ```
    /// Iterate over mutable references to the rows.
    pub fn iter_mut_rows(&mut self) -> impl Iterator<Item: Iterator<Item = &mut D>> {
        self.cells.chunks_mut(self.width).map(|row| row.iter_mut())
    }
}

// Map data to get another grid.
impl<D> FullGrid<D>
where
    D: 'static,
{
    /// Return a grid with the same dimensions as `self`, with function `f` applied to eachcell in
    /// order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// let grid = grid.map(|v| v * 2);
    /// assert_eq!(grid.get(&Point2::new(0, 1)), Some(&6));
    /// ```
    pub fn map<F, ND>(self, f: F) -> FullGrid<ND>
    where
        F: FnMut(D) -> ND,
    {
        FullGrid {
            points: self.points,
            cells: self.cells.into_iter().map(f).collect(),
            width: self.width,
            height: self.height,
            boundaries: self.boundaries,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_unordered_eq;

    #[test]
    fn from_vec() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        assert_eq!(
            grid.points,
            vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
                Point2::new(0, 2),
                Point2::new(1, 2),
            ]
        );
        assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            grid.boundaries,
            PointBoundariesImpl::new(Point2::new(0, 0), Point2::new(1, 2))
        );
    }

    #[test]
    #[should_panic = "must have width > 0"]
    fn from_vec_zero_width() {
        let _: FullGrid<_> = vec![Vec::<()>::new()].into();
    }

    #[test]
    #[should_panic = "must have height > 0"]
    fn from_vec_zero_height() {
        let _: FullGrid<_> = Vec::<Vec<()>>::new().into();
    }

    #[test]
    #[should_panic = "Rows have different lengths (row 0 vs row 1)"]
    fn from_vec_mixed_length_rows() {
        let _: FullGrid<_> = vec![vec![1, 2], vec![3]].into();
    }

    #[test]
    fn from_array() {
        let grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(
            grid.points,
            vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(2, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
                Point2::new(2, 1),
            ]
        );
        assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            grid.boundaries,
            PointBoundariesImpl::new(Point2::new(0, 0), Point2::new(2, 1))
        );
    }

    #[test]
    fn from_iters() {
        let grid: FullGrid<_> = "12,34".split(',').map(|row| row.chars()).collect();
        assert_eq!(
            grid.points,
            vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
            ]
        );
        assert_eq!(grid.cells, vec!['1', '2', '3', '4']);
        assert_eq!(
            grid.boundaries,
            PointBoundariesImpl::new(Point2::new(0, 0), Point2::new(1, 1))
        );
    }

    #[test]
    fn area() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        assert_eq!(grid.area(), (Point2::new(0, 0), Point2::new(2, 3)));
    }

    #[test]
    fn index_present() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        assert_eq!(grid[Point2::new(1, 1)], 4);
    }

    #[test]
    #[should_panic = "x is out of bounds"]
    fn index_bounds_x() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        let _ = grid[Point2::new(2, 2)];
    }

    #[test]
    #[should_panic = "y is out of bounds"]
    fn index_bounds_y() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        let _ = grid[Point2::new(1, 3)];
    }

    #[test]
    fn index_mut_present() {
        let mut grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        grid[Point2::new(1, 1)] = 6;
        assert_eq!(grid.get(&Point2::new(1, 1)), Some(&6));
    }

    #[test]
    #[should_panic = "x is out of bounds"]
    fn index_mut_bounds_x() {
        let mut grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        grid[Point2::new(2, 2)] = 0;
    }

    #[test]
    #[should_panic = "y is out of bounds"]
    fn index_mut_bounds_y() {
        let mut grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        grid[Point2::new(1, 3)] = 0;
    }

    #[test]
    #[should_panic = "must have width > 0"]
    fn new_default_zero_width() {
        FullGrid::<()>::new_default(0, 1);
    }

    #[test]
    #[should_panic = "must have height > 0"]
    fn new_default_zero_height() {
        FullGrid::<()>::new_default(1, 0);
    }

    #[test]
    fn get_unchecked() {
        let grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        unsafe {
            assert_eq!(grid.get_unchecked(&Point2::new(0, 1)), &4);
        }
    }

    #[test]
    fn get_unchecked_mut() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        unsafe {
            assert_eq!(grid.get_unchecked_mut(&Point2::new(0, 1)), &mut 4);
        }
    }

    #[test]
    fn get_many_unchecked() {
        let grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        let points = [Point2::new(1, 0), Point2::new(0, 1)];
        assert_unordered_eq!(
            unsafe { grid.get_many_unchecked(points.iter()) },
            (&Point2::new(1, 0), &2),
            (&Point2::new(0, 1), &4),
        );
    }

    #[test]
    fn get_many_unchecked_mut() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        let points = [Point2::new(1, 0), Point2::new(0, 1)];
        assert_unordered_eq!(
            unsafe { grid.get_many_unchecked_mut(points.iter()) },
            (&Point2::new(1, 0), &mut 2),
            (&Point2::new(0, 1), &mut 4),
        );
    }

    #[test]
    fn insert() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(
            grid.insert(Point2::new(1, 1), 10),
            PointCollectionInsertResult::Replaced(5)
        );
        assert_eq!(
            grid.insert(Point2::new(3, 1), 10),
            PointCollectionInsertResult::OutOfBounds
        );
    }

    #[test]
    #[should_panic = "points cannot be removed from a FullGrid"]
    fn remove_in_bounds() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        grid.remove(&Point2::new(1, 1));
    }

    #[test]
    fn remove_out_of_bounds() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(grid.remove(&Point2::new(3, 1)), None);
    }
}
