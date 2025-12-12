use crate::utils::read_lines::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
struct UnionFind {
  parent: Vec<usize>,
}

impl UnionFind {
  fn new(size: usize) -> Self {
    Self {
      parent: (0..size).collect(),
    }
  }

  fn find(&mut self, i: usize) -> usize {
    if self.parent[i] != i {
      self.parent[i] = self.find(self.parent[i]);
    }

    self.parent[i]
  }

  fn union(&mut self, i: usize, j: usize) {
    let root_i = self.find(i);
    let root_j = self.find(j);

    if root_i != root_j {
      self.parent[root_i] = root_j;
    }
  }
}

fn part_1() {
  let input = read_lines("src/day_8/input.txt").unwrap();

  let mut boxes: Vec<(u32, u32, u32)> = Vec::new();
  let mut distances: HashMap<((u32, u32, u32), (u32, u32, u32)), f64> = HashMap::new();

  for line in &input {
    let nums: Vec<u32> = line
      .split(',')
      .map(|s| s.trim().parse().unwrap())
      .collect();

    boxes.push((nums[0], nums[1], nums[2]));
  }

  for (outside_box_index, outside_box) in boxes.clone().into_iter().enumerate() {
    for inside_box in boxes.clone().into_iter().skip(outside_box_index) {
      if outside_box == inside_box {
        continue;
      }

      let distance: f64 = (((outside_box.0 as i64 - inside_box.0 as i64).pow(2) + (outside_box.1 as i64 - inside_box.1 as i64).pow(2) + (outside_box.2 as i64 - inside_box.2 as i64).pow(2)) as f64).sqrt();

      distances.insert((outside_box, inside_box), distance);
    }
  }

  let mut sorted_distances: Vec<_> = distances.iter().collect();
  sorted_distances.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

  let smallest_1000_distances = sorted_distances.into_iter().take(10).collect::<Vec<_>>();
  
  let mut union_find: UnionFind = UnionFind::new(boxes.len());

  for ((first_box, second_box), _) in smallest_1000_distances {
    let first_box_index: usize = boxes.iter().position(|x| x == first_box).unwrap();
    let second_box_index: usize = boxes.iter().position(|x| x == second_box).unwrap();

    union_find.union(first_box_index, second_box_index);
  }

  let mut circuits_sizes: HashMap<usize, usize> = HashMap::new();
  for i in 0..boxes.len() {
    let root = union_find.find(i);

    *circuits_sizes.entry(root).or_insert(0) += 1;
  }

  let mut sorted_circuits_sizes: Vec<_> = circuits_sizes.iter().collect();
  sorted_circuits_sizes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

  let mut answer: usize = 1;

  for circuit in sorted_circuits_sizes.iter().take(3) {
    answer *= circuit.1;
  }

  println!("{}", answer);
}

fn part_2() {
  let input = read_lines("src/day_8/input.txt").unwrap();

  let mut boxes: Vec<(u32, u32, u32)> = Vec::new();
  let mut distances: HashMap<((u32, u32, u32), (u32, u32, u32)), f64> = HashMap::new();

  for line in &input {
    let nums: Vec<u32> = line
      .split(',')
      .map(|s| s.trim().parse().unwrap())
      .collect();

    boxes.push((nums[0], nums[1], nums[2]));
  }

  for (outside_box_index, outside_box) in boxes.clone().into_iter().enumerate() {
    for inside_box in boxes.clone().into_iter().skip(outside_box_index) {
      if outside_box == inside_box {
        continue;
      }

      let distance: f64 = ((outside_box.0 as i64 - inside_box.0 as i64).pow(2) + (outside_box.1 as i64 - inside_box.1 as i64).pow(2) + (outside_box.2 as i64 - inside_box.2 as i64).pow(2)) as f64;

      distances.insert((outside_box, inside_box), distance);
    }
  }

  let mut sorted_distances: Vec<_> = distances.iter().collect();
  sorted_distances.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

  let mut union_find: UnionFind = UnionFind::new(boxes.len());
  
  let mut answer: usize = 0;

  for ((first_box, second_box), _) in sorted_distances {
    let first_box_index: usize = boxes.iter().position(|x| x == first_box).unwrap();
    let second_box_index: usize = boxes.iter().position(|x| x == second_box).unwrap();

    if union_find.find(first_box_index) == union_find.find(second_box_index) {
      continue;
    }

    union_find.union(first_box_index, second_box_index);

    answer = first_box.0 as usize * second_box.0 as usize;
  }

  println!("{}", answer);
}

pub fn day_8() {
  part_1();
  part_2();
}
