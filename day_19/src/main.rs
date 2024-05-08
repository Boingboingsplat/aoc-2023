use std::{collections::HashMap, ops::Range};

use aoc::Problem;

mod parse;

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PartRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PartRange {
    fn combinations(&self) -> u64 {
        (self.x.end - self.x.start)
        * (self.m.end - self.m.start)
        * (self.a.end - self.a.start)
        * (self.s.end - self.s.start)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Attribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Check {
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Res {
    Accept,
    Reject,
    Send(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule(Attribute, Check, u64, Res);

impl Rule {
    fn apply(&self, part: &Part) -> Option<&Res> {
        let Rule(attr, check, target, res) = self;
        let val = match attr {
            Attribute::X => part.x,
            Attribute::M => part.m,
            Attribute::A => part.a,
            Attribute::S => part.s,
        };
        match check {
            Check::LessThan => {
                if val < *target { return Some(res); }
            },
            Check::GreaterThan => {
                if val > *target { return Some(res); }
            },
        }
        None
    }

    fn apply_range(&self, part_range: &PartRange) -> ((&Res, PartRange), PartRange) {
        // Splits range into accepted section and rejected section
        // Returns result of accepted section
        let Rule(attr, check, target, res) = self;
        let val_range = match attr {
            Attribute::X => &part_range.x,
            Attribute::M => &part_range.m,
            Attribute::A => &part_range.a,
            Attribute::S => &part_range.s,
        };

        let (accepted, rejected) = match check {
            Check::LessThan => (val_range.start..*target, *target..val_range.end),
            Check::GreaterThan => (*target+1..val_range.end, val_range.start..*target+1),
        };

        let (mut accepted_range, mut rejected_range) = (part_range.clone(), part_range.clone());
        match attr {
            Attribute::X => {
                accepted_range.x = accepted;
                rejected_range.x = rejected;
            },
            Attribute::M => {
                accepted_range.m = accepted;
                rejected_range.m = rejected;
            },
            Attribute::A => {
                accepted_range.a = accepted;
                rejected_range.a = rejected;
            },
            Attribute::S => {
                accepted_range.s = accepted;
                rejected_range.s = rejected;
            },
        }
        ((res, accepted_range), rejected_range)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Workflow {
    rules: Vec<Rule>,
    fallback: Res,
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Res {
        self.rules.iter()
            .find_map(|rule| rule.apply(part))
            .unwrap_or(&self.fallback)
    }

    fn apply_range(&self, part_range: PartRange) -> Vec<(&Res, PartRange)> {
        // Returns a set of ranges that result from applying workflow to range
        let mut output = vec![];
        let mut curr_range = part_range;
        for rule in &self.rules {
            let (accepted, rejected) = rule.apply_range(&curr_range);
            output.push(accepted);
            curr_range = rejected;
        }
        output.push((&self.fallback, curr_range));
        output
    }
}

fn test_part(workflow_map: &HashMap<String, Workflow>, name: &str, part: &Part) -> Res {
    let workflow = workflow_map.get(name).unwrap_or_else(|| panic!("Couldn't find workflow {name}"));
    match workflow.apply(part) {
        Res::Accept => Res::Accept,
        Res::Reject => Res::Reject,
        Res::Send(name) => test_part(workflow_map, name, part),
    }
}

struct Day19;
impl Problem for Day19 {
    type Solution = u64;

    fn part_1(input: &str) -> Self::Solution {
        let (workflow_str, part_str) = input.split_once("\n\n").unwrap();

        let mut workflow_map = HashMap::new();
        for line in workflow_str.lines() {
            let (_, (name, workflow)) = parse::parse_workflow(line).unwrap();
            workflow_map.insert(name, workflow);
        }

        part_str.lines()
            .map(|line| parse::parse_part(line).unwrap().1)
            .filter(|part| test_part(&workflow_map, "in", part) == Res::Accept)
            .map(|part| part.x + part.m + part.a + part.s )
            .sum()
    }

    fn part_2(input: &str) -> Self::Solution {
        let (workflow_str, _part_str) = input.split_once("\n\n").unwrap();

        let mut workflow_map = HashMap::new();
        for line in workflow_str.lines() {
            let (_, (name, workflow)) = parse::parse_workflow(line).unwrap();
            workflow_map.insert(name, workflow);
        }

        let mut count = 0;
        let mut range_frontier = vec![("in", PartRange { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 })];

        while let Some((name, part_range)) = range_frontier.pop() {
            let workflow = workflow_map.get(name).unwrap_or_else(|| panic!("Couldn't find workflow {name}"));
            for (res, out_range) in workflow.apply_range(part_range) {
                match res {
                    Res::Accept => { count += out_range.combinations(); },
                    Res::Reject => (),
                    Res::Send(name) => { range_frontier.push((name, out_range))},
                }
            }
        }
        
        count
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day19::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::{test_part_1, test_part_2};

    use super::*; 

    const SAMPLE: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}";

    test_part_1!(Day19, SAMPLE, 19114);

    test_part_2!(Day19, SAMPLE, 167409079868000);
}