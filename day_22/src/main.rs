use std::{collections::{BTreeMap, HashMap, HashSet, VecDeque}, ops::{Add, Div, Mul, Sub}, str::FromStr};
use anyhow::{anyhow, Result};

use aoc::Problem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i64, i64, i64);

impl Point {
    fn vec_length(&self) -> i64 {
        self.0.max(self.1).max(self.2)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|s| s.parse());
        let x = coords.next().ok_or(anyhow!("Not enough point components"))??;
        let y = coords.next().ok_or(anyhow!("Not enough point components"))??;
        let z = coords.next().ok_or(anyhow!("Not enough point components"))??;
        Ok(Point(x, y, z))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: Point,
    dir_vector: Point,
}

impl Brick {
    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let vec_length = self.dir_vector.vec_length();
        let unit_vec = if vec_length == 0 {
            Point(0, 0, 0)
        } else {
            self.dir_vector / vec_length
        };

        (0..=vec_length).map(move |n| self.start + unit_vec * n)
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').ok_or(anyhow!("Missing delimiter '~'"))?;
        let start = start.parse()?;
        let dir_vector = end.parse::<Point>()? - start;
        if dir_vector.0 < 0 || dir_vector.1 < 0 || dir_vector.2 < 0 {
            Err(anyhow!("Brick dir vector had negative components"))
        } else {
            Ok(Brick { start, dir_vector })
        }
    }
}

pub type NodeIndex = usize;
struct NodeData {
    brick: Brick,
    first_outgoing_edge: Option<EdgeIndex>,
}

pub type EdgeIndex = usize;
struct EdgeData {
    source: NodeIndex,
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}


struct SupportGraph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl SupportGraph {
    pub fn new() -> Self {
        SupportGraph { nodes: vec![], edges: vec![] }
    }

    pub fn add_node(&mut self, brick: Brick) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { brick, first_outgoing_edge: None });
        index
    }

    pub fn get_node(&self, brick: &Brick) -> Option<NodeIndex> {
        self.nodes.iter().position(|node| &node.brick == brick)
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            source,
            target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn predecessors(&self, source: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.edges.iter()
            .filter_map(move |edge| {
                if edge.target == source {
                    Some(edge.source)
                } else {
                    None
                }
            })
    }

    pub fn count_predecessors(&self, source: NodeIndex) -> usize {
        self.edges.iter()
            .filter(|edge| edge.target == source)
            .count()
    }

    /// Return a count of blocks which can be removed without any other blocks falling
    pub fn count_nonsupporting_bricks(&self) -> usize {
        let num_nodes = self.nodes.len();
        (0..num_nodes)
            .filter(|i| {
                self.successors(*i)
                    .all(|successor| self.count_predecessors(successor) > 1)
            })
            .count()
        }

    pub fn count_supported_bricks(&self) -> usize {
        let mut is_falling = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        let mut count = 0;

        for brick in 0..self.nodes.len() {
            is_falling[brick] = true;
            queue.push_back(brick);

            while let Some(brick) = queue.pop_front() {
                for child in self.successors(brick) {
                    // If it's not already falling, and all its predecessors are falling 
                    if !is_falling[child] && self.predecessors(child).all(|pred| is_falling[pred]) {
                        // Set it to fall and add it to queue
                        is_falling[child] = true;
                        queue.push_back(child);
                        count += 1;
                    }
                }
            }
            // Reset is_falling vec
            is_falling.fill(false);
        }
        count
    }
}

pub struct Successors<'g> {
    graph: &'g SupportGraph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'g> Iterator for Successors<'g> {
    type Item = NodeIndex;
    
    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[derive(Debug)]
struct BrickStack {
    bricks: Vec<Brick>,
}

impl BrickStack {
    fn new(input: &str) -> Self {
        let mut bricks: Vec<Brick> = input.lines().map(|s| s.parse().unwrap()).collect();
        // Sort bricks in ascending elevation
        bricks.sort_unstable_by(|a, b| a.start.2.cmp(&b.start.2));
        BrickStack { bricks }
    }

    fn apply_gravity(&mut self) {
        let mut new_bricks: Vec<Brick> = vec![];

        for brick in self.bricks.iter() {
            let brick_z = brick.start.2;
            // println!("{brick:?}");
            let new_z = brick.points().map(|point| {
                (1..brick_z).rev()
                    .find(|z| {
                        let search_point = Point(point.0, point.1, *z);
                        new_bricks.iter()
                            .flat_map(|brick| brick.points())
                            .any(|p| p == search_point)
                    }).unwrap_or(0) + 1
            }).max().unwrap();
            let mut new_brick = brick.clone();
            new_brick.start.2 = new_z;
            // println!("{brick:?} -> {new_brick:?}");
            new_bricks.push(new_brick);
        }
        // Sanity check that we didn't lose or gain any bricks
        assert_eq!(self.bricks.len(), new_bricks.len());

        self.bricks = new_bricks;
    }

    fn get_brick_at(&self, point: Point) -> Option<&Brick> {
        self.bricks.iter().find(|brick| brick.points().any(|p| p == point))
    }

    fn get_support_graph(&self) -> SupportGraph {
        let mut graph = SupportGraph::new();
        // Because bricks are iterated over from bottom up, we can always be sure that supporting
        // Bricks will already be in the graph
        for brick in self.bricks.iter() {
            // Add the brick to the graph
            let node_index = graph.add_node(brick.clone());
            // Look for any bricks underneath it, and add edges
            brick.points()
                .filter_map(|point| self.get_brick_at(point - Point(0, 0, 1)))
                .filter(|support| *support != brick) // Make sure bricks can't support themselves
                .for_each(|parent| {
                    if let Some(parent_index) = graph.get_node(parent) {
                        graph.add_edge(parent_index, node_index)
                    }
                });
        }
        graph
    }
}

struct Day22;
impl Problem for Day22 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let mut brick_stack = BrickStack::new(input);
        // dbg!(&brick_stack);
        brick_stack.apply_gravity();
        // dbg!(&brick_stack);
        // brick_stack.count_nonsupporting_bricks()
        let graph = brick_stack.get_support_graph();
        graph.count_nonsupporting_bricks()
    }

    fn part_2(input: &str) -> Self::Solution {
        let mut brick_stack = BrickStack::new(input);
        // dbg!(&brick_stack);
        brick_stack.apply_gravity();
        // dbg!(&brick_stack);
        // brick_stack.count_nonsupporting_bricks()
        let graph = brick_stack.get_support_graph();
        graph.count_supported_bricks()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day22::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9";

    test_part_1!(Day22, SAMPLE, 5);

    test_part_2!(Day22, SAMPLE, 7);
}