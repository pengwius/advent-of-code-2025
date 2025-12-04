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

fn part_1() {
  let input = read_lines("src/day_1/input.txt").unwrap();

  let mut current_position: i16 = 50;
  let mut pointing_at_0: i16 = 0;

  for line in &input {
    let direction: char = line.chars().nth(0).unwrap();
    let distance: u16 = line[1..].parse::<u16>().unwrap();

    match direction {
      'R' => current_position += distance as i16,
      'L' => current_position -= distance as i16,
      _ => panic!("Unknown direction"),
    }

    if !(0..99).contains(&current_position) {
      current_position = current_position.rem_euclid(100);
    }

    if current_position == 0 {
      pointing_at_0 += 1;
    }
  }

  println!("{}", pointing_at_0);
}

fn part_2() {
  let input = read_lines("src/day_1/input.txt").unwrap();

  let mut current_position: i32 = 50;
  let mut clicks_at_0: i32 = 0;

  for line in &input {
    let direction: char = line.chars().nth(0).unwrap();
    let distance: i32 = line[1..].parse::<i32>().unwrap();

    match direction {
      'R' => {
        let start = current_position;
        let end = current_position + distance;

        let count = end / 100 - start / 100;
        clicks_at_0 += count;
        current_position = end.rem_euclid(100);
      },
      'L' => {
        let start = current_position;
        let end = current_position - distance;

        let count = (start - 1).div_euclid(100) - (end - 1).div_euclid(100);
        clicks_at_0 += count;
        current_position = end.rem_euclid(100);
      },
      _ => panic!("Unknown direction"),
    }
  }

  println!("{}", clicks_at_0);
}

pub fn day_1() {
  part_1();
  part_2();
}