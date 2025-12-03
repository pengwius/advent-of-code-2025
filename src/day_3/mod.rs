use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(file_path: &str) -> std::io::Result<Vec<String>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let lines: Vec<String> = reader
    .lines()
    .map(|line| line.unwrap())
    .collect();

  Ok(lines)
}

fn part1() {
  let input = read_lines("src/day_3/input.txt").unwrap();

  let mut answer: u32 = 0;

  for line in &input {
    let tens_digit_index = line
      .chars()
      .take(line.len() - 1)
      .position(|c| c == line.chars().take(line.len() - 1).max().unwrap())
      .unwrap();

    let unity_digit = line
      .chars()
      .skip(tens_digit_index+1)
      .max()
      .unwrap() as u8 - '0' as u8;

    let final_number: u8 = line
      .chars()
      .nth(tens_digit_index)
      .unwrap()
      .to_digit(10)
      .unwrap() as u8 * 10
      + unity_digit;

    answer += final_number as u32;

  }

  println!("{}", answer);
}

fn part2() {
  let input = read_lines("src/day_3/input.txt").unwrap();

  let mut answer: u128 = 0;

  for line in &input {
    let mut number: String = String::new();
    let mut previous_index: usize = 0;

    for i in (0..12).rev() {
      let search_len = line.len() - i - previous_index;

      let highest_number = line
        .chars()
        .skip(previous_index)
        .take(search_len)
        .max()
        .unwrap();

      let highest_number_index = line
        .chars()
        .skip(previous_index)
        .take(search_len)
        .position(|c| c == highest_number)
        .unwrap();

      previous_index += highest_number_index + 1;

      number.push(highest_number);
    }

    answer += number.parse::<u128>().unwrap();
  }

  println!("{}", answer);
}

pub fn day_3() {
  part1();
  part2();
}