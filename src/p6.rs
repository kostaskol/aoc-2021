use std::collections::VecDeque;

use crate::utils;

struct School {
  fishes: Vec<Fish>
}

impl School {
  fn new() -> School {
    School { fishes: Vec::new() }
  }

  fn next_day(&mut self) {
    let mut new_fishes = Vec::new();

    for fish in self.fishes.iter_mut() {
      if fish.next_day() {
        new_fishes.push(Fish::newborn());
      }
      new_fishes.push(*fish);
    }

    self.fishes = new_fishes;
  }

  fn from_input(input: &Vec<String>) -> School {
    let lifetimes = parse_line(&input[0]);

    let mut school = School::new();
    for lifetime in lifetimes {
      school.fishes.push(Fish::new(lifetime));
    }
    school
  }
}

#[derive(Debug, Copy, Clone)]
struct Fish {
  rem: i64
}

impl Fish {
  fn new(rem: i64) -> Self {
    Self { rem }
  }

  fn newborn() -> Self {
    Self { rem: 8 }
  }

  fn next_day(&mut self) -> bool {
    self.rem -= 1;
    if self.rem < 0 {
      self.rem = 6;
      return true;
    }
    false
  }
}

pub fn run(extra: bool) {
  let input = utils::read_lines("inputs/6.txt");
  let mut school = School::from_input(&input);
  let input = parse_line_i32(&input[0]);

  if extra {
    run_two_star(&input);
  } else {
    // run_one_star(&mut school);
    run_one_star(&mut school);
  }
}

fn run_two_star(input: &Vec<i32>) { 
  let mut pipeline = VecDeque::from(vec![0 as i64; 9]);
  for &i in input {
    pipeline[i as usize] += 1;
  }

  for _ in 0..256 {
    let val = pipeline.pop_front().unwrap();
    pipeline.push_back(val);
    pipeline[6] += pipeline[8];
  }

  println!("{}", pipeline.iter().sum::<i64>());
}

fn run_one_star(school: &mut School) {
  for _ in 0..80 {
    school.next_day();
  }

  println!("{}", school.fishes.len());
}

fn parse_line_i32(line: &str) -> Vec<i32> {
  line.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
  line.split(",").map(|s| s.parse::<i64>().unwrap()).collect()
}