use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    fn get_name(&self) -> &str;
    fn get_destinations(&self) -> &Vec<String>;
    fn add_source(&mut self, source: &str);
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

impl FromStr for FlipFlop {
    type Err = &'static str;

    /// Parse things like "%zs -> db, fx"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("%").unwrap();
        let (name, s) = s.split_once(" -> ").unwrap();
        let destinations = s
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(FlipFlop::new(name.to_string(), destinations))
    }
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

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn add_source(&mut self, _source: &str) {}
}

struct Conjunction {
    memory: HashMap<String, Pulse>,
    name: String,
    destinations: Vec<String>,
}

impl FromStr for Conjunction {
    type Err = &'static str;
    /// Parse things like "&sd -> mh, tx, sh, xf, zn, xs"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("&").unwrap();
        let (name, s) = s.split_once(" -> ").unwrap();
        let destinations = s.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(Conjunction::new(name.to_string(), Vec::new(), destinations))
    }
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

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn add_source(&mut self, source: &str) {
        self.memory.insert(source.to_string(), Pulse::Low);
    }
}

struct Broadcast {
    destinations: Vec<String>,
}

impl FromStr for Broadcast {
    type Err = &'static str;
    /// Parse things like "broadcaster -> a, b, c"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("broadcaster -> ").unwrap();
        let destinations = s.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(Broadcast::new(destinations))
    }
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

    fn get_name(&self) -> &str {
        "broadcast"
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn add_source(&mut self, _source: &str) {}
}

struct System {
    modules: HashMap<String, Box<dyn Module>>,
    count: usize,
    done: bool,
}

impl System {
    fn process_pulse(&mut self, pulse: &DirectedPulse) -> Option<Vec<DirectedPulse>> {
        if let Some(module) = self.modules.get_mut(&pulse.destination) {
            module.run(pulse)
        } else if pulse.destination == "rx" && pulse.pulse == Pulse::Low {
            println!("done after {} button presses", self.count);
            self.done = true;
            None
        } else {
            // println!(
            //     "tried to send pulse to non-existant module {} {:?} after {} button pushed",
            //     pulse.destination, pulse.pulse, self.count
            // );
            // self.done = true;
            None
        }
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
        while !self.done {
            self.count += 1;
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
        self.count
    }
}

/// Parse things like "broadcaster -> a, b, c" and "%a -> b"
fn parse_module(line: &str) -> Box<dyn Module> {
    if line.starts_with("broadcaster") {
        Box::new(line.parse::<Broadcast>().unwrap())
    } else if line.starts_with("%") {
        Box::new(line.parse::<FlipFlop>().unwrap())
    } else if line.starts_with("&") {
        Box::new(line.parse::<Conjunction>().unwrap())
    } else {
        panic!()
    }
}
impl FromStr for System {
    type Err = &'static str;
    /// Parse things like "broadcaster -> a, b, c" and "%a -> b"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules = s
            .lines()
            .map(|line| {
                let module = parse_module(line);
                (module.get_name().to_string(), module)
            })
            .collect::<HashMap<_, _>>();
        let source_dest = modules
            .values()
            .map(|module| {
                let source = module.get_name();
                module
                    .get_destinations()
                    .iter()
                    .map(|d| (source.to_string(), d.clone()))
                    .collect::<Vec<_>>()
            })
            .concat();
        source_dest.iter().for_each(|(s, d)| {
            if let Some(module) = modules.get_mut(d) {
                module.add_source(s);
            } else {
                println!("tried to add source for non-existant module {}", d);
            }
        });
        Ok(System {
            modules,
            count: 0,
            done: false,
        })
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
        assert_eq!(super::run(input), 32_000_000);
    }
    #[test]
    fn test2() {
        let input = include_str!("example_data_2.txt");
        assert_eq!(super::run(input), 11_687_500);
    }
}
