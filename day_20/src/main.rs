mod parse;

use std::collections::{HashMap, VecDeque};

use aoc::Problem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Signal {
    source: ModuleId,
    target: ModuleId,
    pulse: Pulse,
}

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pulse_str = match self.pulse {
            Pulse::Low => "-low->",
            Pulse::High => "-high->",
        };
        write!(f, "{} {} {}", &self.source.0, pulse_str, &self.target.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ModuleId(String);

#[derive(Debug, PartialEq, Eq)]
enum ModuleKind {
    Broadcast,
    FlipFlop(bool),
    Conjunction {
        last_pulses: HashMap<ModuleId, Pulse>,
    },
}

impl ModuleKind {
    fn new_flipflop() -> Self {
        ModuleKind::FlipFlop(false)
    }

    fn new_conjunction() -> Self {
        ModuleKind::Conjunction { last_pulses: HashMap::new() }
    }

    fn process_pulse(&mut self, source: &ModuleId, pulse: Pulse) -> Option<Pulse> {
        match self {
            ModuleKind::Broadcast => Some(pulse),
            ModuleKind::FlipFlop(state) => {
                // Flip-flop modules only respond to low pulses
                if pulse == Pulse::Low {
                    // Flip state
                    *state = !*state;
                    if *state { 
                        Some(Pulse::High) 
                    } else { 
                        Some(Pulse::Low) 
                    }
                } else {
                    None
                }
            },
            ModuleKind::Conjunction { last_pulses } => {
                // Set the last input
                last_pulses.insert(source.clone(), pulse);
                if last_pulses.values().all(|p| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Module {
    kind: ModuleKind,
    outputs: Vec<ModuleId>,
}

impl Module {
    fn add_input(&mut self, input: ModuleId) {
        if let ModuleKind::Conjunction { last_pulses } = &mut self.kind {
            last_pulses.insert(input, Pulse::Low);
        }
    }

    fn process_signal(&mut self, input: Signal) -> Vec<Signal> {
        let Signal { source, target: self_id, pulse } = input;
        if let Some(out_pulse) = self.kind.process_pulse(&source, pulse) {
            self.outputs.iter()
                .map(|target| {
                    Signal { 
                        source: self_id.clone(),
                        target: target.clone(),
                        pulse: out_pulse,
                    }
                })
                .collect()
        } else {
            // If no pulse output, return an empty vec
            vec![]
        }
    }
}

struct ModuleMachine {
    map: HashMap<ModuleId, Module>
}

impl ModuleMachine { 
    fn new(modules: Vec<(ModuleId, ModuleKind, Vec<ModuleId>)>) -> Self {
        let mut map = HashMap::new();
        let mut in_out = vec![];
        for (id, kind, outputs) in modules {
            // Build an input to output vec so we can properly initialize Conjunction modules
            in_out.push((id.clone(), outputs.clone()));
            map.insert(id, Module { kind, outputs });
        }
        // Update the inputs of modules
        for (input, outputs) in in_out {
            for output in outputs {
                if let Some(module) = map.get_mut(&output) {
                    module.add_input(input.clone());
                }
            }
        }
        Self { map }
    }

    fn press_button(&mut self) -> Vec<Signal> {
        let mut signals = vec![];

        let mut signal_queue = VecDeque::new();
        signal_queue.push_back(Signal {
            source: ModuleId(String::from("button")),
            target: ModuleId(String::from("broadcaster")),
            pulse: Pulse::Low,
        });

        while let Some(signal) = signal_queue.pop_front() {
            // println!("{signal}");
            signals.push(signal.clone());

            if let Some(module) = self.map.get_mut(&signal.target) {
                signal_queue.extend(module.process_signal(signal));
            }
        }

        signals
    }
}

struct Day20;
impl Problem for Day20 {
    type Solution = usize;

    fn part_1(input: &str) -> Self::Solution {
        let data = input.lines().map(|line| parse::parse_line(line).unwrap().1).collect();
        let mut machine = ModuleMachine::new(data);
        // We could detect when a cycle in the machine state and calculate the final result after
        // 1000 inputs from there... but this is fast enough and I'm feeling lazy right now
        let (low, high) = (0..1000)
            .map(|_| { 
                let signals = machine.press_button(); 
                let low = signals.iter().filter(|s| s.pulse == Pulse::Low).count();
                let high = signals.iter().filter(|s| s.pulse == Pulse::High).count();
                (low, high)
            })
            .fold(
                (0, 0), 
                |(acc_low, acc_high), (low, high)| (acc_low + low, acc_high + high)
            );
        low * high
    }

    fn part_2(input: &str) -> Self::Solution {
        let data = input.lines().map(|line| parse::parse_line(line).unwrap().1).collect();
        let mut machine = ModuleMachine::new(data);
        // The input to "rx" is conjunction module "gq"
        // "gq" has inputs "xj", "qs", "kz", "km"
        // find how many inputs it takes to make each of these send a high Pulse,
        // solution is product of these
        let mut gq_input_map = HashMap::new();
        gq_input_map.insert(ModuleId(String::from("xj")), 0);
        gq_input_map.insert(ModuleId(String::from("qs")), 0);
        gq_input_map.insert(ModuleId(String::from("kz")), 0);
        gq_input_map.insert(ModuleId(String::from("km")), 0);
        
        for n in 1..100_000 {
            let pulses = machine.press_button();
            for (id, count) in gq_input_map.iter_mut() {
                if pulses.iter().any(|s| s.source == *id && s.pulse == Pulse::High) {
                    *count = n;
                }
            }
            if gq_input_map.values().all(|count| *count > 0) {
                break;
            }
        }

        gq_input_map.values().product()
    }
}

fn main() {
    let input = include_str!("input.txt");
    Day20::benchmark(input);
}

#[cfg(test)]
mod tests {
    use aoc::test_part_1;

    use super::*; 

    const SAMPLE_1: &str = "\
        broadcaster -> a, b, c\n\
        %a -> b\n\
        %b -> c\n\
        %c -> inv\n\
        &inv -> a";

    const SAMPLE_2: &str = "\
        broadcaster -> a\n\
        %a -> inv, con\n\
        &inv -> b\n\
        %b -> con\n\
        &con -> output";

    test_part_1!(Day20, SAMPLE_1, 32000000, SAMPLE_2, 11687500);
}