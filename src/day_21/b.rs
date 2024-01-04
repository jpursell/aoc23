use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use ndarray::{s, Array1, Array2};

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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row(), self.col())
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

#[derive(Clone)]
enum Edge {
    Top(Array1<usize>),
    Bottom(Array1<usize>),
    Left(Array1<usize>),
    Right(Array1<usize>),
}

impl Default for Edge {
    fn default() -> Self {
        Edge::Top(Array1::default(0))
    }
}

struct DTTileCore {
    dt: Array2<usize>,
}

impl DTTileCore {
    fn new(nrows: usize, ncols: usize) -> Self {
        DTTileCore {
            dt: Array2::from_elem((nrows, ncols), usize::MAX),
        }
    }
    fn from_point(start: &Position, garden_map: &GardenMap) -> Self {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut core = Self::new(nrows, ncols);
        let start = (start.row() as usize, start.col() as usize);
        *core.dt.get_mut(start).unwrap() = 0;
        core.populate_distance_transform(garden_map);
        core
    }
    fn from_edge(edge: &Edge, garden_map: &GardenMap) -> Self {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut core = Self::new(nrows, ncols);
        match edge {
            Edge::Top(arr) => {
                let mut slice = core.dt.slice_mut(s![nrows - 1, ..]);
                slice.assign(arr);
                slice.map_inplace(|x| {
                    *x += 1;
                });
            }
            Edge::Left(arr) => {
                let mut slice = core.dt.slice_mut(s![.., ncols - 1]);
                slice.assign(arr);
                slice.map_inplace(|x| {
                    *x += 1;
                });
            }
            Edge::Bottom(arr) => {
                let mut slice = core.dt.slice_mut(s![0, ..]);
                slice.assign(arr);
                slice.map_inplace(|x| {
                    *x += 1;
                });
            }
            Edge::Right(arr) => {
                let mut slice = core.dt.slice_mut(s![.., 0]);
                slice.assign(arr);
                slice.map_inplace(|x| {
                    *x += 1;
                });
            }
        };
        core.populate_distance_transform(garden_map);
        core
    }
    fn get_left_edge(&self) -> Edge {
        Edge::Left(self.dt.slice(s![.., 0,]).to_owned())
    }

    fn get_top_edge(&self) -> Edge {
        Edge::Top(self.dt.slice(s![0, ..]).to_owned())
    }

    fn get_bottom_edge(&self) -> Edge {
        let nrows = self.dt.shape()[0];
        Edge::Bottom(self.dt.slice(s![nrows - 1, ..]).to_owned())
    }
    fn get_right_edge(&self) -> Edge {
        let ncols = self.dt.shape()[1];
        Edge::Right(self.dt.slice(s![.., ncols - 1]).to_owned())
    }

    fn count_dt(&self, steps: usize, cmem: &mut CountMem) -> usize {
        let nrows = self.dt.shape()[0];
        let ncols = self.dt.shape()[1];
        let prediction = cmem.predict(
            self.dt[[0, 0]],
            self.dt[[0, ncols - 1]],
            self.dt[[nrows - 1, 0]],
            self.dt[[nrows - 1, ncols - 1]],
            steps,
        );
        if prediction.is_some() {
            return *prediction.unwrap();
        }
        let count = self
            .dt
            .iter()
            .filter(|&&x| x <= steps && x % 2 == steps % 2)
            .count();
        cmem.learn(
            self.dt[[0, 0]],
            self.dt[[0, ncols - 1]],
            self.dt[[nrows - 1, 0]],
            self.dt[[nrows - 1, ncols - 1]],
            count,
            steps,
        );
        count
    }

    fn populate_distance_transform(&mut self, garden_map: &GardenMap) {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        loop {
            let mut changed = false;
            for irow in 0..nrows {
                for icol in 0..ncols {
                    if !garden_map.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = self.dt[[irow, icol]];
                    if irow > 0 {
                        let n = (irow - 1, icol);
                        if garden_map.plot[n] && self.dt[n] != usize::MAX {
                            min_val = min_val.min(self.dt[n] + 1);
                        }
                    }
                    if icol > 0 {
                        let w = (irow, icol - 1);
                        if garden_map.plot[w] && self.dt[w] != usize::MAX {
                            min_val = min_val.min(self.dt[w] + 1);
                        }
                    }
                    if self.dt[[irow, icol]] != min_val {
                        changed = true;
                        *self.dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            for irow in (0..nrows).rev() {
                for icol in (0..ncols).rev() {
                    if !garden_map.plot[(irow, icol)] {
                        continue;
                    }
                    let mut min_val = self.dt[[irow, icol]];
                    if irow < nrows - 1 {
                        let s = (irow + 1, icol);
                        if garden_map.plot[s] && self.dt[s] != usize::MAX {
                            min_val = min_val.min(self.dt[s] + 1);
                        }
                    }
                    if icol < ncols - 1 {
                        let e = (irow, icol + 1);
                        if garden_map.plot[e] && self.dt[e] != usize::MAX {
                            min_val = min_val.min(self.dt[e] + 1);
                        }
                    }
                    if self.dt[[irow, icol]] != min_val {
                        changed = true;
                        *self.dt.get_mut([irow, icol]).unwrap() = min_val;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }
    fn print(&self, garden_map: &GardenMap) {
        let nrows = self.dt.shape()[0];
        let ncols = self.dt.shape()[1];
        for irow in 0..nrows {
            for icol in 0..ncols {
                if garden_map.plot[[irow, icol]] {
                    let d = self.dt[[irow, icol]];
                    if d == usize::MAX {
                        panic!()
                    }
                    print!("{:02}", d);
                } else {
                    assert_eq!(self.dt[[irow, icol]], usize::MAX);
                    print!(" #");
                }
            }
            println!("");
        }
    }
}
struct CompositeDT {
    count: usize,
    left: Vec<Edge>,
    right: Vec<Edge>,
    top: Vec<Edge>,
    bottom: Vec<Edge>,
}

impl CompositeDT {
    fn expand(&self, garden_map: &GardenMap, steps: usize, cmem: &mut CountMem) -> Self {
        let mut top = vec![Edge::default(); self.top.len() + 2];
        let mut left = vec![Edge::default(); self.left.len() + 2];
        let mut right = vec![Edge::default(); self.right.len() + 2];
        let mut bottom = vec![Edge::default(); self.bottom.len() + 2];
        let mut count = self.count;
        let debug = false;
        self.top.iter().enumerate().for_each(|(i, old)| {
            let tile = DTTileCore::from_edge(old, garden_map);
            if debug {
                println!("top tile {} {}", i + 1, tile.count_dt(steps, cmem));
                // tile.print(garden_map);
            }
            count += tile.count_dt(steps, cmem);
            *top.get_mut(i + 1).unwrap() = tile.get_top_edge();
            if i == 0 {
                // top left corner
                let top_left = DTTileCore::from_edge(&tile.get_left_edge(), garden_map);
                if debug {
                    println!("top left {}", top_left.count_dt(steps, cmem));
                    // top_left.print(garden_map);
                }
                count += top_left.count_dt(steps, cmem);
                *top.first_mut().unwrap() = top_left.get_top_edge();
                *left.first_mut().unwrap() = top_left.get_left_edge();
            }
            if i == self.top.len() - 1 {
                // top right corner
                let top_right = DTTileCore::from_edge(&tile.get_right_edge(), garden_map);
                if debug {
                    println!("top right {}", top_right.count_dt(steps, cmem));
                    // top_right.print(garden_map);
                }
                count += top_right.count_dt(steps, cmem);
                *top.last_mut().unwrap() = top_right.get_top_edge();
                *right.first_mut().unwrap() = top_right.get_right_edge();
            }
        });
        self.bottom.iter().enumerate().for_each(|(i, old)| {
            let tile = DTTileCore::from_edge(old, garden_map);
            if debug {
                println!("bottom tile {} {}", i + 1, tile.count_dt(steps, cmem));
                // tile.print(garden_map);
            }
            count += tile.count_dt(steps, cmem);
            *bottom.get_mut(i + 1).unwrap() = tile.get_bottom_edge();
            if i == 0 {
                // bottom left corner
                let bottom_left = DTTileCore::from_edge(&tile.get_left_edge(), garden_map);
                if debug {
                    println!("bottom left {}", bottom_left.count_dt(steps, cmem));
                    // bottom_left.print(garden_map);
                }
                count += bottom_left.count_dt(steps, cmem);
                *bottom.first_mut().unwrap() = bottom_left.get_bottom_edge();
                *left.last_mut().unwrap() = bottom_left.get_left_edge();
            }
            if i == self.bottom.len() - 1 {
                // bottom right corner
                let bottom_right = DTTileCore::from_edge(&tile.get_right_edge(), garden_map);
                if debug {
                    println!("bottom right {}", bottom_right.count_dt(steps, cmem));
                    // bottom_right.print(garden_map);
                }
                count += bottom_right.count_dt(steps, cmem);
                *bottom.last_mut().unwrap() = bottom_right.get_bottom_edge();
                *right.last_mut().unwrap() = bottom_right.get_right_edge();
            }
        });
        self.left.iter().enumerate().for_each(|(i, old)| {
            let tile = DTTileCore::from_edge(old, garden_map);
            if debug {
                println!("left tile {} {}", i + 1, tile.count_dt(steps, cmem));
                // tile.print(garden_map);
            }
            count += tile.count_dt(steps, cmem);
            *left.get_mut(i + 1).unwrap() = tile.get_left_edge();
        });
        self.right.iter().enumerate().for_each(|(i, old)| {
            let tile = DTTileCore::from_edge(old, garden_map);
            if debug {
                println!("right tile {} {}", i + 1, tile.count_dt(steps, cmem));
                // tile.print(garden_map);
            }
            count += tile.count_dt(steps, cmem);
            *right.get_mut(i + 1).unwrap() = tile.get_right_edge();
        });
        CompositeDT {
            count,
            left,
            right,
            top,
            bottom,
        }
    }
    fn new(garden_map: &GardenMap, steps: usize, cmem: &mut CountMem) -> CompositeDT {
        let center = DTTileCore::from_point(&garden_map.start, garden_map);
        let debug = true;
        if debug {
            println!("center {}", center.count_dt(steps, cmem));
            // center.print(garden_map);
        }
        let count = center.count_dt(steps, cmem);
        // make first ring
        let left = vec![center.get_left_edge()];
        let right = vec![center.get_right_edge()];
        let top = vec![center.get_top_edge()];
        let bottom = vec![center.get_bottom_edge()];
        CompositeDT {
            count,
            left,
            right,
            bottom,
            top,
        }
    }
}

struct CountMem {
    data: BTreeMap<(usize, usize, usize, usize), usize>,
    nrows: usize,
}

impl CountMem {
    fn new(nrows: usize) -> CountMem {
        CountMem {
            data: BTreeMap::new(),
            nrows,
        }
    }

    fn normalize(coords: (usize, usize, usize, usize)) -> (usize, usize, usize, usize) {
        let mut offset = coords.0.min(coords.1.min(coords.2.min(coords.3)));
        if offset % 2 == 1 {
            offset -= 1;
        }
        (
            coords.0 - offset,
            coords.1 - offset,
            coords.2 - offset,
            coords.3 - offset,
        )
    }

    fn all_lower(
        &self,
        upper_left: usize,
        upper_right: usize,
        lower_left: usize,
        lower_right: usize,
        steps: usize,
    ) -> bool {
        let max_val = upper_left.max(upper_right.max(lower_left.max(lower_right))) + self.nrows;
        steps > max_val
    }

    fn predict(
        &self,
        upper_left: usize,
        upper_right: usize,
        lower_left: usize,
        lower_right: usize,
        steps: usize,
    ) -> Option<&usize> {
        if !self.all_lower(upper_left, upper_right, lower_left, lower_right, steps) {
            return None;
        }
        let key = CountMem::normalize((upper_left, upper_right, lower_left, lower_right));
        self.data.get(&key)
    }

    fn learn(
        &mut self,
        upper_left: usize,
        upper_right: usize,
        lower_left: usize,
        lower_right: usize,
        count: usize,
        steps: usize,
    ) {
        if !self.all_lower(upper_left, upper_right, lower_left, lower_right, steps) {
            return;
        }
        let key = CountMem::normalize((upper_left, upper_right, lower_left, lower_right));
        assert!(self.data.insert(key, count).is_none());
    }
}

pub fn run(input: &str, steps: usize) -> usize {
    let garden_map = input.parse::<GardenMap>().unwrap();
    let mut cmem = CountMem::new(garden_map.nrows as usize);
    let mut cdt = CompositeDT::new(&garden_map, steps, &mut cmem);
    let debug = true;
    if debug {
        println!("core count {}", cdt.count);
    }
    let mut rings = 0;
    loop {
        let new_cdt = cdt.expand(&garden_map, steps, &mut cmem);
        if debug {
            rings += 1;
            println!("\nring {} count {}", rings, new_cdt.count);
        }
        if new_cdt.count == cdt.count {
            break;
        }
        cdt = new_cdt;
    }
    cdt.count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 6,), 16);
    }
    #[test]
    fn test2_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10,), 50);
    }
    #[test]
    fn test3_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 50,), 1594);
    }
    #[test]
    fn test4_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 100,), 6536);
    }
    #[test]
    fn test5_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 500,), 167004);
    }
    #[test]
    fn test6_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 1000,), 668697);
    }
    #[test]
    fn test7_dt() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 5000,), 16733044);
    }
}
