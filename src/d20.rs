use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Not;
use std::time::Instant;

#[derive(Debug)]
enum SignalType {
    Low,
    High,
}

impl Not for SignalType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            SignalType::High => SignalType::Low,
            SignalType::Low => SignalType::High,
        }
    }
}

enum Module {
    FlipFlop {
        // %
        state: SignalType,
        destinations: Vec<String>,
    },
    Conjunction {
        // &
        input_states: HashMap<String, bool>,
        destinations: Vec<String>,
    },
    Broadcaster {
        destinations: Vec<String>,
    },
}

impl Module {
    fn destinations(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop { destinations, .. } => destinations,
            Module::Conjunction { destinations, .. } => destinations,
            Module::Broadcaster { destinations } => destinations,
        }
    }
}

fn parse_file(file_path: &str) -> HashMap<String, Module> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut modules = HashMap::new();
    let mut conjunctions = HashSet::new();
    for line in file.lines() {
        let (raw_name, raw_dest) = line.split_once(" -> ").unwrap();
        let destinations = raw_dest
            .split(", ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.to_string())
            .collect();
        let mut name_iter = raw_name.chars();
        let c = name_iter.nth(0).unwrap();
        let name = &raw_name[1..].to_string();
        let (module, mod_name) = match c {
            '%' => (
                Module::FlipFlop {
                    state: SignalType::Low,
                    destinations,
                },
                name.clone(),
            ),
            '&' => {
                conjunctions.insert(name.clone());
                (
                    Module::Conjunction {
                        input_states: HashMap::new(),
                        destinations,
                    },
                    name.clone(),
                )
            }
            'b' => (
                Module::Broadcaster { destinations },
                "broadcaster".to_string(),
            ),
            _ => unreachable!(),
        };
        modules.insert(mod_name, module);
    }

    let mut conjunctions_inputs = Vec::new();
    for (module_name, module) in modules.iter() {
        for dest in module.destinations() {
            if conjunctions.contains(dest) {
                conjunctions_inputs.push((dest.clone(), module_name.clone()));
            }
        }
    }
    for (conjunction_name, input) in conjunctions_inputs {
        if let Some(Module::Conjunction { input_states, .. }) = modules.get_mut(&conjunction_name) {
            input_states.insert(input.clone(), false);
        }
    }

    modules
}

impl SignalType {
    pub(crate) fn clone(&self) -> SignalType {
        match self {
            SignalType::High => SignalType::High,
            SignalType::Low => SignalType::Low,
        }
    }
}

#[derive(Debug)]
struct Signal {
    signal_type: SignalType,
    emitter: String,
    target: String,
}

fn broadcast_destinations(
    destinations: &Vec<String>,
    signal_type: SignalType,
    signals: &mut VecDeque<Signal>,
    emitter: &String,
) {
    for destination in destinations {
        signals.push_back(Signal {
            signal_type: signal_type.clone(),
            emitter: emitter.clone(),
            target: destination.clone(),
        })
    }
}

fn handle_signal(
    curr_signal: Signal,
    signals: &mut VecDeque<Signal>,
    modules: &mut HashMap<String, Module>,
) {
    if let Some(target) = modules.get_mut(&curr_signal.target) {
        match target {
            Module::FlipFlop {
                state,
                destinations,
            } => {
                if let SignalType::Low = curr_signal.signal_type {
                    *state = !state.clone();
                    broadcast_destinations(
                        &destinations,
                        state.clone(),
                        signals,
                        &curr_signal.target,
                    );
                }
            }
            Module::Conjunction {
                ref mut input_states,
                destinations,
            } => {
                input_states.insert(
                    curr_signal.emitter.clone(),
                    if let SignalType::High = curr_signal.signal_type {
                        true
                    } else {
                        false
                    },
                );

                broadcast_destinations(
                    &destinations,
                    if input_states.iter().all(|(input, state)| *state) {
                        SignalType::Low
                    } else {
                        SignalType::High
                    },
                    signals,
                    &curr_signal.target,
                );
            }
            Module::Broadcaster { destinations } => broadcast_destinations(
                &destinations,
                curr_signal.signal_type,
                signals,
                &curr_signal.target,
            ),
        }
    }
}

pub fn pulse1(file_path: &str) -> usize {
    let mut modules = parse_file(file_path);

    let mut high_pulse_count = 0;
    let mut low_pulse_count = 0;

    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_back(Signal {
            signal_type: SignalType::Low,
            emitter: "button".to_string(),
            target: "broadcaster".to_string(),
        });

        while let Some(curr_signal) = signals.pop_front() {
            if let SignalType::High = curr_signal.signal_type {
                high_pulse_count += 1
            } else {
                low_pulse_count += 1
            }

            handle_signal(curr_signal, &mut signals, &mut modules);
        }
    }

    high_pulse_count * low_pulse_count
}

pub fn pulse2(file_path: &str) -> usize {
    let mut modules = parse_file(file_path);
    let time_start = Instant::now();

    let mut i = 0;
    loop {
        let mut signals = VecDeque::new();
        signals.push_back(Signal {
            signal_type: SignalType::Low,
            emitter: "button".to_string(),
            target: "broadcaster".to_string(),
        });

        while let Some(curr_signal) = signals.pop_front() {
            if let SignalType::Low = curr_signal.signal_type {
                if curr_signal.target == "rx" {
                    return i;
                }
            }

            handle_signal(curr_signal, &mut signals, &mut modules);
        }
        i += 1;

        if (Instant::now() - time_start).as_secs() >= 10 {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_output::check_results;

    #[test]
    fn p1() {
        check_results("d20", "p1", pulse1);
    }

    // #[test]
    // fn p2() {
    //     check_results("d20", "p2", pulse2);
    // }
}
