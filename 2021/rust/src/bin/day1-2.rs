use std::io::{self, BufRead};

/// https://adventofcode.com/2021/day/1
// cargo r --bin day1 -- < inputs/day1-2.in

fn main() {
    let reports: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    // sum of measurements in the sliding windows
    let mut sum: Vec<usize> = Vec::new();

    'a: for (i, val) in reports.iter().enumerate() {
        let mut tmp = val.to_owned();
        for i in (i + 1)..=(i + 2) {
            if reports.get(i).is_none() {
                break 'a;
            }
            tmp += reports[i];
        }
        sum.push(tmp);
    }

    // println!("{:?}", sum);
    println!("{}", count_larger_than_prev(sum));
}

fn count_larger_than_prev<T: PartialOrd>(vec: Vec<T>) -> usize {
    let mut prev = None;
    let mut count = 0usize;
    for report in vec {
        if let Some(prev) = prev {
            if report > prev {
                count += 1;
            }
        }
        prev = Some(report);
    }
    count
}
