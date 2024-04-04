use aoc::*;

struct Day01;
impl Problem for Day01 {
    type Solution = u32;

    fn part_1(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                let nums = parse_line_part_1(line);
                nums.first().unwrap() * 10 + nums.last().unwrap() 
            })
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        input.lines()
            .map(|line| {
                let nums = parse_line_part_2(line);
                nums.first().unwrap() * 10 + nums.last().unwrap() 
            })
            .sum()
    }
}

fn parse_line_part_1(input: &str) -> Vec<u32> {
    input.chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

fn parse_line_part_2(input: &str) -> Vec<u32> {
    (0..input.len())
        .filter_map(|i| {
            if let Some(n) = input.chars().nth(i).expect("get a char").to_digit(10) { Some(n) }
            else if input[i..].starts_with("one") { Some(1) }
            else if input[i..].starts_with("two") { Some(2) }
            else if input[i..].starts_with("three") { Some(3) }
            else if input[i..].starts_with("four") { Some(4) }
            else if input[i..].starts_with("five") { Some(5) }
            else if input[i..].starts_with("six") { Some(6) }
            else if input[i..].starts_with("seven") { Some(7) }
            else if input[i..].starts_with("eight") { Some(8) }
            else if input[i..].starts_with("nine") { Some(9) }
            else { None }
        })
        .collect()
}

fn main() {
    let input = include_str!("input.txt");
    Day01::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PART_1: &str = "\
        1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";

    const SAMPLE_PART_2: &str = "\
        two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";

    test_part_1!(Day01, SAMPLE_PART_1, 142);
    test_part_2!(Day01, SAMPLE_PART_2, 281);
}