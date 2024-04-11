use aoc::*;
use nom::{bytes::complete::{tag, take_until}, character::complete::{space0, space1}, multi::separated_list1, sequence::preceded, IResult};

#[derive(Debug)]
struct Card {
    winning_nums: Vec<u32>,
    nums: Vec<u32>,
}

impl Card {
    fn matching_nums(&self) -> usize {
        self.nums.iter()
            .filter(|n| self.winning_nums.contains(n))
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self.matching_nums();
        if matches > 0 {
            // 1 << (matches - 1)
            2_u32.pow(matches as u32 - 1)
        } else {
            0
        }
    }
}

fn parse_card(i: &str) -> IResult<&str, Card> {
    let (i, _) = take_until(":")(i)?;
    let (i, _) = tag(": ")(i)?;
    let (i, winning_nums) = preceded(space0, separated_list1(space1, nom::character::complete::u32))(i)?;
    let (i, _) = tag(" | ")(i)?;
    let (i, nums) = preceded(space0, separated_list1(space1, nom::character::complete::u32))(i)?;

    Ok((i, Card { winning_nums, nums }))
}

struct Day04;
impl Problem for Day04 {
    type Solution = u32;

    fn part_1(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                parse_card(line).unwrap().1.score()
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        let card_matches: Vec<_> = input.lines()
            .map(|line| {
                parse_card(line).unwrap().1.matching_nums()
            })
            .collect();
        let mut counts = vec![1; card_matches.len()];
        for (i, &matches) in card_matches.iter().enumerate() {
            for j in 0..matches {
                counts[i + j + 1] += counts[i];
            }
        }
        counts.iter().sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day04::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    test_part_1!(Day04, SAMPLE, 13);
    test_part_2!(Day04, SAMPLE, 30);
}