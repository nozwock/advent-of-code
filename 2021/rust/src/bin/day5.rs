// this code is tra...amazing

use std::{
    fmt,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
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
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Line {
    /// For horv lines only!
    ///
    /// **PANIC:**
    /// If not a horv line.
    fn to_points(&self) -> Vec<Point> {
        // omg wth is this code
        assert!(is_horv(self));
        let xdiff = self.p2.x as isize - self.p1.x as isize;
        let mut is_x_common = false;
        let low_high;
        if xdiff == 0 {
            is_x_common = true;
            low_high = (self.p1.y.min(self.p2.y), self.p2.y.max(self.p1.y));
        } else {
            low_high = (self.p1.x.min(self.p2.x), self.p2.x.max(self.p1.x));
        }
        (low_high.0..=low_high.1)
            .map(|i| {
                if is_x_common {
                    Point::new(self.p1.x, i)
                } else {
                    Point::new(i, self.p1.y)
                }
            })
            .collect()
    }
}

fn parse_line(input: &str) -> Line {
    let line: Vec<(usize, usize)> = input
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
    Some((max_x as usize, max_y as usize))
}

/// Panics if points are outside 2d array
fn mark_ocean_floor<T>(ocean_floor: &mut Vec<Vec<T>>, points: &Vec<Point>)
where
    T: std::ops::AddAssign<usize>,
{
    for point in points {
        ocean_floor[point.y][point.x] += 1_usize;
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
    let horv_lines;
    {
        let input = io::stdin().lock().lines().map(|s| s.unwrap());
        horv_lines = input
            .map(|s| parse_line(&s))
            .filter(is_horv)
            .collect::<Vec<_>>();
    }

    let max_plane_coords = max_coords(&horv_lines).unwrap();
    let mut ocean_floor = vec![vec![0_usize; max_plane_coords.0 + 1]; max_plane_coords.1 + 1];

    // println!("horv lines:"); // not great for large inputs
    // for line in &horv_lines {
    //     println!("{}", line);
    // }

    for line in &horv_lines {
        mark_ocean_floor(&mut ocean_floor, &line.to_points());
    }

    // println!("\n{}", stringify_ocean_floor(&ocean_floor)); // not great for large inputs
    dbg!(count_marked_above::<1>(&ocean_floor));
}
