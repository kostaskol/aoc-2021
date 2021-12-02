use crate::utils;

enum Direction {
  Up(u32),
  Forward(u32),
  Down(u32)
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

pub fn run(extra: bool) {
  let input = utils::read_lines("inputs/2.txt");
  let directions = parse_directions(&input);

  if extra {
    run_two_stars(directions);
  } else {
    run_one_star(directions);
  }
}

// Calculate final distance after following
// directions (up, down, forward)
fn run_one_star(directions: Vec<Direction>) {
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

  println!("{}", position.hor * position.depth);
}

// Calculate final distance after following
// directions (up -- main, down -- aim, forward -- horizontal & vertical position)
fn run_two_stars(directions: Vec<Direction>) {
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

  println!("{}", position.hor * position.depth);
}

fn parse_directions(input: &Vec<String>) -> Vec<Direction> {
  let mut directions: Vec<Direction> = Vec::new();

  for line in input {
    let split_line: Vec<&str> = line.split(" ").collect();
    let dir = split_line[0];
    let distance = split_line[1].parse::<u32>().unwrap();

    directions.push(
      match dir {
        "up" => {
          Direction::Up(distance)
        },
        "forward" => {
          Direction::Forward(distance)
        },
        "down" => {
          Direction::Down(distance)
        },
        &_ => {
          panic!("Unknown direction: {}", dir);
        }
      }
    );
  }

  directions
}