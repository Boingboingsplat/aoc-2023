use std::{collections::HashMap, fmt::Display, hash::Hash};

use aoc::{EnumFromChar, grid::Direction, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumFromChar)]
enum Cell {
    #[char = '.'] Empty,
    #[char = 'O'] Round,
    #[char = '#'] Square,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Dish {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            let c = match *cell {
                Cell::Empty => '.',
                Cell::Round => 'O',
                Cell::Square => '#',
            };
            write!(f, "{}", c)?;
            if i % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Dish {
    fn new(s: &str) -> Self {
        let mut cells = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = width.max(line.chars().count());
            height += 1;
            for c in line.chars().map(|c| c.try_into().unwrap()) {
                cells.push(c);
            }
        }
        Dish { cells, width, height }
    }

    fn shift(&mut self, dir: Direction) {
        // Create a list of slice of mutable references for the row or col we want to shift
        // Area we're shifting to should be at front of the slice
        let slices: Vec<Vec<_>> = match dir {
            Direction::North => {
                (0..self.width).map(|x| {
                    (0..self.height)
                        .map(|y| y * self.width + x)
                        // Each index is unique, so we can get a mut ref to each safely
                        .map(|index| unsafe {
                            &mut *(self.cells.get_unchecked_mut(index) as *mut _)
                        }).collect()
                }).collect()
            },
            Direction::South => {
                (0..self.width).map(|x| {
                    // South simply reverses column order from North
                    (0..self.height).rev()
                        .map(|y| y * self.width + x)
                        // Each index is unique, so we can get a mut ref to each safely
                        .map(|index| unsafe {
                            &mut *(self.cells.get_unchecked_mut(index) as *mut _)
                        }).collect()
                }).collect()
            },
            Direction::West => {
                (0..self.height).map(|y| {
                    (y * self.width .. y * self.width + self.width)
                        // Each index is unique, so we can get a mut ref to each safely
                        .map(|index| unsafe {
                            &mut *(self.cells.get_unchecked_mut(index) as *mut _)
                        }).collect()
                }).collect()
            },
            Direction::East => {
                (0..self.height).map(|y| {
                    // East simply reverse row order from West
                    (y * self.width .. y * self.width + self.width).rev()
                        // Each index is unique, so we can get a mut ref to each safely
                        .map(|index| unsafe {
                            &mut *(self.cells.get_unchecked_mut(index) as *mut _)
                        }).collect()
                }).collect()
            },
        };

        for mut slice in slices {
            // Split slices at square rocks
            for sub_slice in slice.split_mut(|c| *c == &Cell::Square) {
                // Count round rocks in each subslice
                let round_rocks = sub_slice.iter().filter(|&c| *c == &Cell::Round).count();
                // Place the right amount of round rocks at the front, and rest of subslice is empty
                for (i, cell) in sub_slice.iter_mut().enumerate() {
                    if i < round_rocks {
                        **cell = Cell::Round;
                    } else {
                        **cell = Cell::Empty;
                    }
                }
            }
        }
    }

    fn load(&self) -> usize {
        self.cells.iter().enumerate()
            .filter(|(_, &c)| c == Cell::Round)
            .map(|(i, _)| self.height - (i / self.width) )
            .sum()
    }
}

struct Day14;
impl Problem for Day14 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let mut dish = Dish::new(input);
        dish.shift(Direction::North);
        dish.load()
    }

    fn part_2(input: &str) -> Self::Solution {
        let mut dish = Dish::new(input);
        let mut dish_map: HashMap<Dish, usize> = HashMap::new();
        let mut cur = 0;
        let remaining = loop {
            dish.shift(Direction::North);
            dish.shift(Direction::West);
            dish.shift(Direction::South);
            dish.shift(Direction::East);
            cur += 1;
            // Once the dish has been inserted into our map more than once, we found a cycle
            // Cycle length is current iteration - the iteration it was previously inserted at
            // Calculate many more iterations we must do for it to be equivalent to state
            // after 1_000_000_000 iterations
            if let Some(prev) = dish_map.insert(dish.clone(), cur) {
                let cycle_len = cur - prev;
                break (1_000_000_000 - cur) % cycle_len;
            }
            if cur == 1_000_000_000 {
                panic!("Couldn't find a cycle");
            }
        };
        for _ in 0..remaining {
            dish.shift(Direction::North);
            dish.shift(Direction::West);
            dish.shift(Direction::South);
            dish.shift(Direction::East);
        }
        dish.load()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day14::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....";

    test_part_1!(Day14, SAMPLE, 136);

    test_part_2!(Day14, SAMPLE, 64);
}