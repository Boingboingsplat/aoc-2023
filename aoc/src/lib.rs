pub use aoc_macro::EnumFromChar;

pub mod grid;
pub trait Problem {
    type Solution: std::fmt::Debug;
    fn part_1(input: &str) -> Self::Solution;
    fn part_2(input: &str) -> Self::Solution;
    fn benchmark(input: &str) {
        let now = std::time::Instant::now();
        let solution = Self::part_1(input);
        let elapsed = now.elapsed();
        println!("Part 1 solution: {:?} in {:.2?}", solution, elapsed);

        let now = std::time::Instant::now();
        let solution = Self::part_2(input);
        let elapsed = now.elapsed();
        println!("Part 2 solution: {:?} in {:.2?}", solution, elapsed);
    }
}

#[macro_export]
macro_rules! test_part_1 {
    ($t:ident, $( $input:expr, $sol:expr ),+) => {
        #[test]
        fn test_part_1() {
            $( assert_eq!($t::part_1($input), $sol); )+
        }
    };
}

#[macro_export]
macro_rules! test_part_2 {
    ($t:ident, $( $input:expr, $sol:expr ),+) => {
        #[test]
        fn test_part_2() {
            $( assert_eq!($t::part_2($input), $sol); )+
        }
    };
}