use std::{
    collections::HashMap,
    io::{self, BufRead},
};

/// https://adventofcode.com/2021/day/3

fn main() {
    let mut bit_count: Vec<HashMap<char, usize>> = Vec::new();

    io::stdin().lock().lines().for_each(|binary| {
        binary.unwrap().chars().enumerate().for_each(|(i, c)| {
            let map = match bit_count.get_mut(i) {
                Some(map) => map,
                None => {
                    bit_count.insert(i, HashMap::new());
                    bit_count.get_mut(i).unwrap()
                }
            };
            map.insert(c, map.get(&c).unwrap_or(&0) + 1);
        })
    });
    dbg!(&bit_count);

    let gamma_rate = bit_count
        .iter()
        .map(|m| {
            let max = m.values().max().unwrap();
            let mut gamma: char = Default::default();
            for (k, val) in m {
                if val == max {
                    gamma = k.clone();
                    break;
                }
            }
            return gamma;
        })
        .fold(String::new(), |acc: String, c| format!("{acc}{c}"));
    dbg!(&gamma_rate);

    let epsilon_rate = gamma_rate
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .fold(String::new(), |acc: String, c| format!("{acc}{c}"));
    dbg!(&epsilon_rate);

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("power consumption: {}", gamma_rate * epsilon_rate);
}
