use std::{
    error::Error,
    io::{self, Read},
    str::FromStr,
};

/// Reads stdin and return String
pub fn get_stdin_input() -> anyhow::Result<String> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    Ok(buf)
}

/// Parse input items seperated by a delimeter
///
/// # Examples
/// ```
/// use aoc::prelude::*;
///
/// fn main() {
///     let input = get_stdin_input().unwrap();
///     let newline_sep: Vec<i32> = parse_separated_by(&input, '\n').unwrap();
///     let csv_type: Vec<i32> = parse_separated_by(&input, ',').unwrap();
/// }
/// ```
pub fn parse_separated_by<T>(raw: &str, delim: char) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Sync + Send + 'static,
{
    raw.trim()
        .split(delim)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

pub fn parse_lines<T>(line: &Vec<&str>) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Sync + Send + 'static,
{
    line.iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}
