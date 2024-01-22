use std::{
    collections::{HashMap, HashSet},
    str::FromStr, fmt::Display,
};

struct Graph {
    edges: HashMap<String, HashSet<String>>,
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges = HashMap::new();
        for line in s.lines() {
            let (l_node, r_nodes) = line.split_once(": ").unwrap();
            if !edges.contains_key(l_node) {
                edges.insert(l_node.to_string(), HashSet::new());
            }
            for r_node in r_nodes.split(" ") {
                edges.get_mut(l_node).unwrap().insert(r_node.to_string());
                if !edges.contains_key(r_node) {
                    edges.insert(r_node.to_string(), HashSet::new());
                }
                edges.get_mut(r_node).unwrap().insert(l_node.to_string());
            }
        }
        Ok(Graph { edges })
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (l_node, set) in &self.edges{
            write!(f, "{}: ", l_node).unwrap();
            for r_node in set {
                write!(f, "{} ", r_node).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

impl Graph {
    /// Find partition that has cost 3, return product of sizes
    fn run(&self) -> usize {
        todo!();
    }
    
}

pub fn run(input: &str) -> usize {
    let graph = input.parse::<Graph>().unwrap();
    println!("{}", graph);
    graph.run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 54);
    }
}
