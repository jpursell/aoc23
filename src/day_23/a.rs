use std::str::FromStr;

use ndarray::Array2;

struct Node {
    row: usize,
    col: usize,
}
struct Edge {
    start: Node,
    end: Node,
}
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
enum MapSymbol {
    Forest,
    Path,
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for MapSymbol {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(MapSymbol::Forest),
            '.' => Ok(MapSymbol::Path),
            '^' => Ok(MapSymbol::Up),
            'v' => Ok(MapSymbol::Down),
            '<' => Ok(MapSymbol::Left),
            '>' => Ok(MapSymbol::Right),
            _ => Err("Unknown symbol"),
        }
    }
}

impl FromStr for Graph {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| MapSymbol::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = map.len();
        let ncols = map[0].len();
        todo!() // finish parsing graph
        let map = Array2::from_shape_vec((nrows, ncols), map.concat());
        let edges = Vec::new();
        let nodes = Vec::new();

        Ok(Graph{edges, nodes})
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
        assert_eq!(super::run(input), 94);
    }
}
