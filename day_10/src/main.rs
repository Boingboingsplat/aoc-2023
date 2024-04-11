use aoc::*;
use aoc::grid::{Grid, GridDisplay, Vector2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeGridCell {
    Start,
    Pipe(Direction, Direction),
    LeftMark,
    RightMark,
}

impl PipeGridCell {
    fn next_from(&self, dir: &Direction) -> Option<&Direction> {
        match self {
            PipeGridCell::Pipe(dir_1, dir_2) => {
                if *dir_1 == dir.opposite() {
                    Some(dir_2)
                } else if *dir_2 == dir.opposite() {
                    Some(dir_1)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl TryFrom<char> for PipeGridCell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use PipeGridCell as C;
        use Direction as D;
        match c {
            '|' => Ok(C::Pipe(D::North, D::South)),
            '-' => Ok(C::Pipe(D::West, D::East)),
            'L' => Ok(C::Pipe(D::North, D::East)),
            'J' => Ok(C::Pipe(D::North, D::West)),
            '7' => Ok(C::Pipe(D::South, D::West)),
            'F' => Ok(C::Pipe(D::South, D::East)),
            'S' => Ok(C::Start),
            _ => Err(format!("No cell corresponds to character '{}'", c))
        }
    }
}

impl GridDisplay for PipeGridCell {
    fn fmt_cell(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PipeGridCell as C;
        use Direction as D;
        let c = match self {
            C::Start => '█',
            C::Pipe(D::North, D::South) => '║',
            C::Pipe(D::West, D::East) => '═',
            C::Pipe(D::North, D::East) => '╚',
            C::Pipe(D::North, D::West) => '╝',
            C::Pipe(D::South, D::West) => '╗',
            C::Pipe(D::South, D::East) => '╔',
            C::LeftMark => 'L',
            C::RightMark => 'R',
            _ => ' ',
        };
        write!(f, "{}", c)
    }

    fn fmt_empty_cell(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn vector(&self) -> Vector2D {
        use Direction as D;
        match self {
            D::North => Vector2D { x: 0, y: -1 },
            D::South => Vector2D { x: 0, y: 1 },
            D::East => Vector2D { x: 1, y: 0 },
            D::West => Vector2D { x: -1, y: 0 },
        }
    }
    
    fn opposite(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::South,
            D::South => D::North,
            D::East => D::West,
            D::West => D::East,
        }
    }

    fn right_hand(&self) -> Self {
        use Direction as D;
        match self {
            D::North => D::East,
            D::South => D::West,
            D::East => D::South,
            D::West => D::North,
        }
    }
}


const DIRS: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

fn flood_fill_grid(grid: &mut Grid<PipeGridCell>, target: &PipeGridCell) {
    let mut frontier: Vec<_> = grid.iter().indexed()
        .filter_map(|(&p, c)| { if c == target { Some(p) } else { None }})
        .collect();

    while let Some(next) = frontier.pop() {
        for point in DIRS.iter().filter_map(|d| next.offset_by(d.vector())) {
            if grid.check_inbounds(point) && grid.get(point).is_none() {
                frontier.push(point);
                grid.insert(point, *target);
            }
        }
    }
}


struct Day10;
impl Problem for Day10 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        use PipeGridCell as C;
        let grid: Grid<C> = input.into();

        let start_point = grid.iter().indexed()
            .find_map(|(&p, s)| {
                if s == &C::Start { Some(p) } else { None }
            }).unwrap();
        let (mut current_point, mut current_dir) = DIRS.iter().find_map(|dir| {
            let next_point = start_point.offset_by(dir.vector())?;
            let next_dir = *grid.get(next_point)?.next_from(dir)?;
            Some((next_point, next_dir))
        }).unwrap();

        let mut len = 1;
        loop {
            len += 1;
            current_point = current_point.offset_by(current_dir.vector()).expect("Pipe path went out of bounds");
            match grid.get(current_point) {
                Some(C::Start) => { break; },
                Some(pipe) => {
                    current_dir = *pipe.next_from(&current_dir).expect("Pipe path ended unexpectedly");
                }
                _ => panic!("Pipe path ended unexpectedly")
            }
        }
        len / 2
    }

    fn part_2(input: &str) -> Self::Solution {
        use PipeGridCell as C;
        let grid: Grid<C> = input.into();

        let mut current_point = grid.iter().indexed()
            .find_map(|(&p, s)| {
                if s == &C::Start { Some(p) } else { None }
            }).unwrap();
        let mut current_dir = DIRS.iter().find_map(|dir| {
            let next_point = current_point.offset_by(dir.vector())?;
            // If there is a path from the next point
            let _ = *grid.get(next_point)?.next_from(dir)?;
            Some(*dir)
        }).unwrap();

        let mut mark_grid: Grid<C> = Grid::new();
        let mut turn_count = 0;
        loop {
            let next_point = current_point.offset_by(current_dir.vector()).expect("Pipe path went out of bounds");
            let next_cell = *grid.get(next_point).expect("Pipe path ended unexpectedly");

            // Add pipe path and right and and left hand markings to markings grid
            mark_grid.insert(next_point, next_cell);

            if let Some(right) = current_point.offset_by(current_dir.right_hand().vector()) {
                if mark_grid.get(right).is_none() {
                    mark_grid.insert(right, C::RightMark);
                }
            }
            
            if let Some(left) = current_point.offset_by(current_dir.right_hand().opposite().vector()) {
                if mark_grid.get(left).is_none() {
                    mark_grid.insert(left, C::LeftMark);
                }
            }

            // Break loop once we're back at start
            if next_cell == C::Start {
                break;
            }

            let next_dir = *next_cell.next_from(&current_dir).expect("Pipe path ended unexpectedly");
            // Increment/decrement turn count based on turn direction
            if next_dir == current_dir.right_hand() {
                turn_count += 1;
            } else if next_dir == current_dir.right_hand().opposite() {
                turn_count -= 1;
            }

            current_point = next_point;
            current_dir = next_dir;
        }
        // If there's more right turns, right marks inside, otherwise left marks are inside
        let mark = if turn_count > 0 { C::RightMark } else { C::LeftMark };
        flood_fill_grid(&mut mark_grid, &mark);
        println!("{}", mark_grid);
        mark_grid.iter().filter(|&c| c == &mark).count()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day10::benchmark(input);
}

#[cfg(test)]
mod tests {
    use super::*; 

    const SAMPLE_1: &str = "\
        7-F7-\n\
        .FJ|7\n\
        SJLL7\n\
        |F--J\n\
        LJ.LJ";

    const SAMPLE_2: &str = "\
        FF7FSF7F7F7F7F7F---7\n\
        L|LJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L";

    const SAMPLE_3: &str = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...";

    test_part_1!(Day10, SAMPLE_1, 8);

    test_part_2!(Day10, SAMPLE_2, 10, SAMPLE_3, 8);

}