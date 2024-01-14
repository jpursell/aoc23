use std::{collections::BTreeSet, fmt::Display, str::FromStr};

use ndarray::{s, Array2};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    index: [usize; 2],
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { index: [row, col] }
    }
    fn row(&self) -> usize {
        self.index[0]
    }
    fn col(&self) -> usize {
        self.index[1]
    }
    fn neighbor(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Down => Position::new(self.row() + 1, self.col()),
            Direction::Up => Position::new(self.row() - 1, self.col()),
            Direction::Left => Position::new(self.row(), self.col() - 1),
            Direction::Right => Position::new(self.row(), self.col() + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    position: Position,
}

impl Node {
    fn new(row: usize, col: usize) -> Node {
        Node {
            position: Position::new(row, col),
        }
    }
    fn row(&self) -> usize {
        return self.position.row();
    }
    fn col(&self) -> usize {
        return self.position.col();
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N({}, {})", self.row(), self.col())
    }
}
struct Edge {
    start: Node,
    end: Node,
    weight: usize,
}
impl Edge {
    fn new(start: Node, end: Node, weight: usize) -> Edge {
        Edge { start, end, weight }
    }
}
impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E({}, {}, {})", self.start, self.end, self.weight)
    }
}
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

struct FourDirections {
    up: Option<MapSymbol>,
    down: Option<MapSymbol>,
    left: Option<MapSymbol>,
    right: Option<MapSymbol>,
    position: Position,
    direction: Option<Direction>,
}
impl FourDirections {
    fn new(
        map: &Array2<MapSymbol>,
        irow: usize,
        icol: usize,
        direction: Option<Direction>,
    ) -> Result<FourDirections, ()> {
        let nrows = map.shape()[0];
        let ncols = map.shape()[1];
        if irow <= nrows - 1 && icol <= ncols - 1 && map[[irow, icol]] != MapSymbol::Forest {
            let pos = Position::new(irow, icol);
            let own = |x: Option<&MapSymbol>| {
                if x.is_none() {
                    None
                } else {
                    Some(*(x.unwrap()))
                }
            };
            let up = if irow == 0 {
                None
            } else {
                own(map.get(pos.neighbor(&Direction::Up).index))
            };
            let down = own(map.get(pos.neighbor(&Direction::Down).index));
            let left = if icol == 0 {
                None
            } else {
                own(map.get(pos.neighbor(&Direction::Left).index))
            };
            let right = own(map.get(pos.neighbor(&Direction::Right).index));
            Ok(FourDirections {
                up,
                down,
                left,
                right,
                position: pos,
                direction,
            })
        } else {
            Err(())
        }
    }
    fn row(&self) -> usize {
        self.position.row()
    }
    fn col(&self) -> usize {
        self.position.col()
    }
    fn count_directions(&self) -> usize {
        let mut out = 0;
        if self.up.is_some() && self.up.unwrap() != MapSymbol::Forest {
            out += 1;
        }
        if self.down.is_some() && self.down.unwrap() != MapSymbol::Forest {
            out += 1;
        }
        if self.left.is_some() && self.left.unwrap() != MapSymbol::Forest {
            out += 1;
        }
        if self.right.is_some() && self.right.unwrap() != MapSymbol::Forest {
            out += 1;
        }
        if out > 2 {
            assert_ne!(self.up, Some(MapSymbol::Path));
            assert_ne!(self.down, Some(MapSymbol::Path));
            assert_ne!(self.left, Some(MapSymbol::Path));
            assert_ne!(self.right, Some(MapSymbol::Path));
        }
        out
    }
    fn follow_path(&self, map: &Array2<MapSymbol>) -> FourDirections {
        let direction = self.direction.unwrap();
        let new_position = self.position.neighbor(&direction);
        let mut next_fd =
            FourDirections::new(map, new_position.row(), new_position.col(), None).unwrap();
        let op = direction.opposite();
        if Direction::Up != op && next_fd.up.is_some() && next_fd.up.unwrap() != MapSymbol::Forest {
            next_fd.direction = Some(Direction::Up);
        }
        if Direction::Down != op
            && next_fd.down.is_some()
            && next_fd.down.unwrap() != MapSymbol::Forest
        {
            next_fd.direction = Some(Direction::Down);
        }
        if Direction::Left != op
            && next_fd.left.is_some()
            && next_fd.left.unwrap() != MapSymbol::Forest
        {
            next_fd.direction = Some(Direction::Left);
        }
        if Direction::Right != op
            && next_fd.right.is_some()
            && next_fd.right.unwrap() != MapSymbol::Forest
        {
            next_fd.direction = Some(Direction::Right);
        }
        assert!(next_fd.direction.is_some());
        next_fd
    }
}

fn trace_edge(
    map: &Array2<MapSymbol>,
    node_set: &BTreeSet<Position>,
    pos: &Position,
    direction: &Direction,
) -> Edge {
    let mut fd = FourDirections::new(map, pos.row(), pos.col(), Some(*direction)).unwrap();
    let mut weight = 0;
    loop {
        fd = fd.follow_path(&map);
        weight += 1;
        if node_set.contains(&fd.position) {
            break;
        }
    }
    Edge::new(
        Node::new(pos.row(), pos.col()),
        Node::new(fd.row(), fd.col()),
        weight,
    )
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
        let map = Array2::from_shape_vec((nrows, ncols), map.concat()).unwrap();

        // find Nodes
        let mut nodes = Vec::new();
        // find start node
        {
            let start_row = 0;
            let start_col = map
                .slice(s![start_row, ..])
                .iter()
                .enumerate()
                .filter(|(_, x)| **x == MapSymbol::Path)
                .map(|(i, _)| i)
                .nth(0)
                .unwrap();
            nodes.push(Node::new(start_row, start_col));
        }
        // find middle nodes
        {
            let mut middle_nodes = map
                .indexed_iter()
                .filter(|x| *x.1 != MapSymbol::Forest)
                .filter(|((irow, icol), _)| {
                    FourDirections::new(&map, *irow, *icol, None)
                        .unwrap()
                        .count_directions()
                        > 2
                })
                .map(|((irow, icol), _)| Node::new(irow, icol))
                .collect::<Vec<_>>();
            nodes.append(&mut middle_nodes);
        }
        // find end node
        {
            let end_row = nrows - 1;
            let end_col = map
                .slice(s![end_row, ..])
                .iter()
                .enumerate()
                .filter(|(_, x)| **x == MapSymbol::Path)
                .map(|(i, _)| i)
                .nth(0)
                .unwrap();
            nodes.push(Node::new(end_row, end_col));
        }
        let node_set = nodes.iter().map(|x| x.position).collect::<BTreeSet<_>>();
        let mut edges = Vec::new();
        // add first edge
        edges.push(trace_edge(
            &map,
            &node_set,
            &nodes[0].position,
            &Direction::Down,
        ));
        // add other edges
        {
            for node in &nodes[1..nodes.len() - 1] {
                let fd = FourDirections::new(&map, node.row(), node.col(), None).unwrap();
                if fd.up.is_some() && fd.up.unwrap() == MapSymbol::Up {
                    edges.push(trace_edge(
                        &map,
                        &node_set,
                        &fd.position,
                        &Direction::Up,
                    ));
                }
                if fd.down.is_some() && fd.down.unwrap() == MapSymbol::Down {
                    edges.push(trace_edge(
                        &map,
                        &node_set,
                        &fd.position,
                        &Direction::Down,
                    ));
                }
                if fd.left.is_some() && fd.left.unwrap() == MapSymbol::Left {
                    edges.push(trace_edge(
                        &map,
                        &node_set,
                        &fd.position,
                        &Direction::Left,
                    ));
                }
                if fd.right.is_some() && fd.right.unwrap() == MapSymbol::Right {
                    edges.push(trace_edge(
                        &map,
                        &node_set,
                        &fd.position,
                        &Direction::Right,
                    ));
                }
            }
        }
        Ok(Graph { edges, nodes })
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?
        }
        for edge in &self.edges {
            writeln!(f, "{}", edge)?
        }
        Ok(())
    }
}

pub fn run(input: &str) -> usize {
    let graph = input.parse::<Graph>().unwrap();
    println!("{}", graph);
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
