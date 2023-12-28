use rayon::prelude::*;
use simple_tqdm::ParTqdm;
use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Parameter {
    X,
    M,
    A,
    S,
}

impl FromStr for Parameter {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Parameter::X),
            "m" => Ok(Parameter::M),
            "a" => Ok(Parameter::A),
            "s" => Ok(Parameter::S),
            _ => Err("unknown parameter"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    LessThan((Parameter, usize, String)),
    GreaterThan((Parameter, usize, String)),
    Goto(String),
}
impl FromStr for Instruction {
    type Err = &'static str;
    /// Parse a<2006:qkq, m>2090:A, rfg
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(">") {
            let (parameter, s) = s.split_once(">").unwrap();
            let (value, destination) = s.split_once(":").unwrap();
            let value = value.parse::<usize>().unwrap();
            let parameter = parameter.parse::<Parameter>().unwrap();
            return Ok(Instruction::GreaterThan((
                parameter,
                value,
                destination.to_string(),
            )));
        }
        if s.contains("<") {
            let (parameter, s) = s.split_once("<").unwrap();
            let (value, destination) = s.split_once(":").unwrap();
            let value = value.parse::<usize>().unwrap();
            let parameter = parameter.parse::<Parameter>().unwrap();
            return Ok(Instruction::LessThan((
                parameter,
                value,
                destination.to_string(),
            )));
        }
        Ok(Instruction::Goto(s.to_string()))
    }
}

#[derive(Debug)]
struct Workflow {
    instructions: Vec<Instruction>,
}

impl Workflow {
    fn run(&self, part: &Part) -> &str {
        for x in &self.instructions {
            match x {
                Instruction::Goto(destination) => {
                    return &destination;
                }
                Instruction::LessThan((parameter, value, destination)) => {
                    let parameter = part.get_parameter(parameter);
                    if parameter < *value {
                        return &destination;
                    }
                }
                Instruction::GreaterThan((parameter, value, destination)) => {
                    let parameter = part.get_parameter(parameter);
                    if parameter > *value {
                        return &destination;
                    }
                }
            }
        }
        panic!()
    }
    /// Find points where the outcome changes in the parameter space
    ///
    /// GreaterThan creates the break at value + 1
    /// a > 2
    /// 0 1 2 3 4
    ///       | |
    ///
    /// LessThan creates the break at value
    /// a < 2
    /// 0 1 2 3 4
    /// | |
    fn get_breaks(&self, parameter: &Parameter) -> Vec<usize> {
        let mut ret = Vec::new();
        for x in &self.instructions {
            match x {
                Instruction::LessThan((param, value, _)) => {
                    if *param != *parameter {
                        continue;
                    }
                    ret.push(*value);
                }
                Instruction::GreaterThan((param, value, _)) => {
                    if *param != *parameter {
                        continue;
                    }
                    ret.push(*value + 1);
                }
                Instruction::Goto(_) => (),
            }
        }
        ret
    }
}

#[derive(Debug)]
struct NamedWorkflow {
    name: String,
    workflow: Workflow,
}

impl FromStr for NamedWorkflow {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (workflow_name, line) = line.split_once("{").unwrap();
        let (line, _) = line.split_once("}").unwrap();
        let workflow = Workflow {
            instructions: line
                .split(",")
                .map(|s| s.parse::<Instruction>().unwrap())
                .collect::<Vec<_>>(),
        };
        Ok(NamedWorkflow {
            name: workflow_name.to_string(),
            workflow,
        })
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Part {
        Part { x, m, a, s }
    }
    fn get_parameter(&self, parameter: &Parameter) -> usize {
        match parameter {
            Parameter::X => self.x,
            Parameter::M => self.m,
            Parameter::A => self.a,
            Parameter::S => self.s,
        }
    }
}

impl FromStr for Part {
    type Err = &'static str;
    /// Parse {x=787,m=2655,a=1222,s=2876}
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, line) = line.split_once("{").unwrap();
        let (line, _) = line.split_once("}").unwrap();
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;
        line.split(",").for_each(|sub| {
            let (symbol, value) = sub.split_once("=").unwrap();
            let value = value.parse::<usize>().unwrap();
            match symbol {
                "x" => {
                    x = Some(value);
                }
                "m" => {
                    m = Some(value);
                }
                "a" => {
                    a = Some(value);
                }
                "s" => {
                    s = Some(value);
                }
                _ => panic!(),
            };
        });
        assert!(x.is_some());
        assert!(m.is_some());
        assert!(a.is_some());
        assert!(s.is_some());
        let x = x.unwrap();
        let m = m.unwrap();
        let a = a.unwrap();
        let s = s.unwrap();
        Ok(Part { x, m, a, s })
    }
}

#[derive(Debug)]
struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    fn part_accepted(&self, part: &Part) -> bool {
        let mut location = "in";
        loop {
            location = self.workflows[location].run(part);
            match location {
                "R" => {
                    return false;
                }
                "A" => {
                    return true;
                }
                _ => (),
            };
        }
    }

    fn run(&self) -> usize {
        let x_breaks = self.get_breaks(&Parameter::X);
        let m_breaks = self.get_breaks(&Parameter::M);
        let a_breaks = self.get_breaks(&Parameter::A);
        let s_breaks = self.get_breaks(&Parameter::S);
        x_breaks
            .windows(2)
            .collect::<Vec<_>>()
            .into_par_iter()
            .tqdm()
            .map(|w| self.run_x_win(w, &m_breaks, &a_breaks, &s_breaks))
            .sum()
    }

    fn run_x_win(
        &self,
        x_win: &[usize],
        m_breaks: &Vec<usize>,
        a_breaks: &Vec<usize>,
        s_breaks: &Vec<usize>,
    ) -> usize {
        let mut count = 0;
        for m_win in m_breaks.windows(2) {
            for a_win in a_breaks.windows(2) {
                for s_win in s_breaks.windows(2) {
                    if self.part_accepted(&Part::new(x_win[0], m_win[0], a_win[0], s_win[0])) {
                        count += (x_win[1] - x_win[0])
                            * (m_win[1] - m_win[0])
                            * (a_win[1] - a_win[0])
                            * (s_win[1] - s_win[0]);
                    }
                }
            }
        }
        count
    }

    /// Get values in parameter space where things change
    /// Returned vec will be sorted and have endpoints
    fn get_breaks(&self, parameter: &Parameter) -> Vec<usize> {
        let ret = self
            .workflows
            .iter()
            .map(|(_, x)| x.get_breaks(parameter))
            .collect::<Vec<_>>();
        let ret = ret.concat();
        let mut ret = ret.into_iter().collect::<BTreeSet<_>>();
        ret.insert(1);
        ret.insert(4001);
        let mut ret = ret.into_iter().collect::<Vec<_>>();
        ret.sort();
        ret
    }
}

impl FromStr for System {
    type Err = &'static str;
    /// Parse px{a<2006:qkq,m>2090:A,rfg} and {x=787,m=2655,a=1222,s=2876}
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let workflows = s
            .lines()
            .filter(|line| line.len() > 0 && !line.starts_with("{"))
            .map(|line| {
                let named_workflow = line.parse::<NamedWorkflow>().unwrap();
                (named_workflow.name, named_workflow.workflow)
            })
            .collect::<HashMap<String, Workflow>>();
        Ok(System { workflows })
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
        assert_eq!(super::run(input), 167409079868000);
    }
}
