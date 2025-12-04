use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(file_path: &str) -> std::io::Result<Vec<String>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader
    .lines()
    .map(|line| line.unwrap())
    .collect();

  Ok(lines)
}