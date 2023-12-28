use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone)]
struct DirectedPulse {
    source: String,
    destination: String,
    pulse: Pulse,
}

impl DirectedPulse {
    fn new(source: String, destination: String, pulse: Pulse) -> DirectedPulse {
        DirectedPulse {
            source,
            destination,
            pulse,
        }
    }

    fn from_vec(source: String, destinations: Vec<String>, pulse: Pulse) -> Vec<DirectedPulse> {
        destinations
            .into_iter()
            .map(|d| DirectedPulse::new(source.clone(), d, pulse))
            .collect::<Vec<_>>()
    }
}
trait Module {
    fn run(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>>;
}

enum State {
    On,
    Off,
}
struct FlipFlop {
    state: State,
    name: String,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(name: String, destinations: Vec<String>) -> FlipFlop {
        FlipFlop {
            state: State::Off,
            name,
            destinations,
        }
    }
}

impl Module for FlipFlop {
    fn run(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>> {
        match pulse.pulse {
            Pulse::High => None,
            Pulse::Low => match self.state {
                State::On => {
                    self.state = State::Off;
                    Some(DirectedPulse::from_vec(
                        self.name.clone(),
                        self.destinations.clone(),
                        Pulse::Low,
                    ))
                }
                State::Off => {
                    self.state = State::On;
                    Some(DirectedPulse::from_vec(
                        self.name.clone(),
                        self.destinations.clone(),
                        Pulse::High,
                    ))
                }
            },
        }
    }
}

struct Conjunction {
    memory: HashMap<String, Pulse>,
    name: String,
    destinations: Vec<String>,
}

impl Conjunction {
    fn new(name: String, sources: Vec<String>, destinations: Vec<String>) -> Conjunction {
        let memory = sources
            .into_iter()
            .map(|source| (source, Pulse::Low))
            .collect::<HashMap<_, _>>();
        Conjunction {
            memory,
            name,
            destinations,
        }
    }
    fn all_high(&self) -> bool {
        self.memory.values().all(|pulse| *pulse == Pulse::High)
    }
}

impl Module for Conjunction {
    fn run(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>> {
        let mem = self.memory.get_mut(&pulse.source).unwrap();
        *mem = pulse.pulse;
        if self.all_high() {
            Some(DirectedPulse::from_vec(
                self.name.clone(),
                self.destinations.clone(),
                Pulse::Low,
            ))
        } else {
            Some(DirectedPulse::from_vec(
                self.name.clone(),
                self.destinations.clone(),
                Pulse::High,
            ))
        }
    }
}

struct Broadcast {
    destinations: Vec<String>,
}

impl Broadcast {
    fn new(destinations: Vec<String>) -> Broadcast {
        Broadcast { destinations }
    }
}

impl Module for Broadcast {
    fn run(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>> {
        Some(DirectedPulse::from_vec(
            "broadcast".to_string(),
            self.destinations.clone(),
            pulse.pulse,
        ))
    }
}

struct System {
    modules: HashMap<String, Box<dyn Module>>,
}

impl System {
    fn process_pulse(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>> {
        let module = self.modules.get_mut(&pulse.destination).unwrap();
        module.run(pulse)
    }

    fn process_pulses(&mut self, pulses: &Vec<DirectedPulse>) -> Vec<DirectedPulse> {
        pulses
            .iter()
            .map(|pulse| self.process_pulse(pulse))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
            .concat()
    }

    fn run(&mut self) -> usize {
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut pulses = vec![DirectedPulse::new(
                "button".to_string(),
                "broadcast".to_string(),
                Pulse::Low,
            )];
            while !pulses.is_empty() {
                pulses.iter().for_each(|p| match p.pulse {
                    Pulse::High => {
                        high_count += 1;
                    }
                    Pulse::Low => {
                        low_count += 1;
                    }
                });
                pulses = self.process_pulses(&pulses);
            }
        }
        low_count * high_count
    }
}

impl FromStr for System {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub fn run(input: &str) -> usize {
    input.parse::<System>().unwrap().run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 0);
    }
}
