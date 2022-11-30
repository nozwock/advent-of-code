use aoc::prelude::*;

fn pass_time_by_day(lanternfish_count: &mut Vec<usize>) {
    lanternfish_count.rotate_left(1);
    // you can think of it as like,
    // there's an extra 6 for every 0 which turns into an 8 (via left rotate)
    lanternfish_count[6] += lanternfish_count[8];
}

fn main() {
    let input = get_stdin_input().unwrap();
    let lanternfish = input
        .trim()
        .split(',')
        .map(|timer| timer.trim().parse().unwrap())
        .collect::<Vec<u8>>();

    // each index represent a lantern fish's timer state; and the value is the number of fish with that state
    let mut lanternfish_count = vec![0_usize; 9];

    for timer in lanternfish {
        lanternfish_count[timer as usize] += 1; // populating
    }

    for _ in 0..256 {
        pass_time_by_day(&mut lanternfish_count);
    }

    dbg!(lanternfish_count.into_iter().sum::<usize>());
}
