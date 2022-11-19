use std::{
    collections::HashMap,
    io::{self, BufRead},
};

/// https://adventofcode.com/2021/day/3

fn main() {
    let mut bit_count: Vec<HashMap<char, usize>> = Vec::new();

    io::stdin().lock().lines().for_each(|binary| {
        binary.unwrap().chars().enumerate().for_each(|(i, c)| {
            match bit_count.get_mut(i) {
                Some(map) => {
                    map.insert(c, map.get(&c).unwrap_or(&0) + 1);
                }
                None => {
                    bit_count.insert(i, HashMap::from([(c, 1)]));
                }
            };
        })
    });
    // dbg!(&bit_count);

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for map in bit_count {
        let max = map.values().max().unwrap();
        for (k, v) in &map {
            if v == max {
                gamma_rate += &k.to_string();
                continue;
            }
            epsilon_rate += &k.to_string();
        }
    }
    dbg!(&gamma_rate);
    dbg!(&epsilon_rate);

    let gamma_rate = usize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate, 2).unwrap();
    dbg!(&gamma_rate);
    dbg!(&epsilon_rate);

    println!("power consumption = {}", gamma_rate * epsilon_rate);
}
