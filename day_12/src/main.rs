use std::collections::HashMap;

use aoc::{EnumFromChar, Problem};

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumFromChar)]
enum Spring {
    #[char = '.'] Operational,
    #[char = '#'] Damaged,
    #[char = '?'] Unknown,
}

// ???.### 1,1,3
fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    let (s_str, g_str) = line.split_once(' ').unwrap();
    let springs = s_str.chars().map(|c| c.try_into().unwrap()).collect();
    let groups = g_str.split(',').map(|s| s.parse().unwrap()).collect();
    (springs, groups)
}

fn count_combinations<'s, 'g> (
    springs: &'s [Spring],
    groups: &'g [usize],
    memo: &mut HashMap<(&'s [Spring], &'g [usize]), usize>,
) -> usize {
    use Spring as S;
    if let Some(&res) = memo.get(&(springs, groups)) { return res; }

    let res = 'res: { if groups.is_empty() {
        // Base case: No groups and no more damaged springs
        if springs.iter().all(|s| s != &S::Damaged) { 1 } else { 0 }
    } else if let Some(start) = springs.iter().position(|p| p != &S::Operational) {
        // Recurrent case: Some groups and some unknown or damaged springs in slice
        // Short circuit if groups couldn't possibly fit in remaining slice
        if (groups.iter().sum::<usize>() + groups.len() - 1) > springs.len() - start {
            break 'res 0;
        }

        let mut sum = 0;
        let group_end = start + groups[0];
        if springs[start..group_end].iter().all(|s| s != &S::Operational) && (springs.len() == group_end || springs[group_end] != S::Damaged) {
            let next_start = springs.len().min(group_end + 1);
            sum += count_combinations(&springs[next_start..], &groups[1..], memo);
        }
        // If first character is unknown, try without it
        if springs[start] == S::Unknown {
            sum += count_combinations(&springs[start + 1..], groups, memo);
        }
        sum
    } else {
        // Groups aren't empty, but spring slice only contains operational springs
        0
    }};

    memo.insert((springs, groups), res);
    // println!("{:?} {:?} => {}", springs, groups, res);
    res
}

fn unfold_record(record: (Vec<Spring>, Vec<usize>)) -> (Vec<Spring>, Vec<usize>) {
    let (springs, groups) = record;
    let mut new_springs = vec![];
    let mut new_groups = vec![];
    for _ in 0..4 {
        new_springs.extend_from_slice(&springs);
        new_springs.push(Spring::Unknown);
        new_groups.extend_from_slice(&groups);
    }
    new_springs.extend_from_slice(&springs);
    new_groups.extend_from_slice(&groups);
    (new_springs, new_groups)
}
struct Day12;
impl Problem for Day12 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let records: Vec<_> = input.lines().map(parse_line).collect();

        let mut memo = HashMap::new();
        let mut sum = 0;
        (0..records.len()).for_each(|i| {
            sum += count_combinations(&records[i].0, &records[i].1, &mut memo)
        });
        sum
    }

    fn part_2(input: &str) -> Self::Solution {
        let records: Vec<_> = input.lines().map(parse_line).map(unfold_record).collect();

        let mut memo = HashMap::new();
        let mut sum = 0;
        (0..records.len()).for_each(|i| {
            sum += count_combinations(&records[i].0, &records[i].1, &mut memo)
        });
        sum
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day12::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1";

    #[test]
    fn test_count_combinations() {
        let mut memo = HashMap::new();

        let (springs, groups) = parse_line("???.### 1,1,3");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 1);

        let (springs, groups) = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 4);

        let (springs, groups) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 1);

        let (springs, groups) = parse_line("????.#...#... 4,1,1");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 1);

        let (springs, groups) = parse_line("????.######..#####. 1,6,5");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 4);

        let (springs, groups) = parse_line("?###???????? 3,2,1");
        assert_eq!(count_combinations(&springs, &groups, &mut memo), 10);
    }

    test_part_1!(Day12, SAMPLE, 21);

    test_part_2!(Day12, SAMPLE, 525152);
}