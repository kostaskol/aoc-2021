use crate::utils;

pub fn run(extra: bool) -> String {
  let input = utils::read_lines("inputs/2.txt");
  let directions = Direction::parse(&input);

  format!("{}",
    match extra {
      true => run_two_stars(&directions),
      false => run_one_star(&directions)
    }
  )
}

enum Direction {
  Up(u32),
  Forward(u32),
  Down(u32)
}

impl Direction {
  fn parse(input: &Vec<String>) -> Vec<Self> {
    let mut directions: Vec<Self> = Vec::new();

    for line in input {
      let split_line: Vec<&str> = line.split(" ").collect();
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

struct Position {
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

// Calculate final distance after following
// directions (up, down, forward)
fn run_one_star(directions: &Vec<Direction>) -> i32 {
  let mut position = Position::new();

  for direction in directions {
    match direction {
      Direction::Up(distance) => {
        position.depth -= *distance as i32;
      },
      Direction::Forward(distance) => {
        position.hor += *distance as i32;
      },
      Direction::Down(distance) => {
        position.depth += *distance as i32;
      }
    }
  }

  position.hor * position.depth
}

// Calculate final distance after following
// directions (up -- main, down -- aim, forward -- horizontal & vertical position)
fn run_two_stars(directions: &Vec<Direction>) -> i32 {
  let mut position = Position::new();

  for direction in directions {
    match direction {
      Direction::Up(distance) => {
        position.aim -= *distance as i32;
      },
      Direction::Down(distance) => {
        position.aim += *distance as i32;
      },
      Direction::Forward(distance) => {
        position.hor += *distance as i32;
        position.depth += *distance as i32 * position.aim;
      }
    }
  }

  position.hor * position.depth
}
