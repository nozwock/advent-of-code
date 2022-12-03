use aoc::prelude::*;

#[derive(Debug, Clone)]
struct Game {
    opn: Hand,
    own: Hand,
}

// I know, could've used modulo with idx's for this problem

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn wins_against(self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl Game {
    fn play(&self) -> u8 {
        match self.opn {
            Hand::Rock => match self.own {
                Hand::Rock => 1 + 3,
                Hand::Paper => 2 + 6,
                Hand::Scissors => 3 + 0,
            },
            Hand::Paper => match self.own {
                Hand::Rock => 1 + 0,
                Hand::Paper => 2 + 3,
                Hand::Scissors => 3 + 6,
            },
            Hand::Scissors => match self.own {
                Hand::Rock => 1 + 6,
                Hand::Paper => 2 + 0,
                Hand::Scissors => 3 + 3,
            },
        }
    }
}

fn main() -> Result<()> {
    let input = get_stdin_input()?;
    // Part 1
    let rounds = input
        .clone()
        .lines()
        .map(|s| {
            s.trim().split_once(' ').map(|(c1, c2)| {
                let opn = match c1 {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissors,
                    _ => unreachable!(),
                };
                let own = match c2 {
                    "X" => Hand::Rock,
                    "Y" => Hand::Paper,
                    "Z" => Hand::Scissors,
                    _ => unreachable!(),
                };
                Game { opn, own }
            })
        })
        .collect::<Option<Vec<_>>>()
        .context("splitting issue")?;

    dbg!(rounds
        .into_iter()
        .map(|round| round.play() as u32)
        .sum::<u32>());

    // Part 2
    let rounds = input
        .lines()
        .map(|s| {
            s.trim().split_once(' ').map(|(c1, c2)| {
                let opn = match c1 {
                    "A" => Hand::Rock,
                    "B" => Hand::Paper,
                    "C" => Hand::Scissors,
                    _ => unreachable!(),
                };
                let own = match c2 {
                    "X" => match opn {
                        Hand::Rock => Hand::Scissors,
                        Hand::Paper => Hand::Rock,
                        Hand::Scissors => Hand::Paper,
                    },
                    "Y" => opn,
                    "Z" => match opn {
                        Hand::Rock => Hand::Paper,
                        Hand::Paper => Hand::Scissors,
                        Hand::Scissors => Hand::Rock,
                    },
                    _ => unreachable!(),
                };
                Game { opn, own }
            })
        })
        .collect::<Option<Vec<_>>>()
        .context("splitting issue")?;

    dbg!(rounds
        .into_iter()
        .map(|round| round.play() as u32)
        .sum::<u32>());

    Ok(())
}
