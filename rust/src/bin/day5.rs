#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn parse_input(input: &str) -> Line {
    let line: Vec<(i32, i32)> = input
        .split("->")
        .map(|s| {
            let (x, y) = s.trim().split_once(",").unwrap();
            return (str::parse::<i32>(x).unwrap(), str::parse::<i32>(y).unwrap());
        })
        .collect();
    let p1 = *line.get(0).unwrap();
    let p1 = Point { x: p1.0, y: p1.1 };
    let p2 = *line.get(1).unwrap();
    let p2 = Point { x: p2.0, y: p2.1 };
    return Line { p1, p2 };
}

fn is_horv(line: &Line) -> bool {
    return line.p1.x == line.p2.x || line.p1.y == line.p2.y;
}

fn main() {
    let result: Vec<Line> = get_input()
        .lines()
        .map(parse_input)
        .filter(is_horv)
        .collect();

    println!("{:?}", result);
}

fn get_input() -> String {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
        .to_string();
}
