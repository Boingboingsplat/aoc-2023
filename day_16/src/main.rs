use std::collections::HashMap;

use aoc::{grid::{Direction, Grid, Point}, EnumFromChar, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromChar)]
enum Cell {
    #[char = '.'] Empty,
    #[char = '/'] ForwMirror,
    #[char = '\\'] BackMirror,
    #[char = '-'] HorSplit,
    #[char = '|'] VerSplit,
}

impl Cell {
    fn next_dirs(&self, from: &Direction) -> Vec<Direction> {
        use Direction as D;
        match (self, from) {
            (Cell::ForwMirror, D::North) => vec![D::East],
            (Cell::ForwMirror, D::South) => vec![D::West],
            (Cell::ForwMirror, D::East ) => vec![D::North],
            (Cell::ForwMirror, D::West ) => vec![D::South],
            (Cell::BackMirror, D::North) => vec![D::West],
            (Cell::BackMirror, D::South) => vec![D::East],
            (Cell::BackMirror, D::East ) => vec![D::South],
            (Cell::BackMirror, D::West ) => vec![D::North],
            (Cell::HorSplit,   D::North) => vec![D::West, D::East],
            (Cell::HorSplit,   D::South) => vec![D::West, D::East],
            (Cell::VerSplit,   D::East ) => vec![D::North, D::South],
            (Cell::VerSplit,   D::West ) => vec![D::North, D::South],
            // In all other cases it continues on its existing path
            (_, &current) => vec![current],
        }
    }
}

fn count_energized(grid: &Grid<Cell>, start_point: Point, start_dir: Direction) -> usize {
    let mut frontier = vec![];
    // Figure out the exit direction from the first cell
    let origin_cell = grid.get(start_point).unwrap();
    for dir in origin_cell.next_dirs(&start_dir) {
        frontier.push((start_point, dir))
    }

    let mut visited: HashMap<Point, Vec<Direction>> = HashMap::new();

    while let Some((curr, dir)) = frontier.pop() {
        if let Some(dir_visits) = visited.get_mut(&curr) {
            if !dir_visits.contains(&dir) {
                // Add the direction to the list of visited directions
                (*dir_visits).push(dir);
                // Add to the frontier
                if let Some(next) = curr.offset_by(dir.vector()) {
                    if let Some(next_cell) = grid.get(next) {
                        for next_dir in next_cell.next_dirs(&dir) {
                            frontier.push((next, next_dir));
                        }
                    }
                }
            }
            // If the cell was already visited from current dir, do nothing
        } else {
            // Insert into the visited map
            visited.insert(curr, vec![dir]);
            // Add to the frontier
            if let Some(next) = curr.offset_by(dir.vector()) {
                if let Some(next_cell) = grid.get(next) {
                    for next_dir in next_cell.next_dirs(&dir) {
                        frontier.push((next, next_dir));
                    }
                }
            }
        }
    }
    visited.len()
}

struct Day16;
impl Problem for Day16 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let grid: Grid<Cell> = input.into();
        count_energized(&grid, Point { x: 0, y: 0 }, Direction::East)
    }

    fn part_2(input: &str) -> Self::Solution {
        let grid: Grid<Cell> = input.into();
        // Assemble an iterator of all entry points
        (0..grid.width()).map(|x| (Point { x, y: 0 }, Direction::South))
            .chain((0..grid.width()).map(|x| (Point { x, y: grid.height() - 1}, Direction::North)))
            .chain((0..grid.height()).map(|y| (Point { x: 0, y }, Direction::East)))
            .chain((0..grid.height()).map(|y| (Point { x: grid.width() - 1, y }, Direction::West)))
            // Count energized tiles and return the max
            .map(|(start_point, start_dir)| count_energized(&grid, start_point, start_dir))
            .max().unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day16::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str =
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    test_part_1!(Day16, SAMPLE, 46);

    test_part_2!(Day16, SAMPLE, 51);
}