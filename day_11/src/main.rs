use itertools::Itertools;
use aoc::{grid::{Grid, GridIterator, Point}, Problem};

struct Galaxy;
impl TryFrom<char> for Galaxy {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Galaxy),
            _ => Err("Not a galaxy"),
        }
    }
}

fn solve(input: &str, factor: usize) -> usize {
    let galaxy_map: Grid<Galaxy> = input.into();

    let empty_cols: Vec<_> = (0..galaxy_map.width()).filter(|&n| galaxy_map.col_iter(n).next().is_none()).collect();
    let empty_rows: Vec<_> = (0..galaxy_map.height()).filter(|&n| galaxy_map.row_iter(n).next().is_none()).collect();

    let expanded_points: Vec<_> = galaxy_map.iter().indexed()
        .map(|(point, _)| {
            let leading_empty_cols = empty_cols.iter().filter(|&n| n < &point.x).count();
            let leading_empty_rows = empty_rows.iter().filter(|&n| n < &point.y).count();
            Point {
                x: point.x + (leading_empty_cols * (factor - 1)),
                y: point.y + (leading_empty_rows * (factor - 1)),
            }
        }).collect();
    
    expanded_points.iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(b))
        .sum()
}

struct Day11;
impl Problem for Day11 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        solve(input, 2)
    }

    fn part_2(input: &str) -> Self::Solution {
        solve(input, 1_000_000)
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day11::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::test_part_1;

    use super::*; 

    const SAMPLE: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

    test_part_1!(Day11, SAMPLE, 374);

    #[test]
    fn test_part_2() {
        assert_eq!(solve(SAMPLE, 100), 8410);
    }
}