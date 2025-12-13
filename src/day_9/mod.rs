use crate::utils::read_lines::read_lines;

trait VecToTupleExt {
  fn to_tuple(&self) -> (usize, usize);
}

impl VecToTupleExt for Vec<&str> {
  fn to_tuple(&self) -> (usize, usize) {
    (self[0].parse::<usize>().unwrap(), self[1].parse::<usize>().unwrap())
  }
}

fn part_1() {
  let input = read_lines("src/day_9/input.txt").unwrap();

  let mut largest_area: usize = 0;

  for (n, line) in input.iter().enumerate() {
    for line_inside in input.iter().skip(n+1) {

      let x_first: usize = line
        .split(",")
        .collect::<Vec<&str>>()[0]
        .parse::<usize>()
        .unwrap();

      let y_first: usize = line
        .split(",")
        .collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();

      let x_second: usize = line_inside
        .split(",")
        .collect::<Vec<&str>>()[0]
        .parse::<usize>()
        .unwrap();

      let y_second: usize = line_inside
        .split(",")
        .collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();

      let area: usize = (((x_first as isize -x_second as isize).abs() + 1) * ((y_first as isize-y_second as isize).abs() + 1)).try_into().unwrap();

      if area > largest_area {
        largest_area = area;
      }
    }
  }

  println!("{}", largest_area);
}

fn part_2() {
  let input = read_lines("src/day_9/input.txt").unwrap();

  let mut poly_corners: Vec<(usize, usize)> = Vec::new();
  let mut possible_boxes: Vec<([(usize, usize); 4], usize)> = Vec::new();
  let mut poly_borders: Vec<((usize, usize), (usize, usize))> = Vec::new();

  let mut x_len: usize = 0;
  let mut y_len: usize = 0;

  for (n, line) in input.iter().enumerate() {
    for line_inside in input.iter().skip(n+1) {

      let a: (usize, usize) = line
        .split(",")
        .collect::<Vec<&str>>()
        .to_tuple();

      let c: (usize, usize) = line_inside
        .split(",")
        .collect::<Vec<&str>>()
        .to_tuple();

      let b: (usize, usize) = (a.0, c.1);

      let d: (usize, usize) = (c.0, a.1);

      poly_corners.push(a);
      poly_corners.push(b);

      if a.0 > x_len {
        x_len = a.0;
      }

      if c.0 > x_len {
        x_len = c.0;
      }

      if a.1 > y_len {
        y_len = a.1;
      }

      if c.1 > y_len {
        y_len = c.1;
      }

      let box_area: usize = (((a.0 as isize - c.0 as isize).abs() + 1) * ((a.1 as isize - c.1 as isize).abs() + 1)).try_into().unwrap(); 

      possible_boxes.push(([a, b, c, d], box_area));

      if a.0 == c.0 || a.1 == c.1 {
        poly_borders.push((a, c));
      }
    }
  }

  possible_boxes.sort_by(|a, b| b.1.cmp(&a.1));
  
  let mut largest_area: usize = 0;

  for corners in &possible_boxes {
    let mut is_outside_poly: bool = false;

    for corner in corners.0 {
      let mut on_border = false;

      for border in &poly_borders {
        let (p1, p2) = border;

        if p1.0 == p2.0 && p1.0 == corner.0 {
          let min_y = p1.1.min(p2.1);
          let max_y = p1.1.max(p2.1);

          if corner.1 >= min_y && corner.1 <= max_y {
            on_border = true;
            break;
          }
        } else if p1.1 == p2.1 && p1.1 == corner.1 {
          let min_x = p1.0.min(p2.0);
          let max_x = p1.0.max(p2.0);

          if corner.0 >= min_x && corner.0 <= max_x {
            on_border = true;
            break;
          }
        }
      }
      
      if on_border {
        continue;
      }

      let mut intersections: Vec<usize> = Vec::new();
      
      for border in &poly_borders {
        let (p1, p2) = border;
        
        if p1.0 == p2.0 {
          let min_y = p1.1.min(p2.1);
          let max_y = p1.1.max(p2.1);
          
          if corner.1 >= min_y && corner.1 < max_y {
            if p1.0 > corner.0 {
              intersections.push(p1.0);
            }
          }
        }
      }
      
      intersections.sort_unstable();
      intersections.dedup();
      
      if intersections.len() % 2 == 0 {
        is_outside_poly = true;
        break;
      }
    }

    if is_outside_poly {
      continue;
    }

    let min_x = corners.0[0].0.min(corners.0[2].0);
    let max_x = corners.0[0].0.max(corners.0[2].0);
    let min_y = corners.0[0].1.min(corners.0[2].1);
    let max_y = corners.0[0].1.max(corners.0[2].1);

    for border in &poly_borders {
      let (p1, p2) = border;
      
      if p1.0 == p2.0 {
        let b_x = p1.0;
        let b_min_y = p1.1.min(p2.1);
        let b_max_y = p1.1.max(p2.1);

        if b_x > min_x && b_x < max_x {
          if min_y.max(b_min_y) < max_y.min(b_max_y) {
            is_outside_poly = true;
            break;
          }
        }
      } else if p1.1 == p2.1 {
        let b_y = p1.1;
        let b_min_x = p1.0.min(p2.0);
        let b_max_x = p1.0.max(p2.0);

        if b_y > min_y && b_y < max_y {
          if min_x.max(b_min_x) < max_x.min(b_max_x) {
            is_outside_poly = true;
            break;
          }
        }
      }
    }

    if is_outside_poly {
      continue;
    }

    largest_area = corners.1;
    break;
  }

  println!("{:?}", largest_area);
}

pub fn day_9() {
  part_1();
  part_2();
}