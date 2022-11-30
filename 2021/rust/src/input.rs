use std::io::{self, Read};

use anyhow::Result;

pub fn get_stdin_input() -> Result<String> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    Ok(buf)
}
