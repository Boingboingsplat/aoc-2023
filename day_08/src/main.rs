mod parse;

use std::collections::HashMap;

use aoc::*;

#[derive(Debug, Clone, Copy, EnumFromChar)]
pub enum Direction {
    #[char = 'L'] Left,
    #[char = 'R'] Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabeledNode {
    label: String,
}

impl From<&str> for LabeledNode {
    fn from(value: &str) -> Self {
        LabeledNode { label: value.to_string() }
    }
}

pub struct NodeMap {
    map: HashMap<LabeledNode, (LabeledNode, LabeledNode)>,
    dir_list: Vec<Direction>,
}

impl NodeMap {
    fn iter(&self, start: &LabeledNode) -> NodeMapIter {
        // Get a reference to the start node key that's owned by the NodeMap
        let (start, _) = self.map.get_key_value(start).expect("NodeMap didn't contain start node");
        NodeMapIter {
            node_map: &self.map,
            dir_iter: Box::new(self.dir_list.iter().cycle()),
            current_node: start,
        }
    }
}

pub struct NodeMapIter<'a> {
    node_map: &'a HashMap<LabeledNode, (LabeledNode, LabeledNode)>,
    dir_iter: Box<dyn Iterator<Item = &'a Direction> + 'a>, // The + 'a means that the boxed iter won't be 'static and possibly outlive 'a
    current_node: &'a LabeledNode,
}

impl<'a> Iterator for NodeMapIter<'a> {
    type Item = &'a LabeledNode;

    fn next(&mut self) -> Option<Self::Item> {
        let (left, right) = self.node_map.get(&self.current_node)?;
        let next = match self.dir_iter.next().unwrap() {
            Direction::Left => left,
            Direction::Right => right,
        };
        self.current_node = next;
        Some(next)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        nums[0]
    } else {
        let a  = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd(a, b)
    }
}

struct Day08;
impl Problem for Day08 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let node_map = parse::parse_input(input).unwrap().1;
        let n = node_map.iter(&"AAA".into())
            .enumerate()
            .find_map(|(i, node)| {
                if node == &"ZZZ".into() { Some(i) } else { None }
            })
            .unwrap();
        // enumerate() doesn't count the first step, so we add 1
        n + 1
    }

    fn part_2(input: &str) -> Self::Solution {
        let node_map = parse::parse_input(input).unwrap().1;
        let path_lengths: Vec<_> = node_map.map.keys()
            .filter(|node| node.label.ends_with('A'))
            .map(|start| {
                // Part 1 method to find the path length
                let n = node_map.iter(start)
                    .enumerate()
                    .find_map(|(i, node)| {
                        if node.label.ends_with('Z') { Some(i) } else { None }
                    })
                    .unwrap();
                n + 1
            })
            .collect();
        // Get the LCM of the lengths
        lcm(&path_lengths)
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day08::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE_1: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_2: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

    test_part_1!(Day08, SAMPLE_1, 6);
    test_part_2!(Day08, SAMPLE_2, 6);
}