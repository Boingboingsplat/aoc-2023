use aoc::Problem;
use parse::{parse_line_part_1, parse_line_part_2};

mod parse;

fn get_area(steps: &[(i64, i64)]) -> i64 {
    let mut curr = (0, 0);
    let mut vertices = vec![curr];
    let mut trench_len = 0;
    for step in steps {
        trench_len += (step.0 + step.1).abs();
        curr.0 += step.0;
        curr.1 += step.1;
        vertices.push(curr);
    }
    // Make sure last step returns to origin
    assert!(curr == (0, 0));

    // Shoelace formula for area of simple polygon
    let interior_area = vertices.windows(2)
        .map(|slice| {
            let (x1, y1) = slice[0];
            let (x2, y2) = slice[1];
            x1 * y2 - x2 * y1
        })
        .sum::<i64>().abs() / 2;

    // Account for uncounted area of initial trenches
    interior_area + trench_len / 2 + 1
}

struct Day18;
impl Problem for Day18 {
    type Solution = i64;

    fn part_1(input: &str) -> Self::Solution {
        let steps: Vec<_> = input.lines().map(|line| parse_line_part_1(line).unwrap().1).collect();
        get_area(&steps)
    }

    fn part_2(input: &str) -> Self::Solution {
        let steps: Vec<_> = input.lines().map(|line| parse_line_part_2(line).unwrap().1).collect();
        get_area(&steps)
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day18::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)";

    const SAMPLE_2: &str = "\
        D 10 (#00D8D8)\n\
        R 3 (#00D8D8)\n\
        U 3 (#00D8D8)\n\
        R 4 (#00D8D8)\n\
        D 3 (#00D8D8)\n\
        R 5 (#00D8D8)\n\
        U 7 (#00D8D8)\n\
        L 6 (#00D8D8)\n\
        U 3 (#00D8D8)\n\
        L 6 (#00D8D8)";

    test_part_1!(Day18, SAMPLE, 62, SAMPLE_2, 116);

    test_part_2!(Day18, SAMPLE, 952408144115);
}