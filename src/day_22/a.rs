use std::{collections::BTreeSet, fmt::Display, str::FromStr};

use ndarray::{s, Array3, Dim, SliceInfo, SliceInfoElem};

struct Brick {
    x: [u16; 2],
    y: [u16; 2],
    z: [u16; 2],
    id: u16,
}
impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.z[0] == other.z[0]
    }
}
impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Brick {}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z[0].cmp(&other.z[0])
    }
}
impl Brick {
    fn new(mut x: [u16; 2], mut y: [u16; 2], mut z: [u16; 2], id: u16) -> Brick {
        if x[0] > x[1] {
            x.swap(0, 1);
        }
        if y[0] > y[1] {
            y.swap(0, 1);
        }
        if z[0] > z[1] {
            z.swap(0, 1);
        }
        Brick { x, y, z, id }
    }
    fn slice(&self) -> SliceInfo<[SliceInfoElem; 3], Dim<[usize; 3]>, Dim<[usize; 3]>> {
        s![
            self.x[0] as usize..self.x[1] as usize,
            self.y[0] as usize..self.y[1] as usize,
            self.z[0] as usize..self.z[1] as usize
        ]
    }
    /// Return new brick with lower z value
    fn drop(&self, amount: u16) -> Result<Brick, ()> {
        if amount > self.z[0] {
            Err(())
        } else {
            let x = self.x;
            let y = self.y;
            let z = [self.z[0] - amount, self.z[1] - amount];
            Ok(Brick::new(x, y, z, self.id))
        }
    }
    /// Return new brick with lower z value and a height of 1
    fn drop_bottom_rim(&self, amount: u16) -> Result<Brick, ()> {
        if amount > self.z[0] {
            Err(())
        } else {
            let x = self.x;
            let y = self.y;
            let z = [self.z[0] - amount, self.z[0] - amount + 1];
            Ok(Brick::new(x, y, z, self.id))
        }
    }
    /// Return new brick with lower z value that does not overlap
    fn drop_bottom(&self, amount: u16) -> Result<Brick, ()> {
        if amount > self.z[0] {
            Err(())
        } else {
            let height = (self.z[1] - self.z[0]).min(amount);
            let x = self.x;
            let y = self.y;
            let z = [self.z[0] - amount, self.z[0] - amount + height];
            Ok(Brick::new(x, y, z, self.id))
        }
    }
    /// Return new brick that is remainder of self minus drop bottom
    fn drop_top(&self, amount: u16) -> Result<Brick, ()> {
        if amount > self.z[0] {
            Err(())
        } else {
            let brick_height = self.z[1] - self.z[0];
            let height = brick_height.min(amount);
            let x = self.x;
            let y = self.y;
            let z = [self.z[1] - height, self.z[1]];
            Ok(Brick::new(x, y, z, self.id))
        }
    }
}
/// Parse something like 1,0,1 => x,y,z
fn parse_xyz(s: &str) -> Vec<u16> {
    s.split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>()
}

impl FromStr for Brick {
    type Err = &'static str;
    /// Parse something like 1,0,1~1,2,1 => x,y,z~x,y,z
    /// start_xyz~end_xyz (non-slice)
    /// Convert from non-slice to slice coords
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_end = s.split("~").map(|x| parse_xyz(x)).collect::<Vec<_>>();
        if start_end.len() != 2 {
            Err("Wrong number of parts")
        } else {
            if start_end.iter().any(|x| x.len() != 3) {
                Err("Wrong number of components")
            } else {
                let start = &start_end[0];
                let end = &start_end[1];
                Ok(Brick::new(
                    [start[0], end[0] + 1],
                    [start[1], end[1] + 1],
                    [start[2], end[2] + 1],
                    0,
                ))
            }
        }
    }
}

impl Display for Brick {
    /// Print brick back out to match initial non-slice format
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x[0],
            self.y[0],
            self.z[0],
            self.x[1] - 1,
            self.y[1] - 1,
            self.z[1] - 1
        )
    }
}

struct Bricks {
    bricks: Vec<Brick>,
    has_brick: Array3<u16>,
}
impl FromStr for Bricks {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks = s
            .lines()
            .map(|line| line.parse::<Brick>().unwrap())
            .collect::<Vec<_>>();
        Ok(Bricks::new(bricks))
    }
}
impl Display for Bricks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bricks.iter().for_each(|brick| {
            writeln!(f, "{}", brick).unwrap();
        });
        Ok(())
    }
}

impl Bricks {
    fn new(mut bricks: Vec<Brick>) -> Bricks {
        bricks.sort();
        bricks.iter_mut().enumerate().for_each(|(i, b)| {
            b.id = u16::try_from(i + 1).unwrap();
        });
        let x_min = bricks.iter().map(|b| b.x[0]).min().unwrap();
        let y_min = bricks.iter().map(|b| b.y[0]).min().unwrap();
        let x_max = bricks.iter().map(|b| b.x[1]).max().unwrap();
        let y_max = bricks.iter().map(|b| b.y[1]).max().unwrap();
        let z_max = bricks.iter().map(|b| b.z[1]).max().unwrap();
        let nx = x_max - x_min;
        let ny = y_max - y_min;
        let nz = z_max;
        let mut has_brick = Array3::from_elem([nx as usize, ny as usize, nz as usize], 0);
        bricks.iter().for_each(|b| {
            has_brick.slice_mut(b.slice()).map_inplace(|x| {
                *x = b.id;
            })
        });
        Bricks { bricks, has_brick }
    }
    /// Try to move a brick down from gravity. Return if something changed
    fn settle_bricks(&mut self) -> bool {
        let mut changed = false;
        for i in 0..self.bricks.len() {
            let amount = self.max_settle(i);
            if amount > 0 {
                self.perform_settle(i, amount);
                changed = true;
            }
        }
        changed
    }
    /// Determin if brick at i can settle
    fn can_settle(&self, i: usize, amount: u16) -> bool {
        if let Ok(dropped) = self.bricks[i].drop_bottom_rim(amount) {
            self.has_brick
                .slice(dropped.slice())
                .iter()
                .all(|x| *x == 0)
        } else {
            false
        }
    }
    /// Determine max amount a brick can settle at position i
    fn max_settle(&self, i: usize) -> u16 {
        let mut max_settle = 0;
        while self.can_settle(i, max_settle + 1) {
            max_settle += 1;
        }
        max_settle
    }
    /// Settle brick at i
    fn perform_settle(&mut self, i: usize, amount: u16) {
        self.has_brick
            .slice_mut(self.bricks[i].drop_top(amount).unwrap().slice())
            .map_inplace(|x| {
                *x = 0;
            });
        self.has_brick
            .slice_mut(self.bricks[i].drop_bottom(amount).unwrap().slice())
            .map_inplace(|x| {
                *x = self.bricks[i].id;
            });
        *self.bricks.get_mut(i).unwrap() = self.bricks[i].drop(amount).unwrap();
        self.bricks.sort();
    }
    /// Keep settling bricks untill no more can settle
    fn settle_all(&mut self) {
        loop {
            let changed = self.settle_bricks();
            if !changed {
                break;
            }
        }
    }
    /// Return number of bricks that, if removed, would not destabilize the stack
    fn count_non_esential(&self) -> u16 {
        let mut esential = BTreeSet::new();
        for b in &self.bricks {
            if let Ok(drop_rim) = b.drop_bottom_rim(1) {
                let lower_ids = self
                    .has_brick
                    .slice(drop_rim.slice())
                    .into_iter()
                    .filter(|x| **x > 0)
                    .collect::<BTreeSet<_>>();
                if lower_ids.len() == 1 {
                    esential.insert(lower_ids);
                }
            }
        }
        u16::try_from(self.bricks.len() - esential.len()).unwrap()
    }
}

pub fn run(input: &str) -> usize {
    let mut bricks = input.parse::<Bricks>().unwrap();
    bricks.settle_all();
    bricks.count_non_esential() as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 5);
    }
}
