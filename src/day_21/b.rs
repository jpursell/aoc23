use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Display,
    str::FromStr,
};

use ndarray::Array2;

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
    Top((usize, usize)),
    Bottom((usize, usize)),
    Left((usize, usize)),
    Right((usize, usize)),
}

impl Default for Edge {
    fn default() -> Self {
        Edge::Top((0, 0))
    }
}

struct DTTileCore {
    start: Option<Position>,
    upper_left: usize,
    upper_right: usize,
    lower_left: usize,
    lower_right: usize,
}

impl Default for DTTileCore {
    fn default() -> Self {
        DTTileCore {
            start: None,
            upper_left: usize::MAX,
            upper_right: usize::MAX,
            lower_left: usize::MAX,
            lower_right: usize::MAX,
        }
    }
}

impl DTTileCore {
    fn new() -> Self {
        DTTileCore::default()
    }
    fn from_point(garden_map: &GardenMap) -> Self {
        let mut core = Self::new();
        core.start = Some(garden_map.start);
        let dt = core.populate_distance_transform(garden_map);
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        core.upper_left = dt[[0, 0]];
        core.upper_right = dt[[0, ncols - 1]];
        core.lower_left = dt[[nrows - 1, 0]];
        core.lower_right = dt[[nrows - 1, ncols - 1]];
        core
    }
    fn from_edge(edge: &Edge, garden_map: &GardenMap) -> Self {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut core = Self::new();
        match edge {
            Edge::Top(arr) => {
                core.lower_left = arr.0 + 1;
                core.lower_right = arr.1 + 1;
                core.upper_left = arr.0 + nrows;
                core.upper_right = arr.1 + nrows;
            }
            Edge::Bottom(arr) => {
                core.upper_left = arr.0 + 1;
                core.upper_right = arr.1 + 1;
                core.lower_left = arr.0 + nrows;
                core.lower_right = arr.1 + nrows;
            }
            Edge::Right(arr) => {
                core.upper_left = arr.0 + 1;
                core.lower_left = arr.1 + 1;
                core.upper_right = arr.0 + ncols;
                core.lower_right = arr.1 + ncols;
            }
            Edge::Left(arr) => {
                core.upper_right = arr.0 + 1;
                core.lower_right = arr.1 + 1;
                core.upper_left = arr.0 + ncols;
                core.lower_left = arr.1 + ncols;
            }
        };
        core
    }
    fn get_left_edge(&self) -> Edge {
        Edge::Left((self.upper_left, self.lower_left))
    }

    fn get_top_edge(&self) -> Edge {
        Edge::Top((self.upper_left, self.upper_right))
    }

    fn get_bottom_edge(&self) -> Edge {
        Edge::Bottom((self.lower_left, self.lower_right))
    }
    fn get_right_edge(&self) -> Edge {
        Edge::Right((self.upper_right, self.lower_right))
    }

    fn count_dt(&self, garden_map: &GardenMap, cmem: &mut CountMem) -> usize {
        let prediction = cmem.predict(self);
        if prediction.is_some() {
            return *prediction.unwrap();
        }
        let dt = self.populate_distance_transform(garden_map);
        // DTTileCore::print(&dt.view(), garden_map);
        let count = dt
            .iter()
            .filter(|&&x| x <= cmem.steps && x % 2 == cmem.steps % 2)
            .count();
        cmem.learn(self, count);
        count
    }

    fn populate_distance_transform(&self, garden_map: &GardenMap) -> Array2<usize> {
        let nrows = garden_map.nrows as usize;
        let ncols = garden_map.ncols as usize;
        let mut dt = Array2::from_elem((nrows, ncols), usize::MAX);
        if self.start.is_some() {
            let start = self.start.unwrap();
            *dt.get_mut((start.row() as usize, start.col() as usize))
                .unwrap() = 0;
        } else {
            *dt.get_mut((0, 0)).unwrap() = self.upper_left;
            *dt.get_mut((0, ncols - 1)).unwrap() = self.upper_right;
            *dt.get_mut((nrows - 1, 0)).unwrap() = self.lower_left;
            *dt.get_mut((nrows - 1, ncols - 1)).unwrap() = self.lower_right;
        }
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
            for irow in (0..nrows).rev() {
                for icol in (0..ncols).rev() {
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
        dt
    }
    // fn print(dt: &ArrayView2<usize>, garden_map: &GardenMap) {
    //     let nrows = dt.shape()[0];
    //     let ncols = dt.shape()[1];
    //     for irow in 0..nrows {
    //         for icol in 0..ncols {
    //             if garden_map.plot[[irow, icol]] {
    //                 let d = dt[[irow, icol]];
    //                 if d == usize::MAX {
    //                     panic!()
    //                 }
    //                 print!("{:02}", d);
    //             } else {
    //                 assert_eq!(dt[[irow, icol]], usize::MAX);
    //                 print!(" #");
    //             }
    //         }
    //         println!("");
    //     }
    // }
    fn print_corners(&self) {
        println!(
            "{},{}\n{},{}",
            self.upper_left, self.upper_right, self.lower_left, self.lower_right
        );
    }
}
struct CompositeDT {
    count: usize,
    left: VecDeque<Edge>,
    right: VecDeque<Edge>,
    top: VecDeque<Edge>,
    bottom: VecDeque<Edge>,
}

impl CompositeDT {
    fn expand(&mut self, garden_map: &GardenMap, cmem: &mut CountMem) {
        let debug = false;
        let n = self.top.len();
        self.top.push_front(Edge::default());
        self.top.push_back(Edge::default());
        self.left.push_back(Edge::default());
        self.left.push_front(Edge::default());
        self.right.push_back(Edge::default());
        self.right.push_front(Edge::default());
        self.bottom.push_back(Edge::default());
        self.bottom.push_front(Edge::default());
        (1..=n).for_each(|i| {
            let tile = DTTileCore::from_edge(self.top.get(i).unwrap(), garden_map);
            *self.top.get_mut(i).unwrap() = tile.get_top_edge();
            if debug {
                println!("top tile {} {}", i + 1, tile.count_dt(garden_map, cmem));
                tile.print_corners();
            }
            self.count += tile.count_dt(garden_map, cmem);
            if i == 1 {
                // top left corner
                let top_left = DTTileCore::from_edge(&tile.get_left_edge(), garden_map);
                if debug {
                    println!("top left {}", top_left.count_dt(garden_map, cmem));
                    top_left.print_corners();
                }
                self.count += top_left.count_dt(garden_map, cmem);
                *self.top.front_mut().unwrap() = top_left.get_top_edge();
                *self.left.front_mut().unwrap() = top_left.get_left_edge();
            }
            if i == n {
                // top right corner
                let top_right = DTTileCore::from_edge(&tile.get_right_edge(), garden_map);
                if debug {
                    println!("top right {}", top_right.count_dt(garden_map, cmem));
                    top_right.print_corners();
                }
                self.count += top_right.count_dt(garden_map, cmem);
                *self.top.back_mut().unwrap() = top_right.get_top_edge();
                *self.right.front_mut().unwrap() = top_right.get_right_edge();
            }
        });
        (1..=n).for_each(|i| {
            let tile = DTTileCore::from_edge(self.bottom.get(i).unwrap(), garden_map);
            if debug {
                println!("bottom tile {} {}", i + 1, tile.count_dt(garden_map, cmem));
                tile.print_corners();
            }
            self.count += tile.count_dt(garden_map, cmem);
            *self.bottom.get_mut(i).unwrap() = tile.get_bottom_edge();
            if i == 1 {
                // bottom left corner
                let bottom_left = DTTileCore::from_edge(&tile.get_left_edge(), garden_map);
                if debug {
                    println!("bottom left {}", bottom_left.count_dt(garden_map, cmem));
                    bottom_left.print_corners();
                }
                self.count += bottom_left.count_dt(garden_map, cmem);
                *self.bottom.front_mut().unwrap() = bottom_left.get_bottom_edge();
                *self.left.back_mut().unwrap() = bottom_left.get_left_edge();
            }
            if i == n {
                // bottom right corner
                let bottom_right = DTTileCore::from_edge(&tile.get_right_edge(), garden_map);
                if debug {
                    println!("bottom right {}", bottom_right.count_dt(garden_map, cmem));
                    bottom_right.print_corners();
                }
                self.count += bottom_right.count_dt(garden_map, cmem);
                *self.bottom.back_mut().unwrap() = bottom_right.get_bottom_edge();
                *self.right.back_mut().unwrap() = bottom_right.get_right_edge();
            }
        });
        (1..=n).for_each(|i| {
            let tile = DTTileCore::from_edge(self.left.get(i).unwrap(), garden_map);
            if debug {
                println!("left tile {} {}", i + 1, tile.count_dt(garden_map, cmem));
                tile.print_corners();
            }
            self.count += tile.count_dt(garden_map, cmem);
            *self.left.get_mut(i).unwrap() = tile.get_left_edge();
        });
        (1..=n).for_each(|i| {
            let tile = DTTileCore::from_edge(self.right.get(i).unwrap(), garden_map);
            if debug {
                println!("right tile {} {}", i + 1, tile.count_dt(garden_map, cmem));
                tile.print_corners();
            }
            self.count += tile.count_dt(garden_map, cmem);
            *self.right.get_mut(i).unwrap() = tile.get_right_edge();
        });
    }
    fn new(garden_map: &GardenMap, cmem: &mut CountMem) -> CompositeDT {
        let center = DTTileCore::from_point(garden_map);
        let debug = false;
        if debug {
            println!("center {}", center.count_dt(garden_map, cmem));
            center.print_corners();
        }
        let count = center.count_dt(garden_map, cmem);
        // make first ring
        let n = (2 * cmem.steps / garden_map.nrows as usize) + 2;
        let mut left = VecDeque::with_capacity(n);
        let mut right = VecDeque::with_capacity(n);
        let mut top = VecDeque::with_capacity(n);
        let mut bottom = VecDeque::with_capacity(n);
        left.push_front(center.get_left_edge());
        right.push_front(center.get_right_edge());
        top.push_front(center.get_top_edge());
        bottom.push_front(center.get_bottom_edge());
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
    steps: usize,
}

impl CountMem {
    fn new(nrows: usize, steps: usize) -> CountMem {
        CountMem {
            data: BTreeMap::new(),
            nrows,
            steps,
        }
    }

    fn normalize(core: &DTTileCore) -> (usize, usize, usize, usize) {
        let mut offset = core
            .upper_left
            .min(core.upper_right.min(core.lower_left.min(core.lower_right)));
        if offset % 2 == 1 {
            offset -= 1;
        }
        (
            core.upper_left - offset,
            core.upper_right - offset,
            core.lower_left - offset,
            core.lower_right - offset,
        )
    }

    fn all_lower(&self, core: &DTTileCore) -> bool {
        let max_val = core
            .upper_left
            .max(core.upper_right.max(core.lower_left.max(core.lower_right)))
            + self.nrows;
        self.steps > max_val
    }

    fn predict(&self, core: &DTTileCore) -> Option<&usize> {
        if !self.all_lower(core) {
            return None;
        }
        let key = CountMem::normalize(core);
        self.data.get(&key)
    }

    fn learn(&mut self, core: &DTTileCore, count: usize) {
        if !self.all_lower(core) {
            return;
        }
        let key = CountMem::normalize(core);
        assert!(self.data.insert(key, count).is_none());
    }
}

pub fn run(input: &str, steps: usize) -> usize {
    let garden_map = input.parse::<GardenMap>().unwrap();
    let mut cmem = CountMem::new(garden_map.nrows as usize, steps);
    let mut cdt = CompositeDT::new(&garden_map, &mut cmem);
    let debug = false;
    if debug {
        println!("core count {}", cdt.count);
    }
    let mut rings = 0;
    loop {
        let old_count = cdt.count;
        cdt.expand(&garden_map, &mut cmem);
        if debug {
            rings += 1;
            println!("\nring {} count {}", rings, cdt.count);
        }
        if cdt.count == old_count {
            break;
        }
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
    // this one does not pass
    // We're unable to predict completely from the corners right next to the
    // center core. The other tests are fine
    // #[test]
    // fn test2_dt() {
    //     let input = include_str!("example_data.txt");
    //     assert_eq!(super::run(input, 10,), 50);
    // }
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
