use std::slice::Iter;
use std::{fmt::Debug, vec::IntoIter};

#[derive(Clone, Copy, Eq, Hash, PartialEq, new)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}
impl<T: Debug> Debug for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&format!("Point({:?}, {:?})", self.x, self.y));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T = i32> {
    items: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}
impl<T: Debug> Grid<T> {
    pub fn new(items: Vec<Vec<T>>) -> Result<Self, String> {
        let height = items.len();
        if height == 0 {
            return Err("Grid cannot be empty.".to_string());
        }

        let width = items[0].len();
        if width == 0 {
            return Err("Grid rows cannot be empty.".to_string());
        }
        for (i, row) in items.iter().enumerate() {
            let len = row.len();
            if len != width {
                return Err(format!(
                    "Grid rows must have consistent length, row 0 is {} and row {} is {}.",
                    width, i, len
                ));
            }
        }

        return Ok(Self {
            items,
            width,
            height,
        });
    }

    pub fn get<'a>(&'a self, x: usize, y: usize) -> Option<&'a T> {
        return self.items.get(y).and_then(|row| row.get(x));
    }

    pub fn getp<'a>(&'a self, point: Point) -> Option<&'a T> {
        return self.get(point.x, point.y);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.items[y][x] = value;
    }

    pub fn setp(&mut self, point: Point, value: T) {
        self.set(point.x, point.y, value);
    }

    pub fn mutate<F: Fn(T) -> T>(&mut self, x: usize, y: usize, mutator: F) {
        let row = &mut self.items[y];
        row.swap(x, self.width - 1);
        let value = row.pop().unwrap();
        row.push(mutator(value));
        row.swap(x, self.width - 1);
    }

    pub fn mutatep<F: Fn(T) -> T>(&mut self, point: Point, mutator: F) {
        self.mutate(point.x, point.y, mutator);
    }

    pub fn iter(&self) -> Iter<Vec<T>> {
        return self.items.iter();
    }

    pub fn neighbours(&self, point: Point, include_diagonals: bool) -> Vec<Point> {
        let mut results: Vec<Point> = Vec::new();

        if point.x > 0 {
            results.push(Point::new(point.x - 1, point.y));
        }
        if point.x < self.width - 1 {
            results.push(Point::new(point.x + 1, point.y));
        }
        if point.y > 0 {
            results.push(Point::new(point.x, point.y - 1));
        }
        if point.y < self.height - 1 {
            results.push(Point::new(point.x, point.y + 1));
        }

        if include_diagonals {
            if point.x > 0 && point.y > 0 {
                results.push(Point::new(point.x - 1, point.y - 1));
            }
            if point.x > 0 && point.y < self.height - 1 {
                results.push(Point::new(point.x - 1, point.y + 1));
            }
            if point.x < self.width - 1 && point.y > 0 {
                results.push(Point::new(point.x + 1, point.y - 1));
            }
            if point.x < self.width - 1 && point.y < self.height - 1 {
                results.push(Point::new(point.x + 1, point.y + 1));
            }
        }

        return results;
    }

    pub fn pprint(&self) {
        println!("Grid({}x{})", self.width, self.height);
        for row in &self.items {
            println!("{:?}", row);
        }
    }
}

impl<T: Debug> From<Vec<Vec<T>>> for Grid<T> {
    fn from(items: Vec<Vec<T>>) -> Self {
        return Self::new(items).unwrap();
    }
}
impl<T: Debug> Into<Vec<Vec<T>>> for Grid<T> {
    fn into(self) -> Vec<Vec<T>> {
        return self.items;
    }
}
impl<T: Debug> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Grid<T> {
        let items = iter.into_iter().collect::<Vec<Vec<T>>>();
        return Self::new(items).unwrap();
    }
}
impl<T: Debug> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.items.into_iter();
    }
}

type GridCell<T> = (Point<usize>, T);
impl<T> Grid<T> {
    pub fn into_by_cell(self) -> impl Iterator<Item = GridCell<T>> {
        return self.items.into_iter().enumerate().flat_map(|(y, row)| {
            return row.into_iter().enumerate().map(move |(x, value)| {
                return (Point::new(x, y), value);
            });
        });
    }

    pub fn by_cell<'a>(&'a self) -> impl Iterator<Item = GridCell<&'a T>> {
        return self.items.iter().enumerate().flat_map(|(y, row)| {
            return row.iter().enumerate().map(move |(x, value)| {
                return (Point::new(x, y), value);
            });
        });
    }
}
impl<T: Debug> FromIterator<GridCell<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = GridCell<T>>>(iter: I) -> Grid<T> {
        let mut next_x = 0_usize;
        let mut next_y = 0_usize;
        let mut width = usize::MAX;
        let mut items: Vec<Vec<T>> = Vec::new();
        let mut row: Vec<T> = Vec::new();

        for (point, value) in iter {
            if point.x == next_x && point.y == next_y {
                row.push(value);
                next_x += 1;
                if next_x >= width {
                    next_x = 0;
                    next_y += 1;

                    items.push(row);
                    row = Vec::new();
                }
            } else if next_x > 1 && next_y == 0 && point.x == 0 && point.y == 1 {
                // First row ended.
                next_x = 1;
                next_y = 1;
                width = row.len();

                items.push(row);
                row = Vec::new();
                row.push(value);
            } else {
                if next_x == 0 && next_y == 0 {
                    panic!("Expected point (0, 0), got ({}, {}).", point.x, point.y);
                } else if next_y == 0 {
                    panic!(
                        "Expected point ({}, 0) or (0, 1), got ({}, {}).",
                        next_x, point.x, point.y
                    );
                } else {
                    panic!(
                        "Expected point ({}, {}), got ({}, {}).",
                        next_x, next_y, point.x, point.y
                    );
                }
            }
        }
        if !row.is_empty() {
            items.push(row);
        }

        return Self::new(items).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::{
        fmt::Debug,
        panic::{catch_unwind, UnwindSafe},
    };

    use super::*;

    fn basic_grid() -> Grid {
        return Grid::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![0, 0, 0],
        ])
        .unwrap();
    }

    #[test]
    fn new() {
        let grid = basic_grid();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 4);
    }

    #[test]
    fn new_invalid() {
        assert_eq!(
            Grid::new(vec![] as Vec<Vec<i32>>),
            Err("Grid cannot be empty.".to_string())
        );
        assert_eq!(
            Grid::new(vec![vec![]] as Vec<Vec<i32>>),
            Err("Grid rows cannot be empty.".to_string())
        );
        assert_eq!(
            Grid::new(vec![vec![1, 2, 3], vec![5, 6], vec![7, 8, 9]]),
            Err("Grid rows must have consistent length, row 0 is 3 and row 1 is 2.".to_string())
        );
    }

    #[test]
    fn get() {
        let grid = basic_grid();
        assert_eq!(*grid.get(0, 0).unwrap(), 1);
        assert_eq!(*grid.get(1, 1).unwrap(), 5);
        assert_eq!(*grid.get(2, 2).unwrap(), 9);
        assert!(grid.get(3, 3).is_none());
    }

    #[test]
    fn getp() {
        let grid = basic_grid();
        assert_eq!(*grid.getp(Point::new(0, 0)).unwrap(), 1);
        assert_eq!(*grid.getp(Point::new(1, 1)).unwrap(), 5);
        assert_eq!(*grid.getp(Point::new(2, 2)).unwrap(), 9);
        assert!(grid.getp(Point::new(3, 3)).is_none());
    }

    #[test]
    fn set() {
        let mut grid = basic_grid();
        grid.set(0, 0, 10);
        assert_eq!(*grid.get(0, 0).unwrap(), 10);
        grid.set(1, 1, 10);
        assert_eq!(*grid.get(1, 1).unwrap(), 10);
        grid.set(2, 2, 10);
        assert_eq!(*grid.get(2, 2).unwrap(), 10);
    }

    #[test]
    fn setp() {
        let mut grid = basic_grid();
        grid.setp(Point::new(0, 0), 10);
        assert_eq!(*grid.get(0, 0).unwrap(), 10);
        grid.setp(Point::new(1, 1), 10);
        assert_eq!(*grid.get(1, 1).unwrap(), 10);
        grid.setp(Point::new(2, 2), 10);
        assert_eq!(*grid.get(2, 2).unwrap(), 10);
    }

    #[test]
    fn iter_by_row() {
        let grid = basic_grid();
        let mut by_row = grid.into_iter();
        assert_eq!(by_row.next(), Some(vec![1, 2, 3]));
        assert_eq!(by_row.next(), Some(vec![4, 5, 6]));
        assert_eq!(by_row.next(), Some(vec![7, 8, 9]));
        assert_eq!(by_row.next(), Some(vec![0, 0, 0]));
        assert_eq!(by_row.next(), None);
    }

    #[test]
    fn iter_by_cell() {
        let grid = basic_grid();
        let mut by_cell = grid.by_cell();
        assert_eq!(by_cell.next(), Some((Point::new(0, 0), &1)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 0), &2)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 0), &3)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 1), &4)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 1), &5)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 1), &6)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 2), &7)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 2), &8)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 2), &9)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 3), &0)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 3), &0)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 3), &0)));
        assert_eq!(by_cell.next(), None);
    }

    #[test]
    fn into_iter_by_cell() {
        let grid = basic_grid();
        let mut by_cell = grid.into_by_cell();
        assert_eq!(by_cell.next(), Some((Point::new(0, 0), 1)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 0), 2)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 0), 3)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 1), 4)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 1), 5)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 1), 6)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 2), 7)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 2), 8)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 2), 9)));
        assert_eq!(by_cell.next(), Some((Point::new(0, 3), 0)));
        assert_eq!(by_cell.next(), Some((Point::new(1, 3), 0)));
        assert_eq!(by_cell.next(), Some((Point::new(2, 3), 0)));
        assert_eq!(by_cell.next(), None);
    }

    #[test]
    fn from_iter_by_cell() {
        let input = vec![
            ((Point::new(0, 0), 1)),
            ((Point::new(1, 0), 2)),
            ((Point::new(2, 0), 3)),
            ((Point::new(0, 1), 4)),
            ((Point::new(1, 1), 5)),
            ((Point::new(2, 1), 6)),
            ((Point::new(0, 2), 7)),
            ((Point::new(1, 2), 8)),
            ((Point::new(2, 2), 9)),
            ((Point::new(0, 3), 0)),
            ((Point::new(1, 3), 0)),
            ((Point::new(2, 3), 0)),
        ];
        let actual: Grid<i32> = input.into_iter().collect();
        let expected = basic_grid();
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_iter_by_cell_single_row() {
        let input = vec![
            ((Point::new(0, 0), 1)),
            ((Point::new(1, 0), 2)),
            ((Point::new(2, 0), 3)),
        ];
        let actual: Grid<i32> = input.into_iter().collect();
        let expected = Grid::new(vec![vec![1, 2, 3]]).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn from_iter_by_cell_invalid() {
        assert_throws(
            || {
                let input = vec![
                    ((Point::new(1, 0), 1)),
                    ((Point::new(1, 0), 2)),
                    ((Point::new(2, 0), 3)),
                ];
                return input.into_iter().collect::<Grid<i32>>();
            },
            "Expected point (0, 0), got (1, 0).",
        );
        assert_throws(
            || {
                let input = vec![
                    ((Point::new(0, 0), 1)),
                    ((Point::new(1, 0), 2)),
                    ((Point::new(1, 0), 3)),
                ];
                return input.into_iter().collect::<Grid<i32>>();
            },
            "Expected point (2, 0) or (0, 1), got (1, 0).",
        );
        assert_throws(
            || {
                let input = vec![
                    ((Point::new(0, 0), 1)),
                    ((Point::new(1, 0), 2)),
                    ((Point::new(1, 1), 3)),
                ];
                return input.into_iter().collect::<Grid<i32>>();
            },
            "Expected point (2, 0) or (0, 1), got (1, 1).",
        );
        assert_throws(
            || {
                let input = vec![
                    ((Point::new(0, 0), 1)),
                    ((Point::new(1, 0), 2)),
                    ((Point::new(2, 0), 3)),
                    ((Point::new(0, 1), 1)),
                    ((Point::new(2, 1), 3)),
                ];
                return input.into_iter().collect::<Grid<i32>>();
            },
            "Expected point (1, 1), got (2, 1).",
        );
    }

    #[test]
    fn neighbours_no_diagonal() {
        let grid = basic_grid();
        assert_eq!(
            grid.neighbours(Point::new(0, 0), false),
            vec![Point::new(1, 0), Point::new(0, 1)]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 0), false),
            vec![Point::new(0, 0), Point::new(2, 0), Point::new(1, 1)]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 0), false),
            vec![Point::new(1, 0), Point::new(2, 1)]
        );
        assert_eq!(
            grid.neighbours(Point::new(0, 1), false),
            vec![Point::new(1, 1), Point::new(0, 0), Point::new(0, 2)]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 1), false),
            vec![
                Point::new(0, 1),
                Point::new(2, 1),
                Point::new(1, 0),
                Point::new(1, 2)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 1), false),
            vec![Point::new(1, 1), Point::new(2, 0), Point::new(2, 2)]
        );
        assert_eq!(
            grid.neighbours(Point::new(0, 3), false),
            vec![Point::new(1, 3), Point::new(0, 2)]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 3), false),
            vec![Point::new(0, 3), Point::new(2, 3), Point::new(1, 2),]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 3), false),
            vec![Point::new(1, 3), Point::new(2, 2)]
        );
    }

    #[test]
    fn neighbours_diagonal() {
        let grid = basic_grid();
        assert_eq!(
            grid.neighbours(Point::new(0, 0), true),
            vec![Point::new(1, 0), Point::new(0, 1), Point::new(1, 1)]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 0), true),
            vec![
                Point::new(0, 0),
                Point::new(2, 0),
                Point::new(1, 1),
                Point::new(0, 1),
                Point::new(2, 1)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 0), true),
            vec![Point::new(1, 0), Point::new(2, 1), Point::new(1, 1)]
        );
        assert_eq!(
            grid.neighbours(Point::new(0, 1), true),
            vec![
                Point::new(1, 1),
                Point::new(0, 0),
                Point::new(0, 2),
                Point::new(1, 0),
                Point::new(1, 2)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 1), true),
            vec![
                Point::new(0, 1),
                Point::new(2, 1),
                Point::new(1, 0),
                Point::new(1, 2),
                Point::new(0, 0),
                Point::new(0, 2),
                Point::new(2, 0),
                Point::new(2, 2)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 1), true),
            vec![
                Point::new(1, 1),
                Point::new(2, 0),
                Point::new(2, 2),
                Point::new(1, 0),
                Point::new(1, 2)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(0, 3), true),
            vec![Point::new(1, 3), Point::new(0, 2), Point::new(1, 2)]
        );
        assert_eq!(
            grid.neighbours(Point::new(1, 3), true),
            vec![
                Point::new(0, 3),
                Point::new(2, 3),
                Point::new(1, 2),
                Point::new(0, 2),
                Point::new(2, 2)
            ]
        );
        assert_eq!(
            grid.neighbours(Point::new(2, 3), true),
            vec![Point::new(1, 3), Point::new(2, 2), Point::new(1, 2)]
        );
    }

    fn assert_throws<F: FnOnce() -> R + UnwindSafe, R: Debug>(f: F, expected: &str) {
        let catch = catch_unwind(f);
        let unwrapped = catch.unwrap_err();
        let actual = unwrapped
            .downcast_ref::<String>()
            .unwrap_or_else(|| panic!("Thrown value is not a String."));
        assert_eq!(*actual, expected);
    }
}
