use anyhow::anyhow;
use aoc::Problem;

enum Label {
    Add(String, u64),
    Remove(String),
}

impl Label {
    fn hash(&self) -> u64 {
        match self {
            Label::Add(s, _) => hash(s),
            Label::Remove(s) => hash(s),
        }
    }
}

impl std::str::FromStr for Label {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, val) = s.split_once(['=', '-']).ok_or(anyhow!("Label parse error"))?;
        match val {
            "" => Ok(Label::Remove(name.to_string())),
            v => Ok(Label::Add(name.to_string(), v.parse()?)),
        }
    }
}

struct Box(Vec<(String, u64)>);

impl Box {
    fn new() -> Box {
        Box(vec![])
    }

    fn apply_label(&mut self, label: Label) {
        match label {
            Label::Add(s, v) => {
                if let Some(index) = self.0.iter().position(|(name, _)| *name == s) {
                    self.0[index].1 = v;
                } else {
                    self.0.push((s, v));
                }
            },
            Label::Remove(s) => {
                if let Some(index) = self.0.iter().position(|(name, _)| *name == s) {
                    self.0.remove(index);
                }
            },
        }
    }

    fn power(&self, box_num: u64) -> u64 {
        self.0.iter().enumerate()
            .map(|(i, (_, focal_len))| box_num * (i as u64 + 1) * focal_len)
            .sum()
    }
}

fn hash(s: &str) -> u64 {
    s.as_bytes().iter()
        .fold(0, |acc, &c| {
            (acc + c as u64) * 17 % 256
        })
}

struct Day15;
impl Problem for Day15 {
    type Solution = u64;

    fn part_1(input: &str) -> Self::Solution {
        input.split(',')
            .map(hash)
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        let mut boxes: [Box; 256] = core::array::from_fn(|_| Box::new());

        input.split(',')
            .map(|s| s.parse().unwrap())
            .for_each(|label: Label| {
                let index = label.hash() as usize;
                boxes[index].apply_label(label);
            });

        boxes.iter().enumerate()
            .map(|(i, b)| b.power(i as u64 + 1))
            .sum()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day15::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    test_part_1!(Day15, SAMPLE, 1320);

    test_part_2!(Day15, SAMPLE, 145);
}