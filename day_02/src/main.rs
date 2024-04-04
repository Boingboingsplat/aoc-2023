mod parse;

use aoc::*;

#[derive(Debug, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<CubeSet>,
}

impl Game {
    fn min_cubes_required(&self) -> CubeSet {
        self.rounds.iter()
            .fold(
                CubeSet { red: 0, green: 0, blue: 0 },
                |acc, round| {
                    CubeSet {
                        red: acc.red.max(round.red),
                        green: acc.green.max(round.green),
                        blue: acc.blue.max(round.blue),
                    }
                },
            )
    }
}

struct Day02;
impl Problem for Day02 {
    type Solution = u32;

    fn part_1(input: &str) -> Self::Solution {
        input.lines()
            .filter_map(|line| {
                let game = parse::parse_game(line).expect("Couldn't parse line").1;
                let min_cubes = game.min_cubes_required();
                if min_cubes.red <= 12 && min_cubes.green <= 13 && min_cubes.blue <= 14 {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                let game = parse::parse_game(line).expect("Couldn't parse line").1;
                let min_cubes = game.min_cubes_required();
                min_cubes.red * min_cubes.blue * min_cubes.green
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day02::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    test_part_1!(Day02, SAMPLE, 8);
    test_part_2!(Day02, SAMPLE, 2286);
}