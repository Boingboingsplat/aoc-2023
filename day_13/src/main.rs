use aoc::{grid::Grid, EnumFromChar, Problem};

#[derive(Debug, PartialEq, Eq, EnumFromChar)]
enum Cell {
    #[char = '.'] Ash,
    #[char = '#'] Rock,
}

fn check_mirror(a: &[&Cell], b: &[&Cell]) -> bool {
    let len = a.len().min(b.len());
    a.iter().rev().take(len)
        .zip(b.iter().take(len))
        .all(|(a, b)| a == b)
}

fn mirror_hamming(a: &[&Cell], b: &[&Cell]) -> usize {
    let len = a.len().min(b.len());
    a.iter().rev().take(len)
        .zip(b.iter().take(len))
        .map(|(a, b)| { if a == b { 0 } else { 1 }})
        .sum()
}

struct Day13;
impl Problem for Day13 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        input.split("\n\n")
            .map(Grid::from)
            .map(|grid: Grid<Cell>| {
                // Find vertical axis reflection
                let v = (1..grid.width())
                    .find(|&i| {
                        (0..grid.height())
                            .all(|j| {
                                let row: Vec<_> = grid.row_iter(j).collect();
                                check_mirror(&row[..i], &row[i..])
                            })
                    }).unwrap_or(0);

                let h = (1..grid.height())
                    .find(|&i| {
                        (0..grid.width())
                            .all(|j| {
                                let row: Vec<_> = grid.col_iter(j).collect();
                                check_mirror(&row[..i], &row[i..])
                            })
                    }).unwrap_or(0);

                100 * h + v
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        input.split("\n\n")
            .map(Grid::from)
            .map(|grid: Grid<Cell>| {
                // Find vertical axis reflection
                let v = (1..grid.width())
                    .find(|&i| {
                        (0..grid.height())
                            .map(|j| {
                                let row: Vec<_> = grid.row_iter(j).collect();
                                mirror_hamming(&row[..i], &row[i..])
                            })
                            .sum::<usize>() == 1
                    }).unwrap_or(0);

                let h = (1..grid.height())
                    .find(|&i| {
                        (0..grid.width())
                            .map(|j| {
                                let row: Vec<_> = grid.col_iter(j).collect();
                                mirror_hamming(&row[..i], &row[i..])
                            })
                            .sum::<usize>() == 1
                    }).unwrap_or(0);
                100 * h + v
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day13::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";

    test_part_1!(Day13, SAMPLE, 405);

    test_part_2!(Day13, SAMPLE, 400);
}