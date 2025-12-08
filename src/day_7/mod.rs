use crate::utils::read_lines::read_lines;
use std::collections::HashMap;

fn part_1() {
  let mut input = read_lines("src/day_7/input.txt").unwrap();

  let mut splits: u32 = 0;

  for (line_index, line) in input.clone().into_iter().enumerate() {
    for (ch_index, ch) in line.chars().enumerate() {
      if ch_index == 0 || ch_index == line.len() {
        continue;
      }

      if ch == 'S' {
        input[line_index+1].replace_range(ch_index..=ch_index, "|");
        continue;
      }

      if line_index == 0 {
        continue;
      }

      if ch == '.' && input[line_index-1].chars().nth(ch_index).unwrap() == '|' {
        input[line_index].replace_range(ch_index..=ch_index, "|");
        continue;
      }

      if ch == '^' && input[line_index-1].chars().nth(ch_index).unwrap() == '|' {
        splits += 1;

        if !(input[line_index].chars().nth(ch_index-1).unwrap() == '|' && input[line_index-1].chars().nth(ch_index+1).unwrap() == '|') {
          input[line_index].replace_range(ch_index-1..=ch_index-1, "|");
          input[line_index].replace_range(ch_index+1..=ch_index+1, "|");
        }
      }
    }
  }

  println!("{}", splits);
}

fn part_2() {
  let mut input = read_lines("src/day_7/input.txt").unwrap();

  let mut timelines: usize = 1;
  let mut beam_powers: HashMap<(usize, usize), usize> = HashMap::new();

  for (line_index, line) in input.clone().into_iter().enumerate() {
    for (ch_index, ch) in line.chars().enumerate() {
      if ch_index == 0 || ch_index == line.len() {
        continue;
      }

      if ch == 'S' {
        input[line_index+1].replace_range(ch_index..=ch_index, "|");
        beam_powers.insert((line_index+1, ch_index), 1);
        continue;
      }

      if line_index == 0 {
        continue;
      }

      if input[line_index].chars().nth(ch_index).unwrap() == '.' && input[line_index-1].chars().nth(ch_index).unwrap() == '|' {
        input[line_index].replace_range(ch_index..=ch_index, "|");
        let previous_beam_power: usize = *beam_powers.get(&(line_index-1, ch_index)).unwrap_or(&1);
        beam_powers.insert((line_index, ch_index), previous_beam_power);
        continue;
      }

      if ch == '^' && input[line_index-1].chars().nth(ch_index).unwrap() == '|' {
        let current_beam_power: usize = *beam_powers.get(&(line_index-1, ch_index)).unwrap_or(&1);
        timelines += current_beam_power;

        if input[line_index].chars().nth(ch_index-1).unwrap() == '|' { 
          input[line_index].replace_range(ch_index-1..=ch_index-1, "|");
          let current_left_beam_power: usize = *beam_powers.get(&(line_index, ch_index-1)).unwrap_or(&1);
          beam_powers.insert((line_index, ch_index-1), current_beam_power+current_left_beam_power);
        } else {
          input[line_index].replace_range(ch_index-1..=ch_index-1, "|");
          beam_powers.insert((line_index, ch_index-1), current_beam_power); 
        }

        if input[line_index-1].chars().nth(ch_index+1).unwrap() == '|' {
          input[line_index].replace_range(ch_index+1..=ch_index+1, "|");
          let current_right_beam_power: usize = *beam_powers.get(&(line_index-1, ch_index+1)).unwrap_or(&1);
          beam_powers.insert((line_index, ch_index+1), current_beam_power+current_right_beam_power);
        } else {
          input[line_index].replace_range(ch_index+1..=ch_index+1, "|");
          beam_powers.insert((line_index, ch_index+1), current_beam_power); 
        }

      }
    }
  }

  println!("{}", timelines);
}

pub fn day_7() {
  part_1();
  part_2();
}