use aoc::prelude::*;

fn main() {
    let input = get_stdin_input().unwrap();
    // Part 1
    let mut idx = 0;
    let mut elf_food = input.lines().fold(vec![], |mut acc, item| {
        if item.is_empty() {
            idx += 1;
        } else {
            let item = item.parse::<u32>().unwrap();
            if let Some(calorie) = acc.get_mut(idx) {
                *calorie += item;
            } else {
                acc.insert(idx, item);
            }
        }
        acc
    });
    println!("Part 1: {}", elf_food.iter().max().unwrap());

    // Part 2
    let mut sum_top3 = 0;
    for _ in 0..3 {
        let tmp = *elf_food.iter().max().unwrap();
        sum_top3 += tmp;
        elf_food.remove(elf_food.iter().find_position(|&&t| t == tmp).unwrap().0);
    }
    println!("Part 2: {}", sum_top3);
}
