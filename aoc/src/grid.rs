use derive_more::{Add, AddAssign};
use std::{collections::{BTreeSet, HashMap}, fmt::{Debug, Display}};

/// A point with non-negative x and y components
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Add, AddAssign)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// A 2D vector with x and y components
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Add, AddAssign)]
pub struct Vector2D {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// Offsets the point by the given [Vector2D].
    /// 
    /// Returns None if offset would result in a negative x or y component.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Point, Vector2D};
    /// let point = Point { x: 1, y: 2 };
    /// let offset_point = point.offset_by(Vector2D { x: 1, y: -1 });
    /// 
    /// assert_eq!(offset_point, Some(Point { x: 2, y: 1 }));
    /// ```
    pub fn offset_by<V: Into<Vector2D>> (&self, vec_2d: V) -> Option<Point> {
        let vec_2d = vec_2d.into();
        Some(Point {
            x: self.x.checked_add_signed(vec_2d.x)?,
            y: self.y.checked_add_signed(vec_2d.y)?,
        })
    }

    /// Returns the manhattan distance between `self` and `other`.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::Point;
    /// let point_a = Point { x: 0, y: 0 };
    /// let point_b = Point { x: 10, y: 5 };
    /// 
    /// assert_eq!(point_a.manhattan_distance(&point_b), 15);
    /// ```
    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    /// Returns a list of Points that are adjacent to `self`.
    /// 
    /// Omits any points that would have negative components.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::Point;
    /// let point = Point { x: 2, y: 2 };
    /// let mut neighbors = point.neighbors();
    /// 
    /// assert_eq!(neighbors.next(), Some(Point { x: 2, y: 1 }));
    /// assert_eq!(neighbors.next(), Some(Point { x: 2, y: 3 }));
    /// assert_eq!(neighbors.next(), Some(Point { x: 3, y: 2 }));
    /// assert_eq!(neighbors.next(), Some(Point { x: 1, y: 2 }));
    /// assert_eq!(neighbors.next(), None);
    /// ```
    pub fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        Direction::DIRS.iter()
            .filter_map(|d| {
                self.offset_by(d.vector())
            })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl TryFrom<Vector2D> for Point {
    type Error = <isize as TryInto<usize>>::Error;

    fn try_from(value: Vector2D) -> Result<Self, Self::Error> {
        Ok(Point { x: value.x.try_into()?, y: value.y.try_into()? })
    }
} 

impl<T: Into<usize>> From<(T, T)> for Point {
    fn from(value: (T, T)) -> Self {
        let x = value.0.into();
        let y = value.1.into();
        Point { x, y }
    }
}

impl <T: Into<isize>> From<(T, T)> for Vector2D {
    fn from(value: (T, T)) -> Self {
        let x = value.0.into();
        let y = value.1.into();
        Vector2D { x, y }
    }
}

/// Represents a cardinal direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const DIRS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

    /// Returns a unit [Vector2D] in the direction represented.
    pub fn vector(&self) -> Vector2D {
        use Direction as D;
        match self {
            D::North => Vector2D { x: 0, y: -1 },
            D::South => Vector2D { x: 0, y: 1 },
            D::East => Vector2D { x: 1, y: 0 },
            D::West => Vector2D { x: -1, y: 0 },
        }
    }
    
    /// Returns a new `Direction` in the opposite direction of `self`.
    pub fn opposite(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::South,
            D::South => D::North,
            D::East => D::West,
            D::West => D::East,
        }
    }

    /// Returns a new `Direction` that's a right-hand turn from `self`.
    pub fn right_hand(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::East,
            D::South => D::West,
            D::East => D::South,
            D::West => D::North,
        }
    }

    /// Returns a new `Direction` that's a left-hand turn from `self`.
    pub fn left_hand(&self) -> Self {
        self.right_hand().opposite()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}

/// A 2-dimension grid of elements with type `T`.
/// 
/// Not every position within a `Grid` area has to contain an element.
#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    map: HashMap<Point, T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Constructs a new, empty `Grid<T>`.
    pub fn new() -> Self {
        Grid { map: HashMap::new(), width: 0, height: 0 }
    }

    /// Constructs a new `Grid<T>` based on 2-dimenional `Vec<Vec<T>>`.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Point, Grid};
    /// let data: Vec<Vec<u32>> = vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ];
    /// let data_grid = Grid::from_2d_vec(data);
    /// 
    /// let mut manual_grid = Grid::new();
    /// manual_grid.insert(Point { x: 0, y: 0 }, 1);
    /// manual_grid.insert(Point { x: 1, y: 0 }, 2);
    /// manual_grid.insert(Point { x: 0, y: 1 }, 3);
    /// manual_grid.insert(Point { x: 1, y: 1 }, 4);
    /// 
    /// assert_eq!(data_grid, manual_grid);
    /// ```
    pub fn from_2d_vec(input: Vec<Vec<T>>) -> Self {
        let mut map = HashMap::new();
        let height = input.len();
        let mut width = 0;
        for (y, row) in input.into_iter().enumerate() {
            width = width.max(row.len());
            for (x, value) in row.into_iter().enumerate() {
                map.insert(Point { x, y }, value);
            }
        }
        Grid { map, width, height }
    }

    /// Returns true if given point is within area of grid
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Point, Grid};
    /// let mut grid: Grid<u32> = Grid::new();
    /// grid.insert(Point { x: 2, y: 2 }, 1);
    /// 
    /// assert!(grid.check_inbounds(Point { x: 1, y: 1 }));
    /// assert!(!grid.check_inbounds(Point { x: 3, y: 3 }));
    /// ```
    pub fn check_inbounds<P> (&self, point: P) -> bool
    where
        P: Into<Point>,
    {
        let point = point.into();
        point.x < self.width && point.y < self.height
    }

    /// Returns the width of the `Grid`.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the `Grid`.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns `Some(&T)` if an element is in the grid at that point, otherwise `None`.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// let mut grid: Grid<char> = Grid::new();
    /// grid.insert(Point { x: 1, y: 1 }, 'a');
    /// 
    /// assert_eq!(grid.get(Point { x: 1, y: 1 }), Some(&'a'));
    /// assert_eq!(grid.get(Point { x: 0, y: 0 }), None);
    /// ```
    pub fn get<P> (&self, point: P) -> Option<&T>
    where
        P: Into<Point>,
    {
        let point = point.into();
        self.map.get(&point)
    }

    /// Inserts element `T` into `Grid` at given point.
    /// Returns `Some(T)` if replacing a previous element at that point, otherwise `None`.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// let mut grid: Grid<char> = Grid::new();
    /// 
    /// assert_eq!(grid.insert(Point { x: 0, y: 0 }, 'a'), None);
    /// assert_eq!(grid.insert(Point { x: 0, y: 0 }, 'b'), Some('a'));
    /// ```
    pub fn insert<P> (&mut self, point: P, value: T) -> Option<T>
    where
        P: Into<Point>,
    {
        let point = point.into();
        self.width = self.width.max(point.x + 1);
        self.height = self.height.max(point.y + 1);
        self.map.insert(point, value)
    }

    /// Iterates over all elements in the `Grid`, left to right, then top to bottom.
    /// Skips over empty positions in the `Grid`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// // Full grid
    /// let input = "\
    ///     ab\n\
    ///     cd";
    /// let grid: Grid<_> = input.into();
    ///
    /// let mut grid_iter = grid.iter();
    /// assert_eq!(grid_iter.next(), Some(&'a'));
    /// assert_eq!(grid_iter.next(), Some(&'b'));
    /// assert_eq!(grid_iter.next(), Some(&'c'));
    /// assert_eq!(grid_iter.next(), Some(&'d'));
    /// assert_eq!(grid_iter.next(), None);
    ///
    /// // Sparse grid
    /// let mut grid = Grid::new();
    /// grid.insert(Point { x: 2, y: 3 }, 'A');
    /// grid.insert(Point { x: 5, y: 10 }, 'Z');
    ///
    /// let mut grid_iter = grid.iter();
    /// assert_eq!(grid_iter.next(), Some(&'A'));
    /// assert_eq!(grid_iter.next(), Some(&'Z'));
    /// assert_eq!(grid_iter.next(), None);
    /// ```
    pub fn iter(&self) -> GridIter<T> {
        let next = Point { x: 0, y: 0 };
        GridIter { grid: self, next, current: next }
    }

    /// Iterates over neighboring elements to `point` in `Grid`.
    /// Skips over empty postions.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// let input = "\
    ///     abc\n\
    ///     def\n\
    ///     ghi";
    /// let grid: Grid<_> = input.into();
    /// 
    /// // Neighbors of center
    /// let mut n_iter = grid.neighbors_iter(&Point { x: 1, y: 1});
    /// assert_eq!(n_iter.next(), Some(&'b'));
    /// assert_eq!(n_iter.next(), Some(&'d'));
    /// assert_eq!(n_iter.next(), Some(&'f'));
    /// assert_eq!(n_iter.next(), Some(&'h'));
    /// assert_eq!(n_iter.next(), None);
    /// 
    /// // Neighbors of corner
    /// let mut n_iter = grid.neighbors_iter(&Point { x: 0, y: 2});
    /// assert_eq!(n_iter.next(), Some(&'d'));
    /// assert_eq!(n_iter.next(), Some(&'h'));
    /// assert_eq!(n_iter.next(), None);
    /// ```
    pub fn neighbors_iter(&self, point: &Point) -> GridNeighbors<T> {
        const NEIGHBOR_VECS: [Vector2D; 4] = [
            Vector2D { x: 0, y: -1 },
            Vector2D { x: -1, y: 0 },
            Vector2D { x: 1, y: 0 },
            Vector2D { x: 0, y: 1 },
        ];
        let neighbors = NEIGHBOR_VECS.iter().filter_map(|&vec_2d| point.offset_by(vec_2d)).collect();
        GridNeighbors { grid: self, index: 0, neighbors }
    }

    /// Iterates over orthogonally neighboring elements to `point` in `Grid`.
    /// Skips over empty postions.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// let input = "\
    ///     abc\n\
    ///     def\n\
    ///     ghi";
    /// let grid: Grid<_> = input.into();
    /// 
    /// // Orthogonal neighbors of center
    /// let mut n_iter = grid.ortho_iter(&Point { x: 1, y: 1});
    /// assert_eq!(n_iter.next(), Some(&'a'));
    /// assert_eq!(n_iter.next(), Some(&'b'));
    /// assert_eq!(n_iter.next(), Some(&'c'));
    /// assert_eq!(n_iter.next(), Some(&'d'));
    /// assert_eq!(n_iter.next(), Some(&'f'));
    /// assert_eq!(n_iter.next(), Some(&'g'));
    /// assert_eq!(n_iter.next(), Some(&'h'));
    /// assert_eq!(n_iter.next(), Some(&'i'));
    /// assert_eq!(n_iter.next(), None);
    /// 
    /// // Orthogonal neighbors of corner
    /// let mut n_iter = grid.ortho_iter(&Point { x: 2, y: 2});
    /// assert_eq!(n_iter.next(), Some(&'e'));
    /// assert_eq!(n_iter.next(), Some(&'f'));
    /// assert_eq!(n_iter.next(), Some(&'h'));
    /// assert_eq!(n_iter.next(), None);
    /// ```
    pub fn ortho_iter(&self, point: &Point) -> GridNeighbors<T> {
        const NEIGHBOR_VECS: [Vector2D; 8] = [
            Vector2D { x: -1, y: -1 },
            Vector2D { x: 0, y: -1 },
            Vector2D { x: 1, y: -1 },
            Vector2D { x: -1, y: 0 },
            Vector2D { x: 1, y: 0 },
            Vector2D { x: -1, y: 1 },
            Vector2D { x: 0, y: 1 },
            Vector2D { x: 1, y: 1 },
        ];
        let neighbors = NEIGHBOR_VECS.iter().filter_map(|&vec_2d| point.offset_by(vec_2d)).collect();
        GridNeighbors { grid: self, index: 0, neighbors }
    }

    /// Iterates over elements of the grid starting at Point in given Direction.
    /// Skips over empty elements.
    /// 
    /// Returns None when there is no more elements in the grid in that direction.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use aoc::grid::{Direction, Grid, Point};
    /// let input = "\
    ///     abc\n\
    ///     def\n\
    ///     ghi";
    /// let grid: Grid<_> = input.into();
    /// 
    /// let mut r_iter = grid.linear_iter(Point { x: 2, y: 1 }, Direction::West);
    /// assert_eq!(r_iter.next(), Some(&'f'));
    /// assert_eq!(r_iter.next(), Some(&'e'));
    /// assert_eq!(r_iter.next(), Some(&'d'));
    /// assert_eq!(r_iter.next(), None);
    /// ```
    pub fn linear_iter(&self, start: Point, dir: Direction) -> GridLinearIter<T> {
        GridLinearIter {
            grid: self,
            next: Some(start),
            dir,
            current: start,
        }
    }

    /// Iterates over elements of the grid with the give row index, from left to right.
    /// Skips over empty elements.
    /// 
    /// Returns None when there are no more elements in the row.
    ///
    /// # Examples
    /// 
    /// ```
    /// # use aoc::grid::Grid;
    /// let input = "\
    ///     abc\n\
    ///     def\n\
    ///     ghi";
    /// let grid: Grid<_> = input.into();
    /// 
    /// // Row in bounds
    /// let mut r_iter = grid.row_iter(0);
    /// assert_eq!(r_iter.next(), Some(&'a'));
    /// assert_eq!(r_iter.next(), Some(&'b'));
    /// assert_eq!(r_iter.next(), Some(&'c'));
    /// assert_eq!(r_iter.next(), None);
    /// 
    /// // Row out of bounds
    /// let mut r_iter = grid.row_iter(3);
    /// assert_eq!(r_iter.next(), None);
    /// ```
    pub fn row_iter(&self, row: usize) -> GridLinearIter<T> {
        let next = Point { x: 0, y: row }; 
        GridLinearIter {
            grid: self,
            next: Some(next),
            dir: Direction::East,
            current: next,
        }
    }

    /// Iterates over elements of the grid with the give column index, from top to bottom.
    /// Skips over empty elements.
    /// 
    /// Returns None when there are no more elements in the column.
    /// # Examples
    /// 
    /// ```
    /// # use aoc::grid::Grid;
    /// let input = "\
    ///     abc\n\
    ///     def\n\
    ///     ghi";
    /// let grid: Grid<_> = input.into();
    /// 
    /// // Column in bounds
    /// let mut r_iter = grid.col_iter(1);
    /// assert_eq!(r_iter.next(), Some(&'b'));
    /// assert_eq!(r_iter.next(), Some(&'e'));
    /// assert_eq!(r_iter.next(), Some(&'h'));
    /// assert_eq!(r_iter.next(), None);
    /// 
    /// // Column out of bounds
    /// let mut r_iter = grid.col_iter(3);
    /// assert_eq!(r_iter.next(), None);
    /// ```
    pub fn col_iter(&self, col: usize) -> GridLinearIter<T> {
        let next = Point { x: col, y: 0 }; 
        GridLinearIter {
            grid: self,
            next: Some(next),
            dir: Direction::South,
            current: next,
        }
    }
}

impl<T: Clone + Eq> Grid<T> {
    /// Performs a flood fill, starting by inserting or replacing the object at the `start` position with
    /// a clone of `value`, and then repeating on adjacent positions. Only replaces elements that match `replace`.
    /// 
    /// # Examples
    /// ```
    /// # use aoc::grid::{Grid, Point};
    /// // Replacing specific elements
    /// let mut input_grid: Grid<char> = "\
    ///     XXXX\n\
    ///     X..X\n\
    ///     X.XX\n\
    ///     XXX.".into();
    /// 
    /// input_grid.flood_fill(Point { x: 1, y: 1 }, 'O', Some(&'.'));
    /// 
    /// let output_grid: Grid<char> = "\
    ///     XXXX\n\
    ///     XOOX\n\
    ///     XOXX\n\
    ///     XXX.".into();
    /// 
    /// assert_eq!(input_grid, output_grid);
    /// 
    /// // Replacing empty grid elements
    /// let mut input_grid: Grid<char> = Grid::new();
    /// input_grid.insert(Point { x: 2, y: 2}, 'X');
    /// input_grid.insert(Point { x: 3, y: 3}, 'X');
    /// input_grid.flood_fill(Point { x: 0, y: 0 }, 'O', None);
    /// 
    /// let output_grid: Grid<char> = "\
    ///     OOOO\n\
    ///     OOOO\n\
    ///     OOXO\n\
    ///     OOOX".into();
    /// 
    /// assert_eq!(input_grid, output_grid);
    /// ```
    pub fn flood_fill<P> (&mut self, start: P, value: T, replace: Option<&T>)
    where
        P: Into<Point>,
    {
        let start = start.into();
        let mut frontier: BTreeSet<Point> = BTreeSet::new();
        frontier.insert(start);
        while let Some(point) = frontier.pop_first() {
            if self.check_inbounds(point) && replace == self.get(point) {
                self.insert(point, value.clone());
                frontier.extend(point.neighbors())
            }
        }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Grid::new()
    }
}

impl<T, S> From<S> for Grid<T>
where
    T: TryFrom<char>,
    S: Into<String>,
{
    fn from(input: S) -> Self {
        let grid_string = input.into();
        let mut map = HashMap::new();
        let height = grid_string.lines().count();
        let mut width = 0;
        for (y, line) in grid_string.lines().enumerate() {
            width = width.max(line.len());
            for (x, c) in line.chars().enumerate() {
                if let Ok(val) = c.try_into() {
                    map.insert(Point { x, y }, val);
                }
            }
        }
        Grid { map, width, height }
    }
}

pub trait GridIterator<'a, T: 'a>: Iterator<Item = &'a T> {
    fn current_index(&self) -> Point;
    fn indexed(self) -> IndexedGridIter<'a, T> where Self: Sized + 'a {
        IndexedGridIter { grid_iter: Box::new(self) }
    }
}

pub struct IndexedGridIter<'a, T> {
    grid_iter: Box<dyn GridIterator<'a, T> + 'a>,
}

impl<'a, T: 'a> Iterator for IndexedGridIter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.grid_iter.next();
        let point = self.grid_iter.current_index();
        next.map(|next| (point, next))
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    next: Point,
    current: Point,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.y >= self.grid.height {
            None
        } else {
            let val = self.grid.get(self.next);
            self.current = self.next;
            self.next.x += 1;
            if self.next.x >= self.grid.width {
                self.next.x = 0;
                self.next.y += 1;
            }
            match val {
                Some(val) => Some(val),
                None => self.next(),
            }
        }
    }
}

impl<'a, T> GridIterator<'a, T> for GridIter<'a, T> {
    fn current_index(&self) -> Point {
        self.current
    }
}

pub struct GridLinearIter<'a, T> {
    grid: &'a Grid<T>,
    next: Option<Point>,
    dir: Direction,
    current: Point,
}

impl<'a, T> Iterator for GridLinearIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            if self.grid.check_inbounds(next) {
                let val = self.grid.get(next);
                self.current = next;
                self.next = next.offset_by(self.dir.vector());
                match val {
                    Some(val) => Some(val),
                    None => self.next(),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, T> GridIterator<'a, T> for GridLinearIter<'a, T> {
    fn current_index(&self) -> Point {
        self.current
    }
}

pub struct GridNeighbors<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
    neighbors: Vec<Point>,
}

impl<'a, T> Iterator for GridNeighbors<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.neighbors.len() {
            None
        } else {
            let neighbor = self.neighbors[self.index];
            let val = self.grid.get(neighbor);
            self.index += 1;
            match val {
                Some(val) => Some(val),
                None => self.next(),
            }
        }
    }
}

impl<'a, T> GridIterator<'a, T> for GridNeighbors<'a, T> {
    fn current_index(&self) -> Point {
        let i = self.index.saturating_sub(1);
        self.neighbors[i]
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.iter().map(|v| format!("{}", v).chars().count()).max().unwrap_or(0);
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(Point { x, y }) {
                    Some(value) => { write!(f, "{:^width$}", value)? }
                    None => { write!(f, "{:width$}", " ")? },
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexed_grid_iterators() {
        // Full grid
        let input = "\
            ab\n\
            cd";
        let grid: Grid<_> = input.into();

        // Full iterator
        let mut grid_indexed_iter = grid.iter().indexed();
        assert_eq!(grid_indexed_iter.next(), Some((Point { x: 0, y: 0 }, &'a')));
        assert_eq!(grid_indexed_iter.next(), Some((Point { x: 1, y: 0 }, &'b')));
        assert_eq!(grid_indexed_iter.next(), Some((Point { x: 0, y: 1 }, &'c')));
        assert_eq!(grid_indexed_iter.next(), Some((Point { x: 1, y: 1 }, &'d')));
        assert_eq!(grid_indexed_iter.next(), None);

        // Linear iterator
        let mut row_iter = grid.row_iter(0).indexed();
        assert_eq!(row_iter.next(), Some((Point { x: 0, y: 0 }, &'a')));
        assert_eq!(row_iter.next(), Some((Point { x: 1, y: 0 }, &'b')));
        assert_eq!(row_iter.next(), None);

        // Neighbor iterator
        let mut n_iter = grid.neighbors_iter(&Point { x: 0, y: 0 }).indexed();
        assert_eq!(n_iter.next(), Some((Point { x: 1, y: 0 }, &'b')));
        assert_eq!(n_iter.next(), Some((Point { x: 0, y: 1 }, &'c')));
        assert_eq!(n_iter.next(), None);
    }
}