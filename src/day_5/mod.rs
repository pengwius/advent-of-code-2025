use crate::utils::read_lines::read_lines;

fn part_1() {
  let input = read_lines("src/day_5/input.txt").unwrap();

  let mut fresh_ranges: Vec<Vec<usize>> = Vec::new();
  let mut ingredients: Vec<usize> = Vec::new();

  let mut is_after_blank_line: bool = false;
  let mut available_ingredients: isize = 0;

  for line in &input {
    if line.is_empty() {
      is_after_blank_line = true;
      continue;
    }

    if is_after_blank_line {
      ingredients.push(
        line
          .parse::<isize>()
          .unwrap()
          .try_into()
          .unwrap()
      );

      continue;
    }

    let range: Vec<usize> = line
      .split("-")
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|x: &str| x.parse().unwrap())
      .collect::<Vec<usize>>();

    fresh_ranges.push(range);
  }

  for ingredient in &ingredients {
    for range in &fresh_ranges {
      if (range.get(0).unwrap()..=&(range.get(1).unwrap())).contains(&ingredient) {
        available_ingredients += 1;
        break;
      }
    }
  }

  println!("{}", available_ingredients);
}

fn part_2() {
  let input = read_lines("src/day_5/input.txt").unwrap();

  let mut all_possible_ranges: Vec<Vec<usize>> = Vec::new();
  let mut number_of_fresh_ids: usize = 0;

  for line in input {
    if line.is_empty() {
      break;
    }

    let mut range: Vec<usize> = line
      .split("-")
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|x: &str| x.parse().unwrap())
      .collect::<Vec<usize>>();

    for possible_range in all_possible_ranges.clone() {
      if (possible_range.get(0).unwrap()..=&(possible_range.get(1).unwrap())).contains(&range.get(0).unwrap()) {
        range[0] = *possible_range.get(1).unwrap()+1;
      }

      if (possible_range.get(0).unwrap()..=&(possible_range.get(1).unwrap())).contains(&range.get(1).unwrap()) {
        range[1] = *possible_range.get(0).unwrap()-1;
      }

      if (range.get(0).unwrap()..=&(range.get(1).unwrap())).contains(&possible_range.get(0).unwrap()) &&
         (range.get(0).unwrap()..=&(range.get(1).unwrap())).contains(&possible_range.get(1).unwrap()) {
          all_possible_ranges.retain(|x| *x != possible_range)
      }
    }

    if range.get(0).unwrap() <= range.get(1).unwrap() {
      all_possible_ranges.push(range);
    }
  }

  for range in &all_possible_ranges {
    number_of_fresh_ids += range.get(1).unwrap() - range.get(0).unwrap()+1;
  }

  println!("{}", number_of_fresh_ids);
}

pub fn day_5() {
  part_1();
  part_2();
}
