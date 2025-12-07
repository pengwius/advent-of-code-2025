use crate::utils::read_lines::read_lines;

fn part_1() {
  let input = read_lines("src/day_6/input.txt").unwrap();

  let mut splitted_lines: Vec<Vec<&str>> = Vec::new();

  for line in &input {
    let mut splitted_line: Vec<&str> = line
      .split(" ")
      .collect::<Vec<&str>>();
    
    splitted_line.retain(|x| !x.is_empty());

    splitted_lines.push(splitted_line);
  }

  let mut answer: usize = 0;

  for row in 0..splitted_lines.get(0).unwrap().len() {
    let mut numbers: Vec<usize> = Vec::new();

    for column in 0..splitted_lines.len()-1 {
      numbers.push(splitted_lines[column][row].parse::<usize>().unwrap());
    }

    match *splitted_lines.last().unwrap().get(row).unwrap() {
      "+" => answer += numbers.iter().sum::<usize>(),
      "*" => {
        let mut multiplied_value: usize = 1;

        for number in &numbers {
          multiplied_value *= *number as usize;          
        }

        answer += multiplied_value;
      },
      _ => panic!("Unknown operation"),
    }
  }

  println!("{}", answer);
}

fn part_2() {
  let mut input = read_lines("src/day_6/input.txt").unwrap();

  let mut spaces_positions: Vec<usize> = Vec::new();

  for (char_position, char) in input.get(0).unwrap().chars().enumerate() {
    if char != ' ' {
      continue;
    }

    let mut not_space_found: bool = false;

    for x in &input {
      if x.chars().nth(char_position).unwrap() != ' ' {
        not_space_found = true;
        break;
      }
    }

    if !not_space_found {
      spaces_positions.push(char_position);
    }
  }

  let mut splitted_lines: Vec<Vec<&str>> = Vec::new();

  for mut line in &mut input {
    for space in &spaces_positions {
      line.replace_range(space..=space, ",");
    }

    let mut splitted_line: Vec<&str> = line
      .split(",")
      .collect::<Vec<&str>>();
    
    splitted_lines.push(splitted_line);
  }

  let mut answer: usize = 0;
  
  let mut column_counter: usize = 0;
  for outside_row in 0..splitted_lines.get(0).unwrap().len() {
    let mut problem_answer: usize = 0;

    if splitted_lines.last().unwrap().get(outside_row).unwrap().replace(" ", "").as_str() == "*" {
      problem_answer = 1;
    }

    for inside_row in 0..splitted_lines.get(0).unwrap().get(column_counter).unwrap().len() {
      let mut number: String = "".to_string();
      for column in 0..splitted_lines.len()-1 {
        if splitted_lines[column][outside_row].chars().nth(inside_row).unwrap() != ' ' {
          number += &splitted_lines[column][outside_row].chars().nth(inside_row).unwrap().to_string();
        }
      }

      match splitted_lines.last().unwrap().get(outside_row).unwrap().replace(" ", "").as_str() {
        "+" => problem_answer += number.parse::<usize>().unwrap(),
        "*" => problem_answer *= number.parse::<usize>().unwrap(),
        _ => panic!("Unknown operation: {:?}", *splitted_lines.last().unwrap().get(outside_row).unwrap()),
      }

    }

    column_counter += 1;
    answer += problem_answer;
  }

  println!("{}", answer);
}

pub fn day_6() {
  part_1();
  part_2();
}