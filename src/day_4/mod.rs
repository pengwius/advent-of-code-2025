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
  let input = read_lines("src/day_4/input.txt").unwrap();

  let mut accesible_rolls_of_paper: usize = 0;

  for (y_index, line) in input.iter().enumerate() {
    for (x_index, ch) in line.chars().enumerate() {
      if ch != '@' {
        continue;
      }

      let mut rolls_of_paper_in_adjacent: u8 = 0;

      for y in -1isize..2 {
        let row: String = input.get(((y_index as isize)+y) as usize).unwrap_or(&"".to_string()).to_string();

        for x in -1isize..2 {
          if (x_index as isize)+x < 0 || (y_index as isize)+y < 0 || (x == 0 && y == 0) {
            continue;
          }

          if row.chars().nth(((x_index as isize)+x) as usize).unwrap_or('.') == '@' {
            rolls_of_paper_in_adjacent += 1;
          }          
        }
      }

      if rolls_of_paper_in_adjacent < 4 {
        accesible_rolls_of_paper += 1;
      }
    }
  }

  println!("{}", accesible_rolls_of_paper);
}

fn part_2() {
  let mut input = read_lines("src/day_4/input.txt").unwrap();

  let mut removed_rolls_of_paper: usize = 0;

  loop {
    let mut any_removed = false;
    for (y_index, line) in input.clone().iter().enumerate() {
      for (x_index, ch) in line.chars().enumerate() {
        if ch != '@' {
          continue;
        }

        let mut rolls_of_paper_in_adjacent: u8 = 0;

        for y in -1isize..2 {
          let row: &String = &input.get(((y_index as isize)+y) as usize).unwrap_or(&"".to_string()).to_string();

          for x in -1isize..2 {
            if (x_index as isize)+x < 0 || (y_index as isize)+y < 0 || (x == 0 && y == 0) {
              continue;
            }

            if row.chars().nth(((x_index as isize)+x) as usize).unwrap_or('.') == '@' {
              rolls_of_paper_in_adjacent += 1;
            }          
          }
        }

        if rolls_of_paper_in_adjacent < 4 {
          removed_rolls_of_paper += 1;
          input[y_index].replace_range(x_index..=x_index, ".");
          any_removed = true;
        }
      }
    }

    if !any_removed {
      break;
    }
  }

  println!("{}", removed_rolls_of_paper);
}

pub fn day_4() {
  part_1();
  part_2();
}