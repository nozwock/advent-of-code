use std::io::{self, BufRead};

/// https://adventofcode.com/2021/day/1

// cargo r --bin day1 -- < inputs/day1.in

fn main() {
    let mut prev = None;
    let mut count = 0;
    for report in io::stdin().lock().lines() {
        let tmp: i32 = report.unwrap().parse().unwrap();
        if let Some(prev) = prev {
            if tmp > prev {
                count += 1;
            }
        }
        prev = Some(tmp);
    }
    println!("{}", count);
}
