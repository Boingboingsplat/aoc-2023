use derive_more::{Add, AddAssign};
use std::{collections::{BTreeSet, HashMap, HashSet}, fmt::{Debug, Display}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Add, AddAssign)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Add, AddAssign)]
pub struct Vector2D {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn offset_by<V: Into<Vector2D>> (&self, vec_2d: V) -> Option<Point> {
        let vec_2d = vec_2d.into();
        Some(Point {
            x: self.x.checked_add_signed(vec_2d.x)?,
            y: self.y.checked_add_signed(vec_2d.y)?,
        })
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn neighbors(&self) -> Vec<Point> {
        Direction::DIRS.iter()
            .filter_map(|d| {
                self.offset_by(d.vector())
            })
            .collect()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub const DIRS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

    pub fn vector(&self) -> Vector2D {
        use Direction as D;
        match self {
            D::North => Vector2D { x: 0, y: -1 },
            D::South => Vector2D { x: 0, y: 1 },
            D::East => Vector2D { x: 1, y: 0 },
            D::West => Vector2D { x: -1, y: 0 },
        }
    }
    
    pub fn opposite(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::South,
            D::South => D::North,
            D::East => D::West,
            D::West => D::East,
        }
    }

    pub fn right_hand(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::East,
            D::South => D::West,
            D::East => D::South,
            D::West => D::North,
        }
    }

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

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    map: HashMap<Point, T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid { map: HashMap::new(), width: 0, height: 0 }
    }

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

    pub fn check_inbounds<P> (&self, point: P) -> bool
    where
        P: Into<Point>,
    {
        let point = point.into();
        point.x < self.width && point.y < self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get<P> (&self, point: P) -> Option<&T>
    where
        P: Into<Point>,
    {
        let point = point.into();
        self.map.get(&point)
    }

    pub fn insert<P> (&mut self, point: P, value: T) -> Option<T>
    where
        P: Into<Point>,
    {
        let point = point.into();
        self.width = self.width.max(point.x + 1);
        self.height = self.height.max(point.y + 1);
        self.map.insert(point, value)
    }

    pub fn iter(&self) -> GridIter<T> {
        let next = Point { x: 0, y: 0};
        GridIter { grid: self, next, current: next }
    }

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

    pub fn linear_iter(&self, start: Point, dir: Direction) -> GridLinearIter<T> {
        GridLinearIter {
            grid: self,
            next: start,
            dir,
            current: start,
        }
    }

    pub fn row_iter(&self, row: usize) -> GridLinearIter<T> {
        let next = Point { x: 0, y: row }; 
        GridLinearIter {
            grid: self,
            next,
            dir: Direction::East,
            current: next,
        }
    }

    pub fn col_iter(&self, col: usize) -> GridLinearIter<T> {
        let next = Point { x: col, y: 0 }; 
        GridLinearIter {
            grid: self,
            next,
            dir: Direction::South,
            current: next,
        }
    }
}

impl<T: Clone + Eq> Grid<T> {
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
    next: Point,
    dir: Direction,
    current: Point,
}

impl<'a, T> Iterator for GridLinearIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.grid.check_inbounds(self.next) {
            let val = self.grid.get(self.next);
            self.current = self.next;
            self.next = self.next.offset_by(self.dir.vector()).unwrap();
            match val {
                Some(val) => Some(val),
                None => self.next(),
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
    fn test_flood_fill() {
        let mut input_grid: Grid<char> = "\
            ####\n\
            #  #\n\
            # ##\n\
            ###".into();

        input_grid.flood_fill(Point { x: 1, y: 1 }, 'O', Some(&' '));

        let output_grid: Grid<char> = "\
            ####\n\
            #OO#\n\
            #O##\n\
            ###".into();

        assert_eq!(input_grid, output_grid);
    }

    #[test]
    fn test_grid_iterator() {
        // Full grid
        let input = "\
            ab\n\
            cd";
        let grid: Grid<_> = input.into();

        let mut grid_iter = grid.iter();
        assert_eq!(grid_iter.next(), Some(&'a'));
        assert_eq!(grid_iter.next(), Some(&'b'));
        assert_eq!(grid_iter.next(), Some(&'c'));
        assert_eq!(grid_iter.next(), Some(&'d'));
        assert_eq!(grid_iter.next(), None);

        // Sparse grid
        let mut grid = Grid::new();
        grid.insert(Point { x: 2, y: 3 }, 'A');
        grid.insert(Point { x: 5, y: 10 }, 'Z');

        let mut grid_iter = grid.iter();
        assert_eq!(grid_iter.next(), Some(&'A'));
        assert_eq!(grid_iter.next(), Some(&'Z'));
        assert_eq!(grid_iter.next(), None);
    }

    #[test]
    fn test_linear_iterators() {
        let input = "\
            abc\n\
            def\n\
            ghi";
        let grid: Grid<_> = input.into();
        // Test row iter in bounds
        let mut r_iter = grid.row_iter(0);
        assert_eq!(r_iter.next(), Some(&'a'));
        assert_eq!(r_iter.next(), Some(&'b'));
        assert_eq!(r_iter.next(), Some(&'c'));
        assert_eq!(r_iter.next(), None);
        // Test row iter out of bounds
        let mut r_iter = grid.row_iter(3);
        assert_eq!(r_iter.next(), None);

        // Test col iter in bounds
        let mut r_iter = grid.col_iter(1);
        assert_eq!(r_iter.next(), Some(&'b'));
        assert_eq!(r_iter.next(), Some(&'e'));
        assert_eq!(r_iter.next(), Some(&'h'));
        assert_eq!(r_iter.next(), None);
        // Test col iter out of bounds
        let mut r_iter = grid.col_iter(3);
        assert_eq!(r_iter.next(), None);
    }

    #[test]
    fn test_point_neighbors() {
        let input = "\
            abc\n\
            def\n\
            ghi";
        let grid: Grid<_> = input.into();
        // Test neighbors of center
        let mut n_iter = grid.neighbors_iter(&Point { x: 1, y: 1});
        assert_eq!(n_iter.next(), Some(&'b'));
        assert_eq!(n_iter.next(), Some(&'d'));
        assert_eq!(n_iter.next(), Some(&'f'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);

        // Test neighbors of corner
        let mut n_iter = grid.neighbors_iter(&Point { x: 0, y: 2});
        assert_eq!(n_iter.next(), Some(&'d'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);

        // Test orthogonal neighbors of center
        let mut n_iter = grid.ortho_iter(&Point { x: 1, y: 1});
        assert_eq!(n_iter.next(), Some(&'a'));
        assert_eq!(n_iter.next(), Some(&'b'));
        assert_eq!(n_iter.next(), Some(&'c'));
        assert_eq!(n_iter.next(), Some(&'d'));
        assert_eq!(n_iter.next(), Some(&'f'));
        assert_eq!(n_iter.next(), Some(&'g'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), Some(&'i'));
        assert_eq!(n_iter.next(), None);

        // Test orthogonal neighbors of corner
        let mut n_iter = grid.ortho_iter(&Point { x: 2, y: 2});
        assert_eq!(n_iter.next(), Some(&'e'));
        assert_eq!(n_iter.next(), Some(&'f'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);
    }

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