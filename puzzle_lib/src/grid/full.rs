use std::{
    fmt::Debug,
    hash::Hash,
    iter::IntoIterator,
    mem::{self},
    ops::{Index, IndexMut, Range},
    sync::OnceLock,
};

use itertools::Itertools;

use super::{PointBoundaries, PointCollection, PointCollectionInsertResult, PointDataCollection};
use crate::{
    grid::internal::PointOrRef,
    point::{Point2, Point2Range, PointRange},
};

type Boundaries = Point2Range<Range<usize>, Range<usize>>;

/// A 2-dimensional grid with all points present & some arbitrary data stored for each point.
#[derive(Debug, Eq, Clone)]
pub struct FullGrid<D> {
    points: OnceLock<Vec<Point2<usize>>>,
    cells: Vec<D>,
    width: usize,
    height: usize,
    boundaries: Boundaries,
}

impl<D> PartialEq for FullGrid<D>
where
    D: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.cells == other.cells
    }
}
impl<D> Hash for FullGrid<D>
where
    D: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cells.hash(state);
    }
}

impl<D> FullGrid<D> {
    /// Swap two elements in the grid.
    ///
    /// If a equals to b, it’s guaranteed that elements won’t change value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use puzzle_lib::grid::*;
    /// # use puzzle_lib::point::Point2;
    /// let mut grid: FullGrid<u8> = [[1, 2], [3, 4], [5, 6]].into();
    /// assert_eq!(grid[Point2::new(1, 1)], 4);
    /// grid.swap(&Point2::new(1, 1), &Point2::new(1, 0));
    /// assert_eq!(grid[Point2::new(1, 1)], 2);
    /// ```
    pub fn swap(&mut self, a: &Point2<usize>, b: &Point2<usize>) {
        self.cells
            .swap(a.y * self.width + a.x, b.y * self.width + b.x);
    }

    fn get_points_iter(width: usize, height: usize) -> impl Iterator<Item = Point2<usize>> {
        (0..height).flat_map(move |y| (0..width).map(move |x| Point2::new(x, y)))
    }

    fn get_points(&self) -> &Vec<Point2<usize>> {
        self.points
            .get_or_init(|| FullGrid::<D>::get_points_iter(self.width, self.height).collect())
    }
}
impl<D> PointCollection<Point2<usize>> for FullGrid<D> {
    fn contains_point(&self, point: &Point2<usize>) -> bool {
        self.boundaries.contains(point)
    }

    fn into_iter_points(self) -> impl Iterator<Item = Point2<usize>> {
        FullGrid::<D>::get_points_iter(self.width, self.height)
    }

    fn iter_points(&self) -> impl Iterator<Item = &Point2<usize>> {
        self.get_points().iter()
    }

    fn area(&self) -> (Point2<usize>, Point2<usize>) {
        (Point2::new(0, 0), Point2::new(self.width, self.height))
    }
}

impl<D> Extend<(Point2<usize>, D)> for FullGrid<D>
where
    D: 'static,
{
    fn extend<T: IntoIterator<Item = (Point2<usize>, D)>>(&mut self, iter: T) {
        for (point, data) in iter {
            self.insert(point, data);
        }
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

    fn get_many<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, Option<&D>)>
    where
        PR: PointOrRef<Point2<usize>>,
        I: Iterator<Item = PR>,
    {
        points.map(|point| (point, self.get(point.resolve_ref())))
    }

    unsafe fn get_many_unchecked<PR, I>(&self, points: I) -> impl Iterator<Item = (PR, &D)>
    where
        PR: PointOrRef<Point2<usize>>,
        I: Iterator<Item = PR>,
    {
        unsafe { points.map(|point| (point, self.get_unchecked(point.resolve_ref()))) }
    }

    fn get_many_mut<PR, I>(&mut self, points: I) -> impl Iterator<Item = (PR, Option<&mut D>)>
    where
        PR: PointOrRef<Point2<usize>>,
        I: Iterator<Item = PR>,
    {
        let slice: *mut [D] = self.cells.as_mut_slice();
        points.into_iter().map(move |point| {
            let pr = point.resolve_ref();
            let value = unsafe {
                if pr.x < self.width && pr.y < self.height {
                    Some((&mut *slice).get_unchecked_mut(pr.y * self.width + pr.x))
                } else {
                    None
                }
            };
            (point, value)
        })
    }

    unsafe fn get_many_unchecked_mut<PR, I>(
        &mut self,
        points: I,
    ) -> impl Iterator<Item = (PR, &mut D)>
    where
        PR: PointOrRef<Point2<usize>>,
        I: Iterator<Item = PR>,
    {
        // Note that this can return a valid reference to the wrong element when out of bounds.
        // E.g. for a 2x2 grid getting (2, 0) will result in returning the element for (1, 1).
        let slice: *mut [D] = self.cells.as_mut_slice();
        points.into_iter().map(move |point| {
            let pr = point.resolve_ref();
            let value = unsafe { (&mut *slice).get_unchecked_mut(pr.y * self.width + pr.x) };
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
        if self.boundaries.contains(point) {
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
        FullGrid::<D>::get_points_iter(self.width, self.height).zip(self.cells)
    }

    fn iter_pairs(&self) -> impl Iterator<Item = (&Point2<usize>, &D)> {
        self.get_points().iter().zip(self.cells.iter())
    }

    fn iter_mut_pairs(&mut self) -> impl Iterator<Item = (&Point2<usize>, &mut D)> {
        self.get_points();
        self.points.get().unwrap().iter().zip(self.cells.iter_mut())
    }
}
impl<D> PointBoundaries<Point2<usize>, Boundaries> for FullGrid<D> {
    fn boundaries(&self) -> &Boundaries {
        &self.boundaries
    }
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
            points: OnceLock::new(),
            cells,
            width,
            height,
            boundaries: (Point2::new(0, 0)..Point2::new(width, height)).into(),
        }
    }
}

// Create from variations of lists of lists.
impl<II, D> FromIterator<II> for FullGrid<D>
where
    II: Iterator<Item = D>,
{
    fn from_iter<OI: IntoIterator<Item = II>>(iter: OI) -> Self {
        let iter = iter.into_iter();
        let min_height = iter.size_hint().0;
        let mut width = 0;
        let mut cells = Vec::new();

        for (y, row) in iter.enumerate() {
            cells.extend(row);
            if y == 0 {
                width = cells.len();
                assert!(width > 0, "must have width > 0");
                cells.reserve(min_height * width);
            } else {
                let row_width = cells.len() - (y * width);
                assert_eq!(
                    row_width, width,
                    "Rows have different lengths (row 0 vs row {y})"
                );
            }
        }

        // If we get to this point with width == 0 that must mean that the above loop had 0
        // iterators (i.e., 0 rows) as otherwise the assertion in that loop would have already
        // panicked, so this means a height of 0.
        assert!(width > 0, "must have height > 0");
        let height = cells.len() / width;

        Self {
            points: OnceLock::new(),
            cells,
            width,
            height,
            boundaries: (Point2::new(0, 0)..Point2::new(width, height)).into(),
        }
    }
}
impl<D> From<Vec<Vec<D>>> for FullGrid<D> {
    fn from(cells: Vec<Vec<D>>) -> Self {
        cells.into_iter().map(IntoIterator::into_iter).collect()
    }
}
impl<D, const R: usize, const C: usize> From<[[D; R]; C]> for FullGrid<D> {
    fn from(cells: [[D; R]; C]) -> Self {
        Self {
            points: OnceLock::new(),
            cells: cells.into_iter().flatten().collect(),
            width: R,
            height: C,
            boundaries: (Point2::new(0, 0)..Point2::new(R, C)).into(),
        }
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
    use std::hash::{DefaultHasher, Hasher};

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_unordered_eq;

    #[test]
    fn from_vec() {
        let grid: FullGrid<_> = vec![vec![1, 2], vec![3, 4], vec![5, 6]].into();
        assert_eq!(grid.points.get(), None);
        assert_eq!(
            grid.get_points(),
            &vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
                Point2::new(0, 2),
                Point2::new(1, 2),
            ],
        );
        assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            grid.boundaries,
            (Point2::new(0, 0)..Point2::new(2, 3)).into()
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
        assert_eq!(grid.points.get(), None);
        assert_eq!(
            grid.get_points(),
            &vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(2, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
                Point2::new(2, 1),
            ],
        );
        assert_eq!(grid.cells, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(
            grid.boundaries,
            (Point2::new(0, 0)..Point2::new(3, 2)).into(),
        );
    }

    #[test]
    fn from_iters() {
        let grid: FullGrid<_> = "12,34".split(',').map(|row| row.chars()).collect();
        assert_eq!(grid.points.get(), None);
        assert_eq!(
            grid.get_points(),
            &vec![
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(0, 1),
                Point2::new(1, 1),
            ],
        );
        assert_eq!(grid.cells, vec!['1', '2', '3', '4']);
        assert_eq!(
            grid.boundaries,
            (Point2::new(0, 0)..Point2::new(2, 2)).into(),
        );
    }

    #[test]
    fn partial_eq() {
        let grid1: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        let grid2: FullGrid<_> = vec![vec![1, 2, 3], vec![4, 5, 6]].into();
        assert_eq!(grid1, grid2);
    }

    #[test]
    fn hash() {
        let grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        assert_eq!(
            {
                let mut hasher = DefaultHasher::new();
                grid.hash(&mut hasher);
                hasher.finish()
            },
            {
                let mut hasher = DefaultHasher::new();
                grid.cells.hash(&mut hasher);
                hasher.finish()
            },
        );
    }

    #[test]
    fn extend() {
        let mut grid: FullGrid<_> = [[1, 2, 3], [4, 5, 6]].into();
        grid.extend([(Point2::new(1, 1), 9)]);
        assert_eq!(grid[Point2::new(1, 1)], 9);
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
