use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
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
            return Ok(Instruction::GreaterThan((parameter, value, destination.to_string())));
        }
        if s.contains("<") {
            let (parameter, s) = s.split_once("<").unwrap();
            let (value, destination) = s.split_once(":").unwrap();
            let value = value.parse::<usize>().unwrap();
            let parameter = parameter.parse::<Parameter>().unwrap();
            return Ok(Instruction::LessThan((parameter, value, destination.to_string())));
        }
        Ok(Instruction::Goto(s.to_string()))
    }
}

#[derive(Debug)]
struct Workflow {
    instructions: Vec<Instruction>,
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
        Ok(NamedWorkflow{name: workflow_name.to_string(), workflow})
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
                _ => panic!()
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
    parts: Vec<Part>,
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
        let parts = s
            .lines()
            .filter(|line| line.len() > 0 && line.starts_with("{"))
            .map(|line| line.parse::<Part>().unwrap())
            .collect::<Vec<_>>();
        Ok(System { workflows, parts })
    }
}

pub fn run(input: &str) -> usize {
    let system = input.parse::<System>().unwrap();
    println!("{:?}", system);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 19114);
    }
}
