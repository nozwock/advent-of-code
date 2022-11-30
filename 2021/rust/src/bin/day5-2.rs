// copy pasting from day5.rs

// run this with
// cargo r --bin day5-2 < ../inputs/day5.in

use std::{
    fmt,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{} -> {},{}",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y
        )
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Line {
    fn to_points(&self) -> Vec<Point> {
        let points;
        let (xdiff, ydiff) = (self.p2.x - self.p1.x, self.p2.y - self.p1.y);
        if xdiff != 0 {
            let slope = ydiff / xdiff;
            // range over x (since this branch also deals with horizontal lines)
            let range = (self.p1.x.min(self.p2.x), self.p2.x.max(self.p1.x));
            points = (range.0..=range.1)
                .map(|x| Point::new(x, (slope * (x - self.p1.x)) + self.p1.y))
                .collect();
        } else {
            // Special case of undefined slope
            let range = (self.p1.y.min(self.p2.y), self.p2.y.max(self.p1.y));
            points = (range.0..=range.1)
                .map(|y| Point::new(self.p1.x, y))
                .collect();
        }
        points
    }
}

fn parse_line(input: &str) -> Line {
    let line: Vec<(isize, isize)> = input
        .split("->")
        .map(|s| {
            let (x, y) = s.trim().split_once(",").unwrap();
            (str::parse(x).unwrap(), str::parse(y).unwrap())
        })
        .collect();
    let p1 = *line.get(0).unwrap();
    let p1 = Point { x: p1.0, y: p1.1 };
    let p2 = *line.get(1).unwrap();
    let p2 = Point { x: p2.0, y: p2.1 };
    Line { p1, p2 }
}

// Horizontal or Vertical ;-)
fn is_horv(line: &Line) -> bool {
    line.p1.x == line.p2.x || line.p1.y == line.p2.y
}

fn max_coords(lines: &Vec<Line>) -> Option<(usize, usize)> {
    let max_x = lines
        .iter()
        .map(|line| line.p1.x.max(line.p2.x))
        .max_by_key(|x| *x)?;
    let max_y = lines
        .iter()
        .map(|line| line.p1.y.max(line.p2.y))
        .max_by_key(|y| *y)?;
    Some((max_x.abs() as usize, max_y.abs() as usize))
}

/// Panics if points are outside 2d array
fn mark_ocean_floor<T>(ocean_floor: &mut Vec<Vec<T>>, points: &Vec<Point>)
where
    T: std::ops::AddAssign<usize>,
{
    for point in points {
        // hmm this is like a bomb waiting to go out...
        ocean_floor[point.y.abs() as usize][point.x.abs() as usize] += 1_usize;
    }
}

fn count_marked_above<const N: usize>(ocean_floor: &Vec<Vec<usize>>) -> usize {
    ocean_floor
        .iter()
        .map(|inner| inner.iter().filter(|&t| *t > N).count())
        .sum()
}

fn stringify_ocean_floor<T>(ocean_floor: &Vec<Vec<T>>) -> String
where
    T: std::fmt::Display + std::cmp::PartialEq<usize>,
{
    ocean_floor
        .iter()
        .map(|inner| {
            inner
                .iter()
                .map(|n| {
                    if n == &0 {
                        ".".to_owned()
                    } else {
                        format!("{}", n)
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let lines;
    {
        let input = io::stdin()
            .lock()
            .lines()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>();
        lines = input.iter().map(|s| parse_line(&s)).collect::<Vec<_>>();
    }

    let max_plane_coords = max_coords(&lines).unwrap();
    let mut ocean_floor = vec![vec![0_usize; max_plane_coords.0 + 1]; max_plane_coords.1 + 1];

    for line in &lines {
        mark_ocean_floor(&mut ocean_floor, &line.to_points());
    }

    // println!("\n{}", stringify_ocean_floor(&ocean_floor)); // not great for large inputs
    dbg!(count_marked_above::<1>(&ocean_floor));
}
