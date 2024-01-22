use std::{collections::{HashMap, HashSet}, str::FromStr};

struct Graph {
    edges: HashMap<String, HashSet<String>>
}
impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for line in s.lines() {
            let edges = HashMap::new();
            let (l_node, r_nodes) = line.split_once(": ").unwrap();
            if !edges.contains_key(l_node) {
                edges.insert(l_node, HashSet::new());
            }
            for r_node in r_nodes.split(" "){
                edges[l_node].insert(r_node);
                if !edges.contains_key(r_node) {
                    edges.insert(r_node, HashSet::new());
                }
                edges[r_node].insert(l_node);
            }
        }
        Ok(Graph{edges})
    }
}
pub fn run(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 0);
    }
}
