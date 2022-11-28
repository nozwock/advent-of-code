use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
};

/// https://adventofcode.com/2021/day/3

fn bit_count_at(binaries: &Vec<&String>, idx: usize) -> Option<HashMap<char, usize>> {
    let mut count = HashMap::new();
    for binary in binaries {
        count
            .entry(binary.chars().nth(idx)?)
            .and_modify(|count| *count += 1)
            .or_insert(1usize);
    }
    Some(count)
}

fn main() {
    let input = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap())
        .collect::<Vec<_>>();

    let mut o2_rating_max_prev: Option<char> = None;
    let mut o2_rating_filtered_binaries: Vec<&String> = Vec::new();
    let mut o2_rating = String::new();

    let mut co2_rating_min_prev: Option<char> = None;
    let mut co2_rating_filtered_binaries: Vec<&String> = Vec::new();
    let mut co2_rating = String::new();

    for idx in 0..input[0].len() {
        match o2_rating_max_prev {
            Some(prev_bit) => {
                o2_rating_filtered_binaries = o2_rating_filtered_binaries
                    .into_iter()
                    .filter(|binary| binary.chars().nth(idx - 1).unwrap() == prev_bit)
                    .collect();
            }
            None => {
                o2_rating_filtered_binaries = input.iter().collect();
            }
        }

        let count = bit_count_at(&o2_rating_filtered_binaries, idx).unwrap();
        // dbg!(&count);
        // Argh!!! I know this is bad....
        let mut count_equal_flag = false;
        let mut max_bit = *count
            .iter()
            .max_by(|a, b| {
                let cmp = a.1.cmp(b.1);
                if let Ordering::Equal = cmp {
                    count_equal_flag = true;
                }
                cmp
            })
            .map(|(k, _v)| k)
            .unwrap();
        if count_equal_flag {
            max_bit = '1';
        }
        o2_rating.push(max_bit);
        o2_rating_max_prev = Some(max_bit);

        match co2_rating_min_prev {
            Some(prev_bit) => {
                co2_rating_filtered_binaries = co2_rating_filtered_binaries
                    .into_iter()
                    .filter(|binary| binary.chars().nth(idx - 1).unwrap() == prev_bit)
                    .collect();
            }
            None => {
                co2_rating_filtered_binaries = input.iter().collect();
            }
        }

        let count = bit_count_at(&co2_rating_filtered_binaries, idx).unwrap();
        // dbg!(&count);
        // Argh!!! I know this is bad....part 2...
        let mut count_equal_flag = false;
        let mut min_bit = *count
            .iter()
            .min_by(|a, b| {
                let cmp = a.1.cmp(b.1);
                if let Ordering::Equal = cmp {
                    count_equal_flag = true;
                }
                cmp
            })
            .map(|(k, _v)| k)
            .unwrap();
        if count_equal_flag {
            min_bit = '0';
        }
        co2_rating.push(min_bit);
        co2_rating_min_prev = Some(min_bit);
    }
    dbg!(&o2_rating);
    dbg!(&co2_rating);

    let o2_rating = usize::from_str_radix(&o2_rating, 2).unwrap();
    let co2_rating = usize::from_str_radix(&co2_rating, 2).unwrap();
    dbg!(o2_rating);
    dbg!(co2_rating);

    let life_support_rating = o2_rating * co2_rating;
    dbg!(life_support_rating);
}
