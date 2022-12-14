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

fn _get_item_priority(item: char) -> usize {
    ALPHABETS
        .iter()
        .find_position(|&c| c.eq(&item))
        .expect("you deserve it")
        .0
        + 1
}

// ascii -_-
fn get_item_priority(item: char) -> u8 {
    if item.is_lowercase() {
        return item as u8 - b'a' + 1;
    }
    item as u8 - b'A' + 26 + 1
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
            .map(|item| get_item_priority(item) as usize)
            .sum::<usize>()
    );

    Ok(())
}
