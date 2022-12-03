use aoc::prelude::*;
use once_cell::sync::Lazy;

static ALPHABETS: Lazy<Vec<char>> = Lazy::new(|| {
    let mut items = (b'A'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect_vec();
    items.rotate_left(26);
    items
});

fn get_item_priority(item: char) -> usize {
    ALPHABETS
        .iter()
        .find_position(|&c| c.eq(&item))
        .expect("you deserve it")
        .0
        + 1
}

fn main() -> Result<()> {
    let input = get_stdin_input()?;

    // Part 1
    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|racksack| racksack.split_at(racksack.len() / 2))
            .map(|(compart1, compart2)| compart1.chars().filter(|c| compart2.contains(*c)).dedup())
            .fold(vec![], |mut accum, wrongs| {
                accum.extend(wrongs);
                accum
            })
            .into_iter()
            .map(get_item_priority)
            .sum::<usize>()
    );

    Ok(())
}
