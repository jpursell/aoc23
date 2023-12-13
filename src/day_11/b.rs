use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }
}

#[derive(Debug)]
struct Universe {
    points: Vec<Point>,
}

impl Universe {
    fn new(points: Vec<Point>) -> Universe {
        Universe { points }
    }
}

impl FromStr for Universe {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut points = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    points.push(Point::new(row, col));
                }
            }
        }
        Ok(Universe::new(points))
    }
}

fn compute_distance(point0: &Point, point1: &Point) -> u64 {
    (point0.row.abs_diff(point1.row) + point0.col.abs_diff(point1.col)) as u64
}

impl Universe {
    fn find_empty_rows(&self) -> Vec<usize> {
        let rows = self.points.iter().map(|p| p.row).collect::<BTreeSet<_>>();
        let mut empty_rows = (0..*rows.iter().max().unwrap())
            .filter(|row| !rows.contains(row))
            .collect::<Vec<_>>();
        empty_rows.sort();
        empty_rows
    }

    fn find_empty_cols(&self) -> Vec<usize> {
        let cols = self.points.iter().map(|p| p.col).collect::<BTreeSet<_>>();
        let mut empty_cols = (0..*cols.iter().max().unwrap())
            .filter(|col| !cols.contains(col))
            .collect::<Vec<_>>();
        empty_cols.sort();
        empty_cols
    }

    fn expand_points(&mut self, factor: u64) {
        for row in self.find_empty_rows().iter().rev() {
            for point in self.points.iter_mut() {
                if point.row > *row {
                    point.row += (factor - 1) as usize;
                }
            }
        }
        for col in self.find_empty_cols().iter().rev() {
            for point in self.points.iter_mut() {
                if point.col > *col {
                    point.col += (factor - 1) as usize;
                }
            }
        }
    }

    fn calc_distances_sum(&mut self, factor: u64) -> u64 {
        self.expand_points(factor);
        let mut distance = 0;
        for (ipoint, point0) in self.points.iter().enumerate() {
            for jpoint in ipoint + 1..self.points.len() {
                let point1 = &self.points[jpoint];
                distance += compute_distance(point0, point1);
            }
        }
        distance
    }
}

pub fn run(input: &str, factor: u64) -> u64 {
    input.parse::<Universe>().unwrap().calc_distances_sum(factor)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10), 1030);
    }
    #[test]
    fn test2() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 100), 8410);
    }
}
