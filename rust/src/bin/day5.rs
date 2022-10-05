#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_input(input: &str) -> Line {
    let line: Vec<(i32, i32)> = input
        .split("->")
        .map(|s| {
            let (x, y) = s.trim().split_once(",").unwrap();
            return (str::parse::<i32>(x).unwrap(), str::parse::<i32>(y).unwrap());
        })
        .collect();
    let (x1, y1) = line.get(0).unwrap();
    let (x2, y2) = line.get(1).unwrap();
    return Line {
        x1: *x1,
        y1: *y1,
        x2: *x2,
        y2: *y2,
    };
}

fn main() {
    for item in get_input().lines() {
        println!("{:?}", parse_input(item));
    }
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
