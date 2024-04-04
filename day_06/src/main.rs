use aoc::*;

struct Race {
    duration: usize,
    record_dist: usize,
}

impl Race {
    fn record_winning_runs(&self) -> usize {
        (0..=self.duration)
            .filter(|hold_frames| {
                let remaining_frames = self.duration - hold_frames;
                let dist = hold_frames * remaining_frames;
                dist > self.record_dist
            })
            .count()
    }
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok());
    let records = lines.next().unwrap().split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok());
    times.zip(records).map(|(duration, record_dist)| Race { duration, record_dist }).collect()
}

fn parse_input_2(input: &str) -> Race {
    let mut lines = input.lines();
    let duration = lines.next().unwrap().chars().filter(char::is_ascii_digit).collect::<String>().parse::<usize>().unwrap();
    let record_dist = lines.next().unwrap().chars().filter(char::is_ascii_digit).collect::<String>().parse::<usize>().unwrap();
    Race { duration, record_dist }
}

struct Day06;
impl Problem for Day06 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let races = parse_input_1(input);
        races.iter()
            .map(|race| race.record_winning_runs())
            .product()
    }

    fn part_2(input: &str) -> Self::Solution {
        let race = parse_input_2(input);
        race.record_winning_runs()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day06::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200";

    test_part_1!(Day06, SAMPLE, 288);
    test_part_2!(Day06, SAMPLE, 71503);
}