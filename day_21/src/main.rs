use std::collections::HashSet;

use aoc::{grid::{Grid, GridIterator, Point, Vector2D}, EnumFromChar, Problem};

#[derive(Debug, PartialEq, Eq, EnumFromChar)]
enum Cell {
    #[char = 'S'] Start,
    #[char = '.'] GardenPlot,
    #[char = '#'] Rock,
}

struct InfiniteGrid(Grid<Cell>);

impl InfiniteGrid {
    fn get(&self, position: Vector2D) -> Option<&Cell> {
        let x = position.x.rem_euclid(self.0.width() as isize) as usize;
        let y = position.y.rem_euclid(self.0.height() as isize) as usize;
        let point = Point { x, y };
        self.0.get(point)
    }
}


fn count_reachable_spaces(grid: &InfiniteGrid, steps: usize) -> usize {
    let (start_point, _) = grid.0.iter().indexed().find(|(_, cell)| *cell == &Cell::Start).unwrap();
    let mut reachable: HashSet<Vector2D> = HashSet::new();
    reachable.insert(start_point.try_into().unwrap());
    for _ in 0..steps {
        reachable = reachable.iter()
            .flat_map(|pos| {
                pos.neighbors().filter(|pos| grid.get(*pos) != Some(&Cell::Rock))
            })
            .collect();
    }
    reachable.len()
}

struct Day21;
impl Problem for Day21 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let grid = InfiniteGrid(input.into());
        count_reachable_spaces(&grid, 64)
    }

    fn part_2(input: &str) -> Self::Solution {
        // Path extends out like a diamond since there is a full column and row of empty tiles
        // along the start point of the input. Once the path reaches those rows/columns, it always takes exactly
        // one grid length to get to the next grid over.
        //   o
        //  oxo
        // oxoxo
        //  oxo
        //   o
        // First it's in 1 grid, then + 4 = 5 grids, then + 8 = 13, then + 12 = 25, then + 16 = 41
        // Aka it's in 2n^2 - 2n + 1 grids after moving n grid lengths away
        // Required steps is 26501365, which is 202300 * 131 + 65; aka 202300 grids away from start pos
        // Get the values of f(65), f(65 + 131), f(65 + 262) and do a quadratic regression to find the formula
        let grid = InfiniteGrid(input.into());
        let period = grid.0.width();
        let (start_point, _) = grid.0.iter().indexed().find(|(_, cell)| *cell == &Cell::Start).unwrap();
        let mut reachable: HashSet<Vector2D> = HashSet::new();
        reachable.insert(start_point.try_into().unwrap());
        let mut b = vec![];
        for i in 1..(period * 3) {
            reachable = reachable.iter()
                .flat_map(|pos| {
                    pos.neighbors().filter(|pos| grid.get(*pos) != Some(&Cell::Rock))
                })
                .collect();
            if i % period == 65 {
                b.push(reachable.len());
                println!("step {}: {}", i, reachable.len());
                if b.len() == 3 {
                    break;
                }
            }
        }
        // credit to: https://github.com/apprenticewiz/adventofcode/blob/main/2023/rust/day21b/src/main.rs#L83
        // used for calculating the quadratic regression and solving
        let (b0, b1, b2) = (b[0] as i64, b[1] as i64, b[2] as i64);
        let n: i64 = 202300;
        // the following formula comes from inv(A) * B = X,
        // where A is Vandermonde matrix:
        // [ 0 0 1 ]
        // [ 1 1 1 ]
        // [ 4 2 1 ]
        // and B is a column vector from the above values b0, b1, b2
        // credit to: https://gist.github.com/dllu/0ca7bfbd10a199f69bcec92f067ec94c
        // below uses Cramer's Rule to solve for x0, x1, x2
        let det_a: f64 = -2.0;
        let det_a0: f64 = -b0 as f64 + 2.0 * b1 as f64 - b2 as f64;
        let det_a1: f64 = 3.0 * b0 as f64 - 4.0 * b1 as f64 + b2 as f64;
        let det_a2: f64 = -2.0 * b0 as f64;
        let x0: i64 = (det_a0 / det_a) as i64;
        let x1: i64 = (det_a1 / det_a) as i64;
        let x2: i64 = (det_a2 / det_a) as i64;
        (x0 * n * n + x1 * n + x2) as usize
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day21::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........";

    #[test]
    fn test_infinite_grid_reachable_spaces() {
        let grid = InfiniteGrid(SAMPLE.into());
        assert_eq!(count_reachable_spaces(&grid, 6), 16);
        assert_eq!(count_reachable_spaces(&grid, 10), 50);
        assert_eq!(count_reachable_spaces(&grid, 50), 1594);
    }
}