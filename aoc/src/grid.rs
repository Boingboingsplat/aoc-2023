use std::{collections::HashMap, fmt::{Debug, Display}, hash::Hash, ops::{Add, AddAssign, Sub, SubAssign}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: TryInto<isize>> TryFrom<(T, T)> for Point {
    type Error = T::Error;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        Ok(Point { x: value.0.try_into()?, y: value.1.try_into()? })
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
                let x = x as isize;
                let y = y as isize;
                map.insert(Point { x, y }, value);
            }
        }
        Grid { map, width, height }
    }

    pub fn check_inbounds<P> (&self, point: P) -> bool
    where
        P: TryInto<Point>,
        P::Error: Debug,
    {
        let point = point.try_into().expect("Could not convert value into point");
        point.x >= 0 && point.x < self.width as isize && point.y >= 0 && point.y < self.height as isize
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get<P> (&self, point: P) -> Option<&T>
    where
        P: TryInto<Point>,
        P::Error: Debug,
    {
        let point = point.try_into().expect("Could not convert value into point");
        self.map.get(&point)
    }

    pub fn insert<P> (&mut self, point: P, value: T) -> Option<T>
    where
        P: TryInto<Point>,
        P::Error: Debug,
    {
        let point = point.try_into().expect("Could not convert value into point");
        self.width = self.width.max((point.x + 1) as usize);
        self.height = self.height.max((point.y + 1) as usize);
        self.map.insert(point, value)
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator { grid: self, current: Point { x: 0, y: 0 }}
    }

    pub fn point_neighbors_iter(&self, point: Point) -> GridCellNeighbors<T> {
        const NEIGHBORS: [Point; 4] = [
            Point { x: 0, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
        ];
        GridCellNeighbors { grid: self, center: point, index: 0, neighbors: &NEIGHBORS }
    }

    pub fn point_ortho_neighbors_iter(&self, point: Point) -> GridCellNeighbors<T> {
        const NEIGHBORS: [Point; 8] = [
            Point { x: -1, y: -1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: -1, y: 1 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ];
        GridCellNeighbors { grid: self, center: point, index: 0, neighbors: &NEIGHBORS }
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
                    let x = x as isize;
                    let y = y as isize;
                    map.insert(Point { x, y }, val);
                }
            }
        }
        Grid { map, width, height }
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    current: Point,
}

impl<'a, T> GridIterator<'a, T> {
    pub fn indexed(self) -> GridIndexedIterator<'a, T> {
        GridIndexedIterator { grid: self.grid, current: self.current }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y as usize >= self.grid.height {
            None
        } else {
            let val = self.grid.get(self.current);
            self.current.x += 1;
            if self.current.x as usize >= self.grid.width {
                self.current.x = 0;
                self.current.y += 1;
            }
            match val {
                Some(val) => Some(val),
                None => self.next(),
            }
        }
    }
}

pub struct GridIndexedIterator<'a, T> {
    grid: &'a Grid<T>,
    current: Point,
}

impl<'a, T> Iterator for GridIndexedIterator<'a, T> {
    type Item = (&'a Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y as usize >= self.grid.height {
            None
        } else {
            let val = self.grid.map.get_key_value(&self.current);
            self.current.x += 1;
            if self.current.x as usize >= self.grid.width {
                self.current.x = 0;
                self.current.y += 1;
            }
            match val {
                Some(val) => Some(val),
                None => self.next(),
            }
        }
    }
}

pub struct GridCellNeighbors<'a, T> {
    grid: &'a Grid<T>,
    center: Point,
    index: usize,
    neighbors: &'static [Point],
}

impl<'a, T> Iterator for GridCellNeighbors<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.neighbors.len() {
            None
        } else {
            let neighbor = self.center + self.neighbors[self.index];
            let val = self.grid.get(neighbor);
            self.index += 1;
            match val {
                Some(val) => Some(val),
                None => self.next(),
            }
        }
    }
}

impl<'a, T> GridCellNeighbors<'a, T> {
    pub fn indexed(self) -> GridCellNeighborsIndexed<'a, T> {
        GridCellNeighborsIndexed {
            grid: self.grid,
            center: self.center,
            index: self.index,
            neighbors: self.neighbors,
        }
    }
}

pub struct GridCellNeighborsIndexed<'a, T> {
    grid: &'a Grid<T>,
    center: Point,
    index: usize,
    neighbors: &'static [Point],
}

impl<'a, T> Iterator for GridCellNeighborsIndexed<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.neighbors.len() {
            None
        } else {
            let neighbor = self.center + self.neighbors[self.index];
            let val = self.grid.get(neighbor);
            self.index += 1;
            match val {
                Some(val) => Some((neighbor, val)),
                None => self.next(),
            }
        }
    }
}

pub trait GridDisplay {
    fn fmt_cell(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn fmt_empty_cell(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl GridDisplay for char {
    fn fmt_cell(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }

    fn fmt_empty_cell(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")
    }
}

impl<T: GridDisplay> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                match self.get(Point { x, y }) {
                    Some(value) => { T::fmt_cell(value, f)?; }
                    None => { T::fmt_empty_cell(f)?; },
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
    fn test_grid_iterator() {
        // Full grid
        let input = "\
            ab\n\
            cd";
        let grid = Grid::from(input);

        let mut grid_iter = grid.iter();
        assert_eq!(grid_iter.next(), Some(&'a'));
        assert_eq!(grid_iter.next(), Some(&'b'));
        assert_eq!(grid_iter.next(), Some(&'c'));
        assert_eq!(grid_iter.next(), Some(&'d'));
        assert_eq!(grid_iter.next(), None);

        let mut grid_indexed_iter = grid.iter().indexed();
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 0, y: 0 }, &'a')));
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 1, y: 0 }, &'b')));
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 0, y: 1 }, &'c')));
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 1, y: 1 }, &'d')));
        assert_eq!(grid_indexed_iter.next(), None);

        // Sparse grid
        let mut grid = Grid::new();
        grid.insert(Point { x: 2, y: 3 }, 'A');
        grid.insert(Point { x: 5, y: 10 }, 'Z');

        let mut grid_iter = grid.iter();
        assert_eq!(grid_iter.next(), Some(&'A'));
        assert_eq!(grid_iter.next(), Some(&'Z'));
        assert_eq!(grid_iter.next(), None);

        let mut grid_indexed_iter = grid.iter().indexed();
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 2, y: 3 }, &'A')));
        assert_eq!(grid_indexed_iter.next(), Some((&Point { x: 5, y: 10 }, &'Z')));
        assert_eq!(grid_indexed_iter.next(), None);
    }

    #[test]
    fn test_point_neighbors() {
        let input = "\
            abc\n\
            def\n\
            ghi";
        let grid = Grid::from(input);
        // Test neighbors of center
        let mut n_iter = grid.point_neighbors_iter(Point { x: 1, y: 1});
        assert_eq!(n_iter.next(), Some(&'b'));
        assert_eq!(n_iter.next(), Some(&'d'));
        assert_eq!(n_iter.next(), Some(&'f'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);

        // Test neighbors of corner
        let mut n_iter = grid.point_neighbors_iter(Point { x: 0, y: 2});
        assert_eq!(n_iter.next(), Some(&'d'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);

        // Test orthogonal neighbors of center
        let mut n_iter = grid.point_ortho_neighbors_iter(Point { x: 1, y: 1});
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
        let mut n_iter = grid.point_ortho_neighbors_iter(Point { x: 2, y: 2});
        assert_eq!(n_iter.next(), Some(&'e'));
        assert_eq!(n_iter.next(), Some(&'f'));
        assert_eq!(n_iter.next(), Some(&'h'));
        assert_eq!(n_iter.next(), None);
    }
}