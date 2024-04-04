mod parse;

use std::ops::Range;
use itertools::Itertools;
use aoc::*;

#[derive(Debug)]
struct RangeMap {
    dest_range: Range<u64>,
    source_range: Range<u64>,
}

impl RangeMap {
    fn new(dest_start: u64, source_start: u64, len: u64) -> Self {
        RangeMap { 
            dest_range: dest_start..dest_start+len,
            source_range: source_start..source_start+len,
        }
    }

    fn get(&self, num: &u64) -> Option<u64> {
        if self.source_range.contains(num) {
            let idx = num - self.source_range.start;
            Some(self.dest_range.start + idx)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Almanac {
    maps: Vec<RangeMap>,
}

impl Almanac {
    fn new(maps: Vec<RangeMap>) -> Self {
        Almanac { maps }
    }

    fn get(&self, num: &u64) -> u64 {
        self.maps.iter()
            .find_map(|map| map.get(num))
            .unwrap_or(*num)
    }
}

struct Day05;
impl Problem for Day05 {
    type Solution = u64;

    fn part_1(input: &str) -> Self::Solution {
        let (_, (seeds, almanacs)) = parse::parse_input(input).unwrap();
        seeds.into_iter()
            .map(|seed| {
                almanacs.iter().fold(seed, |acc, almanac| {
                    almanac.get(&acc)
                })
            })
            .min().unwrap()
    }

    fn part_2(input: &str) -> Self::Solution {
        // This is bad and slow but I'm too lazy to think of a
        // more clever solution
        let (_, (seeds, almanacs)) = parse::parse_input(input).unwrap();
        seeds.into_iter()
            .tuples()
            .flat_map(|(start, length)| start..start+length)
            .map(|seed| {
                almanacs.iter().fold(seed, |acc, almanac| {
                    almanac.get(&acc)
                })
            })
            .min().unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day05::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

    test_part_1!(Day05, SAMPLE, 35);
    test_part_2!(Day05, SAMPLE, 46);
}