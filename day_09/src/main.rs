use aoc::*;
use itertools::Itertools;

fn next(nums: &[i64]) -> i64 {
    if nums.iter().all(|&n| n == 0) {
        0
    } else {
        nums.last().unwrap() + next(&differences(nums))
    }
}

fn differences(nums: &[i64]) -> Vec<i64> {
    nums.iter().tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

struct Day09;
impl Problem for Day09 {
    type Solution = i64;

    fn part_1(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                let nums: Vec<_> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
                next(&nums)
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                let nums: Vec<_> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).rev().collect();
                next(&nums)
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day09::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

    test_part_1!(Day09, SAMPLE, 114);
    test_part_2!(Day09, SAMPLE, 2);
}