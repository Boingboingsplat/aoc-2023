use std::{collections::{BinaryHeap, HashMap}, hash::Hash};

use aoc::{grid::{Direction, Grid, Point}, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GraphNode {
    point: Point,
    dir: Direction,
    steps: usize,
}

impl GraphNode {
    fn neighbors(&self, min_steps: usize, max_steps: usize) -> Vec<GraphNode> {
        // Special case for starting node with 0 steps
        // It's direction doesn't matter
        if self.steps == 0 {
            return Direction::DIRS.iter()
                .filter_map(|d| {
                    let next = self.point.offset_by(d.vector())?;
                    Some(GraphNode { point: next, dir: *d, steps: 1 })
                })
                .collect()
        }

        let mut neighbors = vec![];
        // Forward only a neighbor if we've made less than max steps in that direction
        if self.steps < max_steps {
            if let Some(next) = self.point.offset_by(self.dir.vector()) {
                neighbors.push(GraphNode { point: next, dir: self.dir, steps: self.steps + 1 })
            }
        }
        // Right and left hand turns only neighbors if we've made min number of steps
        if self.steps >= min_steps {
            if let Some(next) = self.point.offset_by(self.dir.right_hand().vector()) {
                neighbors.push(GraphNode { point: next, dir: self.dir.right_hand(), steps: 1 })
            }
            if let Some(next) = self.point.offset_by(self.dir.left_hand().vector()) {
                neighbors.push(GraphNode { point: next, dir: self.dir.left_hand(), steps: 1 })
            }
        }
        neighbors
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    node: GraphNode,
    f_score: u32,
}

// Custom Ord implementation so we can use it as key in a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn h(node: &GraphNode, goal: &Point) -> u32 {
    // Heuristic function
    // Min estimate is manhattan distance to goal
    node.point.manhattan_distance(goal) as u32
}

// A* search algorithm
fn shortest_path(
    grid: &Grid<u32>,
    start: Point,
    goal: Point,
    min_steps: usize,
    max_steps: usize,
) -> u32 {
    let start_node = GraphNode { point: start, dir: Direction::North, steps: 0 };
    let start_f_score = h(&start_node, &goal);

    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    open_set.push(State { node: start_node, f_score: start_f_score });

    let mut came_from: HashMap<GraphNode, GraphNode> = HashMap::new();

    let mut g_score: HashMap<GraphNode, u32> = HashMap::new();
    g_score.insert(start_node, 0);

    while let Some(State { node: curr, f_score: _ }) = open_set.pop() {
        if curr.point == goal {
            #[cfg(debug_assertions)]
            {
                // Pretty print our search field and path on debug build
                // Just for fun
                let mut search_grid: Grid<char> = Grid::new();
                for GraphNode { point, dir: _, steps: _ } in g_score.keys() {
                    search_grid.insert(*point, '▒');
                }
                for State { node: GraphNode { point, dir: _, steps: _ }, f_score: _ } in open_set.iter() {
                    search_grid.insert(*point, '░');
                }
                let mut current = curr;
                while let Some(prev) = came_from.get(&current) {
                    let c = match prev.dir {
                        Direction::North => '^',
                        Direction::South => 'v',
                        Direction::East => '>',
                        Direction::West => '<',
                    };
                    search_grid.insert(prev.point, c);
                    current = *prev;
                }
                search_grid.insert(start, 'O');
                search_grid.insert(curr.point, '#');

                println!("{search_grid}");
            }
            return g_score[&curr];
        }

        for neighbor in curr.neighbors(min_steps, max_steps) {
            if let Some(cost) = grid.get(neighbor.point) {
                let tentative_g_score = g_score[&curr] + cost;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, curr);
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push(State { node: neighbor, f_score: tentative_g_score + h(&neighbor, &goal) });
                }
            }
        }
    }

    panic!("Couldn't find path to goal")
}

struct Day17;
impl Problem for Day17 {
    type Solution = u32;

    fn part_1(input: &str) -> Self::Solution {
        let grid = Grid::from_2d_vec(
            input.lines()
                .map(|line| {
                    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
                })
                .collect()
        );

        shortest_path(
            &grid, 
            Point { x: 0, y: 0 }, 
            Point { x: grid.width() - 1, y: grid.height() - 1 },
            1,
            3,
        )
    }

    fn part_2(input: &str) -> Self::Solution {
        let grid = Grid::from_2d_vec(
            input.lines()
                .map(|line| {
                    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
                })
                .collect()
        );

        shortest_path(
            &grid, 
            Point { x: 0, y: 0 }, 
            Point { x: grid.width() - 1, y: grid.height() - 1 },
            4,
            10,
        )
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day17::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533";

    test_part_1!(Day17, SAMPLE, 102);

    test_part_2!(Day17, SAMPLE, 94);
}