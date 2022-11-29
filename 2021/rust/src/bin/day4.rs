use std::io::{self, BufRead};

type BingoBoards<T> = Vec<Vec<(T, bool)>>;

fn parse_bingo_boards<T>(raw: &str) -> Vec<BingoBoards<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut board_idx = 0;
    raw.trim().lines().fold(vec![vec![]], |mut acc, line| {
        if !line.is_empty() {
            let item = line
                .trim()
                .split_whitespace()
                .map(|s| (s.parse::<T>().unwrap(), false))
                .collect::<Vec<_>>();
            if let Some(inner) = acc.get_mut(board_idx) {
                inner.push(item);
            } else {
                acc.push(vec![]);
                acc[board_idx].push(item);
            }
        } else {
            board_idx += 1;
        }
        acc
    })
}

fn transpose<T>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: std::clone::Clone,
{
    assert!(!arr.is_empty());
    (0..arr[0].len())
        .map(|i| arr.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn is_winner<T>(board: &BingoBoards<T>) -> bool
where
    T: std::clone::Clone,
{
    // cheking for horizontal win
    let horizontal_win = board
        .iter()
        .any(|row| row.iter().all(|marked| marked.1 == true));
    // cheking for vertical win
    let vertical_win = transpose(board)
        .iter()
        .any(|row| row.iter().all(|marked| marked.1 == true));
    horizontal_win || vertical_win
}

fn mark_board<T: std::cmp::PartialEq>(board: &mut BingoBoards<T>, draw: &T) {
    // maybe I shouldn't have used an iterator here...well...deal with it
    _ = board
        .iter_mut()
        .map(|inner| {
            _ = inner
                .iter_mut()
                .filter(|item| &item.0 == draw)
                .map(|item| item.1 = true)
                .collect::<()>();
        })
        .collect::<()>();
}

fn sum_unmarked<T: std::iter::Sum + Clone>(board: &BingoBoards<T>) -> T {
    board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|item| item.1 == false)
                .map(|item| item.0.clone())
                .sum::<T>()
        })
        .sum::<T>()
}

fn main() {
    let mut input = io::stdin().lock().lines().map(|s| s.unwrap());

    let draws;
    {
        let draws_raw = input.next().unwrap();
        draws = draws_raw
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
    }

    let mut bingo_boards;
    {
        let boards_raw = input.collect::<Vec<_>>().join("\n");
        bingo_boards = parse_bingo_boards::<i64>(&boards_raw);
    }

    // playing the game at last...squid bad
    let mut win_stats = None;
    'the_holy_loop: for draw in &draws {
        for (idx, board) in bingo_boards.iter_mut().enumerate() {
            mark_board(board, draw);
            if is_winner(board) {
                win_stats = Some((idx, draw));
                break 'the_holy_loop;
            }
        }
    }

    let sum = sum_unmarked(&bingo_boards[win_stats.unwrap().0]);
    println!(
        "board `{}` won with a total score of: {}",
        win_stats.unwrap().0 + 1,
        win_stats.unwrap().1 * sum
    );
}
