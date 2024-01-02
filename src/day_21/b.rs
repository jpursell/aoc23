use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    str::FromStr,
};

use ndarray::{s, Array1, Array2, ArrayView2, ArrayViewMut2};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Position {
    index: [i64; 2],
}

impl Position {
    fn new(row: i64, col: i64) -> Position {
        Position { index: [row, col] }
    }
    fn row(&self) -> i64 {
        self.index[0]
    }
    fn col(&self) -> i64 {
        self.index[1]
    }
    fn neighbors(&self) -> [Position; 4] {
        let row = self.row();
        let col = self.col();
        [
            Position::new(row - 1, col),
            Position::new(row + 1, col),
            Position::new(row, col + 1),
            Position::new(row, col - 1),
        ]
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row(), self.col())
    }
}

struct Positions {
    at: BTreeSet<Position>,
}
impl Positions {
    fn step(&self, garden_map: &GardenMap) -> Positions {
        let mut at = BTreeSet::new();
        self.at.iter().for_each(|p| {
            for new_pos in p.neighbors() {
                if garden_map.on_plot(&new_pos) {
                    at.insert(new_pos);
                }
            }
        });
        Positions { at }
    }
    fn compare_from(&self, other: &Positions) {
        for item in self.at.difference(&other.at) {
            println!("added {}", item);
        }
        for item in other.at.difference(&self.at) {
            println!("removed {}", item);
        }
        println!("added {}", self.at.len() - other.at.len());
    }
}

impl From<&GardenMap> for Positions {
    fn from(value: &GardenMap) -> Self {
        let mut at = BTreeSet::new();
        at.insert(value.start);
        Positions { at }
    }
}

#[derive(Debug)]
struct GardenMap {
    plot: Array2<bool>,
    start: Position,
    nrows: i64,
    ncols: i64,
}

impl FromStr for GardenMap {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::default();
        'outer: for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Position::new(irow as i64, icol as i64);
                    break 'outer;
                }
            }
        }
        let plot = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => true,
                        '.' => true,
                        '#' => false,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = plot.len();
        let ncols = plot[0].len();
        let plot = Array2::from_shape_vec((nrows, ncols), plot.concat()).unwrap();
        Ok(GardenMap {
            plot,
            start,
            nrows: nrows as i64,
            ncols: ncols as i64,
        })
    }
}

impl GardenMap {
    fn count_positions(&self, steps: usize) -> usize {
        let mut positions = [Positions::from(self), Positions::from(self)];
        // let mut size = Vec::new();
        for step in 0..steps {
            // if step % 2 == 0 {
            //     size.push(positions[step % 2].at.len() as i64);
            // }
            let new_positions = positions[step % 2].step(self);
            // if (step + 1) > 1 && (step + 1) % 2 == 0 {
            //     println!("\ncompare step {}", step + 1);
            //     new_positions.compare_from(&positions[(step + 1) % 2]);
            // }
            positions[(step + 1) % 2] = new_positions;
        }
        // let growth = size
        //     .windows(2)
        //     .map(|win| win[1] - win[0])
        //     .collect::<Vec<_>>();
        // let growth_change = growth
        //     .windows(2)
        //     .map(|win| win[1] - win[0])
        //     .collect::<Vec<_>>();
        // for (i, val) in growth_change.iter().enumerate() {
        //     println!("step {} growth change {}", i * 2, val);
        // }
        for irow in 0..self.ncols {
            for icol in 0..self.ncols {
                if positions[steps % 2].at.contains(&Position::new(irow, icol)) {
                    print!("O")
                } else {
                    if self.plot[[irow as usize, icol as usize]] {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
            println!("");
        }
        positions[steps % 2].at.len()
    }
    fn count_positions_from_distance(&self, steps: usize) -> usize {
        let dt = self.distance_transform();
        for irow in 0..self.ncols {
            for icol in 0..self.ncols {
                let n = dt[[irow as usize, icol as usize]];
                if n <= steps && n % 2 == steps % 2 {
                    print!("O")
                } else {
                    if self.plot[[irow as usize, icol as usize]] {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }
            println!("");
        }
        dt.iter()
            .filter(|&&x| x <= steps && x % 2 == steps % 2)
            // .inspect(|x| {dbg!(x);})
            .count()
    }
    fn on_plot(&self, position: &Position) -> bool {
        let row = position.row().rem_euclid(self.nrows);
        let col = position.col().rem_euclid(self.ncols);
        self.plot[[row as usize, col as usize]]
    }
    fn distance_transform(&self) -> Array2<usize> {
        let nrows = self.nrows as usize;
        let ncols = self.ncols as usize;
        let mut dt = Array2::from_elem((nrows, ncols), usize::MAX);
        let start = (self.start.row() as usize, self.start.col() as usize);
        *dt.get_mut(start).unwrap() = 0;
        loop {
            let mut changed = false;
            for irow in 0..nrows {
                for icol in 0..ncols {
                    if !self.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = dt[[irow, icol]];
                    if irow > 0 {
                        let n = (irow - 1, icol);
                        if self.plot[n] && dt[n] != usize::MAX {
                            min_val = min_val.min(dt[n] + 1);
                        }
                    }
                    if icol > 0 {
                        let w = (irow, icol - 1);
                        if self.plot[w] && dt[w] != usize::MAX {
                            min_val = min_val.min(dt[w] + 1);
                        }
                    }
                    if dt[[irow, icol]] != min_val {
                        changed = true;
                        *dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            for irow in (0..nrows - 1).rev() {
                for icol in (0..ncols - 1).rev() {
                    if !self.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = dt[[irow, icol]];
                    if irow < nrows - 1 {
                        let s = (irow + 1, icol);
                        if self.plot[s] && dt[s] != usize::MAX {
                            min_val = min_val.min(dt[s] + 1);
                        }
                    }
                    if icol < ncols - 1 {
                        let e = (irow, icol + 1);
                        if self.plot[e] && dt[e] != usize::MAX {
                            min_val = min_val.min(dt[e] + 1);
                        }
                    }
                    if dt[[irow, icol]] != min_val {
                        changed = true;
                        *dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        dt
    }
}

enum Edge {
    Top(Array1<usize>),
    Bottom(Array1<usize>),
    Left(Array1<usize>),
    Right(Array1<usize>),
}
struct DTTile {
    edge: Edge,
    count: usize,
}
impl DTTile {
    fn from_point(start: &Position, garden_map: &GardenMap, steps: usize) -> [DTTile; 4] {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut dt = Array2::from_elem((nrows, ncols), usize::MAX);
        let start = (start.row() as usize, start.col() as usize);
        *dt.get_mut(start).unwrap() = 0;
        DTTile::populate_distance_transform(&mut dt.view_mut(), garden_map);
        let count = DTTile::count_dt(&dt.view(), steps);
        let top_edge = DTTile::get_top_edge(&dt.view());
        let bottom_edge = DTTile::get_bottom_edge(&dt.view());
        let left_edge = DTTile::get_left_edge(&dt.view());
        let right_edge = DTTile::get_right_edge(&dt.view());
        [
            DTTile {
                count,
                edge: top_edge,
            },
            DTTile {
                count: 0,
                edge: bottom_edge,
            },
            DTTile {
                count: 0,
                edge: left_edge,
            },
            DTTile {
                count: 0,
                edge: right_edge,
            },
        ]
    }
    fn from_edge(edge: &Edge, garden_map: &GardenMap, steps: usize) -> DTTile {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut dt = Array2::from_elem((nrows, ncols), usize::MAX);
        match edge {
            Edge::Top(arr) => {
                let mut slice = dt.slice_mut(s![nrows - 1, ..]);
                slice.assign(arr);
            }
            Edge::Left(arr) => {
                let mut slice = dt.slice_mut(s![.., ncols - 1]);
                slice.assign(arr);
            }
            Edge::Bottom(arr) => {
                let mut slice = dt.slice_mut(s![0, ..]);
                slice.assign(arr);
            }
            Edge::Right(arr) => {
                let mut slice = dt.slice_mut(s![.., 0]);
                slice.assign(arr);
            }
        };
        DTTile::populate_distance_transform(&mut dt.view_mut(), garden_map);
        let count = DTTile::count_dt(&dt.view(), steps);
        let out_edge = match edge {
            Edge::Top(_) => DTTile::get_top_edge(&dt.view()),
            Edge::Bottom(_) => DTTile::get_bottom_edge(&dt.view()),
            Edge::Left(_) => DTTile::get_left_edge(&dt.view()),
            Edge::Right(_) => DTTile::get_right_edge(&dt.view()),
        };
        DTTile {
            count,
            edge: out_edge,
        }
    }
    fn get_left_edge(dt: &ArrayView2<usize>) -> Edge {
        Edge::Left(dt.slice(s![.., 0,]).to_owned())
    }

    fn get_top_edge(dt: &ArrayView2<usize>) -> Edge {
        Edge::Top(dt.slice(s![0, ..]).to_owned())
    }

    fn get_bottom_edge(dt: &ArrayView2<usize>) -> Edge {
        let nrows = dt.shape()[0];
        Edge::Bottom(dt.slice(s![nrows - 1, ..]).to_owned())
    }
    fn get_right_edge(dt: &ArrayView2<usize>) -> Edge {
        let ncols = dt.shape()[1];
        Edge::Right(dt.slice(s![.., ncols - 1]).to_owned())
    }

    fn count_dt(dt: &ArrayView2<usize>, steps: usize) -> usize {
        dt.iter()
            .filter(|&&x| x <= steps && x % 2 == steps % 2)
            .count()
    }

    fn populate_distance_transform(dt: &mut ArrayViewMut2<usize>, garden_map: &GardenMap) {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        loop {
            let mut changed = false;
            for irow in 0..nrows {
                for icol in 0..ncols {
                    if !garden_map.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = dt[[irow, icol]];
                    if irow > 0 {
                        let n = (irow - 1, icol);
                        if garden_map.plot[n] && dt[n] != usize::MAX {
                            min_val = min_val.min(dt[n] + 1);
                        }
                    }
                    if icol > 0 {
                        let w = (irow, icol - 1);
                        if garden_map.plot[w] && dt[w] != usize::MAX {
                            min_val = min_val.min(dt[w] + 1);
                        }
                    }
                    if dt[[irow, icol]] != min_val {
                        changed = true;
                        *dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            for irow in (0..nrows - 1).rev() {
                for icol in (0..ncols - 1).rev() {
                    if !garden_map.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = dt[[irow, icol]];
                    if irow < nrows - 1 {
                        let s = (irow + 1, icol);
                        if garden_map.plot[s] && dt[s] != usize::MAX {
                            min_val = min_val.min(dt[s] + 1);
                        }
                    }
                    if icol < ncols - 1 {
                        let e = (irow, icol + 1);
                        if garden_map.plot[e] && dt[e] != usize::MAX {
                            min_val = min_val.min(dt[e] + 1);
                        }
                    }
                    if dt[[irow, icol]] != min_val {
                        changed = true;
                        *dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }
}
struct CompositeDT {
    tiles: BTreeMap<Position, DTTile>,
}

impl CompositeDT {
    fn new(garden_map: &GardenMap, steps: usize) -> CompositeDT {
        // make center
        let center = DTTile::from_point(&garden_map.start, garden_map, steps);
        // todo: properly add center and grow edges
        // todo: thinking about refactoring tiles to use methods to create edges
        let tiles = BTreeMap::new();
        CompositeDT { tiles }
    }
}

pub(crate) enum Mode {
    DistanceTransform,
    Basic,
}
pub fn run(input: &str, steps: usize, mode: Mode) -> usize {
    match mode {
        Mode::Basic => input.parse::<GardenMap>().unwrap().count_positions(steps),
        Mode::DistanceTransform => input
            .parse::<GardenMap>()
            .unwrap()
            .count_positions_from_distance(steps),
    }
}

#[cfg(test)]
mod tests {
    use crate::day_21::b::Mode;

    use super::GardenMap;

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 6, Mode::Basic), 16);
    }
    #[test]
    fn test1_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 6, Mode::DistanceTransform), 16);
    }
    #[test]
    fn test2() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10, Mode::Basic), 50);
    }
    #[test]
    fn test2_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10, Mode::DistanceTransform), 50);
    }
    #[test]
    fn test3() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 50, Mode::Basic), 1594);
    }
    #[test]
    fn test4() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 100, Mode::Basic), 6536);
    }
    #[test]
    fn test5() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 500, Mode::Basic), 167004);
    }
    #[test]
    fn test6() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 1000, Mode::Basic), 668697);
    }
    #[test]
    fn test7() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 5000, Mode::Basic), 16733044);
    }
    #[test]
    fn test_dt() {
        let input = include_str!("example_data.txt");
        let gm = input.parse::<GardenMap>().unwrap();
        let dt = gm.distance_transform();
        let nrows = dt.shape()[0];
        let ncols = dt.shape()[1];
        for irow in 0..nrows {
            for icol in 0..ncols {
                if dt[[irow, icol]] == usize::MAX {
                    print!(" .")
                } else {
                    print!("{:02}", dt[[irow, icol]]);
                }
            }
            println!("");
        }
    }
}
