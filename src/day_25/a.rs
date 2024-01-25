use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    str::FromStr,
};

use petgraph::{data::Build, graph::UnGraph, stable_graph::IndexType, visit::{EdgeRef, NodeRef}, Graph, Undirected};

struct AocGraph {
    graph: Graph<String, String, Undirected>,
    // node_map: HashMap<String, _>,
    // node_num_to_str: BTreeMap<u32, String>,
}

impl FromStr for AocGraph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("trying to make a graph like here https://docs.rs/petgraph/latest/petgraph/graph/struct.Graph.html#");
        // let mut edges = Vec::new();
        // let mut node_map = HashMap::new();
        let mut graph = Graph::<String, String>::new();

        for line in s.lines() {
            let (l_node, r_nodes) = line.split_once(": ").unwrap();
            graph.add_node(weight)
            // if !node_map.contains_key(l_node) {
            //     node_map.insert(l_node.to_string(), graph.add_node(l_node.to_string()));
            // }
            for r_node in r_nodes.split(" ") {
                // if !node_str_to_num.contains_key(r_node) {
                //     node_str_to_num.insert(r_node.to_string(), node_count);
                //     node_num_to_str.insert(node_count, r_node.to_string());
                //     node_count += 1;
                // }
                // edges.push((l_node.to_string(), r_node.to_string()));
                graph.add_edge(l_node.clone(), r_node.clone(), 1);
            }
        }
        // let graph = UnGraph::<String, String>::from_edges(edges.iter());
        Ok(AocGraph {
            graph,
        })
    }
}

impl Display for AocGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for edge in self.graph.edges() {
            let l_node = edge.source();
            let l_node = l_node.index() as u32;
            let l_node = self.node_num_to_str[&l_node];
            write!(f, "{} ", l_node).unwrap();
            for r_node in set {
                write!(f, "{} ", r_node).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

impl AocGraph {
    /// Find partition that has cost 3, return product of sizes
    fn run(&self) -> usize {
        todo!();
    }
}

pub fn run(input: &str) -> usize {
    let graph = input.parse::<AocGraph>().unwrap();
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
