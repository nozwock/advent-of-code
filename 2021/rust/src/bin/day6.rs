use aoc::day6::*;
use aoc::prelude::*;

fn main() {
    let input = get_stdin_input().unwrap();
    let mut lantern_fish = input
        .trim()
        .split(',')
        .map(|timer| LanternFish::new(timer.trim().parse().unwrap()))
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let mut new_fish_buf = vec![];
        for fish in lantern_fish.iter_mut() {
            if let Some(new_fish) = fish.pass_time_by_day() {
                new_fish_buf.push(new_fish);
            }
        }
        lantern_fish.extend(new_fish_buf.into_iter());
    }

    dbg!(lantern_fish.len());
}
