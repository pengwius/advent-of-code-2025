use crate::utils::read_lines::read_lines;

fn part_1() {
  let input = read_lines("src/day_2/input.txt").unwrap();

  let mut ranges_str: Vec<&str> = Vec::new();

  for line in &input {
    let range: Vec<&str> = line
      .split(",")
      .collect::<Vec<&str>>();

    for x in range {
      if x != "" {
        ranges_str.push(x)
      }
    }
  }

  let mut answer: usize = 0;

  for range in ranges_str {
    let start_str: &str = range
      .split("-")
      .collect::<Vec<&str>>()
      .get(0)
      .unwrap();

    let end_str: &str = range
      .split("-")
      .collect::<Vec<&str>>()
      .get(1)
      .unwrap();

    let start: usize = start_str.parse::<usize>().unwrap();
    let end: usize = end_str.parse::<usize>().unwrap();

    for x in start..=end {
      let x_string: String = x.to_string();

      let x_first_part: &str = &x_string[0..(x_string.len()/2)];

      let x_second_part: &str = &x_string[(x_string.len()/2)..x_string.len()];

      if x_first_part == x_second_part {
        answer += x;
      }
    }
  }

  println!("{}", answer);
}

fn part_2() {
  let input = read_lines("src/day_2/input.txt").unwrap();

  let mut ranges_str: Vec<&str> = Vec::new();

  for line in &input {
    let range: Vec<&str> = line
      .split(",")
      .collect::<Vec<&str>>();

    for x in range {
      if x != "" {
        ranges_str.push(x)
      }
    }
  }

  let mut answer: usize = 0;

  for range in ranges_str {
    let start_str: &str = range
      .split("-")
      .collect::<Vec<&str>>()
      .get(0)
      .unwrap();

    let end_str: &str = range
      .split("-")
      .collect::<Vec<&str>>()
      .get(1)
      .unwrap();

    let start: usize = start_str.parse::<usize>().unwrap();
    let end: usize = end_str.parse::<usize>().unwrap();

    for id in start..=end {
      let id_string: String = id.to_string();

      for x in 0..=(id_string.len()/2) {
        let segment_len: usize = id_string.len()/(x+1);
        let segments_number = id_string.len()/segment_len;

        let mut segments: Vec<String> = Vec::new();

        if id_string.len() % segments_number != 0 {
          continue;
        }

        for segment_index in 0..segments_number {
          let segment = &id_string[segment_index*segment_len..segment_index*segment_len+segment_len];
          segments.push(segment.to_string());
        }

        let mut is_invalid: bool = false;

        for segment_index in 0..segments.len()-1 {
          if segments.get(segment_index).unwrap() != segments.get(segment_index+1).unwrap() {
            is_invalid = false;
            break;
          }
          is_invalid = true;
        }

        if is_invalid {
          answer += id;
          break;
        }
      }
    }
  }

  println!("{}", answer); 
}

pub fn day_2() {
  part_1();
  part_2();
}