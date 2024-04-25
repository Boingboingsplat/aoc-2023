use aoc::*;
use aoc::grid::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum SchematicEntry {
    PartNum(u32),
    PartSymbol,
}

fn parse_input(input: &str) -> Grid<SchematicEntry> {
    let mut grid = Grid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => { continue; },
                n if n.is_ascii_digit() => {
                    let num: String = line[x..].chars().take_while(|n| n.is_ascii_digit()).collect();
                    let len = num.len();
                    let num = num.parse().unwrap();
                    for i in 0..len {
                        if grid.get((x + i, y)).is_none() {
                            grid.insert((x + i, y), SchematicEntry::PartNum(num));
                        }
                    }
                },
                _ => { grid.insert((x, y), SchematicEntry::PartSymbol); }
            }
        }
    }
    grid
}

struct Day03;
impl Problem for Day03 {
    type Solution = u32;

    fn part_1(input: &str) -> Self::Solution {
        let grid = parse_input(input);
        grid.iter().indexed()
            .filter_map(|(p, v)| {
                if *v == SchematicEntry::PartSymbol { Some(p) } else { None }
            })
            .flat_map(|p| grid.ortho_iter(&p).dedup())
            .filter_map(|v| {
                match v {
                    SchematicEntry::PartNum(n) => Some(n),
                    SchematicEntry::PartSymbol => None,
                }
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        let grid = parse_input(input);
        grid.iter().indexed()
            .filter_map(|(p, v)| {
                if *v == SchematicEntry::PartSymbol { Some(p) } else { None }
            })
            .filter_map(|p| {
                let mut n_iter = grid.ortho_iter(&p).dedup();
                match (n_iter.next(), n_iter.next(), n_iter.next()) {
                    (Some(SchematicEntry::PartNum(n)), Some(SchematicEntry::PartNum(m)), None) => Some(n * m),
                    _ => None,
                }
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day03::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    test_part_1!(Day03, SAMPLE, 4361);
    test_part_2!(Day03, SAMPLE, 467835);
}