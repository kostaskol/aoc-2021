use crate::utils::parsing::read_file;

pub fn run(extra: bool, test: bool) -> String {
  let input = read_file("2", test);
  let directions = Direction::parse(input);

  format!("{}",
    match extra {
      false => p1::run(directions),
      true => p2::run(directions)
    }
  )
}

pub enum Direction {
  Up(u32),
  Forward(u32),
  Down(u32)
}

impl Direction {
  fn parse(input: Vec<String>) -> Vec<Self> {
    let mut directions: Vec<Self> = Vec::new();

    for line in input {
      let split_line: Vec<&str> = line.split_whitespace().collect();
      let dir = split_line[0];
      let distance = split_line[1].parse::<u32>().unwrap();

      directions.push(
        match dir {
          "up" => Self::Up(distance),
          "forward" => Self::Forward(distance),
          "down" => Self::Down(distance),
          &_ => panic!("Unknown direction: {}", dir)
        }
      );
    }

    directions
  }
}

pub struct Position {
  hor: i32,
  depth: i32,
  aim: i32
}

impl Position {
  fn new() -> Self {
    Self {
      hor: 0,
      depth: 0,
      aim: 0
    }
  }
}

mod p1 {
  use super::{Direction, Position};

  // Calculate final distance after following
  // directions (up, down, forward)
  pub fn run(directions: Vec<Direction>) -> i32 {
    let mut position = Position::new();

    for direction in directions {
      match direction {
        Direction::Up(distance) => {
          position.depth -= distance as i32;
        },
        Direction::Forward(distance) => {
          position.hor += distance as i32;
        },
        Direction::Down(distance) => {
          position.depth += distance as i32;
        }
      }
    }

    position.hor * position.depth
  }
}

mod p2 {
  use super::{Position, Direction};

  // Calculate final distance after following
  // directions (up -- main, down -- aim, forward -- horizontal & vertical position)
  pub fn run(directions: Vec<Direction>) -> i32 {
    let mut position = Position::new();

    for direction in directions {
      match direction {
        Direction::Up(distance) => {
          position.aim -= distance as i32;
        },
        Direction::Down(distance) => {
          position.aim += distance as i32;
        },
        Direction::Forward(distance) => {
          position.hor += distance as i32;
          position.depth += distance as i32 * position.aim;
        }
      }
    }

    position.hor * position.depth
  }
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "150");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "900");
  }
}