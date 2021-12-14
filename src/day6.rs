use crate::utils::read_file;

pub fn run(extra: bool, test: bool) -> String {
  let input = read_file("6", test);

  format!("{}",
    match extra {
      false => p1::run(School::from_input(input)),
      true => p2::run(parse_line(&input[0]))
    }
  )
}

pub struct School {
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

  fn from_input(input: Vec<String>) -> School {
    let lifetimes = parse_line(&input[0]);

    let mut school = School::new();
    for lifetime in lifetimes {
      school.fishes.push(Fish::new(lifetime));
    }
    school
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Fish {
  rem: i32
}

impl Fish {
  fn new(rem: i32) -> Self {
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

fn parse_line(line: &str) -> Vec<i32> {
  line.split(',').map(|s| s.parse::<i32>().unwrap()).collect()
}

mod p2 {
  use std::collections::VecDeque;

  pub fn run(input: Vec<i32>) -> i64 {
    let mut pipeline = VecDeque::from(vec![0_i64; 9]);
    for i in input {
      pipeline[i as usize] += 1;
    }

    for _ in 0..256 {
      let val = pipeline.pop_front().unwrap();
      pipeline.push_back(val);
      pipeline[6] += pipeline[8];
    }

    pipeline.iter().sum::<i64>()
  }
}

mod p1 {
  use super::School;

  // Naive way. Finishes near instantly
  pub fn run(mut school: School) -> i64{
    for _ in 0..80 {
      school.next_day();
    }

    school.fishes.len() as i64
  }
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "5934");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "26984457539");
  }
}